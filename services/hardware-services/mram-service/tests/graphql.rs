use std::path::Path;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use futures::executor::block_on;
use kubos_service::{Config, Service};
use mram_service::schema::{MutationRoot, QueryRoot};
use mram_service::subsystem::Subsystem;
use serde_json::Value;
use tempfile::TempDir;

type MramService = Service<QueryRoot, MutationRoot, Subsystem>;

fn setup_service() -> (TempDir, MramService) {
    let tempdir = TempDir::new().expect("failed to create tempdir");
    let image_path = tempdir.path().join("mram-test.img");

    let config = Config::new_from_str(
        "mram-service",
        &format!(
            r#"
[mram-service]
backend = "file"
image_path = "{}"
image_capacity_bytes = 524288

[mram-service.addr]
ip = "127.0.0.1"
port = 9999
"#,
            image_path.display()
        ),
    )
    .expect("failed to build config");

    let subsystem = Subsystem::from_config(&config).expect("failed to initialize subsystem");
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    (tempdir, service)
}

fn graphql(service: &MramService, query: &str) -> Value {
    let response = block_on(service.schema().execute(query));
    serde_json::to_value(response).expect("failed to serialize GraphQL response")
}

fn data(response: Value) -> Value {
    if let Some(errors) = response.get("errors") {
        panic!("graphql errors: {errors}");
    }
    response
        .get("data")
        .cloned()
        .expect("response missing GraphQL data")
}

#[test]
fn ping_and_initial_storage_stats() {
    let (_tmp, service) = setup_service();

    let data = data(graphql(
        &service,
        r#"
        {
            ping
            storage {
                backend
                capacityBytes
                fileCount
                liveBytes
                freeBytes
            }
        }
        "#,
    ));

    assert_eq!(data["ping"], "pong");
    assert_eq!(data["storage"]["backend"], "file");
    assert_eq!(data["storage"]["capacityBytes"], 524_288);
    assert_eq!(data["storage"]["fileCount"], 0);
    assert_eq!(data["storage"]["liveBytes"], 0);
}

#[test]
fn write_list_read_range_and_delete_file() {
    let (_tmp, service) = setup_service();

    let payload = vec![1_u8, 2, 3, 4, 5, 6, 7, 8];
    let b64 = STANDARD.encode(&payload);

    let write_mutation = format!(
        r#"
        mutation {{
            writeFile(input: {{
                name: "image.bin",
                mimeType: "application/octet-stream",
                compressed: true,
                dataBase64: "{}"
            }}) {{
                success
                errors
                file {{
                    name
                    size
                    mimeType
                    compressed
                }}
            }}
        }}
        "#,
        b64
    );

    let write_data = data(graphql(&service, &write_mutation));
    assert_eq!(write_data["writeFile"]["success"], true);
    assert_eq!(write_data["writeFile"]["file"]["name"], "image.bin");
    assert_eq!(write_data["writeFile"]["file"]["size"], payload.len());
    assert_eq!(
        write_data["writeFile"]["file"]["mimeType"],
        "application/octet-stream"
    );
    assert_eq!(write_data["writeFile"]["file"]["compressed"], true);

    let list_data = data(graphql(
        &service,
        r#"
        {
            files {
                name
                size
                compressed
            }
        }
        "#,
    ));

    assert_eq!(list_data["files"].as_array().unwrap().len(), 1);
    assert_eq!(list_data["files"][0]["name"], "image.bin");
    assert_eq!(list_data["files"][0]["size"], payload.len());

    let read_data = data(graphql(
        &service,
        r#"
        {
            readFile(name: "image.bin", offset: 2, length: 3) {
                name
                totalSize
                offset
                length
                compressed
                dataBase64
            }
        }
        "#,
    ));

    assert_eq!(read_data["readFile"]["name"], "image.bin");
    assert_eq!(read_data["readFile"]["totalSize"], payload.len());
    assert_eq!(read_data["readFile"]["offset"], 2);
    assert_eq!(read_data["readFile"]["length"], 3);
    assert_eq!(read_data["readFile"]["compressed"], true);

    let read_b64 = read_data["readFile"]["dataBase64"]
        .as_str()
        .expect("readFile.dataBase64 must be a string");
    let decoded = STANDARD
        .decode(read_b64)
        .expect("invalid base64 from service");
    assert_eq!(decoded, vec![3, 4, 5]);

    let delete_data = data(graphql(
        &service,
        r#"
        mutation {
            deleteFile(name: "image.bin") {
                success
                errors
                deleted
            }
        }
        "#,
    ));

    assert_eq!(delete_data["deleteFile"]["success"], true);
    assert_eq!(delete_data["deleteFile"]["deleted"], true);

    let file_data = data(graphql(
        &service,
        r#"
        {
            file(name: "image.bin") {
                name
            }
        }
        "#,
    ));

    assert!(file_data["file"].is_null());
}

#[test]
fn overwrite_file_updates_contents() {
    let (_tmp, service) = setup_service();

    let first = STANDARD.encode([0xAA_u8, 0xBB, 0xCC]);
    let second = STANDARD.encode([0x10_u8, 0x11, 0x12, 0x13, 0x14]);

    let _ = data(graphql(
        &service,
        &format!(
            r#"
            mutation {{
                writeFile(input: {{
                    name: "payload.bin",
                    dataBase64: "{}"
                }}) {{ success }}
            }}
            "#,
            first
        ),
    ));

    let overwrite = data(graphql(
        &service,
        &format!(
            r#"
            mutation {{
                writeFile(input: {{
                    name: "payload.bin",
                    compressed: false,
                    dataBase64: "{}"
                }}) {{
                    success
                    file {{ size }}
                }}
            }}
            "#,
            second
        ),
    ));

    assert_eq!(overwrite["writeFile"]["success"], true);
    assert_eq!(overwrite["writeFile"]["file"]["size"], 5);

    let read = data(graphql(
        &service,
        r#"
        {
            readFile(name: "payload.bin") {
                dataBase64
                totalSize
            }
        }
        "#,
    ));

    assert_eq!(read["readFile"]["totalSize"], 5);
    let bytes = STANDARD
        .decode(
            read["readFile"]["dataBase64"]
                .as_str()
                .expect("missing dataBase64"),
        )
        .expect("invalid base64");
    assert_eq!(bytes, vec![0x10, 0x11, 0x12, 0x13, 0x14]);
}

#[test]
fn format_requires_confirm_and_clears_files() {
    let (_tmp, service) = setup_service();

    let payload = STANDARD.encode([9_u8, 8, 7, 6]);

    let _ = data(graphql(
        &service,
        &format!(
            r#"
            mutation {{
                writeFile(input: {{
                    name: "to-clear.bin",
                    dataBase64: "{}"
                }}) {{ success }}
            }}
            "#,
            payload
        ),
    ));

    let rejected = data(graphql(
        &service,
        r#"
        mutation {
            format(confirm: false) {
                success
                errors
            }
        }
        "#,
    ));

    assert_eq!(rejected["format"]["success"], false);
    assert!(
        rejected["format"]["errors"]
            .as_str()
            .unwrap()
            .contains("confirm")
    );

    let before = data(graphql(
        &service,
        r#"
        {
            files { name }
        }
        "#,
    ));
    assert_eq!(before["files"].as_array().unwrap().len(), 1);

    let accepted = data(graphql(
        &service,
        r#"
        mutation {
            format(confirm: true) {
                success
                errors
            }
        }
        "#,
    ));

    assert_eq!(accepted["format"]["success"], true);

    let after = data(graphql(
        &service,
        r#"
        {
            files { name }
            storage { fileCount }
        }
        "#,
    ));

    assert_eq!(after["files"].as_array().unwrap().len(), 0);
    assert_eq!(after["storage"]["fileCount"], 0);
}

#[test]
fn creates_image_file_on_disk() {
    let (tmp, service) = setup_service();
    let image_path = tmp.path().join("mram-test.img");

    let _ = data(graphql(&service, "{ ping }"));

    assert!(Path::new(&image_path).exists());
}
