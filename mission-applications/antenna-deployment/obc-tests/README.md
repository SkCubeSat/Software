# Antenna Deployment OBC Tests

This directory builds and packages a guarded OBC test harness for the
`antenna-deployment` mission application.

## Build and package

From the repository on the PowerEdge computer:

```sh
mission-applications/antenna-deployment/obc-tests/package.sh
```

The package is written to `target/obc-tests/antenna-deployment`. Transfer it:

```sh
transfer -d /home/kubos/antenna-tests target/obc-tests/antenna-deployment
```

## Run on the OBC

The FRAM service and its KubOS configuration must already be available.
By default, the runner uses:

```text
/home/kubos/fram-tests/fram-service/config/fram-hw.toml
```

Read-only tests are the default:

```sh
cd /home/kubos/antenna-tests
./run.sh
```

On non-flight test hardware, enable state write/readback tests:

```sh
ANTENNA_TEST_STATE_WRITE=1 ./run.sh
```

If the FRAM configuration is elsewhere, override it:

```sh
CONFIG=/path/to/fram-hw.toml ./run.sh
```

Every application invocation includes the equivalent of:

```sh
./antenna-deployment show-state \
  -c /home/kubos/fram-tests/fram-service/config/fram-hw.toml
```

The writable tests save and restore `detumbling_complete` and `deployed`. The
only `run-once` case forces `deployed=true`, which exits before GPIO setup and
before deployment pulses.

Do not extend this runner with an expired deploy timer unless deployment loads
are disconnected or the application has gained a simulated GPIO test mode.

## Overrides

- `TARGET`: Rust target triple (default `armv7-unknown-linux-gnueabihf`)
- `CROSS`: cross-build command (default `cross`)
- `STRIP`: target strip command (default `arm-linux-gnueabihf-strip`)
- `NO_STRIP=1`: skip stripping
- `SKIP_BUILD=1`: package an existing binary
- `APP`: application path on the OBC
- `CONFIG`: KubOS configuration containing the `fram-service` address
