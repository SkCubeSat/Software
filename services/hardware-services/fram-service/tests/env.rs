#![cfg(unix)]

use fram_service::env::{CommandEnvStore, EnvStore};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

fn executable_script(dir: &TempDir, name: &str, body: &str) -> String {
    let path = dir.path().join(name);
    fs::write(&path, body).expect("write script");
    let mut perms = fs::metadata(&path).expect("metadata").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).expect("chmod");
    path.to_str().unwrap().to_string()
}

#[test]
fn command_env_store_reads_with_fw_printenv_contract() {
    let tmp = TempDir::new().expect("tempdir");
    let printenv = executable_script(
        &tmp,
        "fw_printenv",
        r#"#!/bin/sh
if [ "$1" = "-n" ] && [ "$2" = "detumbling_complete" ]; then
  printf true
  exit 0
fi
exit 1
"#,
    );

    let mut store = CommandEnvStore::new(printenv, "/no/fw_setenv".to_string());

    assert_eq!(
        store.read("detumbling_complete").expect("read").as_deref(),
        Some("true")
    );
    assert_eq!(store.read("missing_key").expect("read"), None);
}

#[test]
fn command_env_store_reports_missing_env_backend() {
    let tmp = TempDir::new().expect("tempdir");
    let printenv = executable_script(
        &tmp,
        "fw_printenv",
        r#"#!/bin/sh
echo 'Cannot open /envar/uboot.env: No such file or directory' >&2
exit 1
"#,
    );

    let mut store = CommandEnvStore::new(printenv, "/no/fw_setenv".to_string());
    let err = store.read("detumbling_complete").unwrap_err();

    assert!(err.contains("Cannot open /envar/uboot.env"));
}

#[test]
fn command_env_store_writes_with_fw_setenv_contract() {
    let tmp = TempDir::new().expect("tempdir");
    let log = tmp.path().join("fw_setenv.log");
    let setenv = executable_script(
        &tmp,
        "fw_setenv",
        &format!(
            r#"#!/bin/sh
printf '%s\n' "$*" >> '{}'
"#,
            log.display()
        ),
    );

    let mut store = CommandEnvStore::new("/no/fw_printenv".to_string(), setenv);

    store
        .write("detumbling_complete", Some("true"))
        .expect("write bool");
    store.write("deploy_start", None).expect("unset timestamp");

    let log = fs::read_to_string(log).expect("read log");
    assert!(log.contains("detumbling_complete true"));
    assert!(log.contains("deploy_start"));
}
