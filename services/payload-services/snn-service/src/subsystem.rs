use std::sync::Arc;
use std::time::Duration;

use kubos_service::Config;

use crate::driver::{
    self, DriverConfig, DriverHandle, DriverPhase, JobPhase, JobStatus, PendingJob, ResultEntry,
};
use crate::error::SnnError;
use crate::protocol;

const POLL_INTERVAL: Duration = Duration::from_millis(200);

#[derive(Clone)]
pub struct Subsystem {
    handle: DriverHandle,
    config: Arc<DriverConfig>,
    /// Kept alive so the driver thread is joined on drop (well, we don't strictly join,
    /// but holding the handle in an Arc lets us drop the service cleanly during tests).
    _join: Arc<std::thread::JoinHandle<()>>,
}

#[derive(Clone, Debug)]
pub struct SubmitOutcome {
    pub image_id: u32,
    pub queue_position: usize,
    pub queue_depth: usize,
}

#[derive(Clone, Debug)]
pub struct SnnState {
    pub phase: DriverPhase,
    pub current_image_id: Option<u32>,
    pub queue_depth: usize,
    pub queue_capacity: usize,
    pub queued_image_ids: Vec<u32>,
    pub last_error: Option<String>,
}

#[derive(Clone, Debug)]
pub struct HealthInfo {
    pub uart_bus: String,
    pub uart_baud: u32,
    pub queue_depth: usize,
    pub queue_capacity: usize,
    pub jobs_completed: u64,
    pub jobs_failed: u64,
    pub last_error: Option<String>,
    pub phase: DriverPhase,
}

impl Subsystem {
    pub fn from_config(config: &Config) -> Result<Self, String> {
        let driver_config = load_driver_config(config)?;
        Ok(Self::start(driver_config))
    }

    pub fn start(driver_config: DriverConfig) -> Self {
        let handle = DriverHandle::new();
        let join = driver::spawn(driver_config.clone(), handle.clone());
        Self {
            handle,
            config: Arc::new(driver_config),
            _join: Arc::new(join),
        }
    }

    pub fn submit(&self, image: Vec<u8>) -> Result<SubmitOutcome, SnnError> {
        if image.is_empty() {
            return Err(SnnError::Protocol("image is empty".to_string()));
        }
        if image.len() > self.config.max_image_bytes {
            return Err(SnnError::ImageTooLarge {
                size: image.len(),
                limit: self.config.max_image_bytes,
            });
        }

        let crc = protocol::crc32(&image);

        let mut state = self.handle.state.lock().map_err(|_| {
            SnnError::Internal("subsystem state lock poisoned".to_string())
        })?;

        if matches!(state.phase, DriverPhase::ShuttingDown | DriverPhase::Faulted) {
            return Err(SnnError::Internal(format!(
                "driver in unrecoverable phase: {}",
                state.phase.as_str()
            )));
        }
        if state.pending.len() >= self.config.queue_capacity {
            return Err(SnnError::QueueFull(self.config.queue_capacity));
        }

        let image_id = state.next_image_id;
        state.next_image_id = state.next_image_id.wrapping_add(1).max(1);

        state.pending.push_back(PendingJob {
            image_id,
            image,
            crc32: crc,
        });
        let queue_position = state.pending.len() - 1;
        let queue_depth = state.pending.len();
        state.jobs.insert(
            image_id,
            JobStatus {
                image_id,
                phase: JobPhase::Queued,
                queue_position: Some(queue_position),
                error: None,
            },
        );

        drop(state);
        self.handle.cond.notify_one();

        Ok(SubmitOutcome {
            image_id,
            queue_position,
            queue_depth,
        })
    }

    /// Best-effort cancellation. Only succeeds if the job is still `Queued`.
    pub fn cancel(&self, image_id: u32) -> Result<bool, SnnError> {
        let mut state = self.handle.state.lock().map_err(|_| {
            SnnError::Internal("subsystem state lock poisoned".to_string())
        })?;
        if let Some(pos) = state
            .pending
            .iter()
            .position(|job| job.image_id == image_id)
        {
            state.pending.remove(pos);
            if let Some(status) = state.jobs.get_mut(&image_id) {
                status.phase = JobPhase::Cancelled;
                status.queue_position = None;
            }
            // Recompute positions for remaining pending jobs.
            let positions: Vec<(u32, usize)> = state
                .pending
                .iter()
                .enumerate()
                .map(|(idx, job)| (job.image_id, idx))
                .collect();
            for (id, idx) in positions {
                if let Some(status) = state.jobs.get_mut(&id) {
                    status.queue_position = Some(idx);
                }
            }
            return Ok(true);
        }
        Ok(false)
    }

    pub fn inference_status(&self, image_id: u32) -> Option<JobStatus> {
        self.handle
            .state
            .lock()
            .ok()
            .and_then(|guard| guard.jobs.get(&image_id).cloned())
    }

    pub fn get_result(&self, image_id: u32) -> Result<ResultEntry, SnnError> {
        let mut state = self.handle.state.lock().map_err(|_| {
            SnnError::Internal("subsystem state lock poisoned".to_string())
        })?;

        match state.results.get(&image_id) {
            Some(entry) => {
                let mut entry = entry.clone();
                if matches!(entry.phase, JobPhase::ResultReady) {
                    entry.phase = JobPhase::Delivered;
                }
                if let Some(stored) = state.results.get_mut(&image_id) {
                    stored.phase = JobPhase::Delivered;
                }
                if let Some(status) = state.jobs.get_mut(&image_id)
                    && matches!(status.phase, JobPhase::ResultReady)
                {
                    status.phase = JobPhase::Delivered;
                }
                Ok(entry)
            }
            None => match state.jobs.get(&image_id) {
                Some(status) => Err(SnnError::ResultNotReady(image_id, status.phase.as_str().to_string())),
                None => Err(SnnError::UnknownImageId(image_id)),
            },
        }
    }

    /// Submit + poll-to-completion. The poll happens on the tokio executor so the
    /// driver thread is unaffected.
    pub async fn infer(&self, image: Vec<u8>) -> Result<ResultEntry, SnnError> {
        let outcome = self.submit(image)?;
        let id = outcome.image_id;
        loop {
            let status = self
                .inference_status(id)
                .ok_or(SnnError::UnknownImageId(id))?;
            match status.phase {
                JobPhase::ResultReady | JobPhase::Delivered => {
                    return self.get_result(id);
                }
                JobPhase::Failed => {
                    return Err(SnnError::Internal(
                        status.error.unwrap_or_else(|| "inference failed".to_string()),
                    ));
                }
                JobPhase::Cancelled => {
                    return Err(SnnError::Internal("inference cancelled".to_string()));
                }
                _ => tokio::time::sleep(POLL_INTERVAL).await,
            }
        }
    }

    pub fn state(&self) -> SnnState {
        let guard = match self.handle.state.lock() {
            Ok(g) => g,
            Err(_) => {
                return SnnState {
                    phase: DriverPhase::Faulted,
                    current_image_id: None,
                    queue_depth: 0,
                    queue_capacity: self.config.queue_capacity,
                    queued_image_ids: Vec::new(),
                    last_error: Some("subsystem state lock poisoned".to_string()),
                };
            }
        };
        SnnState {
            phase: guard.phase.clone(),
            current_image_id: guard.current_image_id,
            queue_depth: guard.pending.len(),
            queue_capacity: self.config.queue_capacity,
            queued_image_ids: guard.pending.iter().map(|j| j.image_id).collect(),
            last_error: guard.last_error.clone(),
        }
    }

    pub fn health(&self) -> HealthInfo {
        let guard = match self.handle.state.lock() {
            Ok(g) => g,
            Err(_) => {
                return HealthInfo {
                    uart_bus: self.config.uart_bus.clone(),
                    uart_baud: self.config.uart_baud,
                    queue_depth: 0,
                    queue_capacity: self.config.queue_capacity,
                    jobs_completed: 0,
                    jobs_failed: 0,
                    last_error: Some("subsystem state lock poisoned".to_string()),
                    phase: DriverPhase::Faulted,
                };
            }
        };
        HealthInfo {
            uart_bus: self.config.uart_bus.clone(),
            uart_baud: self.config.uart_baud,
            queue_depth: guard.pending.len(),
            queue_capacity: self.config.queue_capacity,
            jobs_completed: guard.jobs_completed,
            jobs_failed: guard.jobs_failed,
            last_error: guard.last_error.clone(),
            phase: guard.phase.clone(),
        }
    }

    pub fn shutdown(&self) {
        self.handle.signal_shutdown();
    }
}

fn load_driver_config(config: &Config) -> Result<DriverConfig, String> {
    let uart_bus = config
        .get("uart_bus")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "/dev/ttyUL2".to_string());
    let uart_baud = config
        .get("uart_baud")
        .and_then(|v| v.as_integer())
        .map(|v| v as u32)
        .unwrap_or(115200);
    let read_line_timeout = duration_ms(config, "read_line_timeout_ms", 2000);
    let ready_timeout = duration_ms(config, "ready_timeout_ms", 2000);
    let rx_ok_timeout = duration_ms(config, "rx_ok_timeout_ms", 5000);
    let processing_timeout = duration_ms(config, "processing_timeout_ms", 60_000);
    let result_info_timeout = duration_ms(config, "result_info_timeout_ms", 2000);
    let result_header_timeout = duration_ms(config, "result_header_timeout_ms", 2000);
    let queue_capacity = config
        .get("queue_capacity")
        .and_then(|v| v.as_integer())
        .map(|v| v as usize)
        .unwrap_or(4);
    let result_retention = config
        .get("result_retention")
        .and_then(|v| v.as_integer())
        .map(|v| v as usize)
        .unwrap_or(4);
    let max_image_bytes = config
        .get("max_image_bytes")
        .and_then(|v| v.as_integer())
        .map(|v| v as usize)
        .unwrap_or(4 * 1024 * 1024);

    if queue_capacity == 0 {
        return Err("queue_capacity must be >= 1".to_string());
    }
    if result_retention == 0 {
        return Err("result_retention must be >= 1".to_string());
    }

    Ok(DriverConfig {
        uart_bus,
        uart_baud,
        read_line_timeout,
        ready_timeout,
        rx_ok_timeout,
        processing_timeout,
        result_info_timeout,
        result_header_timeout,
        queue_capacity,
        result_retention,
        max_image_bytes,
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
