// Integration tests for dosimeter service
use serde_json::{json, Value};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU16, Ordering};
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

static NEXT_TEST_PORT: AtomicU16 = AtomicU16::new(18080);
const MAX_RETRIES: usize = 10;
const RETRY_DELAY_MS: u64 = 200;

fn next_test_port() -> u16 {
    NEXT_TEST_PORT.fetch_add(1, Ordering::Relaxed)
}

fn do_query_once(port: u16, query: &str) -> Result<Value, String> {
    let url = format!("http://127.0.0.1:{}/graphql", port);
    let client = reqwest::blocking::Client::new();

    let response = client
        .post(&url)
        .json(&json!({ "query": query }))
        .send()
        .map_err(|err| format!("request failed: {}", err))?;

    response
        .json()
        .map_err(|err| format!("response parse failed: {}", err))
}

// Helper function to send GraphQL queries with retries.
fn do_query(port: Option<u16>, query: &str) -> Value {
    let port = port.unwrap_or(8080);
    let mut last_error = String::from("query failed");

    for attempt in 0..MAX_RETRIES {
        match do_query_once(port, query) {
            Ok(value) => return value,
            Err(err) => {
                last_error = err;
                if attempt < MAX_RETRIES - 1 {
                    thread::sleep(Duration::from_millis(RETRY_DELAY_MS));
                }
            }
        }
    }

    panic!(
        "Failed to query dosimeter service on port {} after {} attempts: {}",
        port, MAX_RETRIES, last_error
    );
}

fn dosimeter_binary_path() -> PathBuf {
    if let Some(path) = env::var_os("CARGO_BIN_EXE_dosimeter") {
        return PathBuf::from(path);
    }

    let mut path = env::current_exe().expect("Failed to resolve current test executable path");
    path.pop();
    path.pop();
    path.push("dosimeter");
    if cfg!(windows) {
        path.set_extension("exe");
    }
    path
}

fn wait_for_service(port: u16) {
    let query = "{ ping }";

    for _ in 0..MAX_RETRIES {
        if let Ok(result) = do_query_once(port, query) {
            if result["data"]["ping"] == "pong" {
                return;
            }
        }
        thread::sleep(Duration::from_millis(RETRY_DELAY_MS));
    }

    panic!("dosimeter service failed readiness check on port {}", port);
}

struct DosimeterServiceFixture {
    _temp_dir: TempDir,
    child: Option<Child>,
}

impl DosimeterServiceFixture {
    fn setup(port: Option<u16>) -> Self {
        let port = port.unwrap_or(8080);
        let temp_dir = tempfile::tempdir().expect("Failed to create temp test directory");
        let config_path = temp_dir.path().join("config.toml");

        let config = format!(
            r#"
            [dosimeter]
            i2c_bus = "/dev/i2c-1"
            device_addr = 74

            [dosimeter.addr]
            ip = "127.0.0.1"
            port = {}
            "#,
            port
        );

        let mut config_file = File::create(&config_path).expect("Failed to create test config");
        config_file
            .write_all(config.as_bytes())
            .expect("Failed to write test config");

        let binary = dosimeter_binary_path();
        let child = Command::new(&binary)
            .arg("-c")
            .arg(&config_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to start dosimeter binary at '{}': {}",
                    binary.display(),
                    err
                )
            });

        wait_for_service(port);

        DosimeterServiceFixture {
            _temp_dir: temp_dir,
            child: Some(child),
        }
    }
}

impl Drop for DosimeterServiceFixture {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[test]
fn test_ping() {
    let port = next_test_port();

    let _fixture = DosimeterServiceFixture::setup(Some(port));

    let query = r#"{
        ping
    }"#;

    let expected = json!({
        "data": {
            "ping": "pong"
        }
    });

    let result = do_query(Some(port), query);
    assert_eq!(result, expected);
}

#[test]
fn test_read_all_sensors() {
    let port = next_test_port();

    let _fixture = DosimeterServiceFixture::setup(Some(port));

    let query = r#"{
        read {
            address
            name
            adc
            success
        }
    }"#;

    let result = do_query(Some(port), query);

    // Check that we got a response with data
    assert!(result["data"]["read"].is_array());

    // Check we have 8 sensors (7 dosimeters + 1 temp)
    let sensors = result["data"]["read"].as_array().unwrap();
    assert_eq!(sensors.len(), 8);

    // Check that each sensor has the required fields
    for sensor in sensors {
        assert!(sensor["address"].is_string());
        assert!(sensor["name"].is_string());
        assert!(sensor["adc"].is_number());
        assert!(sensor["success"].is_boolean());
    }
}

#[test]
fn test_read_sensor_specific() {
    let port = next_test_port();

    let _fixture = DosimeterServiceFixture::setup(Some(port));

    let query = r#"{
        readSensor(address: "0x84") {
            address
            name
            adc
            success
        }
    }"#;

    let result = do_query(Some(port), query);

    // Verify we got a response
    assert!(result["data"]["readSensor"].is_object());

    // Verify the address matches
    assert_eq!(result["data"]["readSensor"]["address"], "0x84");
    assert_eq!(result["data"]["readSensor"]["name"], "u1");
    assert!(result["data"]["readSensor"]["adc"].is_number());
}

#[test]
fn test_read_temperature_sensor() {
    let port = next_test_port();

    let _fixture = DosimeterServiceFixture::setup(Some(port));

    let query = r#"{
        readSensor(address: "0xF4") {
            address
            name
            adc
            success
        }
    }"#;

    let result = do_query(Some(port), query);

    // Verify we got the temp sensor
    assert_eq!(result["data"]["readSensor"]["address"], "0xF4");
    assert_eq!(result["data"]["readSensor"]["name"], "temp");
}

#[test]
fn test_voltage_convert() {
    let port = next_test_port();

    let _fixture = DosimeterServiceFixture::setup(Some(port));

    let query = r#"{
        voltageConvert(adc: 2048)
    }"#;

    let result = do_query(Some(port), query);

    // Expected from conversion formula: (adc / 4095) * 3300
    let voltage = result["data"]["voltageConvert"].as_f64().unwrap();
    let expected = (2048.0 / 4095.0) * 3300.0;
    assert!(
        (voltage - expected).abs() < 1e-6,
        "Expected ~{}, got {}",
        expected,
        voltage
    );
}

#[test]
fn test_temp_convert() {
    let port = next_test_port();

    let _fixture = DosimeterServiceFixture::setup(Some(port));

    let query = r#"{
        tempConvert(voltageMv: 1650.0)
    }"#;

    let result = do_query(Some(port), query);

    // Expected temp: (1650.0 * (1.0 / -13.6)) + 192.48 ~= 71.16
    let temp = result["data"]["tempConvert"].as_f64().unwrap();
    assert!(
        (temp - 71.16).abs() < 1.0,
        "Expected ~71.16C, got {}C",
        temp
    );
}

#[test]
fn test_voltage_convert_boundary_min() {
    let port = next_test_port();

    let _fixture = DosimeterServiceFixture::setup(Some(port));

    let query = r#"{ voltageConvert(adc: 0) }"#;
    let result = do_query(Some(port), query);

    let voltage = result["data"]["voltageConvert"].as_f64().unwrap();
    assert_eq!(voltage, 0.0);
}

#[test]
fn test_voltage_convert_boundary_max() {
    let port = next_test_port();

    let _fixture = DosimeterServiceFixture::setup(Some(port));

    let query = r#"{ voltageConvert(adc: 4095) }"#;
    let result = do_query(Some(port), query);

    let voltage = result["data"]["voltageConvert"].as_f64().unwrap();
    assert_eq!(voltage, 3300.0);
}

#[test]
fn test_read_radfet_sensor() {
    let port = next_test_port();

    let _fixture = DosimeterServiceFixture::setup(Some(port));

    let query = r#"{
        readSensor(address: "0xE4") {
            address
            name
            success
        }
    }"#;

    let result = do_query(Some(port), query);

    assert_eq!(result["data"]["readSensor"]["address"], "0xE4");
    assert_eq!(result["data"]["readSensor"]["name"], "radfet");
}

#[test]
fn test_multiple_queries_in_one_request() {
    let port = next_test_port();

    let _fixture = DosimeterServiceFixture::setup(Some(port));

    let query = r#"{
        ping
        u1: readSensor(address: "0x84") {
            name
            adc
        }
        voltage: voltageConvert(adc: 2048)
    }"#;

    let result = do_query(Some(port), query);

    // Verify all three queries returned data
    assert_eq!(result["data"]["ping"], "pong");
    assert_eq!(result["data"]["u1"]["name"], "u1");
    assert!(result["data"]["u1"]["adc"].is_number());
    assert!(result["data"]["voltage"].is_number());
}
