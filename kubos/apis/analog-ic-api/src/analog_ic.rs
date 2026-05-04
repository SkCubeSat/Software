/*
 * Copyright (C) 2025 USST CUBICS
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! High-level interface for communicating with the Analog IC payload board.
//!
//! This module provides the `AnalogIc` trait defining the expected API, and
//! `AnalogIcPayload` which implements it using a `rust_i2c::Connection`.
//!
//! The Analog IC board communicates over I2C at address 0x13. It requires
//! an inter-command delay because I2C interrupts on the STM32 have the
//! same priority, and overlapping commands may cause issues.

use crate::commands;
use crate::error::AnalogIcResult;
use crate::rtc;
use crate::telemetry::{self, PayloadData};
use rust_i2c::{Command, Connection};
use std::thread;
use std::time::Duration;

/// Inter-command delay to prevent I2C interrupt overlap on the STM32.
/// The board documentation notes that I2C commands need to be sent
/// carefully because interrupts share the same priority.
const INTER_COMMAND_DELAY: Duration = Duration::from_millis(100);

/// Delay between write and read phases of the Send Data command.
/// The board needs time to load data from the SD card into the
/// transfer buffer.
const SEND_DATA_LOAD_DELAY: Duration = Duration::from_millis(500);

/// Trait defining the expected functionality for the Analog IC payload
pub trait AnalogIc {
    /// Reset the board (0x61).
    ///
    /// Resets the Analog IC board to its initial state.
    fn reset(&self) -> AnalogIcResult<()>;

    /// Send Start command (0x63).
    ///
    /// Forces the main testing loop to run (testing the ICs).
    fn start(&self) -> AnalogIcResult<()>;

    /// Send Power Saving Mode command (0x65).
    ///
    /// Turns off the 5V power plane, puts the controller to sleep.
    /// Power consumption drops to approximately 50 mW.
    fn power_saving_mode(&self) -> AnalogIcResult<()>;

    /// Send Normal Power Mode command (0x66).
    ///
    /// Turns on the 5V power plane. Controller stays in sleep mode
    /// but will run tests every 12 hours.
    fn normal_power_mode(&self) -> AnalogIcResult<()>;

    /// Set RTC Time (0x67).
    ///
    /// Sets the real-time clock on the STM32 using the current OBC
    /// system time. This must be called on every startup/reboot.
    ///
    /// The time is obtained from the POSIX `date` command and converted
    /// to the 8-byte RTC format.
    fn set_rtc_time(&self) -> AnalogIcResult<()>;

    /// Set RTC Time with explicit bytes (0x67).
    ///
    /// Sets the real-time clock using a pre-built 8-byte RTC payload.
    ///
    /// # Arguments
    ///
    /// `rtc_data` - 8 bytes: [year_hi, year_lo, month, date, weekday, hours, minutes, seconds]
    fn set_rtc_time_raw(&self, rtc_data: Vec<u8>) -> AnalogIcResult<()>;

    /// Request collected data from the payload (0xC5).
    ///
    /// This performs the two-phase data retrieval:
    /// 1. Write the Send Data command to tell the board to load its buffer
    /// 2. Read the 107-byte response containing IC readings and timestamp
    ///
    /// Returns parsed `PayloadData` with 27 IC readings and timestamp.
    fn get_payload_data(&self) -> AnalogIcResult<PayloadData>;

    /// Issue a raw command to the board.
    ///
    /// # Arguments
    ///
    /// `cmd` - Command byte
    /// `data` - Data bytes to send with the command
    fn raw_command(&self, cmd: u8, data: Vec<u8>) -> AnalogIcResult<()>;
}

/// Analog IC payload structure containing the low-level I2C connection.
pub struct AnalogIcPayload {
    connection: Connection,
}

impl AnalogIcPayload {
    /// Create a new AnalogIcPayload instance.
    ///
    /// # Arguments
    ///
    /// `connection` - An I2C [`Connection`] to the Analog IC board
    ///
    /// [`Connection`]: ../rust_i2c/struct.Connection.html
    pub fn new(connection: Connection) -> Self {
        AnalogIcPayload { connection }
    }
}

impl AnalogIc for AnalogIcPayload {
    fn reset(&self) -> AnalogIcResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection.write(commands::reset())?;
        Ok(())
    }

    fn start(&self) -> AnalogIcResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection.write(commands::start())?;
        Ok(())
    }

    fn power_saving_mode(&self) -> AnalogIcResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection.write(commands::power_saving_mode())?;
        Ok(())
    }

    fn normal_power_mode(&self) -> AnalogIcResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection.write(commands::normal_power_mode())?;
        Ok(())
    }

    fn set_rtc_time(&self) -> AnalogIcResult<()> {
        let rtc_data = rtc::get_system_time()?;
        self.set_rtc_time_raw(rtc_data)
    }

    fn set_rtc_time_raw(&self, rtc_data: Vec<u8>) -> AnalogIcResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection
            .write(commands::set_rtc_time(rtc_data))?;
        Ok(())
    }

    fn get_payload_data(&self) -> AnalogIcResult<PayloadData> {
        thread::sleep(INTER_COMMAND_DELAY);
        let (command, rx_len) = commands::send_data();
        // The Send Data command uses a two-phase protocol:
        // 1. Write the command (board loads SD card data into buffer)
        // 2. Read the response after a delay
        let raw = self
            .connection
            .transfer(command, rx_len, SEND_DATA_LOAD_DELAY)?;
        telemetry::parse_payload_data(&raw)
    }

    fn raw_command(&self, cmd: u8, data: Vec<u8>) -> AnalogIcResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection.write(Command { cmd, data })?;
        Ok(())
    }
}
