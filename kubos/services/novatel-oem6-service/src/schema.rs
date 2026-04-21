//
// Copyright (C) 2018 Kubos Corporation
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
    // Test query to verify service is running without attempting
    // to communicate with the underlying subsystem
    //
    // {
    //     ping: "pong"
    // }
    async fn ping(&self) -> QueryResult<String> {
        Ok(String::from("pong"))
    }

    //----- Generic Queries -----//

    // Get the last run mutation
    //
    // {
    //     ack: AckCommand
    // }
    async fn ack(&self, ctx: &async_graphql::Context<'_>) -> QueryResult<AckCommand> {
        let context = ctx.data::<ServiceContext>()?;
        let last_cmd = context
            .subsystem()
            .last_cmd
            .read()
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(*last_cmd)
    }

    // Get all errors encountered since the last time this field was queried
    //
    // {
    //     errors: [String]
    // }
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

    // Get the current power state of the system
    //
    // Note: `uptime` is included as an available field in order to conform to
    //       the Kubos Service Outline, but cannot be implemented for this device,
    //       so the value will be 1 if the device is on and 0 if the device is off
    //
    // {
    //     power {
    //         state: PowerState,
    //         uptime: Int
    //     }
    // }
    async fn power(&self, ctx: &async_graphql::Context<'_>) -> QueryResult<GetPowerResponse> {
        let context = ctx.data::<ServiceContext>()?;
        context
            .subsystem()
            .get_power()
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    // Get the current configuration of the system
    //
    // Stretch goal: implement the LOGLIST command
    //
    // {
    //     config: "Not Implemented"
    // }
    async fn config(&self) -> QueryResult<String> {
        Ok(String::from("Not Implemented"))
    }

    // Get the test results of the last run test
    //
    // {
    //     testResults{
    //         success,
    //         telemetryNominal{...},
    //         telemetryDebug{...}
    //     }
    // }
    async fn test_results(&self, ctx: &async_graphql::Context<'_>) -> QueryResult<IntegrationTestResults> {
        let context = ctx.data::<ServiceContext>()?;
        context
            .subsystem()
            .get_test_results()
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    // Get the current system status and errors
    //
    // {
    //     systemStatus {
    //        errors: Vec<String>,
    //        status: Vec<String>
    //     }
    // }
    async fn system_status(&self, ctx: &async_graphql::Context<'_>) -> QueryResult<SystemStatus> {
        let context = ctx.data::<ServiceContext>()?;
        context
            .subsystem()
            .get_system_status()
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    // Get current status of position information gathering
    //
    // {
    //     lockStatus {
    //         positionStatus: SolutionStatus,
    //           positionType: PosVelType,
    //           time {
    //             ms: Int,
    //               week: Int
    //           },
    //         timeStatus: RefTimeStatus,
    //         velocityStatus: SolutionStatus,
    //           velocityType: PosVelType
    //     }
    // }
    async fn lock_status(&self, ctx: &async_graphql::Context<'_>) -> QueryResult<LockStatusGql> {
        let context = ctx.data::<ServiceContext>()?;
        Ok(LockStatusGql(
            context
                .subsystem()
                .get_lock_status()
                .map_err(|e| async_graphql::Error::new(e.to_string()))?,
        ))
    }

    // Get the last known good position information
    //
    // {
    //     lockInfo {
    //        position: Vec<Float>,
    //        time {
    //            ms: Int,
    //            week: Int
    //        },
    //        velocity: Vec<Float>
    //     }
    // }
    async fn lock_info(&self, ctx: &async_graphql::Context<'_>) -> QueryResult<LockInfoGql> {
        let context = ctx.data::<ServiceContext>()?;
        Ok(LockInfoGql(
            context
                .subsystem()
                .get_lock_info()
                .map_err(|e| async_graphql::Error::new(e.to_string()))?,
        ))
    }

    // Get current telemetry information for the system
    async fn telemetry(&self, ctx: &async_graphql::Context<'_>) -> QueryResult<Telemetry> {
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
    // Get all errors encountered while processing this GraphQL request
    //
    // Note: This will only return errors thrown by fields which have
    // already been processed, so it is recommended that this field be specified last.
    //
    // mutation {
    //     errors: [String]
    // }
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

    // Execute a trivial command against the system
    //
    // mutation {
    //     noop {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
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

    // Control the power state of the system
    //
    // Note: Power control of the GPS device will be done by the GPSRM service
    //
    // mutation {
    //     controlPower: "Not Implemented"
    // }
    async fn control_power(&self, ctx: &async_graphql::Context<'_>) -> QueryResult<String> {
        let context = ctx.data::<ServiceContext>()?;
        let mut last_cmd = context
            .subsystem()
            .last_cmd
            .write()
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        *last_cmd = AckCommand::ControlPower;
        Ok(String::from("Not Implemented"))
    }

    // Configure the system
    //
    // config: Vector of configuration requests (ConfigStruct)
    //   - option: Configuration operation which should be performed
    //   - hold: For `LOG_*` requests, specifies whether this request should be excluded
    //           from removal by future 'UNLOG_ALL' requests.
    //           For `UNLOG_ALL` requests, specifies whether the 'hold' value in previous
    //           `LOG_*` requests should be ignored.
    //   - interval: Interval at which log messages should be generated.
    //               Note: Only applies to `LOG_POSITION_DATA` requests. Ignored otherwise
    //   - offset: Offset of interval at which log messages should be generated.
    //             Note: Only applies to `LOG_POSITION_DATA` requests. Ignored otherwise
    //
    // mutation {
    //     configureHardware(config: [{option: ConfigOption, hold: Boolean, interval: Float, offset: Float},...]) {
    //         config: String
    //         errors: String,
    //         success: Boolean,
    //     }
    // }
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

    // Run a system self-test
    //
    // test: Type of self-test to perform
    //
    // mutation {
    //     testHardware(test: TestType) {
    //         ... on IntegrationTestResults {
    //             errors: String,
    //             success: Boolean,
    //             telemetryNominal{...},
    //             telemetryDebug{...}
    //         }
    //         ... on HardwareTestResults {
    //             errors: "Not Implemented",
    //             success: true,
    //             data: Empty
    //         }
    //    }
    // }
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

    // Pass a custom command through to the system
    //
    // command: String containing the hex values to be sent (ex. "C3")
    //          It will be converted to a byte array before transfer.
    //
    // mutation {
    //     issueRawCommand(command: String) {
    //         errors: String,
    //         success: Boolean,
    //         response: String
    //     }
    // }
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
