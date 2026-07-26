use std::{env, path::PathBuf};

use libcsp_cargo_build::{generate_autoconf_header_file, Builder};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap_or_default();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let manifest_path = PathBuf::from(&manifest_dir);
    let lib_cfg_dir = "../clib/cfg/csp";
    let libcsp_path = "../clib/libcsp";

    // This helper structure will take care of the majority of work to compile libcsp using the
    // cc crate.
    let mut csp_builder = Builder::new(PathBuf::from(libcsp_path), PathBuf::from(&out_dir))
        .expect("creating libcsp builder failed");
    // A lot of spam we are not interested in usually.
    csp_builder.compiler_warnings = false;

    // We always re-generate the header file.
    generate_autoconf_header_file(manifest_path.clone(), &csp_builder.cfg)
        .expect("generating header file failed");

    // Copy the file to lib/csp/cfg as well for binding generation.
    std::fs::copy(
        manifest_path.join("autoconfig.h"),
        PathBuf::from(&lib_cfg_dir).join("autoconfig.h"),
    )
    .expect("copying autoconfig.h failed");

    // This file is required for the compile-time configuration of libcsp-rust.
    csp_builder
        .generate_autoconf_rust_file(manifest_path)
        .expect("generating autoconfig.rs failed");

    csp_builder.compile().expect("compiling libcsp failed");

    // If we change the libcsp build configuration, we need to re-run the build.
    println!("cargo::rerun-if-changed=build.rs");
}
