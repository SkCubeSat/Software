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

//! Service queries

use crate::models::*;
use crate::schema::Context;
// use juniper::FieldResult;
use async_graphql::{Object, Result as FieldResult};

/// Telemetry query structure
pub struct Telemetry;

#[Object]
impl Telemetry {
    /// Fetch telemetry data for the motherboard.
    async fn motherboard(&self) -> motherboard_telemetry::MotherboardTelemetry {
        motherboard_telemetry::MotherboardTelemetry {}
    }

    /// Fetch telemetry data for the daughterboard.
    async fn daughterboard(&self) -> daughterboard_telemetry::DaughterboardTelemetry {
        daughterboard_telemetry::DaughterboardTelemetry {}
    }

    /// Get the number of board resets, by category
    async fn reset(&self) -> reset_telemetry::ResetTelemetry {
        reset_telemetry::ResetTelemetry {}
    }

    /// Fetch the current watchdog timeout period, in minutes
    async fn watchdog_period(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<i32> {
        let context = ctx.data::<Context>()?;
        Ok(i32::from(context.subsystem().get_comms_watchdog_period()?))
    }

    /// Get the version information for the EPS motherboard and daughterboard (if accessible)
    async fn version(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<version::VersionData> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_version()?)
    }

    /// Fetch the last error which was encountered by the system while executing a command
    async fn last_eps_error(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<last_error::ErrorData> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_last_eps_error()?)
    }

    /// Check the status of the motherboard and daughterboard
    async fn board_status(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<board_status::BoardData> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_board_status()?)
    }
}

/// Top-level query root structure
pub struct Root;

#[Object]
impl Root {
    /// Test query to verify service is running without attempting to communicate with hardware
    async fn ping(&self) -> FieldResult<String> {
        Ok(String::from("pong"))
    }

    /// Get the last mutation run
    async fn ack(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<subsystem::Mutations> {
        let context = ctx.data::<Context>()?;
        let last_cmd = context.subsystem().last_mutation.read().map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(*last_cmd)
    }

    /// Get all errors encountered since the last time this field was queried
    async fn errors(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<Vec<String>> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_errors()?)
    }

    /// Get the system power status
    async fn power(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<GetPowerResponse> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_power()?)
    }

    /// Get telemetry from the EPS
    async fn telemetry(&self) -> Telemetry {
        Telemetry
    }

    // --- Battery board queries ---

    /// Get all telemetry from the battery board in a single query
    async fn battery_telemetry(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<battery_telemetry::BatteryTelemetryData> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_battery_telemetry()?)
    }

    /// Get the battery board status
    async fn battery_board_status(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<board_status::BoardData> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_battery_board_status()?)
    }

    /// Get the battery board version information
    async fn battery_version(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<version::VersionData> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_battery_version()?)
    }

    /// Get the battery board last error
    async fn battery_last_error(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<last_error::ErrorData> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_battery_last_error()?)
    }

    /// Get the battery heater controller status (0 = disabled, 1 = enabled)
    async fn battery_heater_status(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<i32> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_battery_heater_status()?)
    }

    /// Get the battery board reset telemetry (brown-out count)
    async fn battery_reset_brown_out(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<reset_telemetry::Data> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_battery_reset_telemetry(reset_telemetry::Type::BrownOut)?)
    }

    /// Get the battery board reset telemetry (automatic software reset count)
    async fn battery_reset_automatic_software(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<reset_telemetry::Data> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_battery_reset_telemetry(reset_telemetry::Type::AutomaticSoftware)?)
    }

    /// Get the battery board reset telemetry (manual reset count)
    async fn battery_reset_manual(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<reset_telemetry::Data> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_battery_reset_telemetry(reset_telemetry::Type::Manual)?)
    }
}
