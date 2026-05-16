use fram_service::backend::FileImageBackend;
use fram_service::env::MemoryEnvStore;
use fram_service::schema::{MutationRoot, QueryRoot};
use fram_service::subsystem::Subsystem;
use futures::executor::block_on;
use kubos_service::Service;
use serde_json::Value;
use tempfile::TempDir;

type FramService = Service<QueryRoot, MutationRoot, Subsystem>;

fn setup_service() -> (TempDir, FramService) {
    let tmp = TempDir::new().expect("tempdir");
    let image_path = tmp.path().join("fram-test.img");

    let config = kubos_service::Config::new_from_str(
        "fram-service",
        &format!(
            r#"
[fram-service]
backend = "file"
image_path = "{}"
image_capacity_bytes = 8192

[fram-service.addr]
ip = "127.0.0.1"
port = 9998
"#,
            image_path.display()
        ),
    )
    .expect("config");

    let backend = FileImageBackend::new(image_path.to_str().unwrap(), 8192).expect("backend");
    let subsystem = Subsystem::from_parts(
        "file".to_string(),
        Box::new(backend),
        Box::new(MemoryEnvStore::default()),
    )
    .expect("subsystem");
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    (tmp, service)
}

fn graphql(service: &FramService, query: &str) -> Value {
    let response = block_on(service.schema().execute(query));
    serde_json::to_value(response).expect("serialize")
}

fn data(response: Value) -> Value {
    if let Some(errors) = response.get("errors") {
        panic!("graphql errors: {errors}");
    }

    response.get("data").cloned().expect("data")
}

#[test]
fn ping_health_and_default_state() {
    let (_tmp, service) = setup_service();

    let data = data(graphql(
        &service,
        r#"
        {
            ping
            health {
                backend
                capacityBytes
                framReachable
            }
            missionState {
                deployed
                removeBeforeFlight
                deployStart
            }
        }
        "#,
    ));

    assert_eq!(data["ping"], "pong");
    assert_eq!(data["health"]["backend"], "file");
    assert_eq!(data["health"]["capacityBytes"], 8192);
    assert_eq!(data["health"]["framReachable"], true);
    assert_eq!(data["missionState"]["deployed"], false);
    assert_eq!(data["missionState"]["removeBeforeFlight"], false);
    assert!(data["missionState"]["deployStart"].is_null());
}

#[test]
fn set_flag_and_deploy_start() {
    let (_tmp, service) = setup_service();

    let flag_data = data(graphql(
        &service,
        r#"
        mutation {
            setMissionFlag(key: UHF_ANTENNA_DEPLOYED, value: true, mirrorToEnv: false) {
                success
                errors
                state { uhfAntennaDeployed }
            }
        }
        "#,
    ));

    assert_eq!(flag_data["setMissionFlag"]["success"], true);
    assert_eq!(
        flag_data["setMissionFlag"]["state"]["uhfAntennaDeployed"],
        true
    );

    let deploy_start_data = data(graphql(
        &service,
        r#"
        mutation {
            setDeployStart(timestamp: 1770000000, mirrorToEnv: false) {
                success
                state { deployStart }
            }
        }
        "#,
    ));

    assert_eq!(deploy_start_data["setDeployStart"]["success"], true);
    assert_eq!(
        deploy_start_data["setDeployStart"]["state"]["deployStart"],
        1_770_000_000
    );
}

#[test]
fn initialize_requires_confirmation() {
    let (_tmp, service) = setup_service();

    let data = data(graphql(
        &service,
        r#"
        mutation {
            initializeFlightState(confirm: false) {
                success
                errors
            }
        }
        "#,
    ));

    assert_eq!(data["initializeFlightState"]["success"], false);
    assert!(
        data["initializeFlightState"]["errors"]
            .as_str()
            .unwrap()
            .contains("confirm=true")
    );
}
