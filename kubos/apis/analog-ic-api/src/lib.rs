/*
 * Copyright (C) 2025 USST CUBICS
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![deny(missing_docs)]
#![deny(warnings)]

//! Low-level interface for interacting with the Analog IC payload board.
//!
//! The Analog IC board is a payload on the RADSAT-SK2 satellite that tests
//! radiation effects on integrated circuits. It communicates with the OBC
//! via I2C at address 0x13 and includes functionality for:
//!
//! - Controlling board power modes (power saving / normal)
//! - Starting IC test runs
//! - Setting the RTC clock for timestamping test results
//! - Retrieving collected IC test data (27 readings + timestamp)
//!
//! # Usage
//!
//! ```rust,ignore
//! use analog_ic_api::{AnalogIc, AnalogIcPayload};
//! use rust_i2c::Connection;
//!
//! let connection = Connection::from_path("/dev/i2c-1", 0x13);
//! let payload = AnalogIcPayload::new(connection);
//!
//! // Set the RTC time on startup
//! payload.set_rtc_time().unwrap();
//!
//! // Retrieve test data
//! let data = payload.get_payload_data().unwrap();
//! println!("IC readings: {:?}", data.ic_readings);
//! ```

pub mod analog_ic;
pub mod commands;
pub mod error;
pub mod rtc;
pub mod telemetry;

pub use crate::analog_ic::{AnalogIc, AnalogIcPayload};
pub use crate::error::{AnalogIcError, AnalogIcResult};
pub use crate::telemetry::PayloadData;
