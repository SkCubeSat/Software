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

// update to the original version to use the 'thiserror' crate instead of 'failure' crate

use thiserror::Error;

/// Errors which occur when using ChannelProtocol
#[derive(Debug, Error)]
pub enum ProtocolError {
    /// An error was encountered by serde_cbor
    #[error("Cbor Error: {err}")]
    CborError {
        /// The specific CBOR protocol error
        err: cbor_protocol::ProtocolError,
    },
    /// A general error was encountered when parsing a message
    #[error("Unable to parse message: {err}")]
    MessageParseError {
        /// Underlying error encountered
        err: String,
    },
    /// A timeout occurred when receiving data
    #[error("A receive timeout was encountered")]
    ReceiveTimeout,
    /// An error was encountered when receiving a message
    #[error("Failure receiving message: {err}")]
    ReceiveError {
        /// Underlying error encountered
        err: String,
    },
}

impl From<cbor_protocol::ProtocolError> for ProtocolError {
    fn from(error: cbor_protocol::ProtocolError) -> Self {
        match error {
            cbor_protocol::ProtocolError::Timeout => ProtocolError::ReceiveTimeout,
            err => ProtocolError::CborError { err },
        }
    }
}
