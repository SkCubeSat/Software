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

//! Data returned by `daughterboardTelemetry` telemetry query

// use crate::schema::Context;
use clyde_3g_eps_api::DaughterboardTelemetry::Type as DaughterboardTelemetryType;
// use juniper::FieldResult;
use async_graphql::{Enum, Object, Result as FieldResult};


/// Daughterboard telemetry values
///
/// See Table 11-8 in the EPS' User Manual for more information
#[derive(Copy, Clone, Hash, Debug, Eq, Enum, PartialEq)]
#[graphql(name = "DaughterboardTelemetryType")]
pub enum Type {
   /// Voltage feeding BCR4
    VoltageFeedingBcr4,
    /// Current BCR4 SA4A
    CurrentBcr4Sa4a,
    /// Current BCR4 SA4B
    CurrentBcr4Sa4b,
    /// Array temperature SA4A
    ArrayTempSa4a,
    /// Array temperature SA4B
    ArrayTempSa4b,
    /// Sun detector SA4A
    SunDetectorSa4a,
    /// Sun detector SA4B
    SunDetectorSa4b,
    /// Voltage feeding BCR5
    VoltageFeedingBcr5,
    /// Current BCR5 SA5A
    CurrentBcr5Sa5a,
    /// Current BCR5 SA5B
    CurrentBcr5Sa5b,
    /// Array temperature SA5A
    ArrayTempSa5a,
    /// Array temperature SA5B
    ArrayTempSa5b,
    /// Sun detector SA5A
    SunDetectorSa5a,
    /// Sun detector SA5B
    SunDetectorSa5b,
    /// Voltage feeding BCR6
    VoltageFeedingBcr6,
    /// Current BCR6 SA6A
    CurrentBcr6Sa6a,
    /// Current BCR6 SA6B
    CurrentBcr6Sa6b,
    /// Array temperature SA6A
    ArrayTempSa6a,
    /// Array temperature SA6B
    ArrayTempSa6b,
    /// Sun detector SA6A
    SunDetectorSa6a,
    /// Sun detector SA6b
    SunDetectorSa6b,
    /// Voltage feeding BCR7
    VoltageFeedingBcr7,
    /// Current BCR7 SA7A
    CurrentBcr7Sa7a,
    /// Current BCR7 SA7B
    CurrentBcr7Sa7b,
    /// Array temperature SA7A
    ArrayTempSa7a,
    /// Array temperature SA7B
    ArrayTempSa7b,
    /// Sun detector SA7A
    SunDetectorSa7a,
    /// Sun detector SA7B
    SunDetectorSa7b,
    /// Voltage feeding BCR8
    VoltageFeedingBcr8,
    /// Current BCR8 SA8A
    CurrentBcr8Sa8a,
    /// Current BCR8 SA8B
    CurrentBcr8Sa8b,
    /// Array temperature SA8A
    ArrayTempSa8a,
    /// Array temperature SA8B
    ArrayTempSa8b,
    /// Sun detector SA8A
    SunDetectorSa8a,
    /// Sun detector SA8B
    SunDetectorSa8b,
    /// Voltage feeding BCR9
    VoltageFeedingBcr9,
    /// Current BCR9 SA9A
    CurrentBcr9Sa9a,
    /// Current BCR9 SA9B
    CurrentBcr9Sa9b,
    /// Array temperature SA9A
    ArrayTempSa9a,
    /// Array temperature SA9B
    ArrayTempSa9b,
    /// Sun detector SA9A
    SunDetectorSa9a,
    /// Sun detector SA9B
    SunDetectorSa9b,
    /// Board temperature
    BoardTemperature,
}

impl From<Type> for DaughterboardTelemetryType {
    fn from(t: Type) -> Self {
        match t {
            Type::VoltageFeedingBcr4 => Self::VoltageFeedingBcr4,
            Type::CurrentBcr4Sa4a => Self::CurrentBcr4Sa4a,
            Type::CurrentBcr4Sa4b => Self::CurrentBcr4Sa4b,
            Type::ArrayTempSa4a => Self::ArrayTempSa4a,
            Type::ArrayTempSa4b => Self::ArrayTempSa4b,
            Type::SunDetectorSa4a => Self::SunDetectorSa4a,
            Type::SunDetectorSa4b => Self::SunDetectorSa4b,
            Type::VoltageFeedingBcr5 => Self::VoltageFeedingBcr5,
            Type::CurrentBcr5Sa5a => Self::CurrentBcr5Sa5a,
            Type::CurrentBcr5Sa5b => Self::CurrentBcr5Sa5b,
            Type::ArrayTempSa5a => Self::ArrayTempSa5a,
            Type::ArrayTempSa5b => Self::ArrayTempSa5b,
            Type::SunDetectorSa5a => Self::SunDetectorSa5a,
            Type::SunDetectorSa5b => Self::SunDetectorSa5b,
            Type::VoltageFeedingBcr6 => Self::VoltageFeedingBcr6,
            Type::CurrentBcr6Sa6a => Self::CurrentBcr6Sa6a,
            Type::CurrentBcr6Sa6b => Self::CurrentBcr6Sa6b,
            Type::ArrayTempSa6a => Self::ArrayTempSa6a,
            Type::ArrayTempSa6b => Self::ArrayTempSa6b,
            Type::SunDetectorSa6a => Self::SunDetectorSa6a,
            Type::SunDetectorSa6b => Self::SunDetectorSa6b,
            Type::VoltageFeedingBcr7 => Self::VoltageFeedingBcr7,
            Type::CurrentBcr7Sa7a => Self::CurrentBcr7Sa7a,
            Type::CurrentBcr7Sa7b => Self::CurrentBcr7Sa7b,
            Type::ArrayTempSa7a => Self::ArrayTempSa7a,
            Type::ArrayTempSa7b => Self::ArrayTempSa7b,
            Type::SunDetectorSa7a => Self::SunDetectorSa7a,
            Type::SunDetectorSa7b => Self::SunDetectorSa7b,
            Type::VoltageFeedingBcr8 => Self::VoltageFeedingBcr8,
            Type::CurrentBcr8Sa8a => Self::CurrentBcr8Sa8a,
            Type::CurrentBcr8Sa8b => Self::CurrentBcr8Sa8b,
            Type::ArrayTempSa8a => Self::ArrayTempSa8a,
            Type::ArrayTempSa8b => Self::ArrayTempSa8b,
            Type::SunDetectorSa8a => Self::SunDetectorSa8a,
            Type::SunDetectorSa8b => Self::SunDetectorSa8b,
            Type::VoltageFeedingBcr9 => Self::VoltageFeedingBcr9,
            Type::CurrentBcr9Sa9a => Self::CurrentBcr9Sa9a,
            Type::CurrentBcr9Sa9b => Self::CurrentBcr9Sa9b,
            Type::ArrayTempSa9a => Self::ArrayTempSa9a,
            Type::ArrayTempSa9b => Self::ArrayTempSa9b,
            Type::SunDetectorSa9a => Self::SunDetectorSa9a,
            Type::SunDetectorSa9b => Self::SunDetectorSa9b,
            Type::BoardTemperature => Self::BoardTemperature,
        }
    }
}


/// Daughterboard telemetry structure
pub struct DaughterboardTelemetry;

#[Object]
impl DaughterboardTelemetry {
    async fn value(&self, ctx: &async_graphql::Context<'_>, telemetry_type: Type) -> FieldResult<f64> {
        let context = ctx.data::<crate::schema::Context>()?;
        Ok(context.subsystem().get_daughterboard_telemetry(telemetry_type)? as f64)
    }
}


// macro_rules! make_telemetry {
//     (
//         $($type: ident,)+
//     ) => {
//         /// Daughterboard telemetry values
//         ///
//         /// See Table 11-8 in the EPS' User Manual for more information
//         #[derive(Clone, Hash, Debug, Eq, GraphQLEnum, PartialEq)]
//         pub enum Type {
//             $(
//                 /// $type
//                 $type,
//             )+
//         }

//         impl From<Type> for DaughterboardTelemetryType {
//             fn from(t: Type) -> Self {
//                 match t {
//                     $(Type::$type => Self::$type,)+
//                 }
//             }
//         }

//         graphql_object!(Telemetry: Context as "daughterboard" |&self| {
//             $(
//                 field $type(&executor) -> FieldResult<f64>
//                 {
//                     Ok(executor.context().subsystem().get_daughterboard_telemetry(Type::$type)? as f64)
//                 }
//             )+
//         });
//     }
// }

// make_telemetry!(
//     VoltageFeedingBcr4,
//     CurrentBcr4Sa4a,
//     CurrentBcr4Sa4b,
//     ArrayTempSa4a,
//     ArrayTempSa4b,
//     SunDetectorSa4a,
//     SunDetectorSa4b,
//     VoltageFeedingBcr5,
//     CurrentBcr5Sa5a,
//     CurrentBcr5Sa5b,
//     ArrayTempSa5a,
//     ArrayTempSa5b,
//     SunDetectorSa5a,
//     SunDetectorSa5b,
//     VoltageFeedingBcr6,
//     CurrentBcr6Sa6a,
//     CurrentBcr6Sa6b,
//     ArrayTempSa6a,
//     ArrayTempSa6b,
//     SunDetectorSa6a,
//     SunDetectorSa6b,
//     VoltageFeedingBcr7,
//     CurrentBcr7Sa7a,
//     CurrentBcr7Sa7b,
//     ArrayTempSa7a,
//     ArrayTempSa7b,
//     SunDetectorSa7a,
//     SunDetectorSa7b,
//     VoltageFeedingBcr8,
//     CurrentBcr8Sa8a,
//     CurrentBcr8Sa8b,
//     ArrayTempSa8a,
//     ArrayTempSa8b,
//     SunDetectorSa8a,
//     SunDetectorSa8b,
//     VoltageFeedingBcr9,
//     CurrentBcr9Sa9a,
//     CurrentBcr9Sa9b,
//     ArrayTempSa9a,
//     ArrayTempSa9b,
//     SunDetectorSa9a,
//     SunDetectorSa9b,
//     BoardTemperature,
// );
