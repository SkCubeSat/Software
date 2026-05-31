[![Crates.io](https://img.shields.io/crates/v/libcsp)](https://crates.io/crates/libcsp)
[![docs.rs](https://img.shields.io/docsrs/libcsp)](https://docs.rs/libcsp)

libcsp-rust
=========

This project aims to provide libraries and tools to build and use
the [`libcsp`](https://github.com/libcsp/libcsp) C library in your Rust project.

It provides 3 crates for this:

- [`libcsp`](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust/src/branch/main)
  provides a safe and ergonomic Rust interface on top of the `libcsp-sys` crate.
- [`libcsp-sys`](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust/src/branch/main/libcsp-sys)
  provides the Rust bindings to [`libcsp`](https://github.com/libcsp/libcsp).
- [`libcsp-cargo-build`](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust/src/branch/main/libcsp-cargo-build)
  provides an API to build the `libcsp` using `cargo` with the [`cc`](https://docs.rs/cc/latest/cc/) crate.

In addition, it provides a workspace to allow updating the `libcsp` C sources and the corresponding
bindings more easily inside the `clib` directory. Some of the examples `libcsp` provides were ported
to Rust and are showcased in the `examples` directory.

Please note that this is early-stage/experimental software. Important features might be missing.
PRs and improvement suggestions are welcome! This project was primarily tested on a Linux/POSIX
system so far.

## How it works

We assume that cargo should also take care of building the library.

1. Add the `libcsp-cargo-build` as a build dependency inside your `Cargo.toml`.
2. Add the `libcsp` as a regular dependency inside your `Cargo.toml`.
3. Create a custom `build.rs` script which takes care of building the `libcsp` C library using the
   API provided by `libcsp-cargo-build`. You have to provide the source code for `libcsp` inside
   some directory and pass the directory path to the builder API.
4. You can now write regular Rust code and use the Rust API provided by the `libcsp` crate to use
   the `libcsp` C library.

It is recommended to have a look at the [example build script](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust/src/branch/main/examples/build.rs)
which should give you a general idea of how a build script which does the 4 steps above might look
like.

## Running the example

The example uses both the builder crate and the bindings and API crate and implements the
[server/client example](https://github.com/libcsp/libcsp/blob/develop/examples/csp_server_client.c)
in Rust. You can run the example using the following steps:

1. Clone/Copy `libcsp` into the `lib` folder, for example by using the provided `lib/clone-csp.sh`
   script or adding `libcsp` as a git submodule.
2. You can now use `cargo run` to run the server/client example.

## Compile-time configuration of the `libcsp-sys` library

The `libcsp-sys` requires some compile-time configuration file to be included to work
properly. You can see an example version of the file for the workspace
[here](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust/src/branch/main/examples/autoconfig.rs).
The user has to provide the path to a directory containing this `autoconfig.rs` file using the
`CSP_CONFIG_DIR` environmental variable.

You can automatically generate this file when using `libcsp-cargo-build` by using the
[`generate_autoconf_rust_file`](https://docs.rs/libcsp-cargo-build/latest/libcsp_cargo_build/fn.generate_autoconf_rust_file.html)
method of the Builder object as done in the example build script.

In this workspace, the `CSP_CONFIG_DIR` variable is hardcoded using the following `.cargo/config.toml`
configuration:

```toml
[env]
CSP_CONFIG_DIR = { value = "examples", relative = true }
```

## Generating and update the bindings using the `clib` folder

The `lib` folder in this repository serves as the staging directory for the `libcsp` library to
build. However, it can also be used to update the bindings provided in `libcsp-sys` by providing
some tools and helpers to auto-generate and update the bindings file `bindings.rs`.

If you want to do this, you should install `bindgen-cli` first:

```sh
cargo install bindgen-cli --locked
```

`bindgen` needs some additional information provided by the user to generate the bindings:
An `autoconfig.h` file which is used to configure `libcsp`. Normally, this file is generated
by the C build system. This file is located at `clib/cfg/csp` and is also updated automatically
when running the example application.

After cloning the repository, you can now run the following command to re-generate the bindings
file:

```sh
bindgen --use-core wrapper.h -- "-I./libcsp/include" "-I./cfg" "-I./libcsp/src" > bindings.rs
```

With the bindings file, you can now manually update the FFI bindings provided in
`libcsp-sys/src/lib.rs` or in your own CSP library.
