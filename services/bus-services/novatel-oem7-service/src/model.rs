//
// Copyright (C) 2025 SkCubeSat
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::sync::{Arc, Mutex, RwLock};

use anyhow::anyhow;
use log::{error, info};
use novatel_oem7_api::{OEM7, ReceiverStatusFlags};

use crate::objects::*;

/// High-level subsystem wrapping the OEM7 API for use by the GraphQL schema.
#[derive(Clone)]
pub struct Subsystem {
    /// Serial connection to the OEM7 receiver (mutex for thread safety)
    pub oem: Arc<Mutex<OEM7>>,
    /// Last mutation command executed
    pub last_cmd: Arc<RwLock<AckCommand>>,
    /// Accumulated error messages
    pub errors: Arc<RwLock<Vec<String>>>,
}

impl Subsystem {
    /// Create a new Subsystem connected to an OEM7 receiver.
    pub fn new(bus: &str, baud: u32) -> Result<Self, anyhow::Error> {
        let oem = OEM7::new(bus, baud)
            .map_err(|e| anyhow!("Failed to open OEM7 on {}: {}", bus, e))?;

        info!("OEM7 subsystem initialized on {} at {} baud", bus, baud);

        Ok(Subsystem {
            oem: Arc::new(Mutex::new(oem)),
            last_cmd: Arc::new(RwLock::new(AckCommand::None)),
            errors: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Push an error message onto the errors list.
    fn push_err(&self, msg: String) {
        if let Ok(mut errs) = self.errors.write() {
            errs.push(msg);
        }
    }

    /// Drain errors from the OEM7 API into our error list.
    pub fn get_errors(&self) {
        // Currently a no-op since the API reports errors inline.
        // Preserved for API parity with OEM6 service.
    }

    /// Execute a trivial command (VERSION request) as a connectivity test.
    pub fn noop(&self) -> Result<GenericResponse, anyhow::Error> {
        let mut oem = self.oem.lock().map_err(|e| anyhow!("{}", e))?;
        match oem.request_version() {
            Ok(_) => Ok(GenericResponse {
                success: true,
                errors: String::new(),
                response: String::new(),
            }),
            Err(e) => {
                let msg = format!("Noop: {}", e);
                self.push_err(msg.clone());
                Ok(GenericResponse {
                    success: false,
                    errors: msg,
                    response: String::new(),
                })
            }
        }
    }

    /// Get the current power state by attempting to communicate with the device.
    pub fn get_power(&self) -> Result<GetPowerResponse, anyhow::Error> {
        let mut oem = self.oem.lock().map_err(|e| anyhow!("{}", e))?;
        match oem.send_command("") {
            Ok(_) | Err(novatel_oem7_api::OEM7Error::CommandRejected(_)) => {
                // Device responded (even with error) — it's ON
                Ok(GetPowerResponse {
                    state: PowerState::On,
                    uptime: 1,
                })
            }
            Err(_) => Ok(GetPowerResponse {
                state: PowerState::Off,
                uptime: 0,
            }),
        }
    }

    /// Get the current system status by requesting the RXSTATUS log.
    pub fn get_system_status(&self) -> Result<SystemStatus, anyhow::Error> {
        let mut oem = self.oem.lock().map_err(|e| anyhow!("{}", e))?;
        match oem.request_rxstatus() {
            Ok((_header, rxstatus)) => {
                let status_flags =
                    ReceiverStatusFlags::from_bits_truncate(rxstatus.rx_stat);
                let error_flags = rxstatus.error;

                let mut error_msgs = Vec::new();
                if error_flags != 0 {
                    error_msgs.push(format!("Receiver error word: {:#010X}", error_flags));
                }

                // Also include any accumulated service errors
                if let Ok(errs) = self.errors.read() {
                    for e in errs.iter() {
                        error_msgs.push(e.clone());
                    }
                }

                Ok(SystemStatus {
                    status: ReceiverStatus(status_flags),
                    errors: error_msgs,
                })
            }
            Err(e) => {
                let msg = format!("Failed to get RXSTATUS: {}", e);
                self.push_err(msg.clone());

                // Return status from the errors we've accumulated
                let mut error_msgs = vec![msg];
                if let Ok(errs) = self.errors.read() {
                    for e in errs.iter() {
                        error_msgs.push(e.clone());
                    }
                }

                Ok(SystemStatus {
                    status: ReceiverStatus(ReceiverStatusFlags::empty()),
                    errors: error_msgs,
                })
            }
        }
    }

    /// Get the current lock status by requesting BESTXYZ.
    pub fn get_lock_status(&self) -> Result<LockStatus, anyhow::Error> {
        let mut oem = self.oem.lock().map_err(|e| anyhow!("{}", e))?;
        match oem.request_bestxyz() {
            Ok((header, bestxyz)) => Ok(LockStatus {
                time_status: header.time_status,
                time: OEMTime {
                    week: header.week as i32,
                    ms: header.ms,
                },
                position_status: bestxyz.pos_sol_status,
                position_type: bestxyz.pos_type,
                velocity_status: bestxyz.vel_sol_status,
                velocity_type: bestxyz.vel_type,
            }),
            Err(e) => {
                let msg = format!("Failed to get BESTXYZ: {}", e);
                self.push_err(msg);
                Ok(LockStatus::default())
            }
        }
    }

    /// Get the last known good position and velocity information.
    pub fn get_lock_info(&self) -> Result<LockInfo, anyhow::Error> {
        let mut oem = self.oem.lock().map_err(|e| anyhow!("{}", e))?;
        match oem.request_bestxyz() {
            Ok((header, bestxyz)) => Ok(LockInfo {
                time: OEMTime {
                    week: header.week as i32,
                    ms: header.ms,
                },
                position: [bestxyz.pos_x, bestxyz.pos_y, bestxyz.pos_z],
                velocity: [bestxyz.vel_x, bestxyz.vel_y, bestxyz.vel_z],
            }),
            Err(e) => {
                let msg = format!("Failed to get BESTXYZ: {}", e);
                self.push_err(msg);
                Ok(LockInfo::default())
            }
        }
    }

    /// Get current telemetry (nominal + debug).
    pub fn get_telemetry(&self) -> Result<Telemetry, anyhow::Error> {
        let system_status = self.get_system_status()?;

        // Get lock data (may fail silently)
        let lock_status = self.get_lock_status().ok();
        let lock_info = self.get_lock_info().ok();

        // Get version for debug telemetry
        let debug = {
            let mut oem = self.oem.lock().map_err(|e| anyhow!("{}", e))?;
            match oem.request_version() {
                Ok((_header, version)) => Some(VersionInfo {
                    num_components: version.num_components as i32,
                    components: version
                        .components
                        .iter()
                        .map(|c| VersionComponent {
                            comp_type: c.comp_type as i32,
                            model: c.model.clone(),
                            serial_num: c.serial_num.clone(),
                            hw_version: c.hw_version.clone(),
                            sw_version: c.sw_version.clone(),
                            boot_version: c.boot_version.clone(),
                            compile_date: c.compile_date.clone(),
                            compile_time: c.compile_time.clone(),
                        })
                        .collect(),
                }),
                Err(e) => {
                    self.push_err(format!("Failed to get VERSION: {}", e));
                    None
                }
            }
        };

        Ok(Telemetry {
            nominal: TelemetryNominal {
                system_status,
                lock_status,
                lock_info,
            },
            debug,
        })
    }

    /// Get integration test results (noop + telemetry).
    pub fn get_test_results(&self) -> Result<IntegrationTestResults, anyhow::Error> {
        let noop_result = self.noop()?;
        let telemetry = self.get_telemetry()?;

        Ok(IntegrationTestResults {
            success: noop_result.success,
            errors: noop_result.errors,
            telemetry_nominal: telemetry.nominal,
            telemetry_debug: telemetry.debug,
        })
    }

    /// Configure hardware by sending LOG/UNLOG commands.
    pub fn configure_hardware(
        &self,
        config: Vec<ConfigStruct>,
    ) -> Result<ConfigureHardwareResponse, anyhow::Error> {
        let mut oem = self.oem.lock().map_err(|e| anyhow!("{}", e))?;
        let mut errors = Vec::new();
        let mut config_msgs = Vec::new();

        for item in &config {
            let hold_str = if item.hold { " HOLD" } else { "" };
            let result = match item.option {
                ConfigOption::LogErrorData => {
                    let cmd = format!("LOG RXSTATUSEVENTB ONCHANGED{}", hold_str);
                    config_msgs.push(cmd.clone());
                    oem.send_command(&cmd)
                }
                ConfigOption::LogPositionData => {
                    let cmd = format!(
                        "LOG BESTXYZB ONTIME {} {}{}",
                        item.interval, item.offset, hold_str
                    );
                    config_msgs.push(cmd.clone());
                    oem.send_command(&cmd)
                }
                ConfigOption::UnlogAll => {
                    let cmd = if item.hold {
                        "UNLOGALL THISPORT TRUE".to_string()
                    } else {
                        "UNLOGALL THISPORT".to_string()
                    };
                    config_msgs.push(cmd.clone());
                    oem.send_command(&cmd)
                }
                ConfigOption::UnlogErrorData => {
                    let cmd = "UNLOG THISPORT RXSTATUSEVENTB".to_string();
                    config_msgs.push(cmd.clone());
                    oem.send_command(&cmd)
                }
                ConfigOption::UnlogPositionData => {
                    let cmd = "UNLOG THISPORT BESTXYZB".to_string();
                    config_msgs.push(cmd.clone());
                    oem.send_command(&cmd)
                }
            };

            if let Err(e) = result {
                let msg = format!("{:?}: {}", item.option, e);
                error!("configureHardware error: {}", msg);
                errors.push(msg);
            }
        }

        let success = errors.is_empty();
        let errors_str = errors.join("; ");
        if !success {
            self.push_err(errors_str.clone());
        }

        Ok(ConfigureHardwareResponse {
            config: config_msgs.join(", "),
            errors: errors_str,
            success,
        })
    }

    /// Pass a raw ASCII command through to the receiver.
    pub fn passthrough(&self, command: String) -> Result<GenericResponse, anyhow::Error> {
        let mut oem = self.oem.lock().map_err(|e| anyhow!("{}", e))?;
        match oem.passthrough(&command) {
            Ok(response) => Ok(GenericResponse {
                success: true,
                errors: String::new(),
                response,
            }),
            Err(e) => {
                let msg = format!("{}", e);
                self.push_err(msg.clone());
                Ok(GenericResponse {
                    success: false,
                    errors: msg,
                    response: String::new(),
                })
            }
        }
    }
}
