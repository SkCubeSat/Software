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

//! Command definitions for the Analog IC payload board.
//!
//! Each command is represented as a function returning a `rust_i2c::Command`.
//! The Analog IC board uses a simple command protocol where single-byte
//! commands are sent as raw I2C writes, and responses are read as raw
//! I2C reads (not SMBus register-based operations).

use rust_i2c::Command;

/// Command byte for Reset (0x61)
pub const CMD_RESET: u8 = 0x61;
/// Command byte for Stop (0x62) — not implemented on board
pub const CMD_STOP: u8 = 0x62;
/// Command byte for Start (0x63)
pub const CMD_START: u8 = 0x63;
/// Command byte for Normal (0x64) — not implemented on board
pub const CMD_NORMAL: u8 = 0x64;
/// Command byte for Power Saving Mode (0x65)
pub const CMD_POWER_SAVING_MODE: u8 = 0x65;
/// Command byte for Normal Power Mode (0x66)
pub const CMD_NORMAL_POWER_MODE: u8 = 0x66;
/// Command byte for Set RTC Time (0x67)
pub const CMD_SET_RTC_TIME: u8 = 0x67;
/// Command byte for Get RTC Time (0x68)
pub const CMD_GET_RTC_TIME: u8 = 0x68;
/// Command byte for Check Power Status (0x69)
pub const CMD_CHECK_POWER_STATUS: u8 = 0x69;
/// Command byte for Check Latest Timestamp (0x6A)
pub const CMD_CHECK_LATEST_TIMESTAMP: u8 = 0x6A;
/// Command byte for Send Data / Request Data (0xC5)
pub const CMD_SEND_DATA: u8 = 0xC5;

/// Response length for Get RTC Time — 8 bytes:
/// year(2 bytes big-endian), month, day, weekday, hour, minute, second
pub const GET_RTC_TIME_RESPONSE_LEN: usize = 8;

/// Response length for Check Power Status — 1 byte:
/// 0 = normal, 1 = power-saving
pub const CHECK_POWER_STATUS_RESPONSE_LEN: usize = 1;

/// Response length for Check Latest Timestamp — variable but expected
/// to contain FAT timestamp fields (year, month, day, hour, minute, second)
pub const CHECK_LATEST_TIMESTAMP_RESPONSE_LEN: usize = 6;

/// Expected response length for the Send Data command (107 bytes as
/// documented, but actual data is 5×10 matrix of u16 values + 24-byte
/// ASCII timestamp = 124 bytes based on tested board output).
pub const SEND_DATA_RESPONSE_LEN: usize = 124;

/// Number of rows in the IC data matrix
pub const DATA_MATRIX_ROWS: usize = 5;

/// Number of columns in the IC data matrix
pub const DATA_MATRIX_COLS: usize = 10;

/// Total number of u16 IC readings in a data payload (5×10 = 50)
pub const NUM_IC_READINGS: usize = DATA_MATRIX_ROWS * DATA_MATRIX_COLS;

/// Byte count for the IC readings portion (50 readings × 2 bytes = 100)
pub const IC_DATA_BYTES: usize = NUM_IC_READINGS * 2;

/// Length of the ASCII timestamp string appended to data
pub const TIMESTAMP_ASCII_LEN: usize = 24;

/// Delay required after Start command (0x63) for the main loop to
/// do initial setup and create the test file (1 second).
pub const START_COMMAND_DELAY_MS: u64 = 1000;

/// Build a Reset command (0x61).
///
/// Resets the Analog IC board.
pub fn reset() -> Command {
    Command {
        cmd: CMD_RESET,
        data: vec![],
    }
}

/// Build a Stop command (0x62).
///
/// Note: This command is not implemented on the board.
pub fn stop() -> Command {
    Command {
        cmd: CMD_STOP,
        data: vec![],
    }
}

/// Build a Start command (0x63).
///
/// Forces the main testing loop (running tests for the ICs).
/// Note: After sending this command, a 1-second delay is required
/// for the main loop to do initial setup and create the test file.
pub fn start() -> Command {
    Command {
        cmd: CMD_START,
        data: vec![],
    }
}

/// Build a Normal command (0x64).
///
/// Note: This command is not implemented on the board.
pub fn normal() -> Command {
    Command {
        cmd: CMD_NORMAL,
        data: vec![],
    }
}

/// Build a Power Saving Mode command (0x65).
///
/// Turns off the 5V power plane and puts the controller into sleep mode.
/// Power consumption drops to approximately 50 mW.
pub fn power_saving_mode() -> Command {
    Command {
        cmd: CMD_POWER_SAVING_MODE,
        data: vec![],
    }
}

/// Build a Normal Power Mode command (0x66).
///
/// Turns on the 5V power plane. The controller remains in sleep mode
/// but will run tests every 12 hours.
pub fn normal_power_mode() -> Command {
    Command {
        cmd: CMD_NORMAL_POWER_MODE,
        data: vec![],
    }
}

/// Build a Set RTC Time command (0x67).
///
/// Sets up the real-time clock on the STM32 for timestamping test results.
/// This command must be sent on every startup/reboot of the board.
///
/// # Arguments
///
/// `rtc_data` - 7 bytes representing:
///   - Bytes 0-1: Year (big-endian, e.g. 2005 = 0x07, 0xD5)
///   - Byte 2: Month (1-12)
///   - Byte 3: Date (1-31)
///   - Byte 4: Weekday (0=Monday .. 6=Sunday)
///   - Byte 5: Hours (0-23)
///   - Byte 6: Minutes (0-59)
///   - Byte 7: Seconds (0-59)
pub fn set_rtc_time(rtc_data: Vec<u8>) -> Command {
    Command {
        cmd: CMD_SET_RTC_TIME,
        data: rtc_data,
    }
}

/// Build a Get RTC Time command (0x68).
///
/// Retrieves the current RTC date and time from the payload board.
/// After writing this command, the OBC should read 8 bytes:
///   year (2 bytes, big-endian), month, day, weekday, hour, minute, second.
pub fn get_rtc_time() -> Command {
    Command {
        cmd: CMD_GET_RTC_TIME,
        data: vec![],
    }
}

/// Build a Check Power Status command (0x69).
///
/// Asks the board to return its current power-mode flag.
/// After writing this command, the OBC should read 1 byte:
///   0 = normal, 1 = power-saving.
pub fn check_power_status() -> Command {
    Command {
        cmd: CMD_CHECK_POWER_STATUS,
        data: vec![],
    }
}

/// Build a Check Latest Timestamp command (0x6A).
///
/// Asks the board to return the FAT timestamp (year, month, day, hour,
/// minute, second) of the most recent S_*.CSV data file on the SD card.
/// The response is used to identify which file to request with Send Data.
pub fn check_latest_timestamp() -> Command {
    Command {
        cmd: CMD_CHECK_LATEST_TIMESTAMP,
        data: vec![],
    }
}

/// Build a Send Data command (0xC5) with a file selector byte.
///
/// Asks the payload board to load the specified data file into the
/// transmit buffer. After sending this command, the OBC should perform
/// a read to retrieve the data.
///
/// # Arguments
///
/// `file_byte` - File selector byte identifying which data file to read
pub fn send_data(file_byte: u8) -> Command {
    Command {
        cmd: CMD_SEND_DATA,
        data: vec![file_byte],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_command() {
        let cmd = reset();
        assert_eq!(cmd.cmd, 0x61);
        assert!(cmd.data.is_empty());
    }

    #[test]
    fn test_set_rtc_time_command() {
        let rtc_bytes = vec![0x07, 0xD5, 0x0B, 0x13, 0x05, 0x09, 0x21, 0x0B];
        let cmd = set_rtc_time(rtc_bytes.clone());
        assert_eq!(cmd.cmd, 0x67);
        assert_eq!(cmd.data, rtc_bytes);
    }

    #[test]
    fn test_send_data_command() {
        let cmd = send_data(0x01);
        assert_eq!(cmd.cmd, 0xC5);
        assert_eq!(cmd.data, vec![0x01]);
    }

    #[test]
    fn test_get_rtc_time_command() {
        let cmd = get_rtc_time();
        assert_eq!(cmd.cmd, 0x68);
        assert!(cmd.data.is_empty());
    }

    #[test]
    fn test_check_power_status_command() {
        let cmd = check_power_status();
        assert_eq!(cmd.cmd, 0x69);
        assert!(cmd.data.is_empty());
    }

    #[test]
    fn test_check_latest_timestamp_command() {
        let cmd = check_latest_timestamp();
        assert_eq!(cmd.cmd, 0x6A);
        assert!(cmd.data.is_empty());
    }
}
