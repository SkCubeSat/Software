/*
 * Copyright (C) 2018 Kubos Corporation
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

//! Low level interface for interacting with a ClydeSpace 3G EPS

mod commands;
mod eps;
mod telemetry;

pub use crate::commands::board_status::{BoardStatus, StatusCode};
pub use crate::commands::checksum::Checksum;
pub use crate::commands::last_error::{ErrorCode, LastError};
pub use crate::commands::version::{Version, VersionInfo};
pub use crate::eps::{Clyde3gEps, Eps};
pub use crate::telemetry::daughterboard as DaughterboardTelemetry;
pub use crate::telemetry::motherboard as MotherboardTelemetry;
pub use crate::telemetry::reset as ResetTelemetry;
