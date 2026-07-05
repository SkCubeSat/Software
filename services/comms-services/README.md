# NXTRX4 Communications Service

This service owns the spacecraft communication path between the OBC, two
Needronix NXTRX4 radios, and the rest of the KubOS services running on the
satellite.

The service uses one process for both radios:

- the uplink radio returns inbound RF frames by writing CSP-over-I2C frames to
  the OBC's I2C slave address
- the downlink radio is used for outbound CSP frames to the ground station
- both radios are still exposed for basic health queries through this service's
  GraphQL API

## Packet Stack

The on-wire logical stack is:

```text
GraphQL JSON body or UDP bytes
  -> KubOS SpacePacket
  -> CSP packet or CSP SFP transfer
  -> NXTRX4 radio / RF link
```

The layers have different responsibilities:

- CSP routes traffic between nodes and ports, such as ground, OBC, and radios.
- CSP SFP handles fragmentation and reassembly when a logical payload is too
  large for one CSP packet.
- The KubOS SpacePacket is the inner dispatch envelope. It says whether the
  payload is GraphQL or UDP and which local service port should receive it.
- The GraphQL payload is only the HTTP request body, not a full HTTP request.
  The comms service creates the local HTTP POST after the packet is received.

SpacePacket sequencing is not used for fragmentation. SpacePackets are marked
as unsegmented, and segmented SpacePackets are rejected. Large transfers must
use the SFP CSP port described below.

## CSP Ports

The service uses explicit CSP ports instead of auto-detecting packet format.
This keeps the ground/OBC contract simple and avoids ambiguity while receiving
multi-packet SFP transfers.

Default sample config:

```toml
[comms-services.csp]
obc_node = 1
uplink_packet_csp_port = 10
uplink_sfp_csp_port = 12
ground_node = 2
ground_packet_csp_port = 11
ground_sfp_csp_port = 13
```

Meaning:

- `uplink_packet_csp_port`: ground sends one CSP packet containing one complete
  SpacePacket.
- `uplink_sfp_csp_port`: ground sends one logical SpacePacket fragmented with
  CSP SFP.
- `ground_packet_csp_port`: service sends small downlink responses to this
  ground CSP port as one CSP packet.
- `ground_sfp_csp_port`: service sends larger downlink responses to this ground
  CSP port using CSP SFP.

On uplink, the ground side chooses the port based on message size. On downlink,
the service chooses automatically: if the serialized SpacePacket fits in one CSP
packet, it uses the packet port; otherwise it uses the SFP port.

## Size Limits

With the sample config:

```toml
max_frame_bytes = 260
sfp_mtu = 240
sfp_max_space_packet_bytes = 65541
```

Normal CSP packet mode allows one serialized SpacePacket of about 256 bytes.
After the 16-byte SpacePacket header, that leaves about 240 bytes for the
GraphQL JSON body or UDP payload.

SFP mode allows a much larger logical SpacePacket. The sample limit is 65541
bytes, which allows about 65525 bytes of GraphQL JSON body or UDP payload after
the SpacePacket header. `sfp_mtu` is the per-fragment payload size used by SFP;
the default leaves room in the CSP buffer for SFP and RDP headers.

`max_frame_bytes` is also the buffer size used when reading one CSP frame from
the I2C slave frame-queue backend. It only needs to hold one CSP frame, not an
entire SFP transfer.

## I2C Slave Receive Backend

The NXTRX4 CSP-over-I2C interface is multi-master. The OBC writes commands to
the radio as I2C master, but the radio returns received RF frames by becoming
I2C master and writing to the OBC's configured I2C slave address.

The comms service therefore expects an external kernel/BSP backend to expose
those slave writes as a frame stream. The backend contract is:

```text
one I2C master-write transaction from the radio
  -> one read() from slave_rx_device
  -> one raw CSP-over-I2C frame, including CSP header bytes
```

The backend must not parse CSP, add length prefixes, pad frames, split one I2C
write across multiple reads, or combine multiple writes into one read. CSP and
CSP SFP reassembly stay in libcsp after the frame is injected.

This repo includes reference kernel patches:

```text
../radsat-linux/common/patches/linux/0003-i2c-omap-add-slave-support-linux-4.4.patch
../radsat-linux/common/patches/linux/0004-i2c-add-slave-frameq-backend.patch
```

Apply the OMAP bus-driver patch first, then the frame queue backend patch. The
kernel must report `I2C_FUNC_SLAVE: yes` on the NXTRX4 bus before the backend
can be used. Example backend setup for bus 1 and OBC slave address `0x01`:

```sh
modprobe i2c-slave-frameq max_frame_bytes=260 queue_depth=32
echo slave-frameq 0x1001 > /sys/bus/i2c/devices/i2c-1/new_device
```

The service config then points the uplink radio at the resulting device:

```toml
[comms-services.radios.uplink]
bus = "/dev/i2c-1"
csp_node = 8
i2c_addr = 8
slave_rx_device = "/dev/i2c-slave-frameq-1-01"
```

## Startup

Startup happens in `src/main.rs`:

1. Load the `comms-services` section from the KubOS config.
2. Start the global libcsp router and ping service.
3. Bind two CSP listeners:
   - packet uplink listener
   - SFP uplink listener
4. Register the configured I2C buses as CSP interfaces.
5. Install CSP routes for the uplink radio, downlink radio, and ground node.
6. Open the configured uplink `slave_rx_device` and start the frame injection
   worker.
7. Start `kubos-comms`, passing it read/write callbacks backed by NXTRX4/CSP.
8. Start this service's GraphQL API for telemetry and radio health.

## Routing

`src/csp_interface.rs` builds the OBC-side CSP route table.

The important route is:

```text
ground_node -> downlink radio I2C address
```

That means any CSP packet addressed to `ground_node` is sent out through the
downlink NXTRX4. This is separate from KubOS service routing. KubOS service
routing happens inside the SpacePacket using its destination port.

## Uplink Flow

Packet-port uplink:

```text
ground
  -> CSP packet to obc_node:uplink_packet_csp_port
  -> NXTRX4 uplink radio
  -> NXTRX4 writes one CSP frame to the OBC I2C slave address
  -> frameq backend returns one frame to the service
  -> service injects the CSP frame into libcsp
  -> CspListener receives one CSP packet
  -> CSP payload is parsed as one SpacePacket
```

SFP-port uplink:

```text
ground
  -> CSP SFP transfer to obc_node:uplink_sfp_csp_port
  -> NXTRX4 uplink radio
  -> NXTRX4 writes each CSP/SFP fragment as a CSP frame
  -> frameq backend returns one frame per I2C transaction
  -> service injects each CSP frame into libcsp
  -> CspListener reads multiple CSP packets on one connection
  -> SFP chunks are reassembled into one SpacePacket
```

After either path, `kubos-comms` parses the SpacePacket:

- payload type `GraphQL`: spawn a message handler thread
- payload type `UDP`: send the payload as a UDP datagram to the destination port

For GraphQL, the SpacePacket destination is interpreted as a local service port.
The handler posts to:

```text
http://<comms.ip>:<spacepacket.destination>/
```

The HTTP body is the SpacePacket payload bytes.

## Downlink Flow

GraphQL response path:

```text
local KubOS service
  -> HTTP response body
  -> message handler wraps body in a SpacePacket
  -> NXTRX4 write callback sends it to ground_node
  -> libcsp route table chooses the downlink radio
  -> downlink NXTRX4 transmits to ground
```

Small responses are sent as one CSP packet to `ground_packet_csp_port`. Larger
responses are sent with SFP to `ground_sfp_csp_port`.

Spacecraft-initiated downlinks use the normal KubOS comms endpoint behavior:
local mission software sends UDP to one of the configured `downlink_ports`; the
framework wraps that payload in a UDP SpacePacket and then uses the same downlink
write path.

## GraphQL Health API

The service exposes its own GraphQL endpoint at `[comms-services.addr]`.

Useful queries include:

- `telemetry`: packet counters and comms errors
- `health`: configured CSP nodes, ports, SFP settings, and max packet sizes
- `radioHealth(role: UPLINK | DOWNLINK)`: basic NXTRX4 uptime, radio status, and
  radio interface counters
- `radioPing(role: UPLINK | DOWNLINK, payloadSize: 0)`: CSP ping round-trip to a
  radio
- `radioUptime(role: UPLINK | DOWNLINK)`: radio uptime in seconds
- `radioStatus(role: UPLINK | DOWNLINK)`: free space in the radio TX input
  buffer
- `radioIdent(role: UPLINK | DOWNLINK)`: CMP identity strings
- `radioInterfaceStats(role: UPLINK | DOWNLINK, interface: RADIO | CSP | I2C0 |
  I2C2 | RS485)`: CMP interface counters
- `radioSystemStats(role: UPLINK | DOWNLINK)`: decoded NXTRX4 system, reset, and
  ADC statistics

This GraphQL API is for observing the comms service itself. It is separate from
the GraphQL payloads being forwarded through the radio link.

Testing mutations include:

- `radioSendTextInMorse(role, sourceIdentification, text)`
- `radioSendCompressedMorse(role, sourceIdentification, num1, num2, num3, num4,
  num5, num6)`
- `radioSendAx25Message(role, data, format: TEXT | HEX)`
- `radioReboot(role)`

`sourceIdentification` must be exactly four ASCII bytes. AX.25 `TEXT` payloads
are sent as the UTF-8 bytes of `data`; `HEX` payloads accept separators such as
spaces, dashes, underscores, or colons.

### NMP GraphQL API

Every implemented public `nxtrx4-api::nmp` operation is also exposed. Read
operations are GraphQL queries named `radioNmpGet...`; configuration and action
operations are mutations named `radioNmpSet...` or after the underlying action,
such as `radioNmpUnlock`, `radioNmpClearRouteTable`, and
`radioNmpCopyActiveToFactory`. All take `role` and the NMP `key`.

The API covers CSP addressing and routes, RF/link configuration, interface
timing, digipeater and Morse configuration, key and profile management, and all
implemented housekeeping commands including Config 1/2. Fixed-size byte fields
accept `format: TEXT | HEX` and byte-valued responses include both `text` and
lossless `hex` fields. Route tables and both configuration blocks are returned
as structured GraphQL objects.

The crate's RS-485 NMP module currently defines only its command enum and states
that the service is not implemented, so there is no callable RS-485 baud-rate
operation to expose yet.

Example NMP query and mutation:

```graphql
query {
  radioNmpGetFrequency(role: DOWNLINK, key: 0)
}

mutation {
  radioNmpSetTxEnable(role: DOWNLINK, key: 0, enabled: true)
}
```

NMP writes must still follow the radio protocol: call `radioNmpUnlock` with a
key having the required privilege before protected writes.

Example query:

```graphql
{
  radioPing(role: UPLINK, payloadSize: 8) {
    role
    payloadSize
    roundTripMs
  }
  radioIdent(role: UPLINK) {
    hostname
    model
    revision
    buildDate
    buildTime
  }
}
```

Example mutation:

```graphql
mutation {
  radioSendAx25Message(role: DOWNLINK, data: "43 51 20 52 41 44 53 41 54", format: HEX) {
    success
    message
    verbalResponseText
    verbalResponseHex
  }
}
```

## Current Limitations

- AES-128 uplink encryption is not implemented yet. `uplink_crypto` must remain
  `"none"`.
- NXTRX4 receive requires a patched kernel/BSP: the OMAP bus driver must support
  I2C slave mode and a frame-queue backend must expose radio master-write
  transactions through `slave_rx_device`.
- SFP behavior has been wired into the service and compiles, but end-to-end SFP
  must be tested with a ground-side CSP/SFP implementation and the real radio
  link.
- The ground side must use the same explicit port contract:
  - normal command/query: `uplink_packet_csp_port`
  - large command/query: `uplink_sfp_csp_port`
  - listen for small responses on `ground_packet_csp_port`
  - listen for large responses on `ground_sfp_csp_port`

## Local Checks

Useful checks while changing this service:

```sh
cargo check -p comms-services
cargo test -p comms-services
cargo test -p kubos-comms spacepacket
cargo test -p radsat-csp
```

## OBC Radio Tests

Hardware-facing GraphQL fixtures and a small radio-control CLI live in:

```text
services/comms-services/obc-tests
```

Use these when you need to test the real NXTRX4 radios, CSP/I2C routing, or the
I2C slave receive backend on the OBC. The folder includes request JSON files,
`run.sh` for smoke tests and one-off radio commands, and `package.sh` for
creating a transfer-ready bundle. See
`services/comms-services/obc-tests/README.md` for the full transfer and command
workflow.
