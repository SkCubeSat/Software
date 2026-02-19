//! Error types for the MR2xH40 driver.

use core::fmt;

/// MR2xH40 driver error.
#[derive(Debug, PartialEq, Eq)]
pub enum Error<SpiError> {
    /// SPI transport-level error.
    Spi(SpiError),
    /// Read or write range exceeded the chip capacity.
    OutOfBounds {
        /// Starting address requested.
        offset: u32,
        /// Requested operation length in bytes.
        len: usize,
        /// Device capacity in bytes.
        capacity: u32,
    },
}

impl<SpiError> Error<SpiError> {
    pub(crate) fn out_of_bounds(offset: u32, len: usize, capacity: u32) -> Self {
        Self::OutOfBounds {
            offset,
            len,
            capacity,
        }
    }
}

impl<SpiError: fmt::Display> fmt::Display for Error<SpiError> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spi(err) => write!(f, "SPI error: {err}"),
            Self::OutOfBounds {
                offset,
                len,
                capacity,
            } => write!(
                f,
                "operation out of bounds: offset={offset}, len={len}, capacity={capacity}"
            ),
        }
    }
}

impl<SpiError> From<SpiError> for Error<SpiError> {
    fn from(value: SpiError) -> Self {
        Self::Spi(value)
    }
}
