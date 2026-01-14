use async_graphql::{Context, EmptyMutation, Object};
use kubos_service::{Config, Service};
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use linux_embedded_hal::I2cdev;
use embedded_hal::blocking::i2c::{Write, Read};

// Constants - matching Python
const TEMPERATURE_SCALE_SLOPE: f64 = 1.0 / -13.6;
const TEMPERATURE_SCALE_OFFSET: f64 = 192.48;
const DOSIMETER_I2C_SLAVE_ADDR: u8 = 0x4A;
const DOSIMETER_I2C_DELAY: u64 = 0;
const DOSIMETER_COMMAND_LENGTH: usize = 1;
const DOSIMETER_RESPONSE_LENGTH: usize = 2;
const DOSIMETER_RESPONSE_HIGH_BYTE_MASK: u8 = 0x0F;
const ADC_BIT_RESOLUTION: u32 = 12;
const MAX_ADC_VALUE: f64 = 4095.0; // (1 << 12) - 1
const TIMER_DELAY: u64 = 100; // 0.1 seconds in ms

// Sensor codes - matching Python exactly
const DOSIMETER_LIST: [u8; 7] = [
    0x84, // channel 0 -> u1 -> None
    0xC4, // channel 1 -> u2 -> 50 mil
    0x94, // channel 2 -> u3 -> 100 mil
    0xD4, // channel 3 -> u4 -> 200 mil
    0xA4, // channel 4 -> u5 -> 20 mil
    0xE4, // channel 5 -> radfet
    0xB4, // channel 6 -> u7 -> 300 mil
];
const TEMP_SENSOR: u8 = 0xF4; // channel 7

// Helper function to map hex codes to chip names
fn hex_to_chip(code: u8) -> &'static str {
    match code {
        0x84 => "u1",
        0xC4 => "u2",
        0x94 => "u3",
        0xD4 => "u4",
        0xA4 => "u5",
        0xE4 => "radfet",
        0xB4 => "u7",
        0xF4 => "temp",
        _ => "unknown",
    }
}

// Conversion functions - matching Python
fn to_volt(input: u16) -> f64 {
    let ref_mv = 3300.0;
    let max_adc = 4095.0;
    (input as f64 / max_adc) * ref_mv
}

fn volt_to_temp(input: f64) -> f64 {
    let scale_slope = 1.0 / -13.6;
    let scale_off = 192.48;
    (input * scale_slope) + scale_off
}

// I2C functions - matching Python structure
fn switch_sensor(i2c: &mut I2cdev, sensor_code: u8) -> Result<(), String> {
    i2c.write(DOSIMETER_I2C_SLAVE_ADDR, &[sensor_code])
        .map_err(|e| format!("Failed to switch sensor: {}", e))
}

fn read_sensor_raw(i2c: &mut I2cdev) -> Result<u16, String> {
    let mut buffer = [0u8; 2];
    i2c.read(DOSIMETER_I2C_SLAVE_ADDR, &mut buffer)
        .map_err(|e| format!("Failed to read sensor: {}", e))?;
    
    // Combine bytes: high byte << 8 | low byte
    let value = (buffer[0] as u16) << 8 | (buffer[1] as u16);
    Ok(value)
}

fn read_temperature(i2c: &mut I2cdev, temp_code: u8) -> Result<(), String> {
    switch_sensor(i2c, temp_code)?;
    
    let timestamp = Utc::now();
    let chip_name = hex_to_chip(temp_code);
    
    let value = read_sensor_raw(i2c)?;
    let voltage = to_volt(value);
    let temperature = volt_to_temp(voltage);
    
    // Print CSV format matching Python output
    println!(
        "{}, 0x{:02X}, {}, {}, {:.2}, {:.2}",
        timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
        temp_code,
        chip_name,
        value,
        voltage,
        temperature
    );
    
    Ok(())
}

fn read_dosimeter(i2c: &mut I2cdev, dos_code: u8) -> Result<(), String> {
    switch_sensor(i2c, dos_code)?;
    
    let timestamp = Utc::now();
    let chip_name = hex_to_chip(dos_code);
    
    let value = read_sensor_raw(i2c)?;
    let voltage = to_volt(value);
    
    // Print CSV format matching Python output
    println!(
        "{}, 0x{:02X}, {}, {}, {:.2}",
        timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
        dos_code,
        chip_name,
        value,
        voltage
    );
    
    Ok(())
}

fn loop_sensors(i2c: &mut I2cdev) -> Result<(), String> {
    // Read all dosimeter channels
    for &sensor in &DOSIMETER_LIST {
        if let Err(e) = read_dosimeter(i2c, sensor) {
            eprintln!("Error reading dosimeter 0x{:02X}: {}", sensor, e);
        }
        std::thread::sleep(Duration::from_millis(TIMER_DELAY));
    }
    
    // Read temperature sensor
    if let Err(e) = read_temperature(i2c, TEMP_SENSOR) {
        eprintln!("Error reading temperature: {}", e);
    }
    
    Ok(())
}

// Storage for GraphQL queries
#[derive(Clone, Debug)]
struct SensorReading {
    timestamp: DateTime<Utc>,
    sensor_code: u8,
    chip_name: String,
    raw_value: u16,
    voltage_mv: f64,
    converted_value: Option<f64>, // Some(temp) for temp sensor, None for dosimeter
}

#[derive(Default)]
struct AppState {
    readings: VecDeque<SensorReading>,
    max_readings: usize,
}

impl AppState {
    fn new(max_readings: usize) -> Self {
        Self {
            readings: VecDeque::new(),
            max_readings,
        }
    }

    fn add_reading(&mut self, reading: SensorReading) {
        self.readings.push_back(reading);
        while self.readings.len() > self.max_readings {
            self.readings.pop_front();
        }
    }

    fn get_latest_temperature(&self) -> Option<f64> {
        self.readings.iter().rev()
            .find(|r| r.sensor_code == TEMP_SENSOR)
            .and_then(|r| r.converted_value)
    }

    fn get_latest_dosimeter_readings(&self) -> Vec<String> {
        let mut results = Vec::new();
        for &code in &DOSIMETER_LIST {
            if let Some(reading) = self.readings.iter().rev().find(|r| r.sensor_code == code) {
                results.push(format!(
                    "{}, 0x{:02X}, {}, {}, {:.2}",
                    reading.timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
                    reading.sensor_code,
                    reading.chip_name,
                    reading.raw_value,
                    reading.voltage_mv
                ));
            }
        }
        results
    }
}


// Subsystem
#[derive(Clone)]
pub struct DosimeterSubsystem {
    state: Arc<RwLock<AppState>>,
}

impl DosimeterSubsystem {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AppState::new(10000))),
        }
    }

    pub async fn add_reading(&self, reading: SensorReading) {
        let mut state = self.state.write().await;
        state.add_reading(reading);
    }

    pub async fn get_latest_temperature(&self) -> Option<f64> {
        let state = self.state.read().await;
        state.get_latest_temperature()
    }

    pub async fn get_dosimeter_readings(&self) -> Vec<String> {
        let state = self.state.read().await;
        state.get_latest_dosimeter_readings()
    }
}

// GraphQL Query definitions
#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self) -> &str {
        "pong"
    }

    async fn current_temperature(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<f64>> {
        let subsystem_ctx = ctx.data::<kubos_service::Context<DosimeterSubsystem>>()?;
        Ok(subsystem_ctx.subsystem().get_latest_temperature().await)
    }

    async fn current_dosimeter_readings(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<String>> {
        let subsystem_ctx = ctx.data::<kubos_service::Context<DosimeterSubsystem>>()?;
        Ok(subsystem_ctx.subsystem().get_dosimeter_readings().await)
    }
}

// Main I2C reading task
async fn i2c_reading_task(subsystem: DosimeterSubsystem, i2c_bus: &str) {
    match I2cdev::new(i2c_bus) {
        Ok(mut i2c) => {
            println!("I2C device opened successfully on {}", i2c_bus);
            
            // Continuous data collection loop - matching Python's while(True)
            loop {
                if let Err(e) = loop_sensors(&mut i2c) {
                    eprintln!("Error in sensor loop: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Could not open I2C device {}: {}", i2c_bus, e);
            eprintln!("Service will run but I2C functionality is unavailable");
            
            // Keep the task alive but do nothing
            loop {
                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    kubos_service::Logger::init("dosimeter-service").unwrap();
    
    let config = Config::new("dosimeter")
        .map_err(|err| {
            eprintln!("Failed to load service config: {:?}", err);
            err
        })
        .unwrap();
    
    let subsystem = DosimeterSubsystem::new();
    
    // Spawn the I2C reading task (matches Python's main loop)
    let subsystem_clone = subsystem.clone();
    tokio::spawn(async move {
        i2c_reading_task(subsystem_clone, "/dev/i2c-1").await;
    });
    
    let service = Service::new(
        config,
        subsystem,
        QueryRoot::default(),
        EmptyMutation,
    );
    
    println!("Dosimeter service starting...");
    println!("Using device: 0x{:02X}", DOSIMETER_I2C_SLAVE_ADDR);
    service.start_async().await;
}