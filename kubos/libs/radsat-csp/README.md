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
device, such as I2C, KISS/USART, or CAN, then expose the device-specific
commands from a crate under `kubos/apis`.
