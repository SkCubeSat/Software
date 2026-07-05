# Comms Service OBC Tests

This folder contains GraphQL request fixtures and a shell runner for testing
`comms-services` on the KubOS OBC with the NXTRX4 radios connected. Typed NMP
commands are provided by the separate Rust `comms-cli` crate.

These are hardware-facing tests, so they are intentionally separate from normal
host `cargo test` coverage. Follow the repository testing guide: run local Rust
tests first, then transfer these assets to the OBC when the radio, I2C bus, CSP
routes, or slave receive backend must be exercised.

## Layout

- `requests/*.json` - one GraphQL request per smoke test or radio command.
- `config/comms-hw.toml` - OBC hardware config for the service.
- `run.sh` - OBC service/fixture runner and thin `comms-cli` launcher.
- `build.sh` - cross-builds `comms-services` and `comms-cli` for the OBC target.
- `package.sh` - stages a transfer-ready bundle under `target/obc-tests`.

## Build And Package

From the repository root:

```sh
services/comms-services/obc-tests/package.sh
```

The default package command cross-builds both Rust binaries and writes:

```text
target/obc-tests/comms-services
```

To omit the service binary but retain the NMP CLI:

```sh
INCLUDE_SERVICE_BIN=0 services/comms-services/obc-tests/package.sh
```

To package only request files and scripts:

```sh
INCLUDE_SERVICE_BIN=0 INCLUDE_CLI_BIN=0 services/comms-services/obc-tests/package.sh
```

To keep symbols in both binaries:

```sh
NO_STRIP=1 services/comms-services/obc-tests/package.sh
```

## Transfer

Use the repo transfer helper from the PowerEdge side:

```sh
transfer -- -d /home/kubos/comms-tests target/obc-tests/comms-services
```

`transfer.sh` packs directories as tarballs, sends them over the serial link,
and extracts them under the destination. After the command above, the OBC path
will be:

```text
/home/kubos/comms-tests/comms-services
```

Useful transfer overrides:

```sh
transfer -p /dev/ttyUSB1 -b 921600 -d /home/kubos/comms-tests target/obc-tests/comms-services
```

## Run On The OBC

On the OBC:

```sh
cd /home/kubos/comms-tests/comms-services
./run.sh smoke
```

`START_SERVICE=auto` is the default. The runner first checks whether the GraphQL
endpoint already responds. If it does, the existing service is used. If not, and
`bin/comms-services` exists, the runner starts it with `config/comms-hw.toml`.

To force an already-running service:

```sh
START_SERVICE=0 URL=http://127.0.0.1:8150/graphql ./run.sh smoke
```

To force this package's service binary:

```sh
START_SERVICE=1 ./run.sh smoke
```

## Smoke Sequence

The smoke sequence runs only non-transmitting requests:

- service `ping`, `telemetry`, and `health`
- `radioHealth` for uplink and downlink
- `radioPing`, `radioUptime`, `radioStatus`, `radioIdent`
- `radioInterfaceStats(interface: RADIO)`
- `radioSystemStats`

Run it with:

```sh
./run.sh smoke
```

To POST an individual fixture:

```sh
./run.sh request 10_radio_ping_uplink.json
./run.sh request requests/21_radio_system_stats_downlink.json
```

## Radio-Control CLI

`RadioRole` selects which configured radio to target:

- `UPLINK` targets `[comms-services.radios.uplink]`
- `DOWNLINK` targets `[comms-services.radios.downlink]`

Read-only examples:

```sh
./run.sh ping UPLINK 8
./run.sh uptime DOWNLINK
./run.sh status DOWNLINK
./run.sh ident UPLINK
./run.sh iface DOWNLINK RADIO
./run.sh stats UPLINK
```

RF-transmitting mutation examples:

```sh
./run.sh morse-text DOWNLINK SAT1 CQ
./run.sh morse-compressed DOWNLINK SAT1 0 0 0 0 0 0
./run.sh ax25-text DOWNLINK "CQ RADSAT"
./run.sh ax25-hex DOWNLINK "43 51 20 52 41 44 53 41 54"
```

The `all` sequence does not run RF-transmitting mutation fixtures unless you
opt in:

```sh
RUN_MUTATIONS=1 ./run.sh all
```

Radio reboot is more disruptive and has a separate guard:

```sh
CONFIRM_REBOOT=1 ./run.sh reboot UPLINK
RUN_REBOOT=1 ./run.sh all
```

## NMP CLI

The Clap-based `comms-cli` exposes one subcommand for each of the 49 implemented
public NMP methods. `run.sh nmp` starts or finds the service and delegates to
that binary. The positional order is radio role, decimal NMP key, then command.

Read examples:

```sh
./run.sh nmp DOWNLINK 0 get-frequency
./run.sh nmp UPLINK 0 get-route-table
./run.sh nmp DOWNLINK 0 get-config1
./run.sh nmp DOWNLINK 0 get-fw-crc
```

Protected writes require an unlock first and a key with sufficient privilege:

```sh
./run.sh nmp DOWNLINK 123456 unlock
./run.sh nmp DOWNLINK 123456 set-tx-enable true
./run.sh nmp DOWNLINK 123456 set-preamble-size 100
./run.sh nmp DOWNLINK 123456 set-morse-custom-message "RADSAT TEST"
```

Fixed byte fields default to text and optionally accept hex:

```sh
./run.sh nmp DOWNLINK 123456 set-callsign RADSAT
./run.sh nmp DOWNLINK 123456 set-morse-custom-ident "52 41 44 31" --format hex
./run.sh nmp DOWNLINK 123456 set-itu-key "01 02 03 04 05" --format hex
```

`set-routes` accepts one or more colon-separated route triples:

```sh
./run.sh nmp DOWNLINK 123456 set-routes 2:1:2 8:2:8
```

That example sends routes `{cspAddress: 2, destinationInterface: 1,
nextHop: 2}` and `{cspAddress: 8, destinationInterface: 2, nextHop: 8}` in one
NMP transaction. Run `./run.sh nmp --help` for the complete command list, then
append `--help` to a subcommand for its typed arguments. The packaged CLI can
also be run directly as `bin/comms-cli --url <endpoint> nmp ...`.
Several NMP operations can change RF authorization, access keys, addressing, or
the saved profile; use values approved for the connected test radio.

## Hardware Expectations

The OBC image must have the NXTRX4 I2C slave receive backend configured before
uplink receive or transaction replies can work. The default config expects:

```text
/dev/i2c-slave-frameq-1-01
```

If the bus, CSP node, I2C address, or slave frame queue path differs on a test
setup, edit `config/comms-hw.toml` before running with `START_SERVICE=1`, or run
against an already-started service with the correct config.

## Useful Overrides

```sh
URL=http://127.0.0.1:8150/graphql
START_SERVICE=auto
SERVICE_BIN=/path/to/comms-services
CLI_BIN=/path/to/comms-cli
CONFIG=/path/to/comms-hw.toml
LOG=/tmp/comms-services.log
REQ_DIR=/path/to/requests
RUN_MUTATIONS=1
RUN_REBOOT=1
CONFIRM_REBOOT=1
```
