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

//! Data returned by `motherboardTelemetry` telemetry query

// use crate::schema::Context;
use clyde_3g_eps_api::MotherboardTelemetry::Type as MotherboardTelemetryType;
// use juniper::FieldResult;
use async_graphql::{Enum, Object, Result as FieldResult};

/// Motherboard telemetry values
///
/// See Table 11-7 in the EPS' User Manual for more information
#[derive(Copy, Clone, Debug, Hash, Eq, Enum, PartialEq)]
#[graphql(name = "MotherboardTelemetryType")]
pub enum Type {
     /// Voltage feeding BCR1
    VoltageFeedingBcr1,
    /// Current BCR1 SA1A
    CurrentBcr1Sa1a,
    /// Current BCR1 SA1B
    CurrentBcr1Sa1b,
    /// Array temperature SA1A
    ArrayTempSa1a,
    /// Array temperature SA1B
    ArrayTempSa1b,
    /// Sun detector SA1A
    SunDetectorSa1a,
    /// Sun detector SA1B
    SunDetectorSa1b,
    /// Voltage feeding BCR2
    VoltageFeedingBcr2,
    /// Current BCR2 SA2A
    CurrentBcr2Sa2a,
    /// Current BCR2 SA2B
    CurrentBcr2Sa2b,
    /// Array temperature SA2A
    ArrayTempSa2a,
    /// Array temperature SA2B
    ArrayTempSa2b,
    /// Sun detector SA2A
    SunDetectorSa2a,
    /// Sun detector SA2B
    SunDetectorSa2b,
    /// Voltage feeding BCR3
    VoltageFeedingBcr3,
    /// Current BCR3 SA3A
    CurrentBcr3Sa3a,
    /// Current BCR3 SA3B
    CurrentBcr3Sa3b,
    /// Array temperature SA3A
    ArrayTempSa3a,
    /// Array temperature SA3B
    ArrayTempSa3b,
    /// Sun detector SA3A
    SunDetectorSa3a,
    /// Sun detector SA3B
    SunDetectorSa3b,
    /// BCR output current
    BcrOutputCurrent,
    /// BCR output voltage
    BcrOutputVoltage,
    /// Current draw 3.3V
    CurrentDraw3V3,
    /// Current draw 5V
    CurrentDraw5V,
    /// Output current 12V
    OutputCurrent12V,
    /// Output voltage 12V
    OutputVoltage12V,
    /// Output current battery
    OutputCurrentBattery,
    /// Output voltage battery
    OutputVoltageBattery,
    /// Output current 5V
    OutputCurrent5V,
    /// Output voltage 5V
    OutputVoltage5V,
    /// Output current 3.3V
    OutputCurrent33V,
    /// Output voltage 3.3V
    OutputVoltage33V,
    /// Output voltage switch 1
    OutputVoltageSwitch1,
    /// Output current switch 1
    OutputCurrentSwitch1,
    /// Output voltage switch 2
    OutputVoltageSwitch2,
    /// Output current switch 2
    OutputCurrentSwitch2,
    /// Output voltage switch 3
    OutputVoltageSwitch3,
    /// Output current switch 3
    OutputCurrentSwitch3,
    /// Output voltage switch 4
    OutputVoltageSwitch4,
    /// Output current switch 4
    OutputCurrentSwitch4,
    /// Output voltage switch 5
    OutputVoltageSwitch5,
    /// Output current switch 5
    OutputCurrentSwitch5,
    /// Output voltage switch 6
    OutputVoltageSwitch6,
    /// Output current switch 6
    OutputCurrentSwitch6,
    /// Output voltage switch 7
    OutputVoltageSwitch7,
    /// Output current switch 7
    OutputCurrentSwitch7,
    /// Output voltage switch 8
    OutputVoltageSwitch8,
    /// Output current switch 8
    OutputCurrentSwitch8,
    /// Output voltage switch 9
    OutputVoltageSwitch9,
    /// Output current switch 9
    OutputCurrentSwitch9,
    /// Output voltage switch 10
    OutputVoltageSwitch10,
    /// Output current switch 10
    OutputCurrentSwitch10,
    /// Board temperature
    BoardTemperature,
}

impl From<Type> for MotherboardTelemetryType {
    fn from(t: Type) -> Self {
        match t {
            Type::VoltageFeedingBcr1 => Self::VoltageFeedingBcr1,
            Type::CurrentBcr1Sa1a => Self::CurrentBcr1Sa1a,
            Type::CurrentBcr1Sa1b => Self::CurrentBcr1Sa1b,
            Type::ArrayTempSa1a => Self::ArrayTempSa1a,
            Type::ArrayTempSa1b => Self::ArrayTempSa1b,
            Type::SunDetectorSa1a => Self::SunDetectorSa1a,
            Type::SunDetectorSa1b => Self::SunDetectorSa1b,
            Type::VoltageFeedingBcr2 => Self::VoltageFeedingBcr2,
            Type::CurrentBcr2Sa2a => Self::CurrentBcr2Sa2a,
            Type::CurrentBcr2Sa2b => Self::CurrentBcr2Sa2b,
            Type::ArrayTempSa2a => Self::ArrayTempSa2a,
            Type::ArrayTempSa2b => Self::ArrayTempSa2b,
            Type::SunDetectorSa2a => Self::SunDetectorSa2a,
            Type::SunDetectorSa2b => Self::SunDetectorSa2b,
            Type::VoltageFeedingBcr3 => Self::VoltageFeedingBcr3,
            Type::CurrentBcr3Sa3a => Self::CurrentBcr3Sa3a,
            Type::CurrentBcr3Sa3b => Self::CurrentBcr3Sa3b,
            Type::ArrayTempSa3a => Self::ArrayTempSa3a,
            Type::ArrayTempSa3b => Self::ArrayTempSa3b,
            Type::SunDetectorSa3a => Self::SunDetectorSa3a,
            Type::SunDetectorSa3b => Self::SunDetectorSa3b,
            Type::BcrOutputCurrent => Self::BcrOutputCurrent,
            Type::BcrOutputVoltage => Self::BcrOutputVoltage,
            Type::CurrentDraw3V3 => Self::CurrentDraw3V3,
            Type::CurrentDraw5V => Self::CurrentDraw5V,
            Type::OutputCurrent12V => Self::OutputCurrent12V,
            Type::OutputVoltage12V => Self::OutputVoltage12V,
            Type::OutputCurrentBattery => Self::OutputCurrentBattery,
            Type::OutputVoltageBattery => Self::OutputVoltageBattery,
            Type::OutputCurrent5V => Self::OutputCurrent5V,
            Type::OutputVoltage5V => Self::OutputVoltage5V,
            Type::OutputCurrent33V => Self::OutputCurrent33V,
            Type::OutputVoltage33V => Self::OutputVoltage33V,
            Type::OutputVoltageSwitch1 => Self::OutputVoltageSwitch1,
            Type::OutputCurrentSwitch1 => Self::OutputCurrentSwitch1,
            Type::OutputVoltageSwitch2 => Self::OutputVoltageSwitch2,
            Type::OutputCurrentSwitch2 => Self::OutputCurrentSwitch2,
            Type::OutputVoltageSwitch3 => Self::OutputVoltageSwitch3,
            Type::OutputCurrentSwitch3 => Self::OutputCurrentSwitch3,
            Type::OutputVoltageSwitch4 => Self::OutputVoltageSwitch4,
            Type::OutputCurrentSwitch4 => Self::OutputCurrentSwitch4,
            Type::OutputVoltageSwitch5 => Self::OutputVoltageSwitch5,
            Type::OutputCurrentSwitch5 => Self::OutputCurrentSwitch5,
            Type::OutputVoltageSwitch6 => Self::OutputVoltageSwitch6,
            Type::OutputCurrentSwitch6 => Self::OutputCurrentSwitch6,
            Type::OutputVoltageSwitch7 => Self::OutputVoltageSwitch7,
            Type::OutputCurrentSwitch7 => Self::OutputCurrentSwitch7,
            Type::OutputVoltageSwitch8 => Self::OutputVoltageSwitch8,
            Type::OutputCurrentSwitch8 => Self::OutputCurrentSwitch8,
            Type::OutputVoltageSwitch9 => Self::OutputVoltageSwitch9,
            Type::OutputCurrentSwitch9 => Self::OutputCurrentSwitch9,
            Type::OutputVoltageSwitch10 => Self::OutputVoltageSwitch10,
            Type::OutputCurrentSwitch10 => Self::OutputCurrentSwitch10,
            Type::BoardTemperature => Self::BoardTemperature,
        }
    }
}

/// Motherboard telemetry structure
pub struct MotherboardTelemetry;

#[Object]
impl MotherboardTelemetry {
    async fn value(
        &self,
        ctx: &async_graphql::Context<'_>,
        telemetry_type: Type,
    ) -> FieldResult<f64> {
        let context = ctx.data::<crate::schema::Context>()?;
        Ok(context.subsystem().get_motherboard_telemetry(telemetry_type)? as f64)
    }
}


// macro_rules! make_telemetry {
//     (
//         $($type: ident,)+
//     ) => {
//         /// Motherboard telemetry values
//         ///
//         /// See Table 11-7 in the EPS' User Manual for more information
//         #[derive(Clone, Debug, Hash, Eq, GraphQLEnum, PartialEq)]
//         pub enum Type {
//             $(
//                 /// $type
//                 $type,
//             )+
//         }

//         impl From<Type> for MotherboardTelemetryType {
//             fn from(t: Type) -> Self {
//                 match t {
//                     $(Type::$type => Self::$type,)+
//                 }
//             }
//         }

//         graphql_object!(Telemetry: Context as "MotherboardTelemetry" |&self| {
//             $(
//                 field $type(&executor) -> FieldResult<f64>
//                 {
//                     Ok(f64::from(executor.context().subsystem().get_motherboard_telemetry(Type::$type)?))
//                 }
//             )+
//         });
//     }
// }

// make_telemetry!(
//     VoltageFeedingBcr1,
//     CurrentBcr1Sa1a,
//     CurrentBcr1Sa1b,
//     ArrayTempSa1a,
//     ArrayTempSa1b,
//     SunDetectorSa1a,
//     SunDetectorSa1b,
//     VoltageFeedingBcr2,
//     CurrentBcr2Sa2a,
//     CurrentBcr2Sa2b,
//     ArrayTempSa2a,
//     ArrayTempSa2b,
//     SunDetectorSa2a,
//     SunDetectorSa2b,
//     VoltageFeedingBcr3,
//     CurrentBcr3Sa3a,
//     CurrentBcr3Sa3b,
//     ArrayTempSa3a,
//     ArrayTempSa3b,
//     SunDetectorSa3a,
//     SunDetectorSa3b,
//     BcrOutputCurrent,
//     BcrOutputVoltage,
//     CurrentDraw3V3,
//     CurrentDraw5V,
//     OutputCurrent12V,
//     OutputVoltage12V,
//     OutputCurrentBattery,
//     OutputVoltageBattery,
//     OutputCurrent5V,
//     OutputVoltage5V,
//     OutputCurrent33V,
//     OutputVoltage33V,
//     OutputVoltageSwitch1,
//     OutputCurrentSwitch1,
//     OutputVoltageSwitch2,
//     OutputCurrentSwitch2,
//     OutputVoltageSwitch3,
//     OutputCurrentSwitch3,
//     OutputVoltageSwitch4,
//     OutputCurrentSwitch4,
//     OutputVoltageSwitch5,
//     OutputCurrentSwitch5,
//     OutputVoltageSwitch6,
//     OutputCurrentSwitch6,
//     OutputVoltageSwitch7,
//     OutputCurrentSwitch7,
//     OutputVoltageSwitch8,
//     OutputCurrentSwitch8,
//     OutputVoltageSwitch9,
//     OutputCurrentSwitch9,
//     OutputVoltageSwitch10,
//     OutputCurrentSwitch10,
//     BoardTemperature,
// );
