# FRAM Service

`fram-service` owns access to the 64-Kbit I2C F-RAM used for mission-critical
persistent state. It stores typed flags in a fixed-slot redundant layout and
mirrors selected values to U-Boot environment variables through `fw_printenv`
and `fw_setenv`.

The service uses a file backend by default so the GraphQL API, record CRCs, and
reconciliation behavior can be tested without hardware:

```toml
[fram-service]
backend = "file"
image_path = "/tmp/fram-service.img"
image_capacity_bytes = 8192
```

Flight hardware should use the I2C FRAM backend:

```toml
[fram-service]
backend = "i2c"
i2c_bus = "/dev/i2c-2"
i2c_addr = "0x50"
capacity_bytes = 8192
address_width_bytes = 2
max_transfer_bytes = 32
```

`i2c_addr` is the Linux 7-bit device address, not the 8-bit address byte shown
in the datasheet bus diagrams. For FM24CL64B, use `0x50` when A2/A1/A0 are all
low. Valid values are `0x50..0x57`; do not configure `0xA0` or `0xA1`.
If `i2cdetect` shows more than one responder in that range, confirm which one is
the FRAM before enabling writes.

On the current board schematic, the FRAM address pins are tied low, so the FRAM
is `0x50`. The RV-3028 RTC responds at `0x52`.

The FRAM is byte-addressable and has NoDelay writes. The backend does not add
EEPROM write-cycle polling or delays, but it still reports I2C errors so callers
can handle an OPD-off power domain. I2C bus speed is configured by the Linux
I2C controller/device tree; the FRAM part supports up to 1 MHz.

Mission applications should query this service rather than opening the I2C
device directly. The intended boot flow is:

1. Set system time from the RTC.
2. Call `reconcileMissionState(dryRun: false)`.
3. Read `missionState`.
4. Run deployment logic from the reconciled state.
5. Set completion flags through `setMissionFlag`.

## OBC hardware tests

Hardware-only tests live under `obc-tests/` so normal host tests never require a
real FRAM chip. To build, package, transfer, and run them:

```sh
services/hardware-services/fram-service/obc-tests/package.sh
transfer -- -d /home/kubos/fram-tests target/obc-tests/fram-service
```

Then on the OBC:

```sh
./run.sh
```

See `obc-tests/README.md` for the full cross-build commands, transfer flow, and
hardware test options.
