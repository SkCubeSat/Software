//! Async, layered driver for Everspin MR2xH40 SPI MRAM.
//!
//! The driver is split into layers:
//! - `protocol`: opcodes, capacities, and address packing
//! - `driver`: async command execution over `embedded-hal-async`
//! - `compat`: adapters for integrating blocking SPI backends (for example KubOS/Linux `spidev`)

#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

mod compat;
mod driver;
mod error;
mod io;
mod protocol;

/// Adapter to use blocking `embedded-hal` SPI devices with this async driver.
pub use crate::compat::BlockingToAsync;
/// Async MR2xH40 device driver.
pub use crate::driver::Mr2xH40;
/// Driver error type.
pub use crate::error::Error;
/// I/O adapter error type used by `MramCursor`.
pub use crate::io::IoError;
/// Async embedded-io cursor over MRAM bytes.
pub use crate::io::MramCursor;
/// Total chip capacity in bits.
pub use crate::protocol::CAPACITY_BITS;
/// Total chip capacity in bytes.
pub use crate::protocol::CAPACITY_BYTES;
/// Addressable word width in bits.
pub use crate::protocol::WORD_SIZE_BITS;
