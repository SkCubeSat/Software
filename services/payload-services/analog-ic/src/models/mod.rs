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

use async_graphql::SimpleObject;

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
    /// 27 unsigned 16-bit IC test readings.
    /// Organized as 9 ICs × 3 readings each.
    pub ic_readings: Vec<i32>,
    /// Raw timestamp bytes (6 bytes) from the board's RTC
    pub timestamp_bytes: Vec<i32>,
    /// The full raw data buffer as received from the board
    pub raw_data: Vec<i32>,
}

pub mod subsystem;
