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

//! Error types for the Analog IC API

use std::io;
use thiserror::Error;

/// Errors that can occur when communicating with the Analog IC payload
#[derive(Debug, Error)]
pub enum AnalogIcError {
    /// Generic error condition
    #[error("Analog IC generic error")]
    GenericError,
    /// Error resulting from underlying I/O functions
    #[error("IO Error: {description}")]
    IoError {
        /// Underlying cause captured from I/O function
        cause: io::ErrorKind,
        /// Error description
        description: String,
    },
    /// Error resulting from receiving invalid data from the payload
    #[error("Parsing failed: {source_description}")]
    ParsingFailure {
        /// Source where invalid data was received
        source_description: String,
    },
    /// Error resulting from a failure with a command
    #[error("Command failure: {command}")]
    CommandFailure {
        /// Command which failed
        command: String,
    },
}

impl AnalogIcError {
    /// Convenience function for creating an AnalogIcError::ParsingFailure
    ///
    /// # Arguments
    /// - `source` - Source of parsing failure
    pub fn parsing_failure(source: &str) -> AnalogIcError {
        AnalogIcError::ParsingFailure {
            source_description: String::from(source),
        }
    }
}

/// Convenience converter from io::Error to AnalogIcError
impl From<io::Error> for AnalogIcError {
    fn from(error: io::Error) -> Self {
        AnalogIcError::IoError {
            cause: error.kind(),
            description: error.to_string(),
        }
    }
}

/// Universal return type for Analog IC API functions
pub type AnalogIcResult<T> = Result<T, AnalogIcError>;
