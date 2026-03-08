// Integration tests for dosimeter service
use serde_json::{json, Value};
use std::thread;
use std::time::Duration;

// Helper function to send GraphQL queries
fn do_query(port: Option<u16>, query: &str) -> Value {
    let port = port.unwrap_or(8080);
    let url = format!("http://127.0.0.1:{}/graphql", port);
    
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&url)
        .json(&json!({
            "query": query
        }))
        .send()
        .expect("Failed to send request");
    
    response.json().expect("Failed to parse JSON response")
}

// Simple fixture that ensures service is running
struct DosimeterServiceFixture {
    _port: u16,
}

impl DosimeterServiceFixture {
    fn setup(port: Option<u16>) -> Self {
        let port = port.unwrap_or(8080);
        
        // Wait a moment to ensure service is ready
        thread::sleep(Duration::from_millis(100));
        
        DosimeterServiceFixture { _port: port }
    }
}

#[test]
fn test_ping() {
    let port = 8080;
    
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
    let port = 8080;
    
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
    let port = 8080;
    
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
    let port = 8080;
    
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
    let port = 8080;
    
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
    let port = 8080;
    
    let _fixture = DosimeterServiceFixture::setup(Some(port));

    let query = r#"{
        tempConvert(voltageMv: 1650.0)
    }"#;

    let result = do_query(Some(port), query);
    
    // Expected temp: (1650.0 * (1.0 / -13.6)) + 192.48 ≈ 71.16
    let temp = result["data"]["tempConvert"].as_f64().unwrap();
    assert!((temp - 71.16).abs() < 1.0, "Expected ~71.16°C, got {}°C", temp);
}

#[test]
fn test_voltage_convert_boundary_min() {
    let port = 8080;
    
    let _fixture = DosimeterServiceFixture::setup(Some(port));

    let query = r#"{ voltageConvert(adc: 0) }"#;
    let result = do_query(Some(port), query);
    
    let voltage = result["data"]["voltageConvert"].as_f64().unwrap();
    assert_eq!(voltage, 0.0);
}

#[test]
fn test_voltage_convert_boundary_max() {
    let port = 8080;
    
    let _fixture = DosimeterServiceFixture::setup(Some(port));

    let query = r#"{ voltageConvert(adc: 4095) }"#;
    let result = do_query(Some(port), query);
    
    let voltage = result["data"]["voltageConvert"].as_f64().unwrap();
    assert_eq!(voltage, 3300.0);
}

#[test]
fn test_read_radfet_sensor() {
    let port = 8080;
    
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
    let port = 8080;
    
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
