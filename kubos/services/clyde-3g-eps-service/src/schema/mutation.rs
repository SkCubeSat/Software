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

//! Service mutations

use crate::models::subsystem::Mutations;
use crate::models::{MutationResponse, TestType};
use crate::schema::Context;
// use juniper::FieldResult;
use async_graphql::{Object, Result as FieldResult};

/// Top-level mutation root structure
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Execute a trivial command against the system
    async fn noop(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context.subsystem().set_last_mutation(Mutations::Noop);
        Ok(context.subsystem().reset_watchdog()?)
    }

    /// Manually reset the EPS
    async fn manual_reset(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context.subsystem().set_last_mutation(Mutations::ManualReset);
        Ok(context.subsystem().manual_reset()?)
    }

    /// Reset the communications watchdog timer
    async fn reset_watchdog(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context.subsystem().set_last_mutation(Mutations::ResetWatchdog);
        Ok(context.subsystem().reset_watchdog()?)
    }

    /// Set the communications watchdog timeout period
    async fn set_watchdog_period(
        &self,
        ctx: &async_graphql::Context<'_>,
        period: i32,
    ) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context.subsystem().set_last_mutation(Mutations::SetWatchdogPeriod);
        Ok(context.subsystem().set_watchdog_period(period as u8)?)
    }

    /// Pass a custom command through to the system
    async fn issue_raw_command(
        &self,
        ctx: &async_graphql::Context<'_>,
        command: i32,
        data: Vec<i32>,
    ) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context.subsystem().set_last_mutation(Mutations::RawCommand);
        let data_u8 = data.iter().map(|x| *x as u8).collect();
        Ok(context.subsystem().raw_command(command as u8, data_u8)?)
    }

    /// Perform a system test
    async fn test_hardware(
        &self,
        ctx: &async_graphql::Context<'_>,
        test: TestType,
    ) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context.subsystem().set_last_mutation(Mutations::TestHardware);

        match test {
            TestType::Hardware => Ok(context.subsystem().test_hardware()?),
            TestType::Integration => Ok(MutationResponse {
                errors: "Not Implemented".to_owned(),
                success: false,
            }),
        }
    }

    // --- Battery board mutations ---

    /// Manually reset the battery board
    async fn battery_manual_reset(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context.subsystem().set_last_mutation(Mutations::BatteryManualReset);
        Ok(context.subsystem().battery_manual_reset()?)
    }

    /// Reset the battery board communications watchdog timer
    async fn battery_reset_watchdog(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context.subsystem().set_last_mutation(Mutations::BatteryResetWatchdog);
        Ok(context.subsystem().battery_reset_watchdog()?)
    }

    /// Set the battery heater controller status
    ///
    /// - mode: 0 to disable thermostat control, 1 to enable
    async fn battery_set_heater(
        &self,
        ctx: &async_graphql::Context<'_>,
        mode: i32,
    ) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context.subsystem().set_last_mutation(Mutations::BatterySetHeater);
        Ok(context.subsystem().set_battery_heater_status(mode as u8)?)
    }

    /// Pass a custom command through to the battery board
    async fn battery_issue_raw_command(
        &self,
        ctx: &async_graphql::Context<'_>,
        command: i32,
        data: Vec<i32>,
    ) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context.subsystem().set_last_mutation(Mutations::BatteryRawCommand);
        let data_u8 = data.iter().map(|x| *x as u8).collect();
        Ok(context.subsystem().battery_raw_command(command as u8, data_u8)?)
    }
}
