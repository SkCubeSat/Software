use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid backend config: {0}")]
    InvalidConfig(String),
    #[cfg(feature = "spidev")]
    #[error("MRAM driver error: {0}")]
    Driver(String),
    #[error("operation out of bounds (offset={offset}, len={len}, capacity={capacity})")]
    OutOfBounds {
        offset: u32,
        len: usize,
        capacity: u32,
    },
}

pub trait ByteStorage: Send {
    fn capacity(&self) -> u32;
    fn read(&mut self, offset: u32, out: &mut [u8]) -> Result<(), BackendError>;
    fn write(&mut self, offset: u32, data: &[u8]) -> Result<(), BackendError>;
}

pub struct FileImageBackend {
    file: std::fs::File,
    capacity: u32,
}

impl FileImageBackend {
    pub fn new(path: &str, capacity: u32) -> Result<Self, BackendError> {
        if capacity == 0 {
            return Err(BackendError::InvalidConfig(
                "image_capacity_bytes must be > 0".to_string(),
            ));
        }

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        let current_len = file.metadata()?.len();
        if current_len < capacity as u64 {
            file.set_len(capacity as u64)?;
            file.flush()?;
        }

        Ok(Self { file, capacity })
    }

    fn ensure_in_bounds(&self, offset: u32, len: usize) -> Result<(), BackendError> {
        let len_u32 = u32::try_from(len).map_err(|_| BackendError::OutOfBounds {
            offset,
            len,
            capacity: self.capacity,
        })?;
        let end = offset
            .checked_add(len_u32)
            .ok_or(BackendError::OutOfBounds {
                offset,
                len,
                capacity: self.capacity,
            })?;

        if end > self.capacity {
            return Err(BackendError::OutOfBounds {
                offset,
                len,
                capacity: self.capacity,
            });
        }

        Ok(())
    }
}

impl ByteStorage for FileImageBackend {
    fn capacity(&self) -> u32 {
        self.capacity
    }

    fn read(&mut self, offset: u32, out: &mut [u8]) -> Result<(), BackendError> {
        self.ensure_in_bounds(offset, out.len())?;
        self.file.seek(SeekFrom::Start(offset as u64))?;
        self.file.read_exact(out)?;
        Ok(())
    }

    fn write(&mut self, offset: u32, data: &[u8]) -> Result<(), BackendError> {
        self.ensure_in_bounds(offset, data.len())?;
        self.file.seek(SeekFrom::Start(offset as u64))?;
        self.file.write_all(data)?;
        self.file.flush()?;
        Ok(())
    }
}

#[cfg(feature = "spidev")]
pub struct SpidevBackend {
    mram: rust_mram::Mr2xH40<rust_mram::BlockingToAsync<linux_embedded_hal::SpidevDevice>>,
    capacity: u32,
}

#[cfg(feature = "spidev")]
impl SpidevBackend {
    pub fn new(path: &str, speed_hz: u32, mode: u8) -> Result<Self, BackendError> {
        use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};

        let mut spi = linux_embedded_hal::SpidevDevice::open(path)
            .map_err(|err| BackendError::Driver(err.to_string()))?;

        let mode_flags = match mode {
            0 => SpiModeFlags::SPI_MODE_0,
            3 => SpiModeFlags::SPI_MODE_3,
            _ => {
                return Err(BackendError::InvalidConfig(
                    "spi_mode must be either 0 or 3".to_string(),
                ));
            }
        };

        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(speed_hz)
            .mode(mode_flags)
            .build();

        spi.configure(&options)
            .map_err(|err| BackendError::Driver(err.to_string()))?;

        Ok(Self {
            mram: rust_mram::Mr2xH40::new(rust_mram::BlockingToAsync::new(spi)),
            capacity: rust_mram::CAPACITY_BYTES,
        })
    }
}

#[cfg(feature = "spidev")]
impl ByteStorage for SpidevBackend {
    fn capacity(&self) -> u32 {
        self.capacity
    }

    fn read(&mut self, offset: u32, out: &mut [u8]) -> Result<(), BackendError> {
        futures::executor::block_on(self.mram.read(offset, out))
            .map_err(|err| BackendError::Driver(err.to_string()))
    }

    fn write(&mut self, offset: u32, data: &[u8]) -> Result<(), BackendError> {
        futures::executor::block_on(self.mram.write(offset, data))
            .map_err(|err| BackendError::Driver(err.to_string()))
    }
}
