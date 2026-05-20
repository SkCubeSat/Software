# ESP32-OBC Communication Protocol

## Overview

This document outlines the UART communication protocol between the ESP32 CNN Payload and the On-Board Computer (OBC) for the RADSATSK2 CubeSat mission.

**Baud Rate:** 115200  
**Format:** 8N1  
**Markers:** All commands and responses use `<<` and `>>` delimiters (besides raw byte data mentioned)

---

## Command Summary

| Command | Description |
|---------|-------------|
| `<<run>>` | Execute full pipeline |
| `<<cap>>` | Capture photo only |
| `<<cnn>>` | Run CNN inference only |
| `<<send>>` | Send image to OBC |
| `<<sum>>` | Calculate cloud coverage |
| `<<recv>>` | Receive file from OBC |
| `<<ls>>` | List files on SPIFFS |
| `<<wait>>` | Test OBC handshake |

---

## Full Pipeline Protocol

The pipeline is triggered by `<<run>>` and executes these states in order:
**START → CAP → CNN → WAIT → SUM → SEND → DONE**

### Successful Pipeline

```
OBC → ESP32:  <<run>>

[ESP32 captures image to PSRAM]
[ESP32 runs CNN inference, saves mask to PSRAM]

ESP32 → OBC:  <<wait>>
OBC → ESP32:  <<ack>>

ESP32 → OBC:  <<coverage:0.234567>>

ESP32 → OBC:  <<send:72488>>
OBC → ESP32:  <<ack>>
ESP32 → OBC:  [72488 bytes of raw JPEG data]

ESP32 → OBC:  <<done>>
```

### Pipeline with Error

```
OBC → ESP32:  <<run>>

[ESP32 encounters error during any state]

ESP32 → OBC:  <<error:error_code>>
```

---

## Individual Command Protocols

### Capture Photo (`<<cap>>`)

Captures a photo and stores it in PSRAM.

```
OBC → ESP32:  <<cap>>

[ESP32 captures image]

[Returns to menu - no response on success]
```

**Note:** In standalone mode, no response is sent. Use `<<send>>` to retrieve the image.

---

### Run CNN (`<<cnn>>`)

Runs CNN inference on the image currently in PSRAM.

```
OBC → ESP32:  <<cnn>>

[ESP32 loads model, runs inference, saves mask to PSRAM]

[Returns to menu - no response on success]
```

**Prerequisite:** Image must be in PSRAM (run `<<cap>>` first)

---

### Send Image (`<<send>>`)

Sends the image from PSRAM to OBC.

```
OBC → ESP32:  <<send>>
ESP32 → OBC:  <<send:SIZE>>
OBC → ESP32:  <<ack>>
ESP32 → OBC:  [SIZE bytes of raw binary data]
```

**Example:**
```
OBC → ESP32:  <<send>>
ESP32 → OBC:  <<send:72488>>
OBC → ESP32:  <<ack>>
ESP32 → OBC:  [72488 bytes of JPEG data]
```

**Prerequisite:** Image must be in PSRAM (run `<<cap>>` first)

---

### Calculate Coverage (`<<sum>>`)

Calculates cloud coverage percentage from the mask in PSRAM.

```
OBC → ESP32:  <<sum>>
ESP32 → OBC:  <<coverage:FLOAT>>
```

**Example:**
```
OBC → ESP32:  <<sum>>
ESP32 → OBC:  <<coverage:0.234567>>
```

**Prerequisite:** Mask must be in PSRAM (run `<<cnn>>` first)

---

### Receive File (`<<recv>>`)

Transfers a file from OBC to ESP32 SPIFFS.

```
OBC → ESP32:  <<recv>>
ESP32 → OBC:  <<ready>>
OBC → ESP32:  FILENAME:SIZE\n
ESP32 → OBC:  <<ack:FILENAME:SIZE>>
OBC → ESP32:  [SIZE bytes of raw binary data]
ESP32 → OBC:  <<done:FILENAME:SIZE>>
```

**Example:**
```
OBC → ESP32:  <<recv>>
ESP32 → OBC:  <<ready>>
OBC → ESP32:  model.tflite:147972\n
ESP32 → OBC:  <<ack:model.tflite:147972>>
OBC → ESP32:  [147972 bytes of model data]
ESP32 → OBC:  <<done:model.tflite:147972>>
```

---

### List Files (`<<ls>>`)

Lists files stored on ESP32 SPIFFS.

```
OBC → ESP32:  <<ls>>
ESP32 → OBC:  Listing directory
ESP32 → OBC:  file1.jpg (12345b), file2.bin (6789b), ...
```

---

### Wait/Handshake (`<<wait>>`)

Tests the OBC handshake mechanism.

```
OBC → ESP32:  <<wait>>
ESP32 → OBC:  <<wait>>
OBC → ESP32:  <<ack>>
[ESP32 returns to menu]
```

---

## Error Codes

All errors are reported as `<<error:ERROR_CODE>>`. The pipeline jumps to DONE state on any error.

| Error Code | State | Description |
|------------|-------|-------------|
| `capture_failed` | CAP | Camera returned 0 bytes |
| `capture_malloc_failed` | CAP | Failed to allocate PSRAM for image |
| `cnn_model_load_failed` | CNN | Could not load model.tflite |
| `cnn_malloc_failed` | CNN | Failed to allocate inference buffers |
| `cnn_image_load_failed` | CNN | No image in PSRAM or decode failed |
| `cnn_inference_failed` | CNN | TFLite inference error |
| `cnn_mask_save_failed` | CNN | Failed to allocate mask buffer |
| `sum_mask_not_found` | SUM | No mask in PSRAM |
| `send_no_image` | SEND | No image in PSRAM |
| `send_timeout` | SEND | OBC did not send `<<ack>>` within 30s |
| `wait_timeout` | WAIT | OBC did not send `<<ack>>` within 30s |
| `ready_timeout` | RECV | Timeout waiting for file header |
| `recv_size_mismatch` | RECV | Received bytes != expected size |

---

## Data Flow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                         ESP32                                │
│                                                              │
│  ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐  │
│  │ Capture │───▶│  Image  │───▶│   CNN   │───▶│  Mask   │  │
│  │  State  │    │ (PSRAM) │    │  State  │    │ (PSRAM) │  │
│  └─────────┘    └────┬────┘    └─────────┘    └────┬────┘  │
│                      │                              │        │
│                      ▼                              ▼        │
│                 ┌─────────┐                   ┌─────────┐   │
│                 │  Send   │                   │   Sum   │   │
│                 │  State  │                   │  State  │   │
│                 └────┬────┘                   └────┬────┘   │
│                      │                              │        │
└──────────────────────┼──────────────────────────────┼────────┘
                       │                              │
                       ▼                              ▼
                ┌────────────┐                ┌────────────┐
                │   Image    │                │  Coverage  │
                │   (UART)   │                │   (UART)   │
                └────────────┘                └────────────┘
                       │                              │
                       ▼                              ▼
                ┌─────────────────────────────────────────┐
                │                   OBC                    │
                └─────────────────────────────────────────┘
```

---

## Memory Architecture

| Buffer | Location | Purpose |
|--------|----------|---------|
| Image Buffer | PSRAM (~8MB) | Stores captured JPEG image |
| Mask Buffer | PSRAM | Stores CNN output mask (48×64 bytes) |
| Model | SPIFFS → PSRAM | TFLite model loaded at inference time |
| Tensor Arena | PSRAM (1MB) | TFLite working memory |

Buffers are managed by the Context singleton and freed after pipeline completion.

---

## Timing Considerations

| Operation | Typical Duration |
|-----------|------------------|
| Camera capture | ~2-3 seconds |
| CNN inference | ~5-10 seconds |
| Image transfer (70KB @ 115200) | ~6 seconds |
| Full pipeline | ~15-20 seconds |

**Timeouts:**
- WAIT state: 30 seconds for `<<ack>>`
- SEND state: 30 seconds for `<<ack>>`
- RECV state: 30 seconds for file header

---

## OBC Implementation Notes

1. **Always wait for markers** - Don't assume timing; wait for each `<<marker>>`
2. **Binary data follows `<<ack>>`** - After sending `<<ack>>` to `<<send:SIZE>>`, immediately read SIZE bytes
3. **Parse size carefully** - Extract SIZE from `<<send:SIZE>>` before acknowledging
4. **Handle errors gracefully** - Any `<<error:...>>` means pipeline aborted
5. **Coverage is a float** - Parse `<<coverage:X.XXXXXX>>` as floating point (0.0 to 1.0)