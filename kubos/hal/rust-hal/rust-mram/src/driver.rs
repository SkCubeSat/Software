//! Async MR2xH40 driver layer.

use core::convert::TryFrom;

use embedded_hal::spi::Operation;
use embedded_hal_async::spi::SpiDevice;

use crate::error::Error;
use crate::protocol::{
    command_address_header, CAPACITY_BYTES, CMD_RDSR, CMD_READ, CMD_SLEEP, CMD_WAKE, CMD_WRDI,
    CMD_WREN, CMD_WRITE, CMD_WRSR, STATUS_WEL,
};

/// Async driver for the Everspin MR2xH40 MRAM.
pub struct Mr2xH40<SPI> {
    spi: SPI,
}

impl<SPI> Mr2xH40<SPI>
where
    SPI: SpiDevice<u8>,
{
    /// Creates a new driver from a configured SPI device.
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Returns ownership of the underlying SPI device.
    pub fn release(self) -> SPI {
        self.spi
    }

    /// Returns memory capacity in bytes.
    pub const fn capacity_bytes() -> u32 {
        CAPACITY_BYTES
    }

    /// Reads bytes from `offset` into `bytes`.
    pub async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Error<SPI::Error>> {
        Self::ensure_in_bounds(offset, bytes.len())?;
        if bytes.is_empty() {
            return Ok(());
        }

        let header = command_address_header(CMD_READ, offset);
        let mut operations = [Operation::Write(&header), Operation::Read(bytes)];
        self.spi
            .transaction(&mut operations)
            .await
            .map_err(Error::Spi)
    }

    /// Writes `bytes` into memory starting at `offset`.
    ///
    /// This device does not use page-write restrictions, so multi-byte writes may span
    /// the full array as long as they stay in bounds.
    pub async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Error<SPI::Error>> {
        Self::ensure_in_bounds(offset, bytes.len())?;
        if bytes.is_empty() {
            return Ok(());
        }

        self.write_enable().await?;

        let header = command_address_header(CMD_WRITE, offset);
        let mut operations = [Operation::Write(&header), Operation::Write(bytes)];
        self.spi
            .transaction(&mut operations)
            .await
            .map_err(Error::Spi)
    }

    /// Sends the `WREN` command.
    pub async fn write_enable(&mut self) -> Result<(), Error<SPI::Error>> {
        self.send_simple_command(CMD_WREN).await
    }

    /// Sends the `WRDI` command.
    pub async fn write_disable(&mut self) -> Result<(), Error<SPI::Error>> {
        self.send_simple_command(CMD_WRDI).await
    }

    /// Reads the status register (`RDSR`).
    pub async fn read_status(&mut self) -> Result<u8, Error<SPI::Error>> {
        // Per datasheet, RDSR immediately following READ can return incorrect data.
        // Issuing two successive RDSR commands guarantees the second read is valid.
        let _ = self.read_status_once().await?;
        self.read_status_once().await
    }

    /// Writes the status register (`WRSR`).
    pub async fn write_status(&mut self, status: u8) -> Result<(), Error<SPI::Error>> {
        self.write_enable().await?;

        let payload = [CMD_WRSR, status];
        let mut operations = [Operation::Write(&payload)];
        self.spi
            .transaction(&mut operations)
            .await
            .map_err(Error::Spi)
    }

    /// Returns true if status register bit WEL is set.
    pub async fn is_write_enabled(&mut self) -> Result<bool, Error<SPI::Error>> {
        Ok((self.read_status().await? & STATUS_WEL) != 0)
    }

    /// Puts the device into sleep mode.
    pub async fn sleep(&mut self) -> Result<(), Error<SPI::Error>> {
        self.send_simple_command(CMD_SLEEP).await
    }

    /// Wakes the device from sleep mode.
    pub async fn wake(&mut self) -> Result<(), Error<SPI::Error>> {
        self.send_simple_command(CMD_WAKE).await
    }

    fn ensure_in_bounds(offset: u32, len: usize) -> Result<(), Error<SPI::Error>> {
        if len == 0 {
            if offset <= CAPACITY_BYTES {
                return Ok(());
            }
            return Err(Error::out_of_bounds(offset, len, CAPACITY_BYTES));
        }

        let len_u32 =
            u32::try_from(len).map_err(|_| Error::out_of_bounds(offset, len, CAPACITY_BYTES))?;
        let end = offset
            .checked_add(len_u32)
            .ok_or_else(|| Error::out_of_bounds(offset, len, CAPACITY_BYTES))?;

        if end > CAPACITY_BYTES {
            return Err(Error::out_of_bounds(offset, len, CAPACITY_BYTES));
        }

        Ok(())
    }

    async fn send_simple_command(&mut self, command: u8) -> Result<(), Error<SPI::Error>> {
        let payload = [command];
        let mut operations = [Operation::Write(&payload)];
        self.spi
            .transaction(&mut operations)
            .await
            .map_err(Error::Spi)
    }

    async fn read_status_once(&mut self) -> Result<u8, Error<SPI::Error>> {
        let command = [CMD_RDSR];
        let mut status = [0_u8];
        let mut operations = [Operation::Write(&command), Operation::Read(&mut status)];
        self.spi
            .transaction(&mut operations)
            .await
            .map_err(Error::Spi)?;

        Ok(status[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::convert::Infallible;
    use std::collections::VecDeque;

    use futures::executor::block_on;

    #[derive(Default)]
    struct FakeAsyncSpi {
        writes: Vec<Vec<u8>>,
        reads: VecDeque<Vec<u8>>,
    }

    impl FakeAsyncSpi {
        fn with_reads(reads: Vec<Vec<u8>>) -> Self {
            Self {
                writes: Vec::new(),
                reads: reads.into_iter().collect(),
            }
        }
    }

    impl embedded_hal::spi::ErrorType for FakeAsyncSpi {
        type Error = Infallible;
    }

    impl embedded_hal_async::spi::SpiDevice<u8> for FakeAsyncSpi {
        async fn transaction(
            &mut self,
            operations: &mut [Operation<'_, u8>],
        ) -> Result<(), Self::Error> {
            for op in operations {
                match op {
                    Operation::Write(data) => self.writes.push(data.to_vec()),
                    Operation::Read(buffer) => {
                        let response = self
                            .reads
                            .pop_front()
                            .expect("missing queued read response");
                        assert_eq!(response.len(), buffer.len());
                        buffer.copy_from_slice(&response);
                    }
                    Operation::Transfer(read, write) => {
                        self.writes.push(write.to_vec());
                        let response = self
                            .reads
                            .pop_front()
                            .unwrap_or_else(|| vec![0_u8; read.len()]);
                        assert_eq!(response.len(), read.len());
                        read.copy_from_slice(&response);
                    }
                    Operation::TransferInPlace(data) => {
                        self.writes.push(data.to_vec());
                        let response = self
                            .reads
                            .pop_front()
                            .unwrap_or_else(|| vec![0_u8; data.len()]);
                        assert_eq!(response.len(), data.len());
                        data.copy_from_slice(&response);
                    }
                    Operation::DelayNs(_) => {}
                }
            }

            Ok(())
        }
    }

    #[test]
    fn read_sends_read_header_and_returns_bytes() {
        let spi = FakeAsyncSpi::with_reads(vec![vec![0xAA, 0xBB, 0xCC]]);
        let mut mram = Mr2xH40::new(spi);

        let mut buffer = [0_u8; 3];
        block_on(mram.read(0x1234, &mut buffer)).unwrap();

        assert_eq!(buffer, [0xAA, 0xBB, 0xCC]);

        let spi = mram.release();
        assert_eq!(spi.writes, vec![vec![CMD_READ, 0x00, 0x12, 0x34]]);
    }

    #[test]
    fn write_sends_wren_then_write_header_and_payload() {
        let spi = FakeAsyncSpi::default();
        let mut mram = Mr2xH40::new(spi);

        block_on(mram.write(0x0002, &[0x11, 0x22])).unwrap();

        let spi = mram.release();
        assert_eq!(
            spi.writes,
            vec![
                vec![CMD_WREN],
                vec![CMD_WRITE, 0x00, 0x00, 0x02],
                vec![0x11, 0x22]
            ]
        );
    }

    #[test]
    fn read_out_of_bounds_is_rejected() {
        let spi = FakeAsyncSpi::default();
        let mut mram = Mr2xH40::new(spi);

        let mut buffer = [0_u8; 2];
        let error = block_on(mram.read(CAPACITY_BYTES - 1, &mut buffer)).unwrap_err();

        assert_eq!(
            error,
            Error::OutOfBounds {
                offset: CAPACITY_BYTES - 1,
                len: 2,
                capacity: CAPACITY_BYTES
            }
        );
    }

    #[test]
    fn read_status_uses_command_then_read() {
        let spi = FakeAsyncSpi::with_reads(vec![vec![0b0000_0000], vec![0b0000_0010]]);
        let mut mram = Mr2xH40::new(spi);

        let status = block_on(mram.read_status()).unwrap();
        assert_eq!(status, 0b0000_0010);

        let spi = mram.release();
        assert_eq!(spi.writes, vec![vec![CMD_RDSR], vec![CMD_RDSR]]);
    }

    #[test]
    fn is_write_enabled_checks_wel_bit() {
        let spi = FakeAsyncSpi::with_reads(vec![vec![0b0000_0000], vec![0b0000_0010]]);
        let mut mram = Mr2xH40::new(spi);

        assert!(block_on(mram.is_write_enabled()).unwrap());
    }
}
