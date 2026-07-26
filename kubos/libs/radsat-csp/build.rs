use std::{env, path::PathBuf};

use libcsp_cargo_build::Builder;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set"));
    let libcsp_path = manifest_dir.join("../../../vendor/libcsp");

    let mut csp_builder =
        Builder::new(libcsp_path.clone(), out_dir).expect("creating libcsp builder failed");
    csp_builder.compiler_warnings = false;
    csp_builder.cfg.rtable = true;
    csp_builder
        .cc_mut()
        .file(libcsp_path.join("src/interfaces/csp_if_i2c.c"))
        .file(manifest_dir.join("src/capture_iface.c"));
    csp_builder.compile().expect("compiling libcsp failed");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", libcsp_path.display());
    println!("cargo:rerun-if-changed=csp-config/autoconfig.rs");
}
