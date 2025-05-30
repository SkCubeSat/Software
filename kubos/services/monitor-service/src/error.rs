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

use thiserror::Error;

/// Errors for the monitor service
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum Error {
    /// Error when accessing files in the proc filesystem
    #[error("Error accessing proc filesystem: {0}")]
    ProcAccess(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// UTF-8 conversion error
    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    /// Regex error
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    /// Format error
    #[error("{0}")]
    Format(String),
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Format(error)
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::Format(error.to_string())
    }
}
