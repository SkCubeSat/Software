use std::{env, path::PathBuf};

pub const ENV_KEY_CSP_CONFIG_DIR: &str = "CSP_CONFIG_DIR";
pub const ENV_KEY_TEST: &str = "RUN_TESTS";

fn main() {
    // libcsp is built in a separate project, so linking it for tests will fail.
    // For tests, we do not want to have the link directive to csp.
    let run_tests = if let Ok(val) = env::var(ENV_KEY_TEST) {
        val == "1"
    } else {
        false
    };
    if !run_tests {
        println!("cargo:rustc-link-lib=csp");
    }

    let mut csp_conf_path = if std::env::var("DOCS_RS").is_ok() {
        PathBuf::from("./templates")
    } else {
        match env::var(ENV_KEY_CSP_CONFIG_DIR) {
            Ok(conf_path) => conf_path.into(),
            Err(_e) => {
                println!(
                "cargo:warning={} not set, using CARGO_MANIFEST_DIR to search for autoconfig.rs",
                ENV_KEY_CSP_CONFIG_DIR
            );
                env::var("CARGO_MANIFEST_DIR")
                    .expect("CARGO_MANIFEST_DIR not set")
                    .into()
            }
        }
    };

    let out_path = env::var("OUT_DIR").unwrap();
    csp_conf_path.push("autoconfig.rs");
    if !csp_conf_path.exists() {
        panic!(
            "autoconfig.rs not found at {:?}, is required for library build",
            csp_conf_path
        );
    }
    let out_path_full = PathBuf::from(&out_path).join("autoconfig.rs");
    std::fs::copy(&csp_conf_path, out_path_full).expect("failed to copy autoconfig.rs to OUT_DIR");
    println!("cargo::rerun-if-changed={:?}", &csp_conf_path);
}
