# Ground Station Implementation Guide

This document specifies everything the ground-station software needs to talk
to the RADSAT OBC through the NXTRX4 radio link: how commands are packed, how
responses come back, exact byte layouts, size budgets, and the encryption
contract. It is written for whoever implements the ground side, which does not
exist in this repository yet.

The authoritative implementations on the satellite side are:

- `kubos/libs/kubos-comms/src/spacepacket.rs` — SpacePacket pack/parse
- `kubos/libs/radsat-csp/src/lib.rs` — CSP send/receive and SFP reassembly
- `services/comms-services/src/nxtrx_comms.rs` — uplink decryption, port choice
- `services/comms-services/config.toml` — the deployed addressing values

If this document and the code disagree, the code wins; please fix the doc.

## What the Ground Station Must Implement

1. A **CSP v1 stack** (libcsp v1 is strongly recommended over hand-rolling)
   bound to the ground node address, with the radio as its interface.
2. A **SpacePacket packer/parser** (16-byte header + payload, spec below).
3. An optional **AES-128-GCM encryptor** for uplinks, matching the satellite's
   `uplink_crypto` config.
4. **Two listeners** for downlinked responses: one plain-packet CSP port and
   one SFP CSP port.
5. **Command correlation** by `command_id`, with timeout and retry logic,
   because uplinks can be silently dropped (RF loss, decryption failure,
   parse failure).

## Protocol Stack

Uplink (ground → satellite):

```text
GraphQL JSON body or UDP bytes
  -> SpacePacket (16-byte header + payload)
  -> [optional] AES-128-GCM: nonce(12) || ciphertext || tag(16)
  -> one CSP packet (small) or one CSP SFP transfer (large)
  -> RF to the uplink NXTRX4 radio
```

Downlink (satellite → ground) is the same stack without encryption — all
downlinks are cleartext.

## Addressing

These values must mirror the satellite's `config.toml` `[comms-services.csp]`
section. Defaults:

| setting                 | default | meaning                                      |
|-------------------------|---------|----------------------------------------------|
| `obc_node`              | 1       | CSP address the ground sends commands to     |
| `ground_node`           | 2       | CSP address the ground station binds as      |
| `uplink_packet_csp_port`| 10      | OBC port for single-CSP-packet commands      |
| `uplink_sfp_csp_port`   | 12      | OBC port for SFP (fragmented) commands       |
| `ground_packet_csp_port`| 11      | ground port that receives small downlinks    |
| `ground_sfp_csp_port`   | 13      | ground port that receives SFP downlinks      |

CSP v1 constraints: node addresses are 5 bits (0–31), ports are 6 bits (0–63).
The satellite rejects config values outside those ranges; the ground must obey
them too.

The ground station picks its own ephemeral source port when connecting; the
satellite does not care about it. Responses are **not** sent back on the
uplink connection — they arrive as new CSP connections addressed to
`ground_node:ground_packet_csp_port` or `ground_node:ground_sfp_csp_port`.
Listen on both, always.

## CSP v1 Frame Format

Every CSP packet on the radio link is a 4-byte big-endian header followed by
the payload. Bit layout of the 32-bit header, most significant bit first:

```text
bits 31..30  priority         (2 bits)  use 2 = normal
bits 29..25  source node      (5 bits)
bits 24..20  destination node (5 bits)
bits 19..14  destination port (6 bits)
bits 13..8   source port      (6 bits)
bits 7..0    flags            (8 bits)
```

Flags byte (standard libcsp v1): `0x10` = FFRAG (SFP fragment), `0x02` = RDP,
`0x01` = CRC32, `0x08` = HMAC, `0x04` = XTEA. Plain single-packet commands use
flags `0x00`. The satellite does not use CSP-level HMAC/XTEA/CRC32.

Example: ground node 2 (source port 45) sending to OBC node 1, port 10,
priority normal, no flags:

```text
(2<<30) | (2<<25) | (1<<20) | (10<<14) | (45<<8) | 0x00  =  0x8412AD00
frame = 84 12 AD 00 <payload bytes...>
```

The radio link carries at most `max_frame_bytes` (260) per CSP frame: 4 bytes
of header plus up to 256 bytes of payload (the satellite's `CSP_BUFFER_SIZE`).

## SFP (Small Fragmentation Protocol)

Payloads too large for one CSP packet are sent to `uplink_sfp_csp_port` using
libcsp's SFP. If you use libcsp, this is `csp_sfp_send()` with
`mtu = sfp_mtu` (240) and you can stop reading this section.

On the wire, each SFP fragment is a normal CSP packet on the same connection
with the FFRAG flag (`0x10`) set and an 8-byte big-endian trailer *appended*
to the fragment data:

```text
<chunk bytes (up to mtu)> <offset: u32 BE> <total: u32 BE>
```

- `offset` — byte offset of this chunk within the logical payload
- `total`  — total logical payload length, identical in every fragment

Fragments must arrive in order with contiguous offsets; the satellite aborts
the transfer on any gap, mismatch, or missing FFRAG flag. The satellite waits
up to `sfp_read_timeout_ms` (10 000 ms) between fragments before abandoning a
transfer. Aborted transfers are dropped silently — no NACK.

`sfp_use_rdp = true` in the satellite config means the satellite opens its
**downlink** SFP connections with the RDP option, so the ground SFP listener
must support RDP. For uplink SFP the ground may also use RDP (recommended for
reliability over the RF link).

## SpacePacket Format

The logical command/response unit. All fields big-endian. Total size =
6-byte CCSDS primary header + 10-byte secondary header + payload.

```text
offset  size  field
0       2     version(3) = 0 | packet type(1) = 0 |
              secondary header flag(1) = 1 | payload type(11)
2       2     sequence flags(2) = 0b11 (unsegmented) | sequence count(14) = 0
4       2     data length = 10 + payload length
6       8     command id (u64) - chosen by the ground, echoed in the response
14      2     destination port (u16)
16      n     payload
```

Payload types (in the 11-bit APID field):

| value | type    | uplink meaning                          | downlink meaning              |
|-------|---------|-----------------------------------------|-------------------------------|
| 0     | GraphQL | JSON body POSTed to a local service     | HTTP response body            |
| 1     | UDP     | datagram forwarded to a local UDP port  | spacecraft-initiated datagram |
| 2     | Error   | rejected                                | NACK, UTF-8 message ≤200 B    |

Field semantics:

- **command id** — opaque to the satellite. Use a monotonically increasing
  counter so responses and NACKs can be matched to commands. Downlinks that
  the spacecraft initiates itself (UDP telemetry beacons) carry
  `command_id = 0`, so start your counter at 1.
- **destination port** — on uplink, the TCP port of the target service's
  GraphQL HTTP server on the OBC (for GraphQL) or the target UDP port (for
  UDP passthrough). The comms service POSTs GraphQL payloads to
  `http://<obc-ip>:<destination_port>/`. On downlink this field is always 0.
- **sequence flags** must be `0b11`; the satellite rejects segmented
  SpacePackets. Fragmentation is SFP's job, one layer down.

### Worked Example

GraphQL body `{"query":"{ping}"}` (18 bytes), command id 1, destination port
8150 (the comms service's own GraphQL API):

```text
08 00                      version 0, type 0, sec hdr flag 1, payload type 0
C0 00                      unsegmented, sequence count 0
00 1C                      data length = 10 + 18 = 28
00 00 00 00 00 00 00 01    command id = 1
1F D6                      destination port = 8150
7B 22 71 75 65 72 79 22    {"query"
3A 22 7B 70 69 6E 67 7D    :"{ping}
22 7D                      "}
```

34 bytes total. Unencrypted, this fits in one CSP packet, so it is sent as
the payload of a single CSP frame to `obc_node:uplink_packet_csp_port`.

## Uplink Encryption (AES-128-GCM)

Governed by the satellite's `csp.uplink_crypto` setting. When it is
`"aes-128"`, **every** uplink on both ports must be encrypted with the shared
16-byte key (`csp.uplink_aes_key`, 32 hex chars):

```text
wire payload = nonce (12 bytes) || AES-128-GCM(key, nonce, spacepacket_bytes)
```

The GCM output already ends with the 16-byte tag, so overhead is 28 bytes
total. The plaintext is the **entire serialized SpacePacket**. There is no
additional authenticated data (AAD is empty).

Ground obligations:

- Generate a **unique 96-bit nonce for every packet** — random, or a
  persistent counter. Never reuse a nonce under the same key; that breaks
  both confidentiality and authenticity of GCM completely.
- Expect silence on failure: wrong key, bit corruption, truncation, or
  cleartext-while-enabled are all dropped without a NACK (the satellite will
  not respond to unauthenticated data). Treat as a timeout and retry with a
  fresh nonce.
- There is **no replay protection** at this layer. A recorded uplink replays
  successfully. Do not design ground procedures that rely on the link
  rejecting replays; idempotence or command counters must live in the
  commands themselves.

Downlinks are never encrypted, regardless of this setting.

## Size Budgets

Choose the uplink port by the size of the final wire payload (after
encryption, if enabled). Defaults shown; all derive from `max_frame_bytes`
(260), `sfp_max_space_packet_bytes` (65541), and the 28-byte crypto overhead.

| path                | crypto  | max wire payload | max SpacePacket | max GraphQL/UDP body |
|---------------------|---------|------------------|-----------------|----------------------|
| packet port (10)    | none    | 256              | 256             | 240                  |
| packet port (10)    | aes-128 | 256              | 228             | 212                  |
| SFP port (12)       | none    | 65 541           | 65 541          | 65 525               |
| SFP port (12)       | aes-128 | 65 569           | 65 541          | 65 525               |

Downlink: the satellite applies the same rule in reverse — responses whose
serialized SpacePacket fits in 256 bytes arrive as one CSP packet on ground
port 11; anything larger arrives as an SFP transfer on ground port 13.

## Responses and NACKs

For every GraphQL command the ground should expect exactly one of:

1. **A GraphQL response** — payload type 0, same `command_id`, destination 0,
   payload = the HTTP response body (GraphQL JSON, including any GraphQL-level
   `errors` array).
2. **An Error NACK** — payload type 2, same `command_id`, destination 0,
   payload = UTF-8 error message truncated to 200 bytes. Sent when the target
   service was unreachable or returned an HTTP error, all 50 handler slots
   were busy, the payload type was invalid, or a UDP passthrough send failed.
3. **Nothing** — the uplink was lost, failed decryption, failed SpacePacket
   parsing, or an SFP transfer aborted. Also possible: the response itself was
   lost on the RF downlink. Retry after a timeout.

UDP passthrough uplinks get no positive acknowledgement, only a NACK on
failure. All UDP downlinks (payload type 1, `command_id` 0) — replies from
UDP-based services such as shell-service and file-transfer-service, as well as
spacecraft-initiated datagrams — are not correlated by `command_id`. For the
KubOS shell/file protocols, correlate by the channel ID inside the CBOR
messages: run the standard ground client tools against a local UDP shim that
wraps each outgoing datagram in a SpacePacket (payload type 1,
`destination_port` = the service's UDP port on the OBC) and unwraps downlinked
UDP SpacePackets back into local datagrams.

Timing guidance: the satellite waits at most `comms.timeout` (1500 ms) for the
local service's HTTP response, and at most 50 commands are in flight at once.
A reasonable ground timeout is RF round-trip + ~2–3 s.

## Implementation Checklist

- [ ] CSP v1 stack bound as node 2, radio interface delivering raw 260-byte
      CSP frames
- [ ] Listeners on ground ports 11 (packet) and 13 (SFP, RDP-capable)
- [ ] SpacePacket pack/parse with the exact 16-byte header above
- [ ] Port selection by wire-payload size (≤256 → packet port, else SFP)
- [ ] AES-128-GCM encryption with unique nonces, toggleable to match the
      satellite config
- [ ] `command_id` allocation (monotonic, starting at 1) and correlation of
      responses/NACKs
- [ ] Timeout + retry for silent drops; handling for payload-type-2 NACKs
- [ ] Handler for unsolicited UDP downlinks (`command_id` 0)
- [ ] Key management: the uplink key must never be logged or downlinked

## Interoperability Testing Without Hardware

`kubos/libs/kubos-comms/tests/` shows the satellite-side expectations
(`uplink.rs`, `downlink.rs`, `concurrent.rs` use a mock radio). The
encrypt→decrypt round-trip in
`services/comms-services/src/nxtrx_comms.rs` (`mod tests`) is the reference
for the encryption wire format — a ground implementation should be able to
reproduce its ciphertext byte-for-byte given the same key, nonce, and
plaintext. `radsat-csp`'s tests assert the exact CSP v1 header packing.
