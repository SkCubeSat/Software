use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;

use log::info;
use rust_uart::Connection;
use serial::{BaudRate, CharSize, FlowControl, Parity, PortSettings, StopBits};

use kubos_service::Config;

use crate::error::CnnError;
use crate::protocol::{self, CnnResponse};

/// Configuration extracted from `config.toml`.
#[derive(Clone, Debug)]
pub struct CnnConfig {
    pub uart_bus: String,
    pub uart_baud: u32,
    pub marker_timeout: Duration,
    pub pipeline_timeout: Duration,
    pub image_transfer_timeout: Duration,
    pub file_transfer_timeout: Duration,
}

/// Result of the full `<<run>>` pipeline.
#[derive(Clone, Debug)]
pub struct PipelineResult {
    pub coverage: f64,
    pub image: Vec<u8>,
    pub image_size: usize,
}

/// Result of the `<<send>>` query (image retrieval).
#[derive(Clone, Debug)]
pub struct ImageData {
    pub image: Vec<u8>,
    pub size: usize,
}

/// Result of the `<<recv>>` mutation (file upload to ESP32).
#[derive(Clone, Debug)]
pub struct RecvResult {
    pub filename: String,
    pub size: usize,
}

#[derive(Clone)]
pub struct Subsystem {
    conn: std::sync::Arc<Mutex<Connection>>,
    config: std::sync::Arc<CnnConfig>,
}

impl Subsystem {
    pub fn from_config(config: &Config) -> Result<Self, String> {
        let cnn_config = load_cnn_config(config)?;
        let connection = open_uart(&cnn_config).map_err(|e| format!("uart open failed: {e}"))?;
        info!(
            "cnn subsystem initialized on {} @ {}bps",
            cnn_config.uart_bus, cnn_config.uart_baud
        );
        Ok(Self {
            conn: std::sync::Arc::new(Mutex::new(connection)),
            config: std::sync::Arc::new(cnn_config),
        })
    }

    /// Execute the full pipeline: `<<run>>`.
    /// ESP32 drives the sequence: wait→ack, coverage, send→ack→bytes, done.
    pub fn run_pipeline(&self) -> Result<PipelineResult, CnnError> {
        let conn = self.conn.lock().map_err(|_| {
            CnnError::Internal("uart lock poisoned".to_string())
        })?;
        let timeout = self.config.pipeline_timeout;

        // Send <<run>>
        conn.write(&protocol::cmd_run())?;

        // ESP32 processes capture + CNN, then sends <<wait>>
        protocol::expect_response(&conn, timeout, |r| matches!(r, CnnResponse::Wait), "<<wait>>")?;

        // Respond with <<ack>>
        conn.write(&protocol::cmd_ack())?;

        // ESP32 sends <<coverage:FLOAT>>
        let coverage = match protocol::expect_response(
            &conn,
            timeout,
            |r| matches!(r, CnnResponse::Coverage(_)),
            "<<coverage:…>>",
        )? {
            CnnResponse::Coverage(v) => v,
            _ => return Err(CnnError::Protocol("expected coverage".to_string())),
        };

        // ESP32 sends <<send:SIZE>>
        let size = match protocol::expect_response(
            &conn,
            timeout,
            |r| matches!(r, CnnResponse::SendReady { .. }),
            "<<send:SIZE>>",
        )? {
            CnnResponse::SendReady { size } => size,
            _ => return Err(CnnError::Protocol("expected send:SIZE".to_string())),
        };

        // Respond with <<ack>>
        conn.write(&protocol::cmd_ack())?;

        // Read SIZE bytes of raw image data.
        let image = protocol::read_exact_bytes(&conn, size, self.config.image_transfer_timeout)?;

        // ESP32 sends <<done>>
        protocol::expect_response(&conn, self.config.marker_timeout, |r| {
            matches!(r, CnnResponse::Done)
        }, "<<done>>")?;

        Ok(PipelineResult {
            coverage,
            image,
            image_size: size,
        })
    }

    /// Capture photo only: `<<cap>>`.
    /// No response on success per protocol spec; we wait briefly for any error marker.
    pub fn capture(&self) -> Result<(), CnnError> {
        let conn = self.conn.lock().map_err(|_| {
            CnnError::Internal("uart lock poisoned".to_string())
        })?;

        conn.write(&protocol::cmd_cap())?;

        // The protocol says "no response on success". We wait a short time for a
        // potential <<error:…>> but treat a timeout as success.
        match protocol::read_response(&conn, self.config.marker_timeout) {
            Ok(CnnResponse::Error(code)) => Err(CnnError::PayloadError(code)),
            Ok(_) => Ok(()), // Unexpected but not an error — probably debug output.
            Err(CnnError::Timeout(_)) => Ok(()), // Expected: no response on success.
            Err(e) => Err(e),
        }
    }

    /// Run CNN inference only: `<<cnn>>`.
    /// Same response behaviour as capture.
    pub fn run_cnn(&self) -> Result<(), CnnError> {
        let conn = self.conn.lock().map_err(|_| {
            CnnError::Internal("uart lock poisoned".to_string())
        })?;

        conn.write(&protocol::cmd_cnn())?;

        match protocol::read_response(&conn, self.config.marker_timeout) {
            Ok(CnnResponse::Error(code)) => Err(CnnError::PayloadError(code)),
            Ok(_) => Ok(()),
            Err(CnnError::Timeout(_)) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Retrieve image from ESP32 PSRAM: `<<send>>`.
    pub fn send_image(&self) -> Result<ImageData, CnnError> {
        let conn = self.conn.lock().map_err(|_| {
            CnnError::Internal("uart lock poisoned".to_string())
        })?;

        conn.write(&protocol::cmd_send())?;

        // ESP32 responds with <<send:SIZE>>
        let size = match protocol::expect_response(
            &conn,
            self.config.marker_timeout,
            |r| matches!(r, CnnResponse::SendReady { .. }),
            "<<send:SIZE>>",
        )? {
            CnnResponse::SendReady { size } => size,
            _ => return Err(CnnError::Protocol("expected send:SIZE".to_string())),
        };

        // Send <<ack>>
        conn.write(&protocol::cmd_ack())?;

        // Read SIZE bytes of raw image data.
        let image = protocol::read_exact_bytes(&conn, size, self.config.image_transfer_timeout)?;

        Ok(ImageData { image, size })
    }

    /// Get cloud coverage: `<<sum>>`.
    pub fn get_coverage(&self) -> Result<f64, CnnError> {
        let conn = self.conn.lock().map_err(|_| {
            CnnError::Internal("uart lock poisoned".to_string())
        })?;

        conn.write(&protocol::cmd_sum())?;

        match protocol::expect_response(
            &conn,
            self.config.marker_timeout,
            |r| matches!(r, CnnResponse::Coverage(_)),
            "<<coverage:…>>",
        )? {
            CnnResponse::Coverage(v) => Ok(v),
            _ => Err(CnnError::Protocol("expected coverage".to_string())),
        }
    }

    /// Send a file from OBC to ESP32 SPIFFS: `<<recv>>`.
    /// Reads the file from `file_path` on the OBC filesystem.
    pub fn recv_file(&self, file_path: &str) -> Result<RecvResult, CnnError> {
        // Read the file from OBC disk first.
        let path = Path::new(file_path);
        let file_data = std::fs::read(path)?;
        let file_size = file_data.len();
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| CnnError::Protocol(format!("invalid file path: {file_path}")))?
            .to_string();

        let conn = self.conn.lock().map_err(|_| {
            CnnError::Internal("uart lock poisoned".to_string())
        })?;
        let timeout = self.config.file_transfer_timeout;

        // Send <<recv>>
        conn.write(&protocol::cmd_recv())?;

        // Wait for <<ready>>
        protocol::expect_response(&conn, timeout, |r| {
            matches!(r, CnnResponse::Ready)
        }, "<<ready>>")?;

        // Send FILENAME:SIZE\n
        conn.write(&protocol::file_header(&filename, file_size))?;

        // Wait for <<ack:FILENAME:SIZE>>
        protocol::expect_response(&conn, timeout, |r| {
            matches!(r, CnnResponse::AckRecv { .. })
        }, "<<ack:FILENAME:SIZE>>")?;

        // Send raw file bytes.
        conn.write(&file_data)?;

        // Wait for <<done:FILENAME:SIZE>>
        protocol::expect_response(&conn, timeout, |r| {
            matches!(r, CnnResponse::DoneRecv { .. })
        }, "<<done:FILENAME:SIZE>>")?;

        Ok(RecvResult {
            filename,
            size: file_size,
        })
    }

    /// List files on ESP32 SPIFFS: `<<ls>>`.
    pub fn list_files(&self) -> Result<String, CnnError> {
        let conn = self.conn.lock().map_err(|_| {
            CnnError::Internal("uart lock poisoned".to_string())
        })?;

        conn.write(&protocol::cmd_ls())?;

        // The ESP32 sends plain text listing followed by... we just read all available
        // text for the timeout period. The ls output is not marker-delimited per the
        // protocol spec, so we read lines until timeout.
        let mut output = Vec::with_capacity(512);
        let deadline = std::time::Instant::now() + self.config.marker_timeout;
        loop {
            let remaining = deadline.saturating_duration_since(std::time::Instant::now());
            if remaining.is_zero() {
                break;
            }
            match conn.read(1, remaining) {
                Ok(chunk) if !chunk.is_empty() => output.push(chunk[0]),
                _ => break,
            }
        }

        Ok(String::from_utf8_lossy(&output).trim().to_string())
    }

    /// Test handshake: `<<wait>>`.
    pub fn test_handshake(&self) -> Result<(), CnnError> {
        let conn = self.conn.lock().map_err(|_| {
            CnnError::Internal("uart lock poisoned".to_string())
        })?;

        // Send <<wait>>
        conn.write(&protocol::cmd_wait())?;

        // ESP32 responds with <<wait>>
        protocol::expect_response(&conn, self.config.marker_timeout, |r| {
            matches!(r, CnnResponse::Wait)
        }, "<<wait>> (echo)")?;

        // Respond with <<ack>>
        conn.write(&protocol::cmd_ack())?;

        Ok(())
    }
}

fn open_uart(config: &CnnConfig) -> Result<Connection, rust_uart::UartError> {
    Connection::from_path(
        &config.uart_bus,
        PortSettings {
            baud_rate: BaudRate::from_speed(config.uart_baud as usize),
            char_size: CharSize::Bits8,
            parity: Parity::ParityNone,
            stop_bits: StopBits::Stop1,
            flow_control: FlowControl::FlowNone,
        },
        config.marker_timeout,
    )
}

fn load_cnn_config(config: &Config) -> Result<CnnConfig, String> {
    let uart_bus = config
        .get("uart_bus")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "/dev/ttyS2".to_string());
    let uart_baud = config
        .get("uart_baud")
        .and_then(|v| v.as_integer())
        .map(|v| v as u32)
        .unwrap_or(115200);
    let marker_timeout = duration_ms(config, "marker_timeout_ms", 5000);
    let pipeline_timeout = duration_ms(config, "pipeline_timeout_ms", 30000);
    let image_transfer_timeout = duration_ms(config, "image_transfer_timeout_ms", 15000);
    let file_transfer_timeout = duration_ms(config, "file_transfer_timeout_ms", 30000);

    Ok(CnnConfig {
        uart_bus,
        uart_baud,
        marker_timeout,
        pipeline_timeout,
        image_transfer_timeout,
        file_transfer_timeout,
    })
}

fn duration_ms(config: &Config, key: &str, default_ms: u64) -> Duration {
    let ms = config
        .get(key)
        .and_then(|v| v.as_integer())
        .map(|v| v as u64)
        .unwrap_or(default_ms);
    Duration::from_millis(ms)
}
