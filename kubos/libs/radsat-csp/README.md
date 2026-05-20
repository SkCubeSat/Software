# radsat-csp

Shared Rust integration layer for the CubeSat Space Protocol stack.

This crate builds the sibling `libcsp` C checkout with Cargo and wraps the
University of Stuttgart `libcsp-rust` bindings behind a small API that can be
used by Radsat device APIs and services.

Current milestone:

- One-time CSP stack initialization.
- Router worker thread.
- Reserved-port service worker for CSP ping.
- Client helpers for ping, send, and request/reply transactions.
- Optional Linux I2C CSP interface registration with `--features i2c`.
- Route helpers for sending a CSP node through a specific 7-bit I2C address.
- Loopback smoke test covering ping and an echo transaction.

The crate uses the repository submodules:

- `vendor/libcsp`
- `vendor/libcsp-rust`

After cloning `Software`, initialize the submodules with:

```sh
git submodule update --init --recursive
sh scripts/apply-vendor-patches.sh
```

The vendor patch fixes `libcsp-rust` raw-pointer slice expressions that are
rejected by current Rust compilers under `dangerous_implicit_autorefs`.

The next integration step is to add the hardware transport needed by the target
device, then expose the device-specific commands from a crate under
`kubos/apis`.

## I2C

Build I2C support with:

```sh
cargo build -p radsat-csp --features i2c
```

Basic setup:

```rust,no_run
use std::time::Duration;

use radsat_csp::{
    CspClient, I2cInterfaceConfig, LinuxI2cCspInterface, RouterWorker,
};

fn main() -> radsat_csp::Result<()> {
    let _router = RouterWorker::start();

    let mut i2c = LinuxI2cCspInterface::open(
        I2cInterfaceConfig::new("/dev/i2c-2", 1)
            .with_name("I2C")
            .with_default_route(true),
    )?;

    // Needed only when the CSP node id is not the same as the 7-bit I2C
    // address. This sends CSP node 5 through Linux I2C address 0x12.
    i2c.route_node_via_i2c_addr(5, 0x12)?;

    let csp = CspClient::new().with_timeout(Duration::from_millis(500));
    csp.send(5, 20, &[0x01])?;

    // If the target device exposes a raw CSP frame through an I2C read, feed it
    // into the CSP router. The frame length/read behavior is device-specific.
    i2c.read_frame_from(0x12, 32)?;

    Ok(())
}
```

`LinuxI2cCspInterface` registers `libcsp`'s C I2C interface and uses
`linux_embedded_hal::I2cdev` for physical writes. Transmit frames are sent to the
lower seven bits of the CSP destination address unless libcsp routing supplies a
different `via` address. Use `route_node_via_i2c_addr()` when a device's CSP
node id and Linux 7-bit I2C address are different.

The receive side is intentionally explicit: I2C is master-driven, so this crate
cannot know how a particular device exposes pending CSP replies. Use
`inject_received_frame()` when another driver path already has a raw CSP frame,
or `read_frame_from()` when the target protocol has a known fixed frame length.
For `CspClient::transaction()`, run the device-specific I2C receive polling path
in another thread so replies are injected while the transaction waits.
