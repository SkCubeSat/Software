# MR2xH40 MRAM Driver (`rust-mram`)

Async Rust driver for the Everspin MR2xH40 SPI MRAM using `embedded-hal-async`.

## Architecture

The crate is intentionally layered:

- `protocol`: opcodes, capacities, and address packing
- `driver`: async MRAM operations (`read`, `write`, `read_status`, `sleep`, `wake`)
- `compat`: `BlockingToAsync` adapter for blocking SPI backends such as Linux `spidev`
- `io`: `embedded-io-async` seekable cursor (`MramCursor`)

## Notes for KubOS/Linux

On KubOS, SPI devices are typically exposed through `/dev/spidevX.Y`. Most Linux SPI APIs are
blocking. Use `BlockingToAsync` to inject a blocking SPI device into this async driver.

## Minimal Usage

```rust,no_run
use futures::executor::block_on;
use embedded_io_async::{Read, Seek, Write};
use embedded_io::SeekFrom;
use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};
use linux_embedded_hal::SpidevDevice;
use rust_mram::{BlockingToAsync, Mr2xH40};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut spi = SpidevDevice::open("/dev/spidev1.0")?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(10_000_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)?;

    let async_spi = BlockingToAsync::new(spi);
    let mut mram = Mr2xH40::new(async_spi);

    block_on(async {
        let mut cursor = mram.cursor();
        cursor.seek(SeekFrom::Start(0x100)).await.expect("seek failed");
        cursor
            .write_all(&[0xDE, 0xAD, 0xBE, 0xEF])
            .await
            .expect("write failed");
        cursor.seek(SeekFrom::Start(0x100)).await.expect("seek failed");

        let mut rx = [0u8; 4];
        cursor.read_exact(&mut rx).await.expect("read failed");

        assert_eq!(rx, [0xDE, 0xAD, 0xBE, 0xEF]);
    });

    Ok(())
}
```
