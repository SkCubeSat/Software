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

//! Analog IC payload system data models

use async_graphql::{SimpleObject, Enum};

/// Generic mutation response struct
#[derive(Clone, Debug, SimpleObject)]
pub struct MutationResponse {
    /// Any errors which occurred during the mutation
    pub errors: String,
    /// Success or fail status of the mutation
    pub success: bool,
}

/// GraphQL response for payload telemetry data
#[derive(Clone, Debug, SimpleObject)]
pub struct PayloadDataResponse {
    /// 50 unsigned 16-bit IC test readings (5×10 matrix, flattened).
    pub ic_readings: Vec<i32>,
    /// ASCII timestamp string from the board
    pub timestamp: String,
    /// The full raw data buffer as received from the board
    pub raw_data: Vec<i32>,
}

/// GraphQL response for RTC time data
#[derive(Clone, Debug, SimpleObject)]
pub struct RtcTimeResponse {
    /// Year
    pub year: i32,
    /// Month (1-12)
    pub month: i32,
    /// Day (1-31)
    pub day: i32,
    /// Weekday (0=Monday .. 6=Sunday)
    pub weekday: i32,
    /// Hour (0-23)
    pub hour: i32,
    /// Minute (0-59)
    pub minute: i32,
    /// Second (0-59)
    pub second: i32,
}

/// GraphQL enum for power mode status
#[derive(Copy, Clone, Debug, Eq, PartialEq, Enum)]
pub enum PowerModeStatus {
    /// Normal operating mode
    Normal,
    /// Power-saving / sleep mode
    PowerSaving,
    /// Unknown mode
    Unknown,
}

/// GraphQL response for power status
#[derive(Clone, Debug, SimpleObject)]
pub struct PowerStatusResponse {
    /// Current power mode
    pub mode: PowerModeStatus,
    /// Raw mode flag value from the board
    pub raw_value: i32,
}

/// GraphQL response for latest timestamp
#[derive(Clone, Debug, SimpleObject)]
pub struct LatestTimestampResponse {
    /// Raw timestamp bytes from the board
    pub bytes: Vec<i32>,
}

pub mod subsystem;
