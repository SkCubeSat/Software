//! Error types for the CAN HAL.

use thiserror::Error;

/// Custom errors for CAN actions.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CanError {
    /// A read/write call was made while another call was already in-progress.
    #[error("CAN socket already in-use")]
    PortBusy,
    /// A CAN frame was built with an invalid ID or payload length.
    #[error("Invalid CAN frame: {description}")]
    InvalidFrame {
        /// Error description.
        description: String,
    },
    /// A CAN read timed out before all requested data was received.
    #[error("CAN read timed out")]
    Timeout,
    /// An I/O error was thrown by the kernel.
    #[error("IO Error: {description}")]
    IoError {
        /// The underlying error type.
        cause: std::io::ErrorKind,
        /// Error description.
        description: String,
    },
    /// Interface setup command failed.
    #[error("Interface command failed: {description}")]
    InterfaceError {
        /// Error description.
        description: String,
    },
}

impl From<std::io::Error> for CanError {
    fn from(error: std::io::Error) -> Self {
        CanError::IoError {
            cause: error.kind(),
            description: error.to_string(),
        }
    }
}

/// Errors that occur while reading from and writing to CAN streams.
pub type CanResult<T> = Result<T, CanError>;
