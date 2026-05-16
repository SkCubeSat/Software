use rust_uart::UartError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SnnError {
    #[error("uart: {0}")]
    Uart(#[from] UartError),

    #[error("payload protocol: {0}")]
    Protocol(String),

    #[error("payload reported failure: {0}")]
    PayloadNak(String),

    #[error("crc mismatch: expected {expected:08X}, got {actual:08X}")]
    CrcMismatch { expected: u32, actual: u32 },

    #[error("timed out waiting for {0}")]
    Timeout(&'static str),

    #[error("payload not idle (current state: {0})")]
    NotIdle(String),

    #[error("queue full (capacity {0})")]
    QueueFull(usize),

    #[error("image too large: {size} bytes exceeds limit {limit}")]
    ImageTooLarge { size: usize, limit: usize },

    #[error("unknown image id {0}")]
    UnknownImageId(u32),

    #[error("result for image {0} not ready (phase: {1})")]
    ResultNotReady(u32, String),

    #[error("result for image {0} expired from cache")]
    ResultExpired(u32),

    #[error("invalid base64: {0}")]
    InvalidBase64(String),

    #[error("driver shut down")]
    DriverGone,

    #[error("internal: {0}")]
    Internal(String),
}

impl From<base64::DecodeError> for SnnError {
    fn from(value: base64::DecodeError) -> Self {
        SnnError::InvalidBase64(value.to_string())
    }
}
