//
// Copyright (C) 2025 SkCubeSat
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! # NovAtel OEM7 API
//!
//! Low-level API for communicating with NovAtel OEM7 GNSS receivers.
//!
//! Uses a hybrid ASCII/binary protocol:
//! - **Commands** are sent as ASCII text (e.g., `LOG BESTXYZB ONCE\r\n`)
//! - **Acknowledgments** are received as abbreviated ASCII (`<OK\r\n`)
//! - **Log data** is received in compact binary format (sync `AA 44 12`, header, body, CRC32)

pub mod crc32;
pub mod messages;
pub mod oem7;

pub use crc32::calc_crc;
pub use messages::*;
pub use oem7::*;

/// Result type for OEM7 operations
pub type OEM7Result<T> = Result<T, OEM7Error>;

/// Errors that can occur during OEM7 communication
#[derive(Debug, thiserror::Error)]
pub enum OEM7Error {
    /// Serial port configuration or access error
    #[error("Serial port error: {0}")]
    Serial(#[from] serialport::Error),

    /// I/O error during read/write
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Command was rejected by the receiver
    #[error("Command rejected: {0}")]
    CommandRejected(String),

    /// Timed out waiting for a response
    #[error("Timeout waiting for response")]
    Timeout,

    /// Failed to parse a response
    #[error("Parse error: {0}")]
    ParseError(String),

    /// CRC validation failed on received binary data
    #[error("CRC mismatch: expected {expected:#010X}, got {actual:#010X}")]
    CrcMismatch {
        /// Expected CRC value
        expected: u32,
        /// Actual CRC value computed from received data
        actual: u32,
    },

    /// Could not find the binary sync pattern within the timeout
    #[error("No binary sync pattern found")]
    NoSync,
}
