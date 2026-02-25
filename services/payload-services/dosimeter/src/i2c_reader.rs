use rust_i2c::{Command, Connection};
use std::collections::HashMap;
use std::io;
use std::time::Duration;

pub const DOSIMETER_RESPONSE_LENGTH: usize = 2;
pub const DOSIMETER_RESPONSE_HIGH_BYTE_MASK: u8 = 0x0F;

pub const DOSIMETER_I2C_DELAY_MS: u64 = 0;

pub const RETRIES: usize = 3;
pub const RETRY_BACKOFF_MS: u64 = 10;

pub const MAX_ADC: f64 = 4095.0;
pub const REF_MV: f64 = 3300.0;
pub const TEMP_SLOPE: f64 = 1.0 / -13.6;
pub const TEMP_OFFSET: f64 = 192.48;

pub fn to_mv(adc: u16) -> f64 {
    (adc as f64 / MAX_ADC) * REF_MV
}

pub fn mv_to_temp_c(mv: f64) -> f64 {
    (mv * TEMP_SLOPE) + TEMP_OFFSET
}

pub fn parse_adc_12bit(msb: u8, lsb: u8) -> u16 {
    (((msb & DOSIMETER_RESPONSE_HIGH_BYTE_MASK) as u16) << 8) | (lsb as u16)
}

pub fn read_sensor_adc(conn: &Connection, sensor_code: u8) -> io::Result<u16> {
    let delay = Duration::from_millis(DOSIMETER_I2C_DELAY_MS);

    let mut last_err: Option<io::Error> = None;

    for attempt in 1..=RETRIES {
        match conn.transfer(
            Command {
                cmd: sensor_code,
                data: vec![],
            },
            DOSIMETER_RESPONSE_LENGTH,
            delay,
        ) {
            Ok(data) if data.len() == 2 => {
                let adc = parse_adc_12bit(data[0], data[1]);
                return Ok(adc);
            }
            Ok(data) => {
                last_err = Some(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    format!("Expected 2 bytes, got {}", data.len()),
                ));
            }
            Err(e) => last_err = Some(e),
        }

        if attempt < RETRIES {
            std::thread::sleep(Duration::from_millis(RETRY_BACKOFF_MS));
        }
    }

    Err(last_err.unwrap_or_else(|| io::Error::new(io::ErrorKind::Other, "Unknown I2C error")))
}

pub fn chip_name_map() -> HashMap<u8, &'static str> {
    HashMap::from([
        (0x84, "u1"),
        (0xC4, "u2"),
        (0x94, "u3"),
        (0xD4, "u4"),
        (0xA4, "u5"),
        (0xE4, "radfet"),
        (0xB4, "u7"),
        (0xF4, "temp"),
    ])
}