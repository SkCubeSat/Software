//
// Copyright (C) 2018 Kubos Corporation
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
// Contributed by: William Greer (wgreer184@gmail.com) and Sam Justice (sam.justice1@gmail.com)
//

// update to the original version to use the 'thiserror' crate instead of 'failure' crate

use std::io;
use std::net::AddrParseError;
use std::str::Utf8Error;
use thiserror::Error;

/// This enum defines all errors that can occur within the `comms-service`.
#[derive(Error, Debug)]
pub enum CommsServiceError {
    /// A component of the service's configuration was incorrect
    #[error("Config error: {0}")]
    ConfigError(String),
    /// The mutex guarding the telemetry cache has been poisoned.
    #[error("The mutex guarding the telemetry cache has been poisoned.")]
    MutexPoisoned,
    /// A UDP header was unable to be correctly parsed.
    #[error("A UDP header was unable to be correctly parsed.")]
    HeaderParsing,
    /// The checksum of a UDP packet does not match the one found in the header.
    #[error("The checksum of a UDP packet does not match the one found in the header.")]
    InvalidChecksum,
    /// The number of `write` methods and the number of downlink ports are not the same.
    #[error("The number of write methods and the number of downlink ports are not the same.")]
    ParameterLengthMismatch,
    /// All of the ports allocated for handling packets are binded and unable to be used.
    #[error("All of the ports allocated for handling packets are binded.")]
    NoAvailablePorts,
    /// No data available for reading
    #[error("No data available for reading")]
    NoReadData,
    /// An error was encountered when parsing a packet
    #[error("Parsing error {0}")]
    ParsingError(String),
    /// Generic error encountered
    #[error("Error encountered {0}")]
    GenericError(String),
    /// Unknown payload type encountered
    #[error("Unknown payload type encountered: {0}")]
    UnknownPayloadType(u16),
    /// Error parsing IP address
    #[error("Failed to parse IP address: {0}")]
    AddrParseError(#[from] AddrParseError),
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    /// Error parsing UTF-8
    #[error("UTF-8 parsing error: {0}")]
    Utf8Error(#[from] Utf8Error),
}

/// Result returned by the `comms-service`.
pub type CommsResult<T> = Result<T, CommsServiceError>;
