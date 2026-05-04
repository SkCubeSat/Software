//
// Copyright (C) 2025 USST CUBICS
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

//! Main module for interacting with the underlying Analog IC API

use crate::models::*;
use analog_ic_api::{AnalogIc, AnalogIcPayload, AnalogIcResult};
use async_graphql::Enum;
use kubos_service::{run, process_errors, push_err};
use rust_i2c::*;
use std::sync::{Arc, Mutex, RwLock};

/// I2C slave address for the Analog IC board
const ANALOG_IC_I2C_ADDR: u16 = 0x13;

/// Enum for tracking the last mutation executed
#[derive(Copy, Clone, Debug, Eq, Hash, Enum, PartialEq)]
pub enum Mutations {
    /// No mutation has been run since the service was started
    None,
    /// No-op
    Noop,
    /// Board reset
    Reset,
    /// Start testing
    Start,
    /// Power saving mode
    PowerSavingMode,
    /// Normal power mode
    NormalPowerMode,
    /// Set RTC time
    SetRtcTime,
    /// Raw passthrough command
    RawCommand,
}

/// Main structure for controlling and accessing the Analog IC payload
#[derive(Clone)]
pub struct Subsystem {
    /// Underlying Analog IC API object
    pub payload: Arc<Mutex<Box<dyn AnalogIc + Send>>>,
    /// Last mutation executed
    pub last_mutation: Arc<RwLock<Mutations>>,
    /// Errors accumulated over all queries and mutations
    pub errors: Arc<RwLock<Vec<String>>>,
}

impl Subsystem {
    /// Create a new subsystem instance for the service to use
    pub fn new(payload: Box<dyn AnalogIc + Send>) -> AnalogIcResult<Self> {
        let payload = Arc::new(Mutex::new(payload));

        Ok(Self {
            payload,
            last_mutation: Arc::new(RwLock::new(Mutations::None)),
            errors: Arc::new(RwLock::new(vec![])),
        })
    }

    /// Create the underlying Analog IC object from an I2C bus path
    /// and then create a new subsystem which will use it
    pub fn from_path(bus: &str) -> AnalogIcResult<Self> {
        let analog_ic: Box<dyn AnalogIc + Send> = Box::new(AnalogIcPayload::new(
            Connection::from_path(bus, ANALOG_IC_I2C_ADDR),
        ));
        Subsystem::new(analog_ic)
    }

    /// Reset the board
    pub fn reset(&self) -> Result<MutationResponse, String> {
        let payload = self.payload.lock().unwrap();
        match run!(payload.reset(), self.errors) {
            Ok(_) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    /// Start the IC testing loop
    pub fn start_tests(&self) -> Result<MutationResponse, String> {
        let payload = self.payload.lock().unwrap();
        match run!(payload.start(), self.errors) {
            Ok(_) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    /// Enter power saving mode
    pub fn power_saving_mode(&self) -> Result<MutationResponse, String> {
        let payload = self.payload.lock().unwrap();
        match run!(payload.power_saving_mode(), self.errors) {
            Ok(_) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    /// Enter normal power mode
    pub fn normal_power_mode(&self) -> Result<MutationResponse, String> {
        let payload = self.payload.lock().unwrap();
        match run!(payload.normal_power_mode(), self.errors) {
            Ok(_) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    /// Set the board's RTC using OBC system time
    pub fn set_rtc_time(&self) -> Result<MutationResponse, String> {
        let payload = self.payload.lock().unwrap();
        match run!(payload.set_rtc_time(), self.errors) {
            Ok(_) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    /// Get payload telemetry data (IC readings + timestamp)
    pub fn get_payload_data(&self) -> Result<PayloadDataResponse, String> {
        let payload = self.payload.lock().unwrap();
        let data = run!(payload.get_payload_data(), self.errors)?;

        Ok(PayloadDataResponse {
            ic_readings: data.ic_readings.iter().map(|&v| v as i32).collect(),
            timestamp_bytes: data.timestamp_bytes.iter().map(|&v| v as i32).collect(),
            raw_data: data.raw_data.iter().map(|&v| v as i32).collect(),
        })
    }

    /// Pass raw command values through to the board
    pub fn raw_command(&self, command: u8, data: Vec<u8>) -> Result<MutationResponse, String> {
        let payload = self.payload.lock().unwrap();
        match run!(payload.raw_command(command, data), self.errors) {
            Ok(_) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    /// No-op: simply verifies connectivity by issuing a harmless command.
    /// Uses reset_comms equivalent — for this board, we just return success
    /// since there is no dedicated watchdog reset command.
    pub fn noop(&self) -> Result<MutationResponse, String> {
        Ok(MutationResponse {
            success: true,
            errors: "".to_string(),
        })
    }

    /// Record the last mutation executed by the service
    pub fn set_last_mutation(&self, mutation: Mutations) {
        if let Ok(mut last_cmd) = self.last_mutation.write() {
            *last_cmd = mutation;
        }
    }

    /// Fetch all errors since the last time this function was called,
    /// then clear the errors storage
    pub fn get_errors(&self) -> AnalogIcResult<Vec<String>> {
        match self.errors.write() {
            Ok(mut master_vec) => {
                let current = master_vec.clone();
                master_vec.clear();
                Ok(current)
            }
            _ => Ok(vec![
                "Error: Failed to borrow master errors vector".to_string(),
            ]),
        }
    }
}
