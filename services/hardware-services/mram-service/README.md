# MRAM Service

GraphQL service for managing files stored in MR2xH40-compatible byte storage.
The filesystem layer is implemented with [`littlefs2`](https://docs.rs/littlefs2/0.6.1/littlefs2/).

## Architecture

- `backend.rs`: storage adapters (`file` image backend, optional `spidev` backend)
- `fs.rs`: littlefs2 integration over MRAM byte storage
- `subsystem.rs`: service state + config wiring
- `schema.rs`: GraphQL API

`littlefs2` provides wear-leveling and power-loss resilient metadata semantics. This service stores
per-file metadata (mime/compression/timestamps) as littlefs file attributes.

## Config

```toml
[mram-service]
backend = "file"
image_path = "/tmp/mram-service.img"
image_capacity_bytes = 524288

[mram-service.addr]
ip = "127.0.0.1"
port = 8090
```

`image_capacity_bytes` must currently match MR2xH40 capacity (`524288` bytes).

For hardware:

```toml
[mram-service]
backend = "spidev"
spi_device = "/dev/spidev1.0"
spi_speed_hz = 10000000
spi_mode = 0
```

Build with `--features spidev` to enable hardware backend.

## GraphQL

### Queries

- `ping`
- `storage`
- `files`
- `file(name)`
- `readFile(name, offset, length)` returns base64 payload

### Mutations

- `writeFile(input)` where `input.dataBase64` is the payload
- `deleteFile(name)`
- `format(confirm: true)`

## Example

```graphql
mutation {
  writeFile(input: {
    name: "img-001.bin",
    mimeType: "application/octet-stream",
    compressed: true,
    dataBase64: "AQIDBA=="
  }) {
    success
    errors
    file { name size offset compressed }
  }
}
```
