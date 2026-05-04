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

//! RTC time utility for the Analog IC payload.
//!
//! Provides functions to retrieve the system time from the OBC using the
//! POSIX `date` command and convert it into the 8-byte format required
//! by the Analog IC board's Set RTC Time command (0x67).
//!
//! The `date` command output on KubOS looks like:
//!     `Sat Nov 19 09:33:11 UTC 2005`
//!
//! The 8-byte RTC format is:
//!     [year_hi, year_lo, month, date, weekday, hours, minutes, seconds]
//!
//! Where weekday is 0=Monday .. 6=Sunday.

use crate::error::{AnalogIcError, AnalogIcResult};
use std::process::Command;

/// Parse the 3-letter month abbreviation to a 1-based month number.
fn parse_month(month_str: &str) -> AnalogIcResult<u8> {
    match month_str {
        "Jan" => Ok(1),
        "Feb" => Ok(2),
        "Mar" => Ok(3),
        "Apr" => Ok(4),
        "May" => Ok(5),
        "Jun" => Ok(6),
        "Jul" => Ok(7),
        "Aug" => Ok(8),
        "Sep" => Ok(9),
        "Oct" => Ok(10),
        "Nov" => Ok(11),
        "Dec" => Ok(12),
        _ => Err(AnalogIcError::ParsingFailure {
            source_description: format!("Unknown month abbreviation: '{}'", month_str),
        }),
    }
}

/// Parse the 3-letter day abbreviation to a weekday number (0=Monday .. 6=Sunday).
fn parse_weekday(day_str: &str) -> AnalogIcResult<u8> {
    match day_str {
        "Mon" => Ok(0),
        "Tue" => Ok(1),
        "Wed" => Ok(2),
        "Thu" => Ok(3),
        "Fri" => Ok(4),
        "Sat" => Ok(5),
        "Sun" => Ok(6),
        _ => Err(AnalogIcError::ParsingFailure {
            source_description: format!("Unknown day abbreviation: '{}'", day_str),
        }),
    }
}

/// Parse a date string in the format produced by the POSIX `date` command
/// on KubOS: `Sat Nov 19 09:33:11 UTC 2005`
///
/// Returns 8 bytes: [year_hi, year_lo, month, date, weekday, hours, minutes, seconds]
pub fn parse_date_string(date_str: &str) -> AnalogIcResult<Vec<u8>> {
    let parts: Vec<&str> = date_str.trim().split_whitespace().collect();

    // Expected format: "Sat Nov 19 09:33:11 UTC 2005"
    // Parts:            [0]  [1] [2] [3]      [4]  [5]
    if parts.len() < 6 {
        return Err(AnalogIcError::ParsingFailure {
            source_description: format!(
                "Date string has unexpected format (expected 6 parts, got {}): '{}'",
                parts.len(),
                date_str
            ),
        });
    }

    let weekday = parse_weekday(parts[0])?;
    let month = parse_month(parts[1])?;

    let date: u8 = parts[2].parse().map_err(|_| AnalogIcError::ParsingFailure {
        source_description: format!("Failed to parse day of month: '{}'", parts[2]),
    })?;

    // Parse time "HH:MM:SS"
    let time_parts: Vec<&str> = parts[3].split(':').collect();
    if time_parts.len() != 3 {
        return Err(AnalogIcError::ParsingFailure {
            source_description: format!("Failed to parse time: '{}'", parts[3]),
        });
    }

    let hours: u8 = time_parts[0]
        .parse()
        .map_err(|_| AnalogIcError::ParsingFailure {
            source_description: format!("Failed to parse hours: '{}'", time_parts[0]),
        })?;
    let minutes: u8 = time_parts[1]
        .parse()
        .map_err(|_| AnalogIcError::ParsingFailure {
            source_description: format!("Failed to parse minutes: '{}'", time_parts[1]),
        })?;
    let seconds: u8 = time_parts[2]
        .parse()
        .map_err(|_| AnalogIcError::ParsingFailure {
            source_description: format!("Failed to parse seconds: '{}'", time_parts[2]),
        })?;

    // parts[4] is the timezone (e.g. "UTC"), we skip it
    let year: u16 = parts[5].parse().map_err(|_| AnalogIcError::ParsingFailure {
        source_description: format!("Failed to parse year: '{}'", parts[5]),
    })?;

    let year_hi = (year >> 8) as u8;
    let year_lo = (year & 0xFF) as u8;

    Ok(vec![
        year_hi, year_lo, month, date, weekday, hours, minutes, seconds,
    ])
}

/// Get the current system time from the OBC by running the `date` command,
/// and return it as 8 bytes suitable for the Set RTC Time command.
///
/// # Errors
///
/// Returns an error if the `date` command fails to execute or if its
/// output cannot be parsed.
pub fn get_system_time() -> AnalogIcResult<Vec<u8>> {
    let output = Command::new("date")
        .output()
        .map_err(|e| AnalogIcError::CommandFailure {
            command: format!("Failed to execute 'date' command: {}", e),
        })?;

    if !output.status.success() {
        return Err(AnalogIcError::CommandFailure {
            command: format!(
                "'date' command failed with status: {}",
                output.status
            ),
        });
    }

    let date_str = String::from_utf8_lossy(&output.stdout);
    parse_date_string(&date_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_string_example() {
        // Example from the user: "Sat Nov 19 09:33:11 UTC 2005"
        let result = parse_date_string("Sat Nov 19 09:33:11 UTC 2005").unwrap();
        assert_eq!(result.len(), 8);
        // Year 2005 = 0x07D5
        assert_eq!(result[0], 0x07); // year_hi
        assert_eq!(result[1], 0xD5); // year_lo
        assert_eq!(result[2], 11);   // month (November)
        assert_eq!(result[3], 19);   // date
        assert_eq!(result[4], 5);    // weekday (Saturday = 5)
        assert_eq!(result[5], 9);    // hours
        assert_eq!(result[6], 33);   // minutes
        assert_eq!(result[7], 11);   // seconds
    }

    #[test]
    fn test_parse_date_string_monday() {
        let result = parse_date_string("Mon Jan  1 00:00:00 UTC 2024").unwrap();
        assert_eq!(result[4], 0); // Monday = 0
        assert_eq!(result[2], 1); // January
    }

    #[test]
    fn test_parse_date_string_invalid() {
        let result = parse_date_string("not a date");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_date_string_bad_time() {
        let result = parse_date_string("Mon Jan 1 invalid UTC 2024");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_weekday() {
        assert_eq!(parse_weekday("Mon").unwrap(), 0);
        assert_eq!(parse_weekday("Sun").unwrap(), 6);
        assert!(parse_weekday("xyz").is_err());
    }

    #[test]
    fn test_parse_month() {
        assert_eq!(parse_month("Jan").unwrap(), 1);
        assert_eq!(parse_month("Dec").unwrap(), 12);
        assert!(parse_month("xyz").is_err());
    }
}
