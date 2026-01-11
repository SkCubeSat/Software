use async_graphql::{Context, EmptyMutation, Object};
use kubos_service::{Config, Service};
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use linux_embedded_hal::I2cdev;
use embedded_hal::i2c::I2c;

// Constants
const DOSIMETER_I2C_ADDR: u8 = 0x4A;
const TEMPERATURE_SCALE_SLOPE: f64 = 1.0 / -13.6;
const TEMPERATURE_SCALE_OFFSET: f64 = 192.48;
const DOSIMETER_COMMAND_LENGTH: usize = 1;
const DOSIMETER_RESPONSE_LENGTH: usize = 2;
const ADC_BIT_RESOLUTION: u32 = 12;
const MAX_ADC_VALUE: f64 = ((1 << 12) - 1) as f64;
const TIMER_DELAY_MS: u64 = 100;

// Sensor channel codes
const DOSIMETER_CHANNELS: [u8; 7] = [
    0x84, // channel 0 -> u1 -> None
    0xC4, // channel 1 -> u2 -> 50 mil
    0x94, // channel 2 -> u3 -> 100 mil
    0xD4, // channel 3 -> u4 -> 200 mil
    0xA4, // channel 4 -> u5 -> 20 mil
    0xE4, // channel 5 -> radfet
    0xB4, // channel 6 -> u7 -> 300 mil
];
const TEMP_SENSOR: u8 = 0xF4; // channel 7