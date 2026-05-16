//
// Copyright (C) 2025 SkCubeSat
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

use crate::model::*;
use crate::objects::*;
use async_graphql::{Object, Result as QueryResult};

/// The kubos-service context type for this service
type ServiceContext = kubos_service::Context<Subsystem>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Test query to verify service is running without attempting
    /// to communicate with the underlying subsystem
    async fn ping(&self) -> QueryResult<String> {
        Ok(String::from("pong"))
    }

    /// Get the last run mutation
    async fn ack(&self, ctx: &async_graphql::Context<'_>) -> QueryResult<AckCommand> {
        let context = ctx.data::<ServiceContext>()?;
        let last_cmd = context
            .subsystem()
            .last_cmd
            .read()
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(*last_cmd)
    }

    /// Get all errors encountered since the last time this field was queried
    async fn errors(&self, ctx: &async_graphql::Context<'_>) -> QueryResult<Vec<String>> {
        let context = ctx.data::<ServiceContext>()?;
        context.subsystem().get_errors();

        match context.subsystem().errors.write() {
            Ok(mut master_vec) => {
                let current = master_vec.clone();
                master_vec.clear();
                master_vec.shrink_to_fit();
                Ok(current)
            }
            _ => Ok(vec![
                "Error: Failed to borrow master errors vector".to_owned()
            ]),
        }
    }

    /// Get the current power state of the system
    async fn power(&self, ctx: &async_graphql::Context<'_>) -> QueryResult<GetPowerResponse> {
        let context = ctx.data::<ServiceContext>()?;
        context
            .subsystem()
            .get_power()
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    /// Get the current configuration of the system (not yet implemented)
    async fn config(&self) -> QueryResult<String> {
        Ok(String::from("Not Implemented"))
    }

    /// Get the test results of the last run test
    async fn test_results(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> QueryResult<IntegrationTestResults> {
        let context = ctx.data::<ServiceContext>()?;
        context
            .subsystem()
            .get_test_results()
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    /// Get the current system status and errors
    async fn system_status(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> QueryResult<SystemStatus> {
        let context = ctx.data::<ServiceContext>()?;
        context
            .subsystem()
            .get_system_status()
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    /// Get current status of position information gathering
    async fn lock_status(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> QueryResult<LockStatusGql> {
        let context = ctx.data::<ServiceContext>()?;
        Ok(LockStatusGql(
            context
                .subsystem()
                .get_lock_status()
                .map_err(|e| async_graphql::Error::new(e.to_string()))?,
        ))
    }

    /// Get the last known good position information
    async fn lock_info(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> QueryResult<LockInfoGql> {
        let context = ctx.data::<ServiceContext>()?;
        Ok(LockInfoGql(
            context
                .subsystem()
                .get_lock_info()
                .map_err(|e| async_graphql::Error::new(e.to_string()))?,
        ))
    }

    /// Get current telemetry information for the system
    async fn telemetry(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> QueryResult<Telemetry> {
        let context = ctx.data::<ServiceContext>()?;
        context
            .subsystem()
            .get_telemetry()
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Get all errors encountered while processing this GraphQL request
    async fn errors(&self, ctx: &async_graphql::Context<'_>) -> QueryResult<Vec<String>> {
        let context = ctx.data::<ServiceContext>()?;
        context.subsystem().get_errors();

        match context.subsystem().errors.read() {
            Ok(master_vec) => Ok(master_vec.clone()),
            _ => Ok(vec![
                "Error: Failed to borrow master errors vector".to_owned()
            ]),
        }
    }

    /// Execute a trivial command against the system
    async fn noop(&self, ctx: &async_graphql::Context<'_>) -> QueryResult<GenericResponse> {
        let context = ctx.data::<ServiceContext>()?;
        let mut last_cmd = context
            .subsystem()
            .last_cmd
            .write()
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        *last_cmd = AckCommand::Noop;
        context
            .subsystem()
            .noop()
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    /// Control the power state of the system (not implemented — managed by GPSRM)
    async fn control_power(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> QueryResult<String> {
        let context = ctx.data::<ServiceContext>()?;
        let mut last_cmd = context
            .subsystem()
            .last_cmd
            .write()
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        *last_cmd = AckCommand::ControlPower;
        Ok(String::from("Not Implemented"))
    }

    /// Configure the system by sending LOG/UNLOG commands
    async fn configure_hardware(
        &self,
        ctx: &async_graphql::Context<'_>,
        config: Vec<ConfigStruct>,
    ) -> QueryResult<ConfigureHardwareResponse> {
        let context = ctx.data::<ServiceContext>()?;
        let mut last_cmd = context
            .subsystem()
            .last_cmd
            .write()
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        *last_cmd = AckCommand::ConfigureHardware;
        context
            .subsystem()
            .configure_hardware(config)
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    /// Run a system self-test
    async fn test_hardware(
        &self,
        ctx: &async_graphql::Context<'_>,
        test: TestType,
    ) -> QueryResult<TestResults> {
        let context = ctx.data::<ServiceContext>()?;
        let mut last_cmd = context
            .subsystem()
            .last_cmd
            .write()
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        *last_cmd = AckCommand::TestHardware;
        match test {
            TestType::Integration => Ok(TestResults::Integration(
                context
                    .subsystem()
                    .get_test_results()
                    .map_err(|e| async_graphql::Error::new(e.to_string()))?,
            )),
            TestType::Hardware => Ok(TestResults::Hardware(HardwareTestResults {
                errors: "Not Implemented".to_owned(),
                success: true,
                data: "".to_owned(),
            })),
        }
    }

    /// Pass a raw ASCII command through to the receiver
    ///
    /// Unlike the OEM6 service which sends hex-encoded binary bytes,
    /// this sends the command string as-is in ASCII format.
    /// Example: `issueRawCommand(command: "LOG VERSIONA ONCE")`
    async fn issue_raw_command(
        &self,
        ctx: &async_graphql::Context<'_>,
        command: String,
    ) -> QueryResult<GenericResponse> {
        let context = ctx.data::<ServiceContext>()?;
        let mut last_cmd = context
            .subsystem()
            .last_cmd
            .write()
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        *last_cmd = AckCommand::IssueRawCommand;
        context
            .subsystem()
            .passthrough(command)
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }
}
