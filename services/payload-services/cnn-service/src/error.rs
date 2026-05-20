use rust_uart::UartError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CnnError {
    #[error("uart: {0}")]
    Uart(#[from] UartError),

    #[error("payload protocol: {0}")]
    Protocol(String),

    #[error("payload reported error: {0}")]
    PayloadError(String),

    #[error("timed out waiting for {0}")]
    Timeout(&'static str),

    #[error("file I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("internal: {0}")]
    Internal(String),
}
