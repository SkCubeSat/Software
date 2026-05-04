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
//! The command protocol uses single-byte commands, with Set RTC Time
//! requiring an additional 8-byte data payload.

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
/// Command byte for Send Data / Request Data (0xC5)
pub const CMD_SEND_DATA: u8 = 0xC5;

/// Expected response length for the Send Data command.
/// 27 unsigned 16-bit readings (54 bytes) + 6 bytes timestamp = 60 bytes.
///
/// Note: The payload documentation states "107 bytes" but based on the
/// data description (27 u16 values + 6 timestamp bytes), the actual
/// payload is 60 bytes. We use 107 as specified in the documentation
/// to match the board's actual transfer size (which may include padding
/// or additional framing).
pub const SEND_DATA_RESPONSE_LEN: usize = 107;

/// Number of test IC readings in a data payload
pub const NUM_IC_READINGS: usize = 27;

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
/// `rtc_data` - 8 bytes representing:
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

/// Build a Send Data command (0xC5).
///
/// Asks the payload board to load the latest collected data into the
/// transmit buffer. After sending this command, the OBC should perform
/// a read to retrieve the data.
///
/// Returns the command and the expected response length.
pub fn send_data() -> (Command, usize) {
    (
        Command {
            cmd: CMD_SEND_DATA,
            data: vec![],
        },
        SEND_DATA_RESPONSE_LEN,
    )
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
        let (cmd, rx_len) = send_data();
        assert_eq!(cmd.cmd, 0xC5);
        assert_eq!(rx_len, SEND_DATA_RESPONSE_LEN);
    }
}
