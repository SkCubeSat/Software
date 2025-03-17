use clyde_3g_eps_api::{BoardStatus, ErrorCode, LastError, StatusCode, Version, VersionInfo};
use clyde_3g_eps_api::{Checksum, Clyde3gEps, Eps};
use clyde_3g_eps_api::{DaughterboardTelemetry, MotherboardTelemetry, ResetTelemetry};
use eps_api::{EpsError, EpsResult};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Subsystem {
    pub sim_eps: Arc<Mutex<Box<dyn Clyde3gEps + Send>>>,
}

impl Subsystem {
    pub fn new() -> EpsResult<Subsystem> {
        let sim_eps = Arc::new(Mutex::new(
            Box::new(ClydeSim::new()) as Box<dyn Clyde3gEps + Send>
        ));

        // Clone Arc for the watchdog thread
        let thread_eps = sim_eps.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(60));
            let _ = thread_eps.lock().unwrap().reset_comms_watchdog();
        });

        Ok(Subsystem { sim_eps })
    }
}

/// Clyde EPS Simulator that cycles battery level
struct ClydeSim {
    start_time: Instant,
    reset_counts: ResetCounts,
    watchdog_period: u8,
    last_error: LastError,
}

struct ResetCounts {
    motherboard_brownout: u8,
    motherboard_auto_software: u8,
    motherboard_manual: u8,
    motherboard_watchdog: u8,
    daughterboard_brownout: u8,
    daughterboard_auto_software: u8,
    daughterboard_manual: u8,
    daughterboard_watchdog: u8,
}

impl ClydeSim {
    fn new() -> Self {
        ClydeSim {
            start_time: Instant::now(),
            reset_counts: ResetCounts {
                motherboard_brownout: 0,
                motherboard_auto_software: 0,
                motherboard_manual: 0,
                motherboard_watchdog: 0,
                daughterboard_brownout: 0,
                daughterboard_auto_software: 0,
                daughterboard_manual: 0,
                daughterboard_watchdog: 0,
            },
            watchdog_period: 10, // Default watchdog period (minutes)
            last_error: LastError {
                motherboard: ErrorCode::None,
                daughterboard: Some(ErrorCode::None),
            },
        }
    }

    /// Calculate battery level (0-100) based on elapsed time
    /// Cycle from 0 to 100 and back in 60 seconds
    fn get_battery_level(&self) -> f64 {
        let elapsed_seconds = self.start_time.elapsed().as_secs() % 60;

        // First 30 seconds: 0 -> 100
        // Next 30 seconds: 100 -> 0
        if elapsed_seconds < 30 {
            (elapsed_seconds as f64 / 30.0) * 100.0
        } else {
            100.0 - ((elapsed_seconds - 30) as f64 / 30.0) * 100.0
        }
    }
}

impl Clyde3gEps for ClydeSim {
    fn get_board_status(&self) -> EpsResult<BoardStatus> {
        Ok(BoardStatus {
            motherboard: StatusCode::POWER_ON_RESET,
            daughterboard: Some(StatusCode::POWER_ON_RESET),
        })
    }

    fn get_checksum(&self) -> EpsResult<Checksum> {
        Ok(Checksum {
            motherboard: 123,
            daughterboard: Some(456),
        })
    }

    fn get_version_info(&self) -> EpsResult<VersionInfo> {
        Ok(VersionInfo {
            motherboard: Version {
                revision: 1,
                firmware_number: 100,
            },
            daughterboard: Some(Version {
                revision: 1,
                firmware_number: 101,
            }),
        })
    }

    fn get_last_error(&self) -> EpsResult<LastError> {
        Ok(self.last_error.clone())
    }

    fn manual_reset(&self) -> EpsResult<()> {
        // In a real implementation, we might want to update reset counts here
        Ok(())
    }

    fn reset_comms_watchdog(&self) -> EpsResult<()> {
        // Just acknowledge the command
        Ok(())
    }

    fn get_motherboard_telemetry(&self, telem_type: MotherboardTelemetry::Type) -> EpsResult<f64> {
        // Return battery level for OutputVoltageBattery, otherwise return a default value
        match telem_type {
            MotherboardTelemetry::Type::OutputVoltageBattery => Ok(self.get_battery_level()),
            MotherboardTelemetry::Type::BoardTemperature => Ok(25.0), // 25°C
            MotherboardTelemetry::Type::BcrOutputVoltage => Ok(5.0),  // 5V
            MotherboardTelemetry::Type::OutputVoltage5V => Ok(5.0),   // 5V
            MotherboardTelemetry::Type::OutputVoltage33V => Ok(3.3),  // 3.3V
            MotherboardTelemetry::Type::OutputVoltage12V => Ok(12.0), // 12V
            // Return a default value for other telemetry types
            _ => Ok(10.0),
        }
    }

    fn get_daughterboard_telemetry(
        &self,
        _telem_type: DaughterboardTelemetry::Type,
    ) -> EpsResult<f64> {
        // Return a default value for all daughterboard telemetry
        Ok(5.0)
    }

    fn get_reset_telemetry(
        &self,
        telem_type: ResetTelemetry::Type,
    ) -> EpsResult<ResetTelemetry::Data> {
        match telem_type {
            ResetTelemetry::Type::BrownOut => Ok(ResetTelemetry::Data {
                motherboard: self.reset_counts.motherboard_brownout,
                daughterboard: Some(self.reset_counts.daughterboard_brownout),
            }),
            ResetTelemetry::Type::AutomaticSoftware => Ok(ResetTelemetry::Data {
                motherboard: self.reset_counts.motherboard_auto_software,
                daughterboard: Some(self.reset_counts.daughterboard_auto_software),
            }),
            ResetTelemetry::Type::Manual => Ok(ResetTelemetry::Data {
                motherboard: self.reset_counts.motherboard_manual,
                daughterboard: Some(self.reset_counts.daughterboard_manual),
            }),
            ResetTelemetry::Type::Watchdog => Ok(ResetTelemetry::Data {
                motherboard: self.reset_counts.motherboard_watchdog,
                daughterboard: Some(self.reset_counts.daughterboard_watchdog),
            }),
        }
    }

    fn set_comms_watchdog_period(&self, period: u8) -> EpsResult<()> {
        // In a real implementation we would update the watchdog period
        // For simulation, we'll just acknowledge the command
        Ok(())
    }

    fn get_comms_watchdog_period(&self) -> EpsResult<u8> {
        Ok(self.watchdog_period)
    }

    fn raw_command(&self, _cmd: u8, _data: Vec<u8>) -> EpsResult<()> {
        // For simulation, just acknowledge the command
        Ok(())
    }
}
