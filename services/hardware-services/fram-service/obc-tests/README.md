# FRAM OBC Tests

This folder contains tests that are meant to run on the OBC with the real
64-Kbit I2C FRAM connected. These are intentionally separate from the normal
Rust `tests/` directory so `cargo test -p fram-service` remains host-safe.

## Layout

- `config/fram-hw.toml` - hardware config for the service.
- `requests/*.json` - simple GraphQL request fixtures for manual inspection.
- `src/main.rs` - compiled OBC test binary, `fram-obc-tests`.
- `build.sh` - shorthand cross-build command for the service and test binary.
- `package.sh` - builds and stages a transfer-ready test bundle.
- `run.sh` - runs the service, GraphQL smoke requests, and compiled tests on the OBC.

The compiled test binary does two things by default:

1. Writes a 32-byte test pattern to FRAM scratch offset `4096`, reads it back,
   and restores the original bytes.
2. Calls the FRAM GraphQL service for `ping`, `health`, and `missionState`.

Offset `4096` is outside the service's mission-state record area. The mission
record layout currently reserves bytes `0..2048`.

## Build

From the repository root:

```sh
services/hardware-services/fram-service/obc-tests/build.sh
```

The build script strips the release binaries by default using
`arm-linux-gnueabihf-strip` or `llvm-strip` if available. Override the strip tool
or keep symbols with:

```sh
STRIP=/path/to/arm-linux-gnueabihf-strip services/hardware-services/fram-service/obc-tests/build.sh
NO_STRIP=1 services/hardware-services/fram-service/obc-tests/build.sh
```

Equivalent explicit commands:

```sh
cross build \
  --target armv7-unknown-linux-gnueabihf \
  -p fram-service \
  --release \
  --features i2c

cross build \
  --target armv7-unknown-linux-gnueabihf \
  -p fram-service \
  --release \
  --features i2c,obc-tests \
  --bin fram-obc-tests
```

## Package and Transfer

Create a transfer-ready bundle:

```sh
services/hardware-services/fram-service/obc-tests/package.sh
```

The bundle is written to:

```text
target/obc-tests/fram-service
```

Transfer it to the OBC using the repo transfer helper:

```sh
transfer -- -d /home/kubos/fram-tests target/obc-tests/fram-service
```

The transfer helper unpacks directories under the destination, so the OBC path
will be:

```text
/home/kubos/fram-tests/fram-service
```

## Run on the OBC

On the OBC:

```sh
cd /home/kubos/fram-tests/fram-service
./run.sh
```

`run.sh` starts `bin/fram-service` with `config/fram-hw.toml`, waits for the
GraphQL endpoint, runs the request fixtures, then runs `bin/fram-obc-tests`.

If the service is already running:

```sh
START_SERVICE=0 URL=http://127.0.0.1:8091/graphql ./run.sh
```

To use a different I2C bus or address:

```sh
I2C_BUS=/dev/i2c-2 I2C_ADDR=0x50 ./run.sh
```

`I2C_ADDR` is the Linux 7-bit address. For FM24CL64B that is `0x50..0x57`
depending on A2/A1/A0. Do not pass the datasheet's 8-bit address byte
`0xA0`/`0xA1`.

To read-probe the FM24CL64B address range without starting the service or
writing scratch bytes:

```sh
FRAM_TEST_SCAN_ONLY=1 ./run.sh
```

If the scan reports more than one candidate, identify the FRAM from the
schematic or by using a known scratch-write-safe address before changing
`config/fram-hw.toml`.

On the current board schematic, FM24CL64B A0/A1/A2 are tied low, so the FRAM is
`0x50`. The RV-3028 RTC responds at `0x52`; do not run the scratch-write test
against `0x52`.

## Mission Flag Write Test

By default, the compiled test avoids writing mission-state keys. It only writes
to the scratch offset.

To also test the GraphQL mission write path, enable the guarded write/restore
test:

```sh
FRAM_TEST_MISSION_WRITE=1 ./run.sh
```

That test toggles `detumbling_complete` with `mirrorToEnv: false`, verifies the
readback, and restores the original value. Do not enable it during a real flight
configuration unless you intend to touch that mission flag.

To test the U-Boot environment mirror as well:

```sh
FRAM_TEST_ENV_WRITE=1 ./run.sh
```

That test toggles `detumbling_complete` with `mirrorToEnv: true`, verifies the
FRAM state through GraphQL, verifies the U-Boot env value through
`fw_printenv -n detumbling_complete`, then restores the original FRAM and env
values. It writes persistent U-Boot env, so keep it gated.

If `fw_printenv` prints `Cannot open /envar/uboot.env`, the U-Boot env partition
or file is not mounted/present on the current image. Check `/etc/fw_env.config`,
`mount | grep /envar`, and `ls -l /envar/uboot.env` before running
`FRAM_TEST_ENV_WRITE=1`.

## Useful Overrides

```sh
URL=http://127.0.0.1:8091/graphql
SERVICE_BIN=/path/to/fram-service
TEST_BIN=/path/to/fram-obc-tests
CONFIG=/path/to/fram-hw.toml
SCRATCH_OFFSET=4096
FRAM_TEST_MISSION_WRITE=1
FRAM_TEST_ENV_WRITE=1
FRAM_TEST_FW_PRINTENV=/usr/sbin/fw_printenv
FRAM_TEST_FW_SETENV=/usr/sbin/fw_setenv
FRAM_TEST_SCAN_ONLY=1
START_SERVICE=0
```
