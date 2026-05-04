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

//! Service mutations

use crate::models::subsystem::Mutations;
use crate::models::MutationResponse;
use crate::schema::Context;
use async_graphql::{Object, Result as FieldResult};

/// Top-level mutation root structure
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Execute a trivial command against the system (no-op).
    ///
    /// Verifies the service is responsive without sending any
    /// command to the hardware.
    async fn noop(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context.subsystem().set_last_mutation(Mutations::Noop);
        Ok(context.subsystem().noop()?)
    }

    /// Reset the Analog IC board (0x61).
    ///
    /// Resets the board to its initial state.
    async fn reset(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context.subsystem().set_last_mutation(Mutations::Reset);
        Ok(context.subsystem().reset()?)
    }

    /// Start the IC testing loop (0x63).
    ///
    /// Forces the main testing loop to run, testing all 9 ICs.
    async fn start(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context.subsystem().set_last_mutation(Mutations::Start);
        Ok(context.subsystem().start_tests()?)
    }

    /// Enter power saving mode (0x65).
    ///
    /// Turns off the 5V power plane and puts the controller to sleep.
    /// Power consumption drops to approximately 50 mW.
    async fn power_saving_mode(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context
            .subsystem()
            .set_last_mutation(Mutations::PowerSavingMode);
        Ok(context.subsystem().power_saving_mode()?)
    }

    /// Enter normal power mode (0x66).
    ///
    /// Turns on the 5V power plane. The controller remains in sleep mode
    /// but will run tests every 12 hours.
    async fn normal_power_mode(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context
            .subsystem()
            .set_last_mutation(Mutations::NormalPowerMode);
        Ok(context.subsystem().normal_power_mode()?)
    }

    /// Set the board's RTC time (0x67).
    ///
    /// Reads the current system time from the OBC using the POSIX `date`
    /// command and sends it to the Analog IC board as an 8-byte RTC payload.
    /// This must be called on every startup/reboot of the board.
    async fn set_rtc_time(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context
            .subsystem()
            .set_last_mutation(Mutations::SetRtcTime);
        Ok(context.subsystem().set_rtc_time()?)
    }

    /// Pass a custom command through to the board.
    ///
    /// # Arguments
    ///
    /// * `command` - Decimal value of the command byte to send
    /// * `data` - Decimal values of the command data bytes. Use `[0]` if no data.
    async fn issue_raw_command(
        &self,
        ctx: &async_graphql::Context<'_>,
        command: i32,
        data: Vec<i32>,
    ) -> FieldResult<MutationResponse> {
        let context = ctx.data::<Context>()?;
        context
            .subsystem()
            .set_last_mutation(Mutations::RawCommand);
        let data_u8 = data.iter().map(|x| *x as u8).collect();
        Ok(context
            .subsystem()
            .raw_command(command as u8, data_u8)?)
    }
}
