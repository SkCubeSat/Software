extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("/usr/local/include/csp/csp.h") // Adjust if libcsp is installed elsewhere
        .clang_arg("-I/usr/local/include")
        //.raw_line("unsafe extern \"C\" {")  
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_file = out_path.join("bindings.rs");

    bindings
        .write_to_file(&bindings_file)
        .expect("Couldn't write bindings!");

    add_unsafe_to_extern_blocks(&bindings_file);
    
    println!("cargo:rustc-link-lib=csp"); // Link against libcsp
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