use async_graphql::{Context, Object, Result, SimpleObject};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::i2c_reader::{read_sensor_adc, chip_name_map, to_mv, mv_to_temp_c};
use rust_i2c::Connection;

const DOSIMETER_LIST: [u8; 7] = [0x84, 0xC4, 0x94, 0xD4, 0xA4, 0xE4, 0xB4];
const TEMP_SENSOR: u8 = 0xF4;
const TIMER_DELAY_MS: u64 = 100;

#[derive(Clone)]
pub struct Subsystem {
    pub i2c_connection: Arc<Mutex<Connection>>,
}

impl Subsystem {
    pub fn new(connection: Connection) -> Self {
        Subsystem {
            i2c_connection: Arc::new(Mutex::new(connection)),
        }
    }
}

#[derive(Debug, SimpleObject)]
pub struct SensorReading {
    /// Sensor address (hex)
    pub address: String,
    /// Sensor name (u1, u2, etc.)
    pub name: String,
    /// Raw ADC value
    pub adc: i32,
    /// Whether the reading was successful
    pub success: bool,
    /// Error message if reading failed
    pub error: Option<String>,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Test service query
    async fn ping(&self) -> &str {
        "pong"
    }

    /// Read all sensor values from the dosimeter
    async fn read(&self, ctx: &Context<'_>) -> Result<Vec<SensorReading>> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let conn = context.subsystem().i2c_connection.lock().map_err(|err| {
            log::error!("Failed to get lock on I2C connection: {:?}", err);
            async_graphql::Error::new(format!("I2C connection lock error: {}", err))
        })?;

        let map = chip_name_map();
        let mut readings = Vec::new();

        // Read dosimeter sensors
        for &code in &DOSIMETER_LIST {
            let name = map.get(&code).copied().unwrap_or("unknown");
            let address = format!("0x{:02X}", code);

            match read_sensor_adc(&conn, code) {
                Ok(adc) => {
                    readings.push(SensorReading {
                        address,
                        name: name.to_string(),
                        adc: adc as i32,
                        success: true,
                        error: None,
                    });
                }
                Err(e) => {
                    readings.push(SensorReading {
                        address,
                        name: name.to_string(),
                        adc: 0,
                        success: false,
                        error: Some(e.to_string()),
                    });
                }
            }

            std::thread::sleep(Duration::from_millis(TIMER_DELAY_MS));
        }

        // Read temperature sensor
        {
            let code = TEMP_SENSOR;
            let name = map.get(&code).copied().unwrap_or("temp");
            let address = format!("0x{:02X}", code);

            match read_sensor_adc(&conn, code) {
                Ok(adc) => {
                    readings.push(SensorReading {
                        address,
                        name: name.to_string(),
                        adc: adc as i32,
                        success: true,
                        error: None,
                    });
                }
                Err(e) => {
                    readings.push(SensorReading {
                        address,
                        name: name.to_string(),
                        adc: 0,
                        success: false,
                        error: Some(e.to_string()),
                    });
                }
            }
        }

        Ok(readings)
    }

    /// Read a specific sensor by address
    async fn read_sensor(
        &self,
        ctx: &Context<'_>,
        address: String,
    ) -> Result<SensorReading> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let conn = context.subsystem().i2c_connection.lock().map_err(|err| {
            log::error!("Failed to get lock on I2C connection: {:?}", err);
            async_graphql::Error::new(format!("I2C connection lock error: {}", err))
        })?;

        // Parse address (expect "0x84" format or just "84")
        let addr_str = address.trim_start_matches("0x").trim_start_matches("0X");
        let code = u8::from_str_radix(addr_str, 16).map_err(|e| {
            async_graphql::Error::new(format!("Invalid address format: {}", e))
        })?;

        let map = chip_name_map();
        let name = map.get(&code).copied().unwrap_or("unknown");
        let address_formatted = format!("0x{:02X}", code);

        match read_sensor_adc(&conn, code) {
            Ok(adc) => {
                Ok(SensorReading {
                    address: address_formatted,
                    name: name.to_string(),
                    adc: adc as i32,
                    success: true,
                    error: None,
                })
            }
            Err(e) => Ok(SensorReading {
                address: address_formatted,
                name: name.to_string(),
                adc: 0,
                success: false,
                error: Some(e.to_string()),
            }),
        }
    }

    /// Convert ADC reading to voltage in millivolts
    async fn voltage_convert(&self, adc: i32) -> f64 {
        to_mv(adc as u16)
    }

    /// Convert voltage (in millivolts) to temperature in Celsius
    async fn temp_convert(&self, voltage_mv: f64) -> f64 {
        mv_to_temp_c(voltage_mv)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Placeholder mutation (can add calibration, etc.)
    async fn noop(&self) -> bool {
        true
    }
}