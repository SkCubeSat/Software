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

//! Service queries

use crate::models::*;
use crate::models::subsystem;
use crate::schema::Context;
use async_graphql::{Object, Result as FieldResult};

/// Top-level query root structure
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Test query to verify service is running without attempting
    /// to communicate with hardware
    async fn ping(&self) -> FieldResult<String> {
        Ok(String::from("pong"))
    }

    /// Get the last mutation run
    async fn ack(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<subsystem::Mutations> {
        let context = ctx.data::<Context>()?;
        let last_cmd = context
            .subsystem()
            .last_mutation
            .read()
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(*last_cmd)
    }

    /// Get all errors encountered since the last time this field was queried
    async fn errors(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<Vec<String>> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_errors()?)
    }

    /// Retrieve the latest payload telemetry data.
    ///
    /// This follows the updated data collection flow:
    /// 1. Check Latest Timestamp (0x6A) to identify the data file
    /// 2. Send Data command (0xC5) with file byte
    /// 3. Read the response containing:
    ///    - 50 unsigned 16-bit IC test readings (5×10 matrix, little-endian)
    ///    - ASCII timestamp string
    async fn telemetry(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<PayloadDataResponse> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_payload_data()?)
    }

    /// Get the current RTC time from the payload board (0x68).
    ///
    /// Returns the year, month, day, weekday, hour, minute, and second
    /// as currently set on the board's STM32 RTC.
    async fn rtc_time(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<RtcTimeResponse> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_rtc_time()?)
    }

    /// Check the current power status of the board (0x69).
    ///
    /// Returns the power mode flag: Normal (0) or PowerSaving (1).
    async fn power_status(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<PowerStatusResponse> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().check_power_status()?)
    }

    /// Check the latest data file timestamp on the SD card (0x6A).
    ///
    /// Returns the FAT timestamp of the most recent S_*.CSV file
    /// on the board's SD card.
    async fn latest_timestamp(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<LatestTimestampResponse> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().check_latest_timestamp()?)
    }
}
