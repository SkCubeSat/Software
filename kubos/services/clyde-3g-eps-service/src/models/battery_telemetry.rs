//
// Copyright (C) 2019 Kubos Corporation
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

//! Data returned by `batteryTelemetry` query
//!
//! Returns all battery board telemetry in a single query.

use async_graphql::{Object, Result as FieldResult, SimpleObject};

/// Battery telemetry data - all values from the battery board returned at once
#[derive(Clone, Debug, SimpleObject)]
pub struct BatteryTelemetryData {
    /// Battery output voltage (V)
    pub battery_output_voltage: f64,
    /// Battery current magnitude (mA)
    pub battery_current_magnitude: f64,
    /// Battery current direction (raw ADC; < 512 = Charging, >= 512 = Discharging)
    pub battery_current_direction: f64,
    /// Motherboard temperature (°C)
    pub motherboard_temperature: f64,
    /// 5V bus current draw (mA)
    pub current_draw_5v: f64,
    /// 5V bus output voltage (V)
    pub output_voltage_5v: f64,
    /// 3.3V bus current draw (mA)
    pub current_draw_3v3: f64,
    /// 3.3V bus output voltage (V)
    pub output_voltage_3v3: f64,
    /// Daughterboard 1 temperature (°C)
    pub daughterboard_1_temp: f64,
    /// Daughterboard 1 heater status (raw ADC; < 512 = Off, >= 512 = On)
    pub daughterboard_1_heater: f64,
    /// Daughterboard 2 temperature (°C)
    pub daughterboard_2_temp: f64,
    /// Daughterboard 2 heater status (raw ADC; < 512 = Off, >= 512 = On)
    pub daughterboard_2_heater: f64,
    /// Daughterboard 3 temperature (°C)
    pub daughterboard_3_temp: f64,
    /// Daughterboard 3 heater status (raw ADC; < 512 = Off, >= 512 = On)
    pub daughterboard_3_heater: f64,
    /// Daughterboard 4 temperature (°C)
    pub daughterboard_4_temp: f64,
    /// Daughterboard 4 heater status (raw ADC; < 512 = Off, >= 512 = On)
    pub daughterboard_4_heater: f64,
}
