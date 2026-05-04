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
//! The payload board collects data from 9 testing ICs every 12 hours.
//! Each IC is tested 5 times; the highest and lowest readings are
//! discarded, leaving 3 readings per IC (27 total unsigned 16-bit values).
//! The data also includes a 6-byte timestamp indicating when the data
//! was collected.

use crate::commands::NUM_IC_READINGS;
use crate::error::{AnalogIcError, AnalogIcResult};

/// Number of ICs being tested
pub const NUM_ICS: usize = 9;

/// Number of retained readings per IC (5 tests, discard highest and lowest)
pub const READINGS_PER_IC: usize = 3;

/// Parsed payload data from the Analog IC board
#[derive(Clone, Debug)]
pub struct PayloadData {
    /// 27 unsigned 16-bit IC test readings.
    /// Organized as 9 ICs × 3 readings each (big-endian from the board).
    pub ic_readings: Vec<u16>,
    /// Raw timestamp bytes (6 bytes) indicating when the data was collected.
    /// Format is board-specific (likely: year_hi, year_lo, month, day, hour, minute
    /// or similar — matching the RTC format minus weekday and seconds).
    pub timestamp_bytes: Vec<u8>,
    /// The full raw data buffer as received from the board (107 bytes)
    pub raw_data: Vec<u8>,
}

impl PayloadData {
    /// Get the reading for a specific IC and reading index.
    ///
    /// # Arguments
    ///
    /// * `ic_index` - IC index (0-8)
    /// * `reading_index` - Reading index (0-2)
    ///
    /// # Returns
    ///
    /// The u16 reading value, or `None` if indices are out of range.
    pub fn get_reading(&self, ic_index: usize, reading_index: usize) -> Option<u16> {
        if ic_index >= NUM_ICS || reading_index >= READINGS_PER_IC {
            return None;
        }
        let idx = ic_index * READINGS_PER_IC + reading_index;
        self.ic_readings.get(idx).copied()
    }
}

/// Parse raw bytes received from the Analog IC board into structured data.
///
/// # Arguments
///
/// * `raw` - Raw byte buffer received from the board (expected 107 bytes)
///
/// # Returns
///
/// Parsed `PayloadData` containing the 27 IC readings and 6-byte timestamp.
///
/// # Errors
///
/// Returns `AnalogIcError::ParsingFailure` if the data length is less than
/// the minimum required (54 bytes for readings + 6 bytes for timestamp).
pub fn parse_payload_data(raw: &[u8]) -> AnalogIcResult<PayloadData> {
    // Minimum required: 27 readings * 2 bytes + 6 timestamp bytes = 60 bytes
    let min_len = NUM_IC_READINGS * 2 + 6;
    if raw.len() < min_len {
        return Err(AnalogIcError::ParsingFailure {
            source_description: format!(
                "Expected at least {} bytes of payload data, got {}",
                min_len,
                raw.len()
            ),
        });
    }

    // Parse 27 unsigned 16-bit readings (big-endian)
    let mut ic_readings = Vec::with_capacity(NUM_IC_READINGS);
    for i in 0..NUM_IC_READINGS {
        let offset = i * 2;
        let value = u16::from_be_bytes([raw[offset], raw[offset + 1]]);
        ic_readings.push(value);
    }

    // Extract the 6-byte timestamp following the readings
    let timestamp_start = NUM_IC_READINGS * 2; // byte 54
    let timestamp_bytes = raw[timestamp_start..timestamp_start + 6].to_vec();

    Ok(PayloadData {
        ic_readings,
        timestamp_bytes,
        raw_data: raw.to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_payload_data_valid() {
        // Create a 107-byte buffer with known values
        let mut raw = vec![0u8; SEND_DATA_RESPONSE_LEN];
        // Set first IC reading to 0x0102 = 258
        raw[0] = 0x01;
        raw[1] = 0x02;
        // Set last IC reading (index 26) to 0x1A2B
        raw[52] = 0x1A;
        raw[53] = 0x2B;
        // Set timestamp bytes
        raw[54] = 0x07; // year hi
        raw[55] = 0xD5; // year lo
        raw[56] = 0x0B; // month
        raw[57] = 0x13; // date
        raw[58] = 0x09; // hour
        raw[59] = 0x21; // minute

        let data = parse_payload_data(&raw).unwrap();
        assert_eq!(data.ic_readings.len(), NUM_IC_READINGS);
        assert_eq!(data.ic_readings[0], 0x0102);
        assert_eq!(data.ic_readings[26], 0x1A2B);
        assert_eq!(data.timestamp_bytes, vec![0x07, 0xD5, 0x0B, 0x13, 0x09, 0x21]);
        assert_eq!(data.raw_data.len(), SEND_DATA_RESPONSE_LEN);
    }

    #[test]
    fn test_parse_payload_data_too_short() {
        let raw = vec![0u8; 50]; // Less than minimum 60 bytes
        let result = parse_payload_data(&raw);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_reading() {
        let mut raw = vec![0u8; SEND_DATA_RESPONSE_LEN];
        // IC 2, reading 1 => index 2*3+1 = 7, bytes 14-15
        raw[14] = 0xAB;
        raw[15] = 0xCD;

        let data = parse_payload_data(&raw).unwrap();
        assert_eq!(data.get_reading(2, 1), Some(0xABCD));
        assert_eq!(data.get_reading(9, 0), None); // Out of range
        assert_eq!(data.get_reading(0, 3), None); // Out of range
    }
}
