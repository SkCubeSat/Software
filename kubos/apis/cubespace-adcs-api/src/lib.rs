/*
 * Copyright (C) 2018 Kubos Corporation
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

#![deny(missing_docs)]

//! High level CubeSpace ADCS command and telemetry interfaces

pub mod codec;
pub mod commands;
mod packet;
pub mod telemetry;
#[cfg(test)]
mod tests;
pub mod types;

pub use crate::commands::*;
pub use crate::packet::*;
pub use crate::telemetry::*;
pub use crate::types::*;
use thiserror::Error;

/// Errors for ADCS devices
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AdcsError {
    /// Generic error
    #[error("Generic error")]
    Generic,
    /// Configuration error
    #[error("Configuration error")]
    Config,
    /// No response received from subsystem
    #[error("No response received from subsystem")]
    NoResponse,
    /// An error was thrown by the subsystem
    #[error("An error was thrown by the subsystem")]
    Internal,
    /// Mutex-related error
    #[error("Mutex-related error")]
    Mutex,
    /// Requested function has not been implemented
    #[error("Requested function has not been implemented")]
    NotImplemented,
    /// Requested telecommand ID is not present in the ADCS command database
    #[error("Unknown telecommand ID: {id}")]
    UnknownCommand {
        /// Requested telecommand ID.
        id: u8,
    },
    /// Requested telemetry ID is not present in the ADCS telemetry database
    #[error("Unknown telemetry ID: {id}")]
    UnknownTelemetry {
        /// Requested telemetry ID.
        id: u8,
    },
    /// A payload length did not match the ADCS database definition
    #[error("Invalid payload length: expected at least {expected} bytes, received {actual}")]
    InvalidPayloadLength {
        /// Expected payload length in bytes.
        expected: usize,
        /// Actual payload length in bytes.
        actual: usize,
    },
    /// A database or packet value could not be parsed or encoded
    #[error("Invalid ADCS value: {description}")]
    InvalidValue {
        /// Error description.
        description: String,
    },
    /// The structured ADCS database could not be parsed
    #[error("ADCS database parse error: {description}")]
    Parse {
        /// Error description.
        description: String,
    },
}

/// ADCS specific result type
pub type AdcsResult<T> = Result<T, AdcsError>;
