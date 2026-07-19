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

//! Battery Board Telemetry
//!
//! This module provides the enum, commands and parsers necessary for working
//! with telemetry from the battery board (I2C address 0x2A).
//!
//! The macro `make_telemetry!` is responsible for generating the enum `Type`, the
//! `parse` function and the `command` function.

use crate::telemetry::lib::get_adc_result;
use eps_api::EpsResult;
use rust_i2c::Command;

const TELEM_CMD: u8 = 0x10;

make_telemetry!(
    /// TLM_VBAT - Battery Output Voltage (V)
    BatteryOutputVoltage => {vec![0xE2, 0x80], |d| 0.008_993 * d},
    /// TLM_IBAT - Battery Current Magnitude (mA)
    BatteryCurrentMagnitude => {vec![0xE2, 0x84], |d| 14.662_757 * d},
    /// TLM_IDIRBAT - Battery Current Direction (raw ADC, < 512 = Charging, >= 512 = Discharging)
    BatteryCurrentDirection => {vec![0xE2, 0x8E], |d| d},
    /// TLM_TBRD - Motherboard Temperature (*C)
    MotherboardTemperature => {vec![0xE3, 0x08], |d| (0.372_434 * d) - 273.15},
    /// TLM_IPCM5V - Current Draw of 5V Bus (mA)
    CurrentDraw5V => {vec![0xE2, 0x14], |d| 1.327_547 * d},
    /// TLM_VPCM5V - Output Voltage of 5V Bus (V)
    OutputVoltage5V => {vec![0xE2, 0x10], |d| 0.005_865 * d},
    /// TLM_IPCM3V3 - Current Draw of 3.3V Bus (mA)
    CurrentDraw3V3 => {vec![0xE2, 0x04], |d| 1.327_547 * d},
    /// TLM_VPCM3V3 - Output Voltage of 3.3V Bus (V)
    OutputVoltage3V3 => {vec![0xE2, 0x00], |d| 0.004_311 * d},
    /// TLM_TBAT1 - Daughterboard 1 Temperature (*C)
    Daughterboard1Temp => {vec![0xE3, 0x98], |d| (0.397_600 * d) - 238.57},
    /// TLM_HBAT1 - Daughterboard 1 Heater Status (raw ADC, < 512 = Off, >= 512 = On)
    Daughterboard1Heater => {vec![0xE3, 0x9F], |d| d},
    /// TLM_TBAT2 - Daughterboard 2 Temperature (*C)
    Daughterboard2Temp => {vec![0xE3, 0xA8], |d| (0.397_600 * d) - 238.57},
    /// TLM_HBAT2 - Daughterboard 2 Heater Status (raw ADC, < 512 = Off, >= 512 = On)
    Daughterboard2Heater => {vec![0xE3, 0xAF], |d| d},
    /// TLM_TBAT3 - Daughterboard 3 Temperature (*C)
    Daughterboard3Temp => {vec![0xE3, 0xB8], |d| (0.397_600 * d) - 238.57},
    /// TLM_HBAT3 - Daughterboard 3 Heater Status (raw ADC, < 512 = Off, >= 512 = On)
    Daughterboard3Heater => {vec![0xE3, 0xBF], |d| d},
    /// TLM_TBAT4 - Daughterboard 4 Temperature (*C)
    Daughterboard4Temp => {vec![0xE3, 0xC8], |d| (0.397_600 * d) - 238.57},
    /// TLM_HBAT4 - Daughterboard 4 Heater Status (raw ADC, < 512 = Off, >= 512 = On)
    Daughterboard4Heater => {vec![0xE3, 0xCF], |d| d},
);
