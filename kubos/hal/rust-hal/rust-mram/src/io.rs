//! `embedded-io-async` adapters for treating MRAM as a seekable byte stream.

use core::cmp::min;
use core::fmt;

use embedded_hal_async::spi::SpiDevice;
use embedded_io::ErrorKind;
use embedded_io::SeekFrom;
use embedded_io_async::{ErrorType, Read, Seek, Write};

use crate::driver::Mr2xH40;
use crate::error::Error;
use crate::protocol::CAPACITY_BYTES;

/// I/O adapter error.
#[derive(Debug)]
pub enum IoError<SpiError> {
    /// Error returned by the underlying MRAM driver.
    Driver(Error<SpiError>),
    /// Invalid seek target.
    InvalidSeek,
}

impl<SpiError> From<Error<SpiError>> for IoError<SpiError> {
    fn from(value: Error<SpiError>) -> Self {
        Self::Driver(value)
    }
}

impl<SpiError> fmt::Display for IoError<SpiError> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Driver(Error::Spi(_)) => write!(f, "spi transport error"),
            Self::Driver(Error::OutOfBounds {
                offset,
                len,
                capacity,
            }) => write!(
                f,
                "operation out of bounds: offset={offset}, len={len}, capacity={capacity}"
            ),
            Self::InvalidSeek => write!(f, "invalid seek target"),
        }
    }
}

impl<SpiError: fmt::Debug> core::error::Error for IoError<SpiError> {}

impl<SpiError: fmt::Debug> embedded_io::Error for IoError<SpiError> {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Driver(Error::OutOfBounds { .. }) | Self::InvalidSeek => ErrorKind::InvalidInput,
            Self::Driver(Error::Spi(_)) => ErrorKind::Other,
        }
    }
}

/// Seekable async byte-stream view over MR2xH40 memory.
pub struct MramCursor<'a, SPI>
where
    SPI: SpiDevice<u8>,
{
    device: &'a mut Mr2xH40<SPI>,
    position: u32,
}

impl<'a, SPI> MramCursor<'a, SPI>
where
    SPI: SpiDevice<u8>,
{
    /// Creates a new cursor at position `0`.
    pub fn new(device: &'a mut Mr2xH40<SPI>) -> Self {
        Self {
            device,
            position: 0,
        }
    }

    /// Creates a new cursor starting at the provided byte offset.
    pub fn at(device: &'a mut Mr2xH40<SPI>, position: u32) -> Result<Self, IoError<SPI::Error>> {
        if position > CAPACITY_BYTES {
            return Err(IoError::InvalidSeek);
        }

        Ok(Self { device, position })
    }

    /// Current cursor position.
    pub fn position(&self) -> u32 {
        self.position
    }

    /// Mutable access to the underlying MRAM device.
    pub fn device_mut(&mut self) -> &mut Mr2xH40<SPI> {
        self.device
    }
}

impl<SPI> Mr2xH40<SPI>
where
    SPI: SpiDevice<u8>,
{
    /// Opens an async I/O cursor positioned at `0`.
    pub fn cursor(&mut self) -> MramCursor<'_, SPI> {
        MramCursor::new(self)
    }
}

impl<SPI> ErrorType for MramCursor<'_, SPI>
where
    SPI: SpiDevice<u8>,
    SPI::Error: fmt::Debug,
{
    type Error = IoError<SPI::Error>;
}

impl<SPI> Read for MramCursor<'_, SPI>
where
    SPI: SpiDevice<u8>,
    SPI::Error: fmt::Debug,
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }

        let remaining = CAPACITY_BYTES.saturating_sub(self.position) as usize;
        if remaining == 0 {
            return Ok(0);
        }

        let read_len = min(buf.len(), remaining);
        let target = &mut buf[..read_len];

        self.device
            .read(self.position, target)
            .await
            .map_err(IoError::from)?;

        self.position += read_len as u32;
        Ok(read_len)
    }
}

impl<SPI> Write for MramCursor<'_, SPI>
where
    SPI: SpiDevice<u8>,
    SPI::Error: fmt::Debug,
{
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }

        let remaining = CAPACITY_BYTES.saturating_sub(self.position) as usize;
        if remaining == 0 {
            return Ok(0);
        }

        let write_len = min(buf.len(), remaining);
        let payload = &buf[..write_len];

        self.device
            .write(self.position, payload)
            .await
            .map_err(IoError::from)?;

        self.position += write_len as u32;
        Ok(write_len)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<SPI> Seek for MramCursor<'_, SPI>
where
    SPI: SpiDevice<u8>,
    SPI::Error: fmt::Debug,
{
    async fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        let cap = CAPACITY_BYTES as i64;
        let cur = self.position as i64;

        let next = match pos {
            SeekFrom::Start(offset) => i64::try_from(offset).map_err(|_| IoError::InvalidSeek)?,
            SeekFrom::End(offset) => cap.checked_add(offset).ok_or(IoError::InvalidSeek)?,
            SeekFrom::Current(offset) => cur.checked_add(offset).ok_or(IoError::InvalidSeek)?,
        };

        if !(0..=cap).contains(&next) {
            return Err(IoError::InvalidSeek);
        }

        self.position = next as u32;
        Ok(self.position as u64)
    }
}
