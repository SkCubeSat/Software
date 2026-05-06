use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

use thiserror::Error;

pub const DEFAULT_CAPACITY_BYTES: u32 = 8192;
pub const DEFAULT_MAX_TRANSFER_BYTES: usize = 32;
pub const FM24CL64B_MIN_I2C_ADDR: u8 = 0x50;
pub const FM24CL64B_MAX_I2C_ADDR: u8 = 0x57;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid backend config: {0}")]
    InvalidConfig(String),
    #[cfg(feature = "i2c")]
    #[error("I2C driver error: {0}")]
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

pub fn validate_fm24cl64b_i2c_addr(device_addr: u8) -> Result<(), BackendError> {
    if (FM24CL64B_MIN_I2C_ADDR..=FM24CL64B_MAX_I2C_ADDR).contains(&device_addr) {
        Ok(())
    } else {
        Err(BackendError::InvalidConfig(format!(
            "i2c_addr must be the Linux 7-bit FM24CL64B address 0x50..0x57; got 0x{device_addr:02X}. Do not use the 8-bit datasheet address byte 0xA0/0xA1."
        )))
    }
}

pub fn memory_address_bytes(offset: u32, address_width_bytes: u8) -> Result<Vec<u8>, BackendError> {
    match address_width_bytes {
        1 => Ok(vec![(offset & 0xFF) as u8]),
        2 => Ok(vec![((offset >> 8) & 0xFF) as u8, (offset & 0xFF) as u8]),
        _ => Err(BackendError::InvalidConfig(
            "address_width_bytes must be 1 or 2".to_string(),
        )),
    }
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
        ensure_in_bounds(offset, len, self.capacity)
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

#[cfg(feature = "i2c")]
pub struct I2cFramBackend {
    i2c: linux_embedded_hal::I2cdev,
    device_addr: u8,
    capacity: u32,
    address_width_bytes: u8,
    max_transfer_bytes: usize,
}

#[cfg(feature = "i2c")]
impl I2cFramBackend {
    pub fn new(
        bus: &str,
        device_addr: u8,
        capacity: u32,
        address_width_bytes: u8,
        max_transfer_bytes: usize,
    ) -> Result<Self, BackendError> {
        if capacity == 0 {
            return Err(BackendError::InvalidConfig(
                "capacity_bytes must be > 0".to_string(),
            ));
        }

        if !matches!(address_width_bytes, 1 | 2) {
            return Err(BackendError::InvalidConfig(
                "address_width_bytes must be 1 or 2".to_string(),
            ));
        }

        if max_transfer_bytes == 0 {
            return Err(BackendError::InvalidConfig(
                "max_transfer_bytes must be > 0".to_string(),
            ));
        }
        validate_fm24cl64b_i2c_addr(device_addr)?;

        let addressable_bytes = 1_u32
            .checked_shl(u32::from(address_width_bytes) * 8)
            .ok_or_else(|| BackendError::InvalidConfig("address width too large".to_string()))?;
        if capacity > addressable_bytes {
            return Err(BackendError::InvalidConfig(format!(
                "capacity {capacity} exceeds {addressable_bytes} bytes addressable by {address_width_bytes} address bytes"
            )));
        }

        let i2c = linux_embedded_hal::I2cdev::new(bus)
            .map_err(|err| BackendError::Driver(err.to_string()))?;

        Ok(Self {
            i2c,
            device_addr,
            capacity,
            address_width_bytes,
            max_transfer_bytes,
        })
    }

    fn ensure_in_bounds(&self, offset: u32, len: usize) -> Result<(), BackendError> {
        ensure_in_bounds(offset, len, self.capacity)
    }

    fn address_bytes(&self, offset: u32) -> Vec<u8> {
        memory_address_bytes(offset, self.address_width_bytes)
            .expect("address width was validated during I2C backend initialization")
    }
}

#[cfg(feature = "i2c")]
impl ByteStorage for I2cFramBackend {
    fn capacity(&self) -> u32 {
        self.capacity
    }

    fn read(&mut self, offset: u32, out: &mut [u8]) -> Result<(), BackendError> {
        use embedded_hal::i2c::I2c;

        self.ensure_in_bounds(offset, out.len())?;

        let mut cursor = offset;
        for chunk in out.chunks_mut(self.max_transfer_bytes) {
            let addr = self.address_bytes(cursor);
            // FM24CL64B selective read: write the two-byte memory address,
            // repeated START, then read sequential data bytes.
            self.i2c
                .write_read(self.device_addr, &addr, chunk)
                .map_err(|err| BackendError::Driver(err.to_string()))?;
            cursor += chunk.len() as u32;
        }

        Ok(())
    }

    fn write(&mut self, offset: u32, data: &[u8]) -> Result<(), BackendError> {
        use embedded_hal::i2c::I2c;

        self.ensure_in_bounds(offset, data.len())?;

        let mut cursor = offset;
        for chunk in data.chunks(self.max_transfer_bytes) {
            let mut frame = self.address_bytes(cursor);
            frame.extend_from_slice(chunk);
            // FM24CL64B write: two-byte memory address followed by one or more
            // data bytes. F-RAM has no EEPROM-style write delay.
            self.i2c
                .write(self.device_addr, &frame)
                .map_err(|err| BackendError::Driver(err.to_string()))?;
            cursor += chunk.len() as u32;
        }

        Ok(())
    }
}

fn ensure_in_bounds(offset: u32, len: usize, capacity: u32) -> Result<(), BackendError> {
    let len_u32 = u32::try_from(len).map_err(|_| BackendError::OutOfBounds {
        offset,
        len,
        capacity,
    })?;
    let end = offset
        .checked_add(len_u32)
        .ok_or(BackendError::OutOfBounds {
            offset,
            len,
            capacity,
        })?;

    if end > capacity {
        return Err(BackendError::OutOfBounds {
            offset,
            len,
            capacity,
        });
    }

    Ok(())
}
