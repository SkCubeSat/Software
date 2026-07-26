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
//!
//! **Important:** The STM32 uses a raw I2C protocol, not SMBus. Commands
//! are written as raw bytes, and responses are read as raw byte streams.
//! The `connection.write()` method handles writing the command byte
//! followed by data. For reading responses, we use `connection.read()`
//! with a dummy command (the STM32 doesn't interpret the read-phase
//! register byte — it just sends whatever is in its buffer).

use crate::commands;
use crate::error::AnalogIcResult;
use crate::rtc;
use crate::telemetry::{self, PayloadData, PowerMode, RtcTime};
use rust_i2c::{Command, Connection};
use std::thread;
use std::time::Duration;

/// Inter-command delay to prevent I2C interrupt overlap on the STM32.
/// The board documentation notes that I2C commands need to be sent
/// carefully because interrupts share the same priority.
const INTER_COMMAND_DELAY: Duration = Duration::from_millis(100);

/// Delay after writing the Send Data command (0xC5) to allow the
/// STM32 to load data from the SD card into the transfer buffer.
const SEND_DATA_LOAD_DELAY: Duration = Duration::from_millis(500);

/// Delay after the Start command (0x63) for the main loop to do
/// initial setup and create the test file.
const START_DELAY: Duration = Duration::from_millis(commands::START_COMMAND_DELAY_MS);

/// Delay after a read command (0x68, 0x69, 0x6A) to allow the STM32
/// to prepare its response buffer.
const READ_COMMAND_DELAY: Duration = Duration::from_millis(100);

/// Dummy command byte used for raw reads. The STM32 ignores the
/// register byte on reads and simply sends its prepared buffer.
const DUMMY_READ_CMD: u8 = 0x00;

/// Trait defining the expected functionality for the Analog IC payload
pub trait AnalogIc {
    /// Reset the board (0x61).
    ///
    /// Resets the Analog IC board to its initial state.
    fn reset(&self) -> AnalogIcResult<()>;

    /// Send Start command (0x63).
    ///
    /// Forces the main testing loop to run (testing the ICs).
    /// Includes a 1-second delay after sending for the board to
    /// complete initial setup and create the test file.
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

    /// Set RTC Time (0x67) using OBC system time.
    ///
    /// Gets the current time from the POSIX `date` command, sends it
    /// to the board, then verifies by reading back with Get RTC Time.
    fn set_rtc_time(&self) -> AnalogIcResult<()>;

    /// Set RTC Time with explicit bytes (0x67).
    ///
    /// # Arguments
    ///
    /// `rtc_data` - 8 bytes: [year_hi, year_lo, month, date, weekday, hours, minutes, seconds]
    fn set_rtc_time_raw(&self, rtc_data: Vec<u8>) -> AnalogIcResult<()>;

    /// Get RTC Time (0x68).
    ///
    /// Retrieves the current RTC date and time from the payload board.
    /// Returns 8 bytes: year(2, big-endian), month, day, weekday, hour, minute, second.
    fn get_rtc_time(&self) -> AnalogIcResult<RtcTime>;

    /// Check Power Status (0x69).
    ///
    /// Returns the board's current power-mode flag (normal or power-saving).
    fn check_power_status(&self) -> AnalogIcResult<PowerMode>;

    /// Check Latest Timestamp (0x6A).
    ///
    /// Returns the FAT timestamp of the most recent S_*.CSV data file
    /// on the SD card. The returned bytes identify which file to request.
    fn check_latest_timestamp(&self) -> AnalogIcResult<Vec<u8>>;

    /// Request collected data from the payload (0xC5).
    ///
    /// New protocol flow:
    /// 1. Check Latest Timestamp (0x6A) to identify the data file
    /// 2. Send Data command (0xC5) with file byte parameter
    /// 3. Read the response containing IC readings and timestamp
    ///
    /// Returns parsed `PayloadData` with IC readings and timestamp.
    fn get_payload_data(&self) -> AnalogIcResult<PayloadData>;

    /// Request collected data with an explicit file byte (0xC5).
    ///
    /// # Arguments
    ///
    /// `file_byte` - File selector byte identifying which data file to read
    fn get_payload_data_with_file(&self, file_byte: u8) -> AnalogIcResult<PayloadData>;

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

    /// Helper: Write a command and then read a response after a delay.
    ///
    /// This performs separate write and read operations rather than an
    /// SMBus combined transfer, matching the protocol the STM32 expects.
    /// The write sends the command byte, then after a delay, a separate
    /// read retrieves the response bytes.
    fn write_then_read(
        &self,
        command: Command,
        rx_len: usize,
        delay: Duration,
    ) -> AnalogIcResult<Vec<u8>> {
        // Phase 1: Write the command
        self.connection.write(command)?;

        // Wait for the STM32 to prepare its response
        thread::sleep(delay);

        // Phase 2: Read the response with a dummy command byte
        // (the STM32 just sends whatever is in its buffer)
        let dummy = Command {
            cmd: DUMMY_READ_CMD,
            data: vec![],
        };
        let data = self.connection.read(dummy, rx_len)?;
        Ok(data)
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
        // The board needs time to do initial setup after start
        thread::sleep(START_DELAY);
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
        // New flow per updated protocol:
        // 1. Check current RTC status (0x68)
        let _current_rtc = self.get_rtc_time();

        // 2. Get OBC system time and send Set RTC command (0x67)
        let rtc_data = rtc::get_system_time()?;
        self.set_rtc_time_raw(rtc_data)?;

        // 3. Verify by reading back RTC (0x68)
        let _verify_rtc = self.get_rtc_time();

        Ok(())
    }

    fn set_rtc_time_raw(&self, rtc_data: Vec<u8>) -> AnalogIcResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection
            .write(commands::set_rtc_time(rtc_data))?;
        Ok(())
    }

    fn get_rtc_time(&self) -> AnalogIcResult<RtcTime> {
        thread::sleep(INTER_COMMAND_DELAY);
        let raw = self.write_then_read(
            commands::get_rtc_time(),
            commands::GET_RTC_TIME_RESPONSE_LEN,
            READ_COMMAND_DELAY,
        )?;
        telemetry::parse_rtc_time(&raw)
    }

    fn check_power_status(&self) -> AnalogIcResult<PowerMode> {
        thread::sleep(INTER_COMMAND_DELAY);
        let raw = self.write_then_read(
            commands::check_power_status(),
            commands::CHECK_POWER_STATUS_RESPONSE_LEN,
            READ_COMMAND_DELAY,
        )?;
        telemetry::parse_power_status(&raw)
    }

    fn check_latest_timestamp(&self) -> AnalogIcResult<Vec<u8>> {
        thread::sleep(INTER_COMMAND_DELAY);
        let raw = self.write_then_read(
            commands::check_latest_timestamp(),
            commands::CHECK_LATEST_TIMESTAMP_RESPONSE_LEN,
            READ_COMMAND_DELAY,
        )?;
        Ok(raw)
    }

    fn get_payload_data(&self) -> AnalogIcResult<PayloadData> {
        // New flow per updated protocol:
        // 1. Check latest timestamp (0x6A) to identify the data file
        let ts_bytes = self.check_latest_timestamp()?;

        // Use the first byte of the timestamp response as the file selector
        let file_byte = if ts_bytes.is_empty() { 0x00 } else { ts_bytes[0] };

        // 2-3. Request data with the file byte
        self.get_payload_data_with_file(file_byte)
    }

    fn get_payload_data_with_file(&self, file_byte: u8) -> AnalogIcResult<PayloadData> {
        thread::sleep(INTER_COMMAND_DELAY);

        // Write the Send Data command with the file byte
        let command = commands::send_data(file_byte);
        let raw = self.write_then_read(
            command,
            commands::SEND_DATA_RESPONSE_LEN,
            SEND_DATA_LOAD_DELAY,
        )?;

        telemetry::parse_payload_data(&raw)
    }

    fn raw_command(&self, cmd: u8, data: Vec<u8>) -> AnalogIcResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection.write(Command { cmd, data })?;
        Ok(())
    }
}
