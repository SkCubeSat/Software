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

//! Telemetry data structures and parsing for the Analog IC payload.
//!
//! The payload board collects data from ICs arranged in a 5×10 matrix.
//! Each value is an unsigned 16-bit integer stored in little-endian byte
//! order. The data response also includes a 24-byte ASCII timestamp
//! string in the format "TS:YYYY-MM-DD hh:mm:ss\r\n".

use crate::commands::{DATA_MATRIX_COLS, DATA_MATRIX_ROWS, IC_DATA_BYTES, NUM_IC_READINGS};
use crate::error::{AnalogIcError, AnalogIcResult};

/// Parsed payload data from the Analog IC board
#[derive(Clone, Debug)]
pub struct PayloadData {
    /// 50 unsigned 16-bit IC test readings arranged in a 5×10 matrix
    /// (little-endian from the board).
    pub ic_readings: Vec<u16>,
    /// ASCII timestamp string from the board (up to 24 bytes),
    /// e.g. "TS:2025-07-25 14:30:00"
    pub timestamp: String,
    /// The full raw data buffer as received from the board
    pub raw_data: Vec<u8>,
}

impl PayloadData {
    /// Get the reading at a specific row and column of the data matrix.
    ///
    /// # Arguments
    ///
    /// * `row` - Row index (0-4)
    /// * `col` - Column index (0-9)
    ///
    /// # Returns
    ///
    /// The u16 reading value, or `None` if indices are out of range.
    pub fn get_reading(&self, row: usize, col: usize) -> Option<u16> {
        if row >= DATA_MATRIX_ROWS || col >= DATA_MATRIX_COLS {
            return None;
        }
        let idx = row * DATA_MATRIX_COLS + col;
        self.ic_readings.get(idx).copied()
    }

    /// Get the data as a 2D matrix (5 rows × 10 columns).
    pub fn as_matrix(&self) -> Vec<Vec<u16>> {
        let mut matrix = Vec::with_capacity(DATA_MATRIX_ROWS);
        for r in 0..DATA_MATRIX_ROWS {
            let start = r * DATA_MATRIX_COLS;
            let end = start + DATA_MATRIX_COLS;
            matrix.push(self.ic_readings[start..end].to_vec());
        }
        matrix
    }
}

/// RTC time data returned by the Get RTC Time command (0x68)
#[derive(Clone, Debug)]
pub struct RtcTime {
    /// Year
    pub year: u16,
    /// Month (1-12)
    pub month: u8,
    /// Day (1-31)
    pub day: u8,
    /// Weekday (0=Monday .. 6=Sunday)
    pub weekday: u8,
    /// Hour (0-23)
    pub hour: u8,
    /// Minute (0-59)
    pub minute: u8,
    /// Second (0-59)
    pub second: u8,
}

/// Power status returned by the Check Power Status command (0x69)
#[derive(Clone, Debug, PartialEq)]
pub enum PowerMode {
    /// Normal operating mode
    Normal,
    /// Power-saving / sleep mode
    PowerSaving,
    /// Unknown mode flag
    Unknown(u8),
}

/// Parse raw bytes received from the Analog IC board into structured data.
///
/// The board sends `IC_DATA_BYTES` (100) bytes of little-endian u16 readings,
/// followed by a 24-byte ASCII timestamp string.
///
/// # Arguments
///
/// * `raw` - Raw byte buffer received from the board
///
/// # Errors
///
/// Returns `AnalogIcError::ParsingFailure` if the data is shorter than
/// the minimum required length (100 bytes for readings).
pub fn parse_payload_data(raw: &[u8]) -> AnalogIcResult<PayloadData> {
    if raw.len() < IC_DATA_BYTES {
        return Err(AnalogIcError::ParsingFailure {
            source_description: format!(
                "Expected at least {} bytes of payload data, got {}",
                IC_DATA_BYTES,
                raw.len()
            ),
        });
    }

    // Parse 50 unsigned 16-bit readings in little-endian byte order
    // (matching the working Python script: data_raw[i] | (data_raw[i+1] << 8))
    let mut ic_readings = Vec::with_capacity(NUM_IC_READINGS);
    for i in 0..NUM_IC_READINGS {
        let offset = i * 2;
        let value = u16::from_le_bytes([raw[offset], raw[offset + 1]]);
        ic_readings.push(value);
    }

    // Extract the ASCII timestamp following the readings
    let timestamp = if raw.len() > IC_DATA_BYTES {
        let ts_raw = &raw[IC_DATA_BYTES..];
        // Decode printable ASCII characters, ignoring nulls/garbage
        ts_raw
            .iter()
            .filter(|&&b| b > 0 && b < 128)
            .map(|&b| b as char)
            .collect::<String>()
            .trim()
            .to_string()
    } else {
        String::new()
    };

    Ok(PayloadData {
        ic_readings,
        timestamp,
        raw_data: raw.to_vec(),
    })
}

/// Parse an 8-byte RTC time response from the Get RTC Time command (0x68).
///
/// Format: year(2 bytes, big-endian), month, day, weekday, hour, minute, second
pub fn parse_rtc_time(raw: &[u8]) -> AnalogIcResult<RtcTime> {
    if raw.len() < 8 {
        return Err(AnalogIcError::ParsingFailure {
            source_description: format!(
                "Expected 8 bytes for RTC time, got {}",
                raw.len()
            ),
        });
    }

    Ok(RtcTime {
        year: u16::from_be_bytes([raw[0], raw[1]]),
        month: raw[2],
        day: raw[3],
        weekday: raw[4],
        hour: raw[5],
        minute: raw[6],
        second: raw[7],
    })
}

/// Parse a 1-byte power status response from the Check Power Status
/// command (0x69).
pub fn parse_power_status(raw: &[u8]) -> AnalogIcResult<PowerMode> {
    if raw.is_empty() {
        return Err(AnalogIcError::ParsingFailure {
            source_description: "Expected at least 1 byte for power status, got 0".to_string(),
        });
    }

    Ok(match raw[0] {
        0 => PowerMode::Normal,
        1 => PowerMode::PowerSaving,
        other => PowerMode::Unknown(other),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::SEND_DATA_RESPONSE_LEN;

    #[test]
    fn test_parse_payload_data_valid() {
        let mut raw = vec![0u8; SEND_DATA_RESPONSE_LEN];
        // Set first reading to 0x0201 (little-endian: low=0x01, high=0x02 => 513)
        raw[0] = 0x01;
        raw[1] = 0x02;
        // Set reading at row 4, col 9 (index 49, bytes 98-99)
        raw[98] = 0x2B;
        raw[99] = 0x1A;

        let data = parse_payload_data(&raw).unwrap();
        assert_eq!(data.ic_readings.len(), NUM_IC_READINGS);
        assert_eq!(data.ic_readings[0], 0x0201); // little-endian
        assert_eq!(data.ic_readings[49], 0x1A2B); // little-endian
    }

    #[test]
    fn test_parse_payload_data_too_short() {
        let raw = vec![0u8; 50]; // Less than minimum 100 bytes
        let result = parse_payload_data(&raw);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_reading() {
        let mut raw = vec![0u8; SEND_DATA_RESPONSE_LEN];
        // Row 2, col 1 => index 2*10+1 = 21, bytes 42-43
        raw[42] = 0xCD;
        raw[43] = 0xAB;

        let data = parse_payload_data(&raw).unwrap();
        assert_eq!(data.get_reading(2, 1), Some(0xABCD));
        assert_eq!(data.get_reading(5, 0), None); // Out of range
        assert_eq!(data.get_reading(0, 10), None); // Out of range
    }

    #[test]
    fn test_as_matrix() {
        let raw = vec![0u8; SEND_DATA_RESPONSE_LEN];
        let data = parse_payload_data(&raw).unwrap();
        let matrix = data.as_matrix();
        assert_eq!(matrix.len(), 5);
        assert_eq!(matrix[0].len(), 10);
    }

    #[test]
    fn test_parse_rtc_time() {
        let raw = vec![0x07, 0xD5, 11, 19, 5, 9, 33, 11];
        let rtc = parse_rtc_time(&raw).unwrap();
        assert_eq!(rtc.year, 2005);
        assert_eq!(rtc.month, 11);
        assert_eq!(rtc.day, 19);
        assert_eq!(rtc.weekday, 5);
        assert_eq!(rtc.hour, 9);
        assert_eq!(rtc.minute, 33);
        assert_eq!(rtc.second, 11);
    }

    #[test]
    fn test_parse_power_status() {
        assert_eq!(parse_power_status(&[0]).unwrap(), PowerMode::Normal);
        assert_eq!(parse_power_status(&[1]).unwrap(), PowerMode::PowerSaving);
        assert_eq!(parse_power_status(&[2]).unwrap(), PowerMode::Unknown(2));
        assert!(parse_power_status(&[]).is_err());
    }
}
