use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use log::{error, info, warn};
use rust_uart::{Connection, UartError};
use serial::{BaudRate, CharSize, FlowControl, Parity, PortSettings, StopBits};

use crate::error::SnnError;
use crate::protocol::{self, PayloadLine};

pub const MAX_LINE_BYTES: usize = 256;

/// Per-driver tunables (mirrored from `config.toml`).
#[derive(Clone, Debug)]
pub struct DriverConfig {
    pub uart_bus: String,
    pub uart_baud: u32,
    pub read_line_timeout: Duration,
    pub ready_timeout: Duration,
    pub rx_ok_timeout: Duration,
    pub processing_timeout: Duration,
    pub result_info_timeout: Duration,
    pub result_header_timeout: Duration,
    pub queue_capacity: usize,
    pub result_retention: usize,
    pub max_image_bytes: usize,
}

/// Overall driver lifecycle phase, surfaced via GraphQL `state` query.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DriverPhase {
    /// Service has started but hasn't confirmed the payload is alive yet.
    Initializing,
    /// Payload reachable and idle; ready to accept work.
    Idle,
    /// Currently executing a job (`current_image_id` will be set).
    Busy,
    /// Driver is shutting down; no new work accepted.
    ShuttingDown,
    /// Hard fault; UART unreachable. Inspect `last_error` for details.
    Faulted,
}

impl DriverPhase {
    pub fn as_str(&self) -> &'static str {
        match self {
            DriverPhase::Initializing => "INITIALIZING",
            DriverPhase::Idle => "IDLE",
            DriverPhase::Busy => "BUSY",
            DriverPhase::ShuttingDown => "SHUTTING_DOWN",
            DriverPhase::Faulted => "FAULTED",
        }
    }
}

/// Per-job lifecycle phase, surfaced via GraphQL `inferenceStatus(id)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JobPhase {
    Queued,
    SendingImage,
    Processing,
    ResultReady,
    Delivered,
    Failed,
    Cancelled,
}

impl JobPhase {
    pub fn as_str(&self) -> &'static str {
        match self {
            JobPhase::Queued => "QUEUED",
            JobPhase::SendingImage => "SENDING_IMAGE",
            JobPhase::Processing => "PROCESSING",
            JobPhase::ResultReady => "RESULT_READY",
            JobPhase::Delivered => "DELIVERED",
            JobPhase::Failed => "FAILED",
            JobPhase::Cancelled => "CANCELLED",
        }
    }
}

#[derive(Clone, Debug)]
pub struct PendingJob {
    pub image_id: u32,
    pub image: Vec<u8>,
    pub crc32: u32,
}

#[derive(Clone, Debug)]
pub struct ResultEntry {
    pub image_id: u32,
    pub bitmap: Vec<u8>,
    pub size: u32,
    pub crc32: u32,
    pub phase: JobPhase,
    pub error: Option<String>,
}

#[derive(Clone, Debug)]
pub struct JobStatus {
    pub image_id: u32,
    pub phase: JobPhase,
    pub queue_position: Option<usize>,
    pub error: Option<String>,
}

/// Mutable state shared between the driver thread and GraphQL handlers.
/// Always behind `Arc<Mutex<…>>` paired with a `Condvar` for wake-ups.
pub struct SharedState {
    pub phase: DriverPhase,
    pub current_image_id: Option<u32>,
    pub pending: VecDeque<PendingJob>,
    /// Per-job status, keyed by image id. Includes pending, in-flight, and recently finished.
    pub jobs: HashMap<u32, JobStatus>,
    pub results: HashMap<u32, ResultEntry>,
    /// Insertion order for LRU eviction of `results`.
    pub result_lru: VecDeque<u32>,
    pub last_error: Option<String>,
    pub jobs_completed: u64,
    pub jobs_failed: u64,
    pub next_image_id: u32,
    pub shutdown: bool,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            phase: DriverPhase::Initializing,
            current_image_id: None,
            pending: VecDeque::new(),
            jobs: HashMap::new(),
            results: HashMap::new(),
            result_lru: VecDeque::new(),
            last_error: None,
            jobs_completed: 0,
            jobs_failed: 0,
            next_image_id: 1,
            shutdown: false,
        }
    }
}

impl SharedState {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone)]
pub struct DriverHandle {
    pub state: Arc<Mutex<SharedState>>,
    pub cond: Arc<Condvar>,
}

impl Default for DriverHandle {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(SharedState::new())),
            cond: Arc::new(Condvar::new()),
        }
    }
}

impl DriverHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn signal_shutdown(&self) {
        if let Ok(mut guard) = self.state.lock() {
            guard.shutdown = true;
        }
        self.cond.notify_all();
    }
}

/// Spawn the driver on a dedicated OS thread. UART I/O is blocking, so we deliberately
/// stay off the tokio runtime here.
pub fn spawn(config: DriverConfig, handle: DriverHandle) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new()
        .name("snn-driver".to_string())
        .spawn(move || run(config, handle))
        .expect("failed to spawn snn driver thread")
}

fn run(config: DriverConfig, handle: DriverHandle) {
    info!("snn driver starting on {} @ {}bps", config.uart_bus, config.uart_baud);

    let connection = match open_uart(&config) {
        Ok(c) => c,
        Err(err) => {
            error!("failed to open uart {}: {err}", config.uart_bus);
            let mut guard = handle.state.lock().expect("state lock");
            guard.phase = DriverPhase::Faulted;
            guard.last_error = Some(format!("uart open failed: {err}"));
            return;
        }
    };

    // Best-effort initial handshake: send STATUS, expect IDLE (or BUSY → wait briefly).
    match initial_handshake(&connection, &config) {
        Ok(()) => {
            let mut guard = handle.state.lock().expect("state lock");
            guard.phase = DriverPhase::Idle;
        }
        Err(err) => {
            warn!("initial handshake failed (continuing): {err}");
            let mut guard = handle.state.lock().expect("state lock");
            // Stay Initializing; first job submission will retry the handshake implicitly.
            guard.last_error = Some(format!("handshake: {err}"));
        }
    }

    loop {
        let job = {
            let mut guard = handle.state.lock().expect("state lock");
            while guard.pending.is_empty() && !guard.shutdown {
                guard = handle.cond.wait(guard).expect("cond wait");
            }
            if guard.shutdown {
                guard.phase = DriverPhase::ShuttingDown;
                break;
            }
            // Mark job as in-flight.
            let job = guard.pending.pop_front().expect("pending non-empty");
            guard.current_image_id = Some(job.image_id);
            guard.phase = DriverPhase::Busy;
            if let Some(status) = guard.jobs.get_mut(&job.image_id) {
                status.phase = JobPhase::SendingImage;
                status.queue_position = None;
            }
            recalc_queue_positions(&mut guard);
            job
        };

        let result = execute_job(&connection, &config, &job, &handle);

        let mut guard = handle.state.lock().expect("state lock");
        guard.current_image_id = None;
        guard.phase = DriverPhase::Idle;
        match result {
            Ok(entry) => {
                guard.jobs_completed += 1;
                if let Some(status) = guard.jobs.get_mut(&job.image_id) {
                    status.phase = JobPhase::ResultReady;
                }
                store_result(&mut guard, &config, entry);
            }
            Err(err) => {
                guard.jobs_failed += 1;
                let msg = err.to_string();
                guard.last_error = Some(msg.clone());
                if let Some(status) = guard.jobs.get_mut(&job.image_id) {
                    status.phase = JobPhase::Failed;
                    status.error = Some(msg);
                }
                // Unrecoverable wire errors push us to Faulted; logical NAKs leave us Idle.
                if matches!(err, SnnError::Uart(_) | SnnError::Timeout(_)) {
                    guard.phase = DriverPhase::Faulted;
                }
            }
        }
        // Wake any infer-style waiters polling the shared state.
        handle.cond.notify_all();
    }

    info!("snn driver shut down");
}

fn open_uart(config: &DriverConfig) -> Result<Connection, UartError> {
    Connection::from_path(
        &config.uart_bus,
        PortSettings {
            baud_rate: BaudRate::from_speed(config.uart_baud as usize),
            char_size: CharSize::Bits8,
            parity: Parity::ParityNone,
            stop_bits: StopBits::Stop1,
            flow_control: FlowControl::FlowNone,
        },
        config.read_line_timeout,
    )
}

/// On startup, send STATUS once and look for IDLE. We tolerate a stray PAYLOAD_READY
/// preceding it (boot announcement) and a BUSY response (payload mid-cycle from before
/// we started — wait it out for one processing window).
fn initial_handshake(conn: &Connection, config: &DriverConfig) -> Result<(), SnnError> {
    conn.write(&protocol::cmd_status())?;
    let deadline = Instant::now() + config.processing_timeout;
    loop {
        let line = read_line(conn, config.read_line_timeout)?;
        match PayloadLine::parse(&line) {
            PayloadLine::Idle => return Ok(()),
            PayloadLine::PayloadReady => continue,
            PayloadLine::Busy { .. } | PayloadLine::Processing { .. } => {
                if Instant::now() >= deadline {
                    return Err(SnnError::NotIdle("BUSY".to_string()));
                }
                std::thread::sleep(Duration::from_millis(500));
                conn.write(&protocol::cmd_status())?;
            }
            PayloadLine::ResultReadyNotify { .. } => {
                // Stale completion from a previous run we don't know about.
                // Drain it by reading whatever the payload offers next.
                continue;
            }
            other => return Err(protocol::unexpected_line("IDLE", &other)),
        }
    }
}

/// Run the full multi-step protocol for one image. Updates `JobPhase` along the way.
pub(crate) fn execute_job(
    conn: &Connection,
    config: &DriverConfig,
    job: &PendingJob,
    handle: &DriverHandle,
) -> Result<ResultEntry, SnnError> {
    let id = job.image_id;

    // Send SEND header, expect READY.
    conn.write(&protocol::cmd_send(id, job.image.len() as u32, job.crc32))?;
    expect_line(conn, config.ready_timeout, |line| {
        matches!(line, PayloadLine::Ready)
    }, "READY (after SEND)")?;

    // Stream raw image bytes.
    conn.write(&job.image)?;

    // Expect RX_OK <id>.
    expect_line(conn, config.rx_ok_timeout, |line| {
        matches!(line, PayloadLine::RxOk { image_id } if *image_id == id)
    }, "RX_OK <id>")?;

    set_job_phase(handle, id, JobPhase::Processing);

    // Wait for RESULT_READY <id> notify form. `expect_line` discards intermediate lines
    // (e.g. an informational `PROCESSING <id>`) and only returns once the predicate matches.
    expect_line(
        conn,
        config.processing_timeout,
        |line| matches!(line, PayloadLine::ResultReadyNotify { image_id } if *image_id == id),
        "RESULT_READY <id> (notify)",
    )?;

    // GET_RESULT_INFO <id> -> RESULT_INFO <id> READY <size> <crc>
    conn.write(&protocol::cmd_get_result_info(id))?;
    let (info_size, info_crc) = match expect_line(
        conn,
        config.result_info_timeout,
        |line| matches!(line, PayloadLine::ResultInfo { image_id, .. } if *image_id == id),
        "RESULT_INFO <id> ...",
    )? {
        PayloadLine::ResultInfo {
            phase, size, crc32, ..
        } => {
            if phase != "READY" {
                return Err(SnnError::Protocol(format!(
                    "RESULT_INFO phase != READY (got {phase})"
                )));
            }
            (size, crc32)
        }
        other => return Err(protocol::unexpected_line("RESULT_INFO", &other)),
    };

    // GET_RESULT <id> -> RESULT_READY <id> <size> <crc>
    conn.write(&protocol::cmd_get_result(id))?;
    let (hdr_size, hdr_crc) = match expect_line(
        conn,
        config.result_header_timeout,
        |line| matches!(line, PayloadLine::ResultHeader { image_id, .. } if *image_id == id),
        "RESULT_READY <id> <size> <crc>",
    )? {
        PayloadLine::ResultHeader { size, crc32, .. } => (size, crc32),
        other => return Err(protocol::unexpected_line("RESULT_READY <header>", &other)),
    };

    if hdr_size != info_size || hdr_crc != info_crc {
        return Err(SnnError::Protocol(format!(
            "RESULT_INFO/RESULT_READY mismatch: info=({info_size}, {info_crc:08X}) hdr=({hdr_size}, {hdr_crc:08X})"
        )));
    }

    // OBC sends READY, then payload streams the bitmap bytes.
    conn.write(&protocol::cmd_ready())?;
    let bitmap = conn.read(hdr_size as usize, config.processing_timeout)?;

    let actual_crc = protocol::crc32(&bitmap);
    if actual_crc != hdr_crc {
        return Err(SnnError::CrcMismatch {
            expected: hdr_crc,
            actual: actual_crc,
        });
    }

    // Acknowledge.
    conn.write(&protocol::cmd_result_rx_ok(id))?;

    Ok(ResultEntry {
        image_id: id,
        bitmap,
        size: hdr_size,
        crc32: hdr_crc,
        phase: JobPhase::ResultReady,
        error: None,
    })
}

/// Read a single line (terminated by '\n'). Strips trailing '\r'. Bounded by `MAX_LINE_BYTES`
/// to prevent runaway reads when the wire is producing junk.
fn read_line(conn: &Connection, timeout: Duration) -> Result<String, SnnError> {
    let deadline = Instant::now() + timeout;
    let mut buf = Vec::with_capacity(64);
    loop {
        if Instant::now() >= deadline {
            return Err(SnnError::Timeout("line"));
        }
        if buf.len() >= MAX_LINE_BYTES {
            return Err(SnnError::Protocol(format!(
                "line exceeded {MAX_LINE_BYTES} bytes without newline"
            )));
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        let chunk = conn.read(1, remaining.max(Duration::from_millis(1)))?;
        let byte = chunk[0];
        if byte == b'\n' {
            // Strip optional trailing '\r'.
            if buf.last().copied() == Some(b'\r') {
                buf.pop();
            }
            return Ok(String::from_utf8_lossy(&buf).into_owned());
        }
        buf.push(byte);
    }
}

/// Read lines until one matches `pred`. Returns the matching parsed line.
/// Unknown / unrelated lines are logged and discarded.
fn expect_line(
    conn: &Connection,
    timeout: Duration,
    pred: impl Fn(&PayloadLine) -> bool,
    context: &'static str,
) -> Result<PayloadLine, SnnError> {
    let deadline = Instant::now() + timeout;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(SnnError::Timeout(context));
        }
        let raw = read_line(conn, remaining)?;
        let line = PayloadLine::parse(&raw);
        if pred(&line) {
            return Ok(line);
        }
        if let PayloadLine::Error { .. } = &line {
            return Err(protocol::unexpected_line(context, &line));
        }
        warn!("snn: discarding unexpected line while waiting for {context}: {raw:?}");
    }
}

fn set_job_phase(handle: &DriverHandle, image_id: u32, phase: JobPhase) {
    if let Ok(mut guard) = handle.state.lock()
        && let Some(status) = guard.jobs.get_mut(&image_id)
    {
        status.phase = phase;
    }
}

fn recalc_queue_positions(state: &mut SharedState) {
    for (idx, job) in state.pending.iter().enumerate() {
        if let Some(status) = state.jobs.get_mut(&job.image_id) {
            status.queue_position = Some(idx);
            status.phase = JobPhase::Queued;
        }
    }
}

fn store_result(state: &mut SharedState, config: &DriverConfig, entry: ResultEntry) {
    let id = entry.image_id;
    state.results.insert(id, entry);
    state.result_lru.push_back(id);
    while state.result_lru.len() > config.result_retention {
        if let Some(evicted) = state.result_lru.pop_front() {
            state.results.remove(&evicted);
            // Drop job-status entries for evicted results too, so memory stays bounded.
            if let Some(status) = state.jobs.get(&evicted)
                && matches!(
                    status.phase,
                    JobPhase::ResultReady
                        | JobPhase::Delivered
                        | JobPhase::Failed
                        | JobPhase::Cancelled
                )
            {
                state.jobs.remove(&evicted);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_uart::mock::MockStream;

    fn test_config() -> DriverConfig {
        DriverConfig {
            uart_bus: "mock".to_string(),
            uart_baud: 115200,
            read_line_timeout: Duration::from_secs(1),
            ready_timeout: Duration::from_secs(1),
            rx_ok_timeout: Duration::from_secs(1),
            processing_timeout: Duration::from_secs(2),
            result_info_timeout: Duration::from_secs(1),
            result_header_timeout: Duration::from_secs(1),
            queue_capacity: 4,
            result_retention: 4,
            max_image_bytes: 1024 * 1024,
        }
    }

    /// Build a Connection wrapping a MockStream pre-loaded with `output` and accepting
    /// any writes (no input verification — the protocol output is what we care about).
    fn mock_connection(output: Vec<u8>) -> Connection {
        let mut mock = MockStream::default();
        mock.write.set_result(Ok(()));
        mock.read.set_output(output);
        Connection::new(Box::new(mock))
    }

    #[test]
    fn read_line_strips_trailing_cr() {
        let conn = mock_connection(b"PAYLOAD_READY\r\n".to_vec());
        let line = read_line(&conn, Duration::from_secs(1)).unwrap();
        assert_eq!(line, "PAYLOAD_READY");
    }

    #[test]
    fn read_line_handles_bare_lf() {
        let conn = mock_connection(b"IDLE\nREADY\n".to_vec());
        assert_eq!(read_line(&conn, Duration::from_secs(1)).unwrap(), "IDLE");
        assert_eq!(read_line(&conn, Duration::from_secs(1)).unwrap(), "READY");
    }

    #[test]
    fn execute_job_happy_path() {
        let image = b"fake-jpeg-bytes".to_vec();
        let bitmap = vec![0xAA; 32];
        let bitmap_crc = protocol::crc32(&bitmap);

        // Prebake the payload's side of the conversation. This must match exactly the
        // sequence `execute_job` reads.
        let mut wire = Vec::new();
        // After SEND header → READY
        wire.extend_from_slice(b"READY\n");
        // After image bytes → RX_OK <id>
        wire.extend_from_slice(b"RX_OK 42\n");
        // PROCESSING + RESULT_READY notify
        wire.extend_from_slice(b"PROCESSING 42\n");
        wire.extend_from_slice(b"RESULT_READY 42\n");
        // GET_RESULT_INFO → RESULT_INFO line
        wire.extend_from_slice(format!("RESULT_INFO 42 READY {} {:08X}\n", bitmap.len(), bitmap_crc).as_bytes());
        // GET_RESULT → RESULT_READY header line
        wire.extend_from_slice(format!("RESULT_READY 42 {} {:08X}\n", bitmap.len(), bitmap_crc).as_bytes());
        // bitmap blob
        wire.extend_from_slice(&bitmap);

        let conn = mock_connection(wire);
        let handle = DriverHandle::new();
        // Seed a job-status entry so set_job_phase has something to update.
        handle.state.lock().unwrap().jobs.insert(
            42,
            JobStatus {
                image_id: 42,
                phase: JobPhase::SendingImage,
                queue_position: None,
                error: None,
            },
        );

        let job = PendingJob {
            image_id: 42,
            image,
            crc32: 0xDEADBEEF,
        };
        let entry = execute_job(&conn, &test_config(), &job, &handle).expect("happy path");
        assert_eq!(entry.image_id, 42);
        assert_eq!(entry.size, bitmap.len() as u32);
        assert_eq!(entry.crc32, bitmap_crc);
        assert_eq!(entry.bitmap, bitmap);
    }

    #[test]
    fn execute_job_detects_crc_mismatch() {
        let image = b"fake-jpeg-bytes".to_vec();
        let bitmap = vec![0xBB; 16];
        // Lie about the CRC in the header line.
        let bad_crc: u32 = 0x00000000;

        let mut wire = Vec::new();
        wire.extend_from_slice(b"READY\n");
        wire.extend_from_slice(b"RX_OK 7\n");
        wire.extend_from_slice(b"RESULT_READY 7\n");
        wire.extend_from_slice(format!("RESULT_INFO 7 READY {} {:08X}\n", bitmap.len(), bad_crc).as_bytes());
        wire.extend_from_slice(format!("RESULT_READY 7 {} {:08X}\n", bitmap.len(), bad_crc).as_bytes());
        wire.extend_from_slice(&bitmap);

        let conn = mock_connection(wire);
        let handle = DriverHandle::new();
        handle.state.lock().unwrap().jobs.insert(
            7,
            JobStatus {
                image_id: 7,
                phase: JobPhase::SendingImage,
                queue_position: None,
                error: None,
            },
        );

        let job = PendingJob {
            image_id: 7,
            image,
            crc32: 0xDEADBEEF,
        };
        let err = execute_job(&conn, &test_config(), &job, &handle).expect_err("should fail");
        match err {
            SnnError::CrcMismatch { expected, actual } => {
                assert_eq!(expected, bad_crc);
                assert_eq!(actual, protocol::crc32(&bitmap));
            }
            other => panic!("expected CrcMismatch, got {other:?}"),
        }
    }

    #[test]
    fn execute_job_propagates_payload_error() {
        // Payload NAKs the SEND header.
        let wire = b"ERR BAD_SIZE image too big\n".to_vec();
        let conn = mock_connection(wire);
        let handle = DriverHandle::new();
        handle.state.lock().unwrap().jobs.insert(
            1,
            JobStatus {
                image_id: 1,
                phase: JobPhase::SendingImage,
                queue_position: None,
                error: None,
            },
        );

        let job = PendingJob {
            image_id: 1,
            image: vec![0; 8],
            crc32: 0,
        };
        let err = execute_job(&conn, &test_config(), &job, &handle).expect_err("should fail");
        assert!(matches!(err, SnnError::PayloadNak(_)), "got {err:?}");
    }
}
