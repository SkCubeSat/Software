use libcsp_cargo_build::{generate_autoconf_header_file, Builder};
use std::{env, path::PathBuf};
use std::fs;


fn main() {

    // LIBCSP

    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_path = "../../third_party/libcsp";
    let lib_cfg_dir = "../../third_party/cfg/csp";
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let manifest_path = PathBuf::from(&manifest_dir);

    let mut csp_builder = Builder::new(PathBuf::from(lib_path), PathBuf::from(&out_dir))
        .expect("creating libcsp builder failed");

    csp_builder.compiler_warnings = false;

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


    // libcubeOBC

    let cubeobc_path = "../../third_party/libcubeobc";
    let cubeobc_api_src = "../../third_party/libcubeobc/api/src";

    
    

    cc::Build::new()
        .include(format!("{}/include", cubeobc_path))
        .include(format!("{}/include/cubeObc", cubeobc_path))
        .include(cubeobc_api_src) 
        .include("../../third_party/c_overrides")
        .file("../../third_party/c_overrides/cubeobc_weak.c")
        .compile("cubeobc_overrides");

    let bindings = bindgen::Builder::default()
        .header("../../third_party/c_overrides/cubeobc_weak.h")
        // .header(format!("-I{}/include/cubeObc/cubeobc.h", cubeobc_path))
        .clang_arg(format!("-I{}/include", cubeobc_path))
        .clang_arg(format!("-I{}", cubeobc_api_src))
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_file = out_path.join("bindings.rs");

    bindings
            .write_to_file(&bindings_file)
            .expect("Couldn't write bindings!");

    add_unsafe_to_extern_blocks(&bindings_file);

    

    let cubeobc_release_path = PathBuf::from(&manifest_dir)
        .join("../../third_party/libcubeobc/build/libcubeobc/RELEASE");

    println!("cargo:warning=Linking to libcubeobc at: {}", cubeobc_release_path.display());
    println!("cargo:warning=Expecting file: {}", cubeobc_release_path.join("libcubeobc.a").display());
    println!("cargo:rustc-link-search=native={}", cubeobc_release_path.display());
    // println!("cargo:rustc-link-search=native={}", cubeobc_path);
    println!("cargo:rustc-link-lib=static=cubeobc");
}

/// Reads the generated bindings, adds `unsafe` to `extern "C"` blocks, and saves it back.
fn add_unsafe_to_extern_blocks(file_path: &PathBuf) {
    // Read the generated file
    let content = fs::read_to_string(file_path)
        .expect("Failed to read bindings file");

    // Replace `extern "C" {` with `unsafe extern "C" {`
    let modified_content = content.replace("extern \"C\" {", "unsafe extern \"C\" {");

    // Write the modified content back
    fs::write(file_path, modified_content)
        .expect("Failed to write modified bindings");
}


