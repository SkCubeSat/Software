use fram_service::backend::FileImageBackend;
use fram_service::env::MemoryEnvStore;
use fram_service::model::{MissionKey, MissionValue};
use fram_service::reconcile::{SourceValue, merge_value};
use fram_service::subsystem::Subsystem;
use tempfile::TempDir;

fn subsystem_with_env(values: Vec<(&str, &str)>) -> (TempDir, Subsystem) {
    let tmp = TempDir::new().expect("tempdir");
    let path = tmp.path().join("fram.img");
    let backend = FileImageBackend::new(path.to_str().unwrap(), 8192).expect("backend");
    let env = MemoryEnvStore::new(
        values
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string())),
    );
    let subsystem = Subsystem::from_parts("file".to_string(), Box::new(backend), Box::new(env))
        .expect("subsystem");
    (tmp, subsystem)
}

#[test]
fn remove_before_flight_uses_or_policy() {
    let merged = merge_value(
        MissionKey::RemoveBeforeFlight,
        &SourceValue::Value(MissionValue::Bool(false)),
        &SourceValue::Value(MissionValue::Bool(true)),
        100,
    );

    assert_eq!(merged, MissionValue::Bool(true));
}

#[test]
fn completion_flags_require_no_explicit_false() {
    let merged = merge_value(
        MissionKey::SolarPanelDeployed,
        &SourceValue::Value(MissionValue::Bool(true)),
        &SourceValue::Value(MissionValue::Bool(false)),
        100,
    );

    assert_eq!(merged, MissionValue::Bool(false));
}

#[test]
fn deploy_start_uses_latest_valid_timestamp() {
    let merged = merge_value(
        MissionKey::DeployStart,
        &SourceValue::Value(MissionValue::Timestamp(Some(100))),
        &SourceValue::Value(MissionValue::Timestamp(Some(200))),
        300,
    );

    assert_eq!(merged, MissionValue::Timestamp(Some(200)));
}

#[test]
fn live_reconcile_repairs_missing_fram_from_env() {
    let (_tmp, subsystem) = subsystem_with_env(vec![("remove_before_flight", "true")]);

    let report = subsystem.reconcile(false).expect("reconcile");

    assert!(report.success);
    assert!(report.state.remove_before_flight);
    let action = report
        .actions
        .iter()
        .find(|action| action.key == "remove_before_flight")
        .expect("action");
    assert!(action.action.contains("write_fram"));
}

#[test]
fn dry_run_reports_repairs_without_changing_state() {
    let (_tmp, subsystem) = subsystem_with_env(vec![("deployed", "true")]);

    let report = subsystem.reconcile(true).expect("reconcile");
    assert!(report.success);
    assert!(
        report
            .actions
            .iter()
            .any(|action| action.key == "deployed" && action.action.contains("dry_run"))
    );

    let state = subsystem.mission_state(false).expect("state");
    assert!(!state.deployed);
}
