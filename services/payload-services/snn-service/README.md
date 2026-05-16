# SNN Service

GraphQL service that owns the UART link to the SNN payload board and runs the
multi-step inference protocol on its behalf. Mission applications submit a JPEG
and pull back a bitmap result without ever touching the wire.

## Architecture

- `protocol.rs` — line parser (`PayloadLine`), command serializers (`cmd_*`), CRC-32 helper.
- `driver.rs` — long-running OS thread that owns the `rust-uart` `Connection`. Drives the
  protocol state machine end-to-end for one image at a time. Waits on a `Condvar` when
  there's no work; updates `Arc<Mutex<SharedState>>` so GraphQL queries can observe progress.
- `subsystem.rs` — cloneable handle that GraphQL handlers use. Owns the FIFO of pending
  jobs and the LRU cache of completed results. `submit`, `cancel`, `infer`, `get_result`,
  `inference_status`, `state`, `health`.
- `schema.rs` — `async-graphql` `QueryRoot` / `MutationRoot`.
- `error.rs` — `SnnError`, surfaced to GraphQL as the `error` field on each response.

UART I/O is blocking, so the driver lives on its own `std::thread` rather than the tokio
runtime. The handlers and the driver communicate exclusively through `SharedState`
(a `std::sync::Mutex`) plus a `Condvar` to wake the driver when a job is queued or
when shutdown is signalled.

## Wire protocol

A single inference cycle, OBC ↔ payload:

```
   payload                              OBC
      │                                  │
      │  PAYLOAD_READY (boot, optional)  │
      │ ───────────────────────────────► │
      │                                  │  STATUS
      │ ◄─────────────────────────────── │
      │  IDLE                            │
      │ ───────────────────────────────► │
      │                                  │  SEND <id> <size> <crc32>
      │ ◄─────────────────────────────── │
      │  READY                           │
      │ ───────────────────────────────► │
      │                                  │  <raw JPEG bytes>
      │ ◄─────────────────────────────── │
      │  RX_OK <id>                      │
      │ ───────────────────────────────► │
      │                                  │
      │  ┄┄┄ payload runs SNN ┄┄┄        │
      │                                  │
      │  RESULT_READY <id>               │
      │ ───────────────────────────────► │
      │                                  │  GET_RESULT_INFO <id>
      │ ◄─────────────────────────────── │
      │  RESULT_INFO <id> READY          │
      │              <size> <crc32>      │
      │ ───────────────────────────────► │
      │                                  │  GET_RESULT <id>
      │ ◄─────────────────────────────── │
      │  RESULT_READY <id>               │
      │              <size> <crc32>      │
      │ ───────────────────────────────► │
      │                                  │  READY
      │ ◄─────────────────────────────── │
      │  <raw bitmap bytes>              │
      │ ───────────────────────────────► │
      │                                  │  RESULT_RX_OK <id>
      │                                  │
```

Notes:

- Lines are ASCII, terminated by `\n` (a trailing `\r` is tolerated).
- `id` is a `u32`, `size` is decimal bytes, `crc32` is uppercase hex (e.g. `6DE17C6C`).
- The CRC is IEEE/zlib (`crc32fast::Hasher`). The service computes CRC32 of the JPEG it
  is about to send and verifies CRC32 of the bitmap it just received.
- `PROCESSING <id>`, if the payload emits one between `RX_OK` and `RESULT_READY`, is
  consumed and discarded.
- An `ERR <code> <message>` line at any point aborts the cycle and surfaces as
  `SnnError::PayloadNak`. The service stays `Idle` (the wire is back in sync after the
  payload error) and the next submission proceeds normally.

## Concurrency & queueing

- The driver processes one image at a time — the payload itself is single-threaded.
- A configurable FIFO (`queue_capacity`, default `4`) holds *pending* jobs. Each slot
  retains the JPEG in memory, so this is a real RAM cost; tune for the mission profile.
  Submissions past the limit return `accepted: false` with `error: "queue full …"`.
- Completed results live in an LRU cache (`result_retention`, default `4`). After a
  successful `getResult` a result is marked `DELIVERED`; LRU eviction happens regardless
  of `DELIVERED` status. A mission app that does not fetch in time gets
  `result for image N expired from cache`.
- `cancel(id)` only succeeds while the job is still `QUEUED`. From `SENDING_IMAGE` onward
  the protocol cannot be aborted mid-cycle without desynchronising the wire.

State is **ephemeral**. Restarting `snn-service` clears the queue, the in-flight job, and
the result cache. There is no persistence; mission apps must treat the service as
short-term result custody only.

## Config

`/etc/kubos-config.toml`:

```toml
[snn-service]
uart_bus = "/dev/ttyUL2"
uart_baud = 115200

# Per-step UART timeouts (ms). `processing_timeout_ms` bounds the wait for the SNN
# inference to complete and is also reused as the bulk-bitmap read timeout.
read_line_timeout_ms = 2000
ready_timeout_ms = 2000
rx_ok_timeout_ms = 5000
processing_timeout_ms = 60000
result_info_timeout_ms = 2000
result_header_timeout_ms = 2000

queue_capacity = 4
result_retention = 4
max_image_bytes = 4194304   # rejects oversize submissions before they hit the wire

[snn-service.addr]
ip = "127.0.0.1"
port = 8092
```

## GraphQL surface

### Queries

| Field | Returns | Notes |
| --- | --- | --- |
| `ping` | `String` | `"pong"` |
| `health` | `HealthInfo` | UART config, phase, queue depth, jobs completed/failed, last error |
| `state` | `SnnState` | Driver phase, current image id, queued image ids, last error |
| `inferenceStatus(imageId)` | `JobStatus?` | Per-job phase + queue position + error |
| `getResult(imageId)` | `ResultPayload` | Bitmap as base64, with size + CRC. Marks job `DELIVERED`. |

`DriverPhase`: `INITIALIZING`, `IDLE`, `BUSY`, `SHUTTING_DOWN`, `FAULTED`.
`JobPhase`: `QUEUED`, `SENDING_IMAGE`, `PROCESSING`, `RESULT_READY`, `DELIVERED`,
`FAILED`, `CANCELLED`.

### Mutations

| Field | Returns | Notes |
| --- | --- | --- |
| `submitImage(imageBase64)` | `SubmitResponse` | Async path. Assigns image id, enqueues, returns immediately. |
| `infer(imageBase64)` | `InferenceResult` | Sync convenience: submit + poll-to-completion server-side. Holds the HTTP request open for the full cycle. |
| `cancel(imageId)` | `CancelResponse` | Best-effort. Only honoured while `QUEUED`. |

The async path (`submitImage` → `inferenceStatus` → `getResult`) is the canonical one
and is more robust to transient HTTP/network hiccups during the multi-second processing
window. `infer` is provided for tests and one-shot mission apps that want a single
blocking call.

## Example: a mission app inferring an image

Async path (recommended):

```graphql
mutation Submit($img: String!) {
  submitImage(imageBase64: $img) {
    success
    accepted
    imageId
    queuePosition
    error
  }
}

query Status($id: Int!) {
  inferenceStatus(imageId: $id) {
    phase
    error
  }
}

query Fetch($id: Int!) {
  getResult(imageId: $id) {
    imageId
    sizeBytes
    crc32
    bitmapBase64
  }
}
```

Mission-app pseudocode:

```rust
let id = client.submit_image(&base64_jpeg).await?;
loop {
    match client.inference_status(id).await?.phase {
        JobPhase::ResultReady | JobPhase::Delivered => break,
        JobPhase::Failed | JobPhase::Cancelled => bail!("inference failed"),
        _ => tokio::time::sleep(Duration::from_millis(500)).await,
    }
}
let result = client.get_result(id).await?;
let bitmap = base64::decode(result.bitmap_base64)?;
```

Sync convenience:

```graphql
mutation Infer($img: String!) {
  infer(imageBase64: $img) {
    success
    imageId
    sizeBytes
    crc32
    bitmapBase64
    error
  }
}
```

## Tests

```sh
cargo test -p snn-service
```

The suite covers the line parser, every command formatter, the CRC32 against the
canonical `"123456789"` test vector, and three driver integration tests against a
`MockStream`: a happy-path full inference cycle, a CRC mismatch on the returned bitmap,
and a payload `ERR …` propagated as `SnnError::PayloadNak`.

Hardware bring-up is not covered by the unit tests; verify against the real payload
board with a small mission-app harness exercising submit + poll + fetch and a known
JPEG/expected bitmap pair, plus a power-cycle of the payload mid-`PROCESSING` to confirm
the driver returns to `IDLE` cleanly on the next handshake.

## Operational notes

- The service is intended to run from the user partition and be restartable on orbit
  (it has no Buildroot package). Restarting clears all state — mission apps that want
  to survive a service restart should fetch results promptly rather than relying on the
  retention window.
- A `FAULTED` driver phase indicates an unrecoverable UART error; the service does not
  attempt to reopen the port automatically. Restart the service after diagnosing the
  bus.
- `max_image_bytes` is a guardrail against accidentally submitting something larger
  than the payload's expected input size — keep it in sync with the SNN's actual JPEG
  buffer.
