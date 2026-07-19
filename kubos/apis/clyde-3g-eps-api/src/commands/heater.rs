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

use eps_api::{EpsError, EpsResult};
use rust_i2c::Command;

/// Get Heater Controller Status (0x90)
///
/// Return the current status of the battery heater controller.
/// Returns 0x00 if disabled or 0x01 if enabled.
pub mod get_heater_controller_status {
    use super::*;

    /// Parse heater controller status response
    pub fn parse(data: &[u8]) -> EpsResult<u8> {
        if data.len() == 2 {
            Ok(data[1])
        } else {
            Err(EpsError::parsing_failure("Heater Controller Status"))
        }
    }

    /// Build the get heater controller status command
    pub fn command() -> (Command, usize) {
        (
            Command {
                cmd: 0x90,
                data: vec![0x00],
            },
            2,
        )
    }
}

/// Set Heater Controller Status (0x91)
///
/// Control the operation of the battery heater circuitry.
/// - 0x00: Thermostat control circuitry disabled. Heater will remain off.
/// - 0x01: Thermostat control circuitry enabled. Heater will switch on when appropriate.
pub mod set_heater_controller_status {
    use super::*;

    /// Build the set heater controller status command
    ///
    /// # Arguments
    /// `mode` - 0x00 to disable, 0x01 to enable
    pub fn command(mode: u8) -> Command {
        Command {
            cmd: 0x91,
            data: vec![mode],
        }
    }
}
