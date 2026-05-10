use crate::error::SnnError;

/// Parsed lines we expect from the payload board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PayloadLine {
    /// Sent unsolicited at boot.
    PayloadReady,
    /// Status query response: payload is currently idle.
    Idle,
    /// Status query response: payload is processing image `id`.
    Busy { phase: String, image_id: Option<u32> },
    /// Payload is ready to accept the next byte stream (image bytes or our READY ack).
    Ready,
    /// Image `id` was received successfully.
    RxOk { image_id: u32 },
    /// Payload acknowledges and starts processing.
    Processing { image_id: u32 },
    /// Inference completed for image `id` (notification form, no size/crc).
    ResultReadyNotify { image_id: u32 },
    /// Response to `GET_RESULT_INFO id`. Phase is e.g. "READY".
    ResultInfo {
        image_id: u32,
        phase: String,
        size: u32,
        crc32: u32,
    },
    /// Response to `GET_RESULT id` immediately before the bitmap blob.
    ResultHeader {
        image_id: u32,
        size: u32,
        crc32: u32,
    },
    /// Payload reports an error: e.g. `ERR <code> <description>`.
    Error { code: String, message: String },
    /// Anything we don't recognise. Kept for diagnostics.
    Unknown(String),
}

impl PayloadLine {
    /// Parse a single line (without trailing newline) into a structured response.
    pub fn parse(line: &str) -> PayloadLine {
        let line = line.trim();
        if line.is_empty() {
            return PayloadLine::Unknown(String::new());
        }

        let mut parts = line.split_ascii_whitespace();
        let head = match parts.next() {
            Some(h) => h,
            None => return PayloadLine::Unknown(line.to_string()),
        };
        let rest: Vec<&str> = parts.collect();

        match head {
            "PAYLOAD_READY" => PayloadLine::PayloadReady,
            "IDLE" => PayloadLine::Idle,
            "READY" => PayloadLine::Ready,
            "RX_OK" => match rest.first().and_then(|s| parse_u32(s)) {
                Some(id) => PayloadLine::RxOk { image_id: id },
                None => PayloadLine::Unknown(line.to_string()),
            },
            "PROCESSING" => match rest.first().and_then(|s| parse_u32(s)) {
                Some(id) => PayloadLine::Processing { image_id: id },
                None => PayloadLine::Unknown(line.to_string()),
            },
            "RESULT_READY" => parse_result_ready(&rest, line),
            "RESULT_INFO" => parse_result_info(&rest, line),
            "BUSY" => PayloadLine::Busy {
                phase: rest.first().map(|s| s.to_string()).unwrap_or_default(),
                image_id: rest.get(1).and_then(|s| parse_u32(s)),
            },
            "ERR" | "ERROR" => PayloadLine::Error {
                code: rest.first().map(|s| s.to_string()).unwrap_or_default(),
                message: rest.get(1..).map(|t| t.join(" ")).unwrap_or_default(),
            },
            _ => PayloadLine::Unknown(line.to_string()),
        }
    }
}

/// `RESULT_READY` overload:
///   * `RESULT_READY <id>`                       — completion notification
///   * `RESULT_READY <id> <size> <crc32>`        — header before bitmap blob
fn parse_result_ready(rest: &[&str], line: &str) -> PayloadLine {
    match rest.len() {
        1 => match parse_u32(rest[0]) {
            Some(id) => PayloadLine::ResultReadyNotify { image_id: id },
            None => PayloadLine::Unknown(line.to_string()),
        },
        3 => match (parse_u32(rest[0]), parse_u32(rest[1]), parse_hex32(rest[2])) {
            (Some(id), Some(size), Some(crc)) => PayloadLine::ResultHeader {
                image_id: id,
                size,
                crc32: crc,
            },
            _ => PayloadLine::Unknown(line.to_string()),
        },
        _ => PayloadLine::Unknown(line.to_string()),
    }
}

/// `RESULT_INFO <id> <phase> <size> <crc32>`
fn parse_result_info(rest: &[&str], line: &str) -> PayloadLine {
    if rest.len() != 4 {
        return PayloadLine::Unknown(line.to_string());
    }
    match (
        parse_u32(rest[0]),
        parse_u32(rest[2]),
        parse_hex32(rest[3]),
    ) {
        (Some(id), Some(size), Some(crc)) => PayloadLine::ResultInfo {
            image_id: id,
            phase: rest[1].to_string(),
            size,
            crc32: crc,
        },
        _ => PayloadLine::Unknown(line.to_string()),
    }
}

fn parse_u32(s: &str) -> Option<u32> {
    s.parse::<u32>().ok()
}

fn parse_hex32(s: &str) -> Option<u32> {
    let stripped = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    u32::from_str_radix(stripped, 16).ok()
}

/// Render an OBC-to-payload command as a single ASCII line, terminated by `\n`.
pub fn cmd_status() -> Vec<u8> {
    b"STATUS\n".to_vec()
}

pub fn cmd_send(image_id: u32, size: u32, crc32: u32) -> Vec<u8> {
    format!("SEND {image_id} {size} {crc32:08X}\n").into_bytes()
}

pub fn cmd_get_result_info(image_id: u32) -> Vec<u8> {
    format!("GET_RESULT_INFO {image_id}\n").into_bytes()
}

pub fn cmd_get_result(image_id: u32) -> Vec<u8> {
    format!("GET_RESULT {image_id}\n").into_bytes()
}

pub fn cmd_ready() -> Vec<u8> {
    b"READY\n".to_vec()
}

pub fn cmd_result_rx_ok(image_id: u32) -> Vec<u8> {
    format!("RESULT_RX_OK {image_id}\n").into_bytes()
}

/// CRC-32 (IEEE) of a byte slice — same algorithm the payload uses.
pub fn crc32(data: &[u8]) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

/// Convert a parsed `PayloadLine::Error` (or any unexpected line) into an `SnnError`.
pub fn unexpected_line(context: &str, line: &PayloadLine) -> SnnError {
    match line {
        PayloadLine::Error { code, message } => {
            SnnError::PayloadNak(format!("[{code}] {message}").trim().to_string())
        }
        other => SnnError::Protocol(format!("expected {context}, got {other:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_payload_ready() {
        assert_eq!(PayloadLine::parse("PAYLOAD_READY"), PayloadLine::PayloadReady);
        assert_eq!(PayloadLine::parse("PAYLOAD_READY\r"), PayloadLine::PayloadReady);
    }

    #[test]
    fn parses_idle_and_ready() {
        assert_eq!(PayloadLine::parse("IDLE"), PayloadLine::Idle);
        assert_eq!(PayloadLine::parse("READY"), PayloadLine::Ready);
    }

    #[test]
    fn parses_rx_ok() {
        assert_eq!(
            PayloadLine::parse("RX_OK 42"),
            PayloadLine::RxOk { image_id: 42 }
        );
    }

    #[test]
    fn parses_processing() {
        assert_eq!(
            PayloadLine::parse("PROCESSING 42"),
            PayloadLine::Processing { image_id: 42 }
        );
    }

    #[test]
    fn parses_result_ready_notification() {
        assert_eq!(
            PayloadLine::parse("RESULT_READY 42"),
            PayloadLine::ResultReadyNotify { image_id: 42 }
        );
    }

    #[test]
    fn parses_result_ready_header() {
        assert_eq!(
            PayloadLine::parse("RESULT_READY 42 12288 A1B2C3D4"),
            PayloadLine::ResultHeader {
                image_id: 42,
                size: 12288,
                crc32: 0xA1B2C3D4,
            }
        );
    }

    #[test]
    fn parses_result_info() {
        assert_eq!(
            PayloadLine::parse("RESULT_INFO 42 READY 12288 A1B2C3D4"),
            PayloadLine::ResultInfo {
                image_id: 42,
                phase: "READY".to_string(),
                size: 12288,
                crc32: 0xA1B2C3D4,
            }
        );
    }

    #[test]
    fn parses_busy_with_phase() {
        assert_eq!(
            PayloadLine::parse("BUSY PROCESSING 42"),
            PayloadLine::Busy {
                phase: "PROCESSING".to_string(),
                image_id: Some(42),
            }
        );
    }

    #[test]
    fn parses_error() {
        assert_eq!(
            PayloadLine::parse("ERR BAD_CRC image 42"),
            PayloadLine::Error {
                code: "BAD_CRC".to_string(),
                message: "image 42".to_string(),
            }
        );
    }

    #[test]
    fn unknown_line_preserved() {
        assert_eq!(
            PayloadLine::parse("WAT IS THIS"),
            PayloadLine::Unknown("WAT IS THIS".to_string())
        );
    }

    #[test]
    fn formats_send_command() {
        assert_eq!(
            cmd_send(42, 813244, 0x6DE17C6C),
            b"SEND 42 813244 6DE17C6C\n".to_vec()
        );
    }

    #[test]
    fn formats_get_result_info() {
        assert_eq!(cmd_get_result_info(42), b"GET_RESULT_INFO 42\n".to_vec());
    }

    #[test]
    fn formats_result_rx_ok() {
        assert_eq!(cmd_result_rx_ok(42), b"RESULT_RX_OK 42\n".to_vec());
    }

    #[test]
    fn crc32_known_vector() {
        // CRC-32 of "123456789" == 0xCBF43926, the canonical test vector.
        assert_eq!(crc32(b"123456789"), 0xCBF43926);
    }
}
