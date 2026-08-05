# Antenna Deployment OBC Tests

This directory builds and packages a guarded OBC test harness for the
`antenna-deployment` mission application.

## Build and package

From the repository on the PowerEdge computer:

```sh
sh mission-applications/antenna-deployment/obc-tests/package.sh
```

`package.sh` explicitly makes both runners and the packaged antenna binary
executable, so no local `chmod` step is normally required before transfer.

The package is written to `target/obc-tests/antenna-deployment`. Transfer it:

```sh
transfer -p /dev/ttyUSB0 -d /home/kubos/antenna-tests target/obc-tests/antenna-deployment
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

## Hardware-test all three failed attempts

This runner checks three real VHF GPIO pulses, three real UHF GPIO pulses, and
the check-only cycle that follows them. It temporarily changes deployment state
in FRAM, then restores the original values. Disconnect flight deployment loads
and keep both sense inputs inactive before running it:

```sh
ANTENNA_HARDWARE_TEST_MODE=three-attempts \
ANTENNA_HARDWARE_TEST_CONFIRM=DEPLOY_3_TIMES \
./run-hardware.sh
```

This uses the real GPIO path and keeps every 90-second wait. If either sense
input becomes active, the application stops retrying that antenna and the test
reports fewer than three attempts.

## Run with deployment hardware

`run-hardware.sh` invokes the application's real `run-once` deployment path.
It uses the stored mission state and retains the normal 30-minute hold timer.
If that timer has elapsed, the application can pulse the VHF and UHF deployment
GPIO outputs and physically fire connected deployment loads.

Only run this on hardware prepared for a deployment test:

```sh
cd /home/kubos/antenna-tests
./run-hardware.sh
```

Review the displayed mission state, then type `DEPLOY` at the confirmation
prompt. For a deliberately non-interactive test, provide the same confirmation
through the environment:

```sh
ANTENNA_HARDWARE_TEST_CONFIRM=DEPLOY ./run-hardware.sh
```

The hardware runner does not alter flags or shorten/bypass the hold timer. Use
the application's manual state commands separately if you are preparing a
controlled deployment test state.

If the bench image does not provide `/envar/uboot.env`, explicitly use
FRAM-only mode:

```sh
ANTENNA_FRAM_ONLY=1 ./run-hardware.sh
```

This skips U-Boot reconciliation and makes state mutations write only to FRAM.
It is intended only for controlled bench testing; omit it for normal or flight
operation so the redundant U-Boot state copy remains enabled.

Prefix manual state commands with the same variable while `/envar/uboot.env`
is unavailable, for example:

```sh
ANTENNA_FRAM_ONLY=1 ./bin/antenna-deployment set-deploy-start 0 \
  -c /home/kubos/fram-tests/fram-service/config/fram-hw.toml --stdout
```

## Overrides

- `TARGET`: Rust target triple (default `armv7-unknown-linux-gnueabihf`)
- `CROSS`: cross-build command (default `cross`)
- `STRIP`: target strip command (default `arm-linux-gnueabihf-strip`)
- `NO_STRIP=1`: skip stripping
- `SKIP_BUILD=1`: package an existing binary
- `APP`: application path on the OBC
- `CONFIG`: KubOS configuration containing the `fram-service` address
- `ANTENNA_HARDWARE_TEST_CONFIRM=DEPLOY`: confirm a non-interactive hardware run
- `ANTENNA_HARDWARE_TEST_MODE=three-attempts`: prepare and check three real attempts
- `ANTENNA_HARDWARE_TEST_CONFIRM=DEPLOY_3_TIMES`: confirm three real hardware attempts
- `ANTENNA_FRAM_ONLY=1`: bench-only mode without U-Boot state mirroring
