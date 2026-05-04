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
    /// This sends the Send Data command (0xC5) to the board, which loads
    /// the latest collected data from the SD card into the transfer buffer,
    /// then reads the 107-byte response containing:
    /// - 27 unsigned 16-bit IC test readings (9 ICs × 3 readings each)
    /// - 6 bytes of timestamp data
    async fn telemetry(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> FieldResult<PayloadDataResponse> {
        let context = ctx.data::<Context>()?;
        Ok(context.subsystem().get_payload_data()?)
    }
}
