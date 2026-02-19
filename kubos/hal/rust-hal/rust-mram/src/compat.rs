//! Compatibility adapters.

use embedded_hal::spi::{ErrorType, Operation, SpiDevice as BlockingSpiDevice};
use embedded_hal_async::spi::SpiDevice as AsyncSpiDevice;

/// Adapter that exposes a blocking SPI device through the async SPI trait.
///
/// This is useful on KubOS/Linux where `spidev` backends are typically blocking.
pub struct BlockingToAsync<SPI> {
    inner: SPI,
}

impl<SPI> BlockingToAsync<SPI> {
    /// Creates a new adapter.
    pub fn new(inner: SPI) -> Self {
        Self { inner }
    }

    /// Returns the wrapped blocking SPI device.
    pub fn into_inner(self) -> SPI {
        self.inner
    }

    /// Mutable reference to the wrapped blocking SPI device.
    pub fn inner_mut(&mut self) -> &mut SPI {
        &mut self.inner
    }
}

impl<SPI> ErrorType for BlockingToAsync<SPI>
where
    SPI: ErrorType,
{
    type Error = SPI::Error;
}

impl<SPI, Word> AsyncSpiDevice<Word> for BlockingToAsync<SPI>
where
    SPI: BlockingSpiDevice<Word>,
    Word: Copy + 'static,
{
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, Word>],
    ) -> Result<(), Self::Error> {
        self.inner.transaction(operations)
    }
}
