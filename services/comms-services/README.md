# NXTRX4 Communications Service

This service owns the spacecraft communication path between the OBC, two
Needronix NXTRX4 radios, and the rest of the KubOS services running on the
satellite.

The service uses one process for both radios:

- the uplink radio is polled over I2C for inbound CSP frames from RF
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

## Startup

Startup happens in `src/main.rs`:

1. Load the `comms-services` section from the KubOS config.
2. Start the global libcsp router and ping service.
3. Bind two CSP listeners:
   - packet uplink listener
   - SFP uplink listener
4. Register the configured I2C buses as CSP interfaces.
5. Install CSP routes for the uplink radio, downlink radio, and ground node.
6. Start `kubos-comms`, passing it read/write callbacks backed by NXTRX4/CSP.
7. Start this service's GraphQL API for telemetry and radio health.

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
  -> OBC I2C poll worker injects CSP frame into libcsp
  -> CspListener receives one CSP packet
  -> CSP payload is parsed as one SpacePacket
```

SFP-port uplink:

```text
ground
  -> CSP SFP transfer to obc_node:uplink_sfp_csp_port
  -> NXTRX4 uplink radio
  -> OBC I2C poll worker injects CSP frames into libcsp
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

This GraphQL API is for observing the comms service itself. It is separate from
the GraphQL payloads being forwarded through the radio link.

## Current Limitations

- AES-128 uplink encryption is not implemented yet. `uplink_crypto` must remain
  `"none"`.
- The I2C receive path currently polls a configured fixed frame length. Validate
  this against the NXTRX4 behavior on hardware; if the radio returns
  variable-length frames, add a length/status read before injecting frames into
  libcsp.
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

