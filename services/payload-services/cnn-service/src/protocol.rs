use std::time::{Duration, Instant};

use log::warn;
use rust_uart::Connection;

use crate::error::CnnError;

/// Maximum number of bytes to scan when searching for a `<<…>>` marker.
const MAX_MARKER_SCAN: usize = 1024;

/// Parsed responses from the ESP32 CNN payload.
#[derive(Debug, Clone, PartialEq)]
pub enum CnnResponse {
    /// `<<wait>>` — payload requesting OBC handshake.
    Wait,
    /// `<<coverage:FLOAT>>` — cloud coverage result.
    Coverage(f64),
    /// `<<send:SIZE>>` — payload ready to send SIZE bytes of image data.
    SendReady { size: usize },
    /// `<<ready>>` — payload ready for next phase.
    Ready,
    /// `<<ack>>` — payload acknowledgement.
    Ack,
    /// `<<ack:FILENAME:SIZE>>` — recv acknowledgement with file info.
    AckRecv { filename: String, size: usize },
    /// `<<done>>` — pipeline complete.
    Done,
    /// `<<done:FILENAME:SIZE>>` — recv transfer complete.
    DoneRecv { filename: String, size: usize },
    /// `<<error:ERROR_CODE>>` — payload error.
    Error(String),
    /// Anything we don't recognise.
    Unknown(String),
}

impl CnnResponse {
    /// Parse the inner content of a `<<…>>` marker into a structured response.
    pub fn parse(content: &str) -> CnnResponse {
        let content = content.trim();
        if content.is_empty() {
            return CnnResponse::Unknown(String::new());
        }

        match content {
            "wait" => return CnnResponse::Wait,
            "ready" => return CnnResponse::Ready,
            "ack" => return CnnResponse::Ack,
            "done" => return CnnResponse::Done,
            _ => {}
        }

        if let Some(rest) = content.strip_prefix("coverage:") {
            return match rest.parse::<f64>() {
                Ok(v) => CnnResponse::Coverage(v),
                Err(_) => CnnResponse::Unknown(content.to_string()),
            };
        }
        if let Some(rest) = content.strip_prefix("send:") {
            return match rest.parse::<usize>() {
                Ok(v) => CnnResponse::SendReady { size: v },
                Err(_) => CnnResponse::Unknown(content.to_string()),
            };
        }
        if let Some(rest) = content.strip_prefix("error:") {
            return CnnResponse::Error(rest.to_string());
        }
        if let Some(rest) = content.strip_prefix("ack:") {
            if let Some((filename, size_str)) = rest.rsplit_once(':') {
                if let Ok(size) = size_str.parse::<usize>() {
                    return CnnResponse::AckRecv {
                        filename: filename.to_string(),
                        size,
                    };
                }
            }
            return CnnResponse::Unknown(content.to_string());
        }
        if let Some(rest) = content.strip_prefix("done:") {
            if let Some((filename, size_str)) = rest.rsplit_once(':') {
                if let Ok(size) = size_str.parse::<usize>() {
                    return CnnResponse::DoneRecv {
                        filename: filename.to_string(),
                        size,
                    };
                }
            }
            return CnnResponse::Unknown(content.to_string());
        }

        CnnResponse::Unknown(content.to_string())
    }
}

// ---------------------------------------------------------------------------
// Command builders
// ---------------------------------------------------------------------------

pub fn cmd_run() -> Vec<u8> { b"<<run>>".to_vec() }
pub fn cmd_cap() -> Vec<u8> { b"<<cap>>".to_vec() }
pub fn cmd_cnn() -> Vec<u8> { b"<<cnn>>".to_vec() }
pub fn cmd_send() -> Vec<u8> { b"<<send>>".to_vec() }
pub fn cmd_sum() -> Vec<u8> { b"<<sum>>".to_vec() }
pub fn cmd_recv() -> Vec<u8> { b"<<recv>>".to_vec() }
pub fn cmd_ls() -> Vec<u8> { b"<<ls>>".to_vec() }
pub fn cmd_wait() -> Vec<u8> { b"<<wait>>".to_vec() }
pub fn cmd_ack() -> Vec<u8> { b"<<ack>>".to_vec() }

/// Build the file-header line sent during `<<recv>>`: `FILENAME:SIZE\n`.
pub fn file_header(filename: &str, size: usize) -> Vec<u8> {
    format!("{filename}:{size}\n").into_bytes()
}

// ---------------------------------------------------------------------------
// UART I/O helpers
// ---------------------------------------------------------------------------

/// Read bytes from the UART until a complete `<<…>>` marker is found.
/// Returns the inner content (between `<<` and `>>`).
pub fn read_marker(conn: &Connection, timeout: Duration) -> Result<String, CnnError> {
    let deadline = Instant::now() + timeout;
    let mut scanned: usize = 0;

    // Phase 1: scan for the opening '<<'.
    let mut found_first = false;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(CnnError::Timeout("marker opening <<"));
        }
        if scanned >= MAX_MARKER_SCAN {
            return Err(CnnError::Protocol(format!(
                "scanned {MAX_MARKER_SCAN} bytes without finding << marker"
            )));
        }
        let byte = read_byte(conn, remaining)?;
        scanned += 1;
        if byte == b'<' {
            if found_first {
                break;
            }
            found_first = true;
        } else {
            found_first = false;
        }
    }

    // Phase 2: read until the closing '>>'.
    let mut content = Vec::with_capacity(64);
    let mut found_first_close = false;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(CnnError::Timeout("marker closing >>"));
        }
        if scanned >= MAX_MARKER_SCAN {
            return Err(CnnError::Protocol(
                "marker content exceeded max scan length".to_string(),
            ));
        }
        let byte = read_byte(conn, remaining)?;
        scanned += 1;
        if byte == b'>' {
            if found_first_close {
                return Ok(String::from_utf8_lossy(&content).into_owned());
            }
            found_first_close = true;
        } else {
            if found_first_close {
                content.push(b'>');
                found_first_close = false;
            }
            content.push(byte);
        }
    }
}

/// Read a marker, parse it, and return the structured response.
pub fn read_response(conn: &Connection, timeout: Duration) -> Result<CnnResponse, CnnError> {
    let content = read_marker(conn, timeout)?;
    Ok(CnnResponse::parse(&content))
}

/// Read a marker, expecting a specific variant. Errors and unknowns are handled.
pub fn expect_response(
    conn: &Connection,
    timeout: Duration,
    pred: impl Fn(&CnnResponse) -> bool,
    context: &'static str,
) -> Result<CnnResponse, CnnError> {
    let deadline = Instant::now() + timeout;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(CnnError::Timeout(context));
        }
        let resp = read_response(conn, remaining)?;
        if pred(&resp) {
            return Ok(resp);
        }
        if let CnnResponse::Error(code) = &resp {
            return Err(CnnError::PayloadError(code.clone()));
        }
        warn!("cnn: discarding unexpected response while waiting for {context}: {resp:?}");
    }
}

/// Read exactly `count` raw bytes from the UART (for image / file transfer).
pub fn read_exact_bytes(
    conn: &Connection,
    count: usize,
    timeout: Duration,
) -> Result<Vec<u8>, CnnError> {
    let data = conn.read(count, timeout)?;
    if data.len() != count {
        return Err(CnnError::Protocol(format!(
            "expected {count} bytes, got {}",
            data.len()
        )));
    }
    Ok(data)
}

/// Read all available text until a `<<` marker opening is detected.
/// Used by `<<ls>>` where the ESP32 sends plain text followed by a marker.
pub fn read_text_until_marker(
    conn: &Connection,
    timeout: Duration,
) -> Result<(String, CnnResponse), CnnError> {
    let deadline = Instant::now() + timeout;
    let mut text = Vec::with_capacity(256);
    let mut found_first_open = false;

    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(CnnError::Timeout("ls text or marker"));
        }
        if text.len() >= MAX_MARKER_SCAN {
            return Err(CnnError::Protocol(
                "ls output exceeded max scan length".to_string(),
            ));
        }
        let byte = read_byte(conn, remaining)?;
        if byte == b'<' {
            if found_first_open {
                // Got '<<' — now read the marker content inline.
                let mut content = Vec::with_capacity(64);
                let mut found_first_close = false;
                loop {
                    let remaining = deadline.saturating_duration_since(Instant::now());
                    if remaining.is_zero() {
                        return Err(CnnError::Timeout("ls closing marker"));
                    }
                    let b = read_byte(conn, remaining)?;
                    if b == b'>' {
                        if found_first_close {
                            let marker_text = String::from_utf8_lossy(&content).into_owned();
                            let resp = CnnResponse::parse(&marker_text);
                            let listing = String::from_utf8_lossy(&text).into_owned();
                            return Ok((listing, resp));
                        }
                        found_first_close = true;
                    } else {
                        if found_first_close {
                            content.push(b'>');
                            found_first_close = false;
                        }
                        content.push(b);
                    }
                }
            }
            found_first_open = true;
        } else {
            if found_first_open {
                text.push(b'<');
                found_first_open = false;
            }
            text.push(byte);
        }
    }
}

/// Read a single byte from the UART.
fn read_byte(conn: &Connection, timeout: Duration) -> Result<u8, CnnError> {
    let chunk = conn.read(1, timeout)?;
    if chunk.is_empty() {
        return Err(CnnError::Timeout("single byte"));
    }
    Ok(chunk[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_wait() {
        assert_eq!(CnnResponse::parse("wait"), CnnResponse::Wait);
    }

    #[test]
    fn parse_ready() {
        assert_eq!(CnnResponse::parse("ready"), CnnResponse::Ready);
    }

    #[test]
    fn parse_ack() {
        assert_eq!(CnnResponse::parse("ack"), CnnResponse::Ack);
    }

    #[test]
    fn parse_done() {
        assert_eq!(CnnResponse::parse("done"), CnnResponse::Done);
    }

    #[test]
    fn parse_coverage() {
        assert_eq!(
            CnnResponse::parse("coverage:0.234567"),
            CnnResponse::Coverage(0.234567)
        );
    }

    #[test]
    fn parse_send_ready() {
        assert_eq!(
            CnnResponse::parse("send:72488"),
            CnnResponse::SendReady { size: 72488 }
        );
    }

    #[test]
    fn parse_error() {
        assert_eq!(
            CnnResponse::parse("error:capture_failed"),
            CnnResponse::Error("capture_failed".to_string())
        );
    }

    #[test]
    fn parse_ack_recv() {
        assert_eq!(
            CnnResponse::parse("ack:model.tflite:147972"),
            CnnResponse::AckRecv {
                filename: "model.tflite".to_string(),
                size: 147972,
            }
        );
    }

    #[test]
    fn parse_done_recv() {
        assert_eq!(
            CnnResponse::parse("done:model.tflite:147972"),
            CnnResponse::DoneRecv {
                filename: "model.tflite".to_string(),
                size: 147972,
            }
        );
    }

    #[test]
    fn parse_unknown() {
        assert_eq!(
            CnnResponse::parse("something_else"),
            CnnResponse::Unknown("something_else".to_string())
        );
    }

    #[test]
    fn cmd_builders() {
        assert_eq!(cmd_run(), b"<<run>>");
        assert_eq!(cmd_cap(), b"<<cap>>");
        assert_eq!(cmd_cnn(), b"<<cnn>>");
        assert_eq!(cmd_send(), b"<<send>>");
        assert_eq!(cmd_sum(), b"<<sum>>");
        assert_eq!(cmd_recv(), b"<<recv>>");
        assert_eq!(cmd_ls(), b"<<ls>>");
        assert_eq!(cmd_wait(), b"<<wait>>");
        assert_eq!(cmd_ack(), b"<<ack>>");
    }

    #[test]
    fn file_header_format() {
        assert_eq!(
            file_header("model.tflite", 147972),
            b"model.tflite:147972\n"
        );
    }
}
