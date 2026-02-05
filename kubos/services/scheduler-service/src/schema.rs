/*
 * Copyright (C) 2019 Kubos Corporation
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

//!
//! GraphQL schema for scheduler service's public interface
//!

use crate::mode::*;
use crate::scheduler::{Scheduler, SAFE_MODE};
use crate::task_list::{import_raw_task_list, import_task_list, remove_task_list};
use async_graphql::{Context, Object, Result, SimpleObject};
use serde::Deserialize;

// Generic GraphQL Response
#[derive(Debug, Deserialize, SimpleObject)]
pub struct GenericResponse {
    pub success: bool,
    pub errors: String,
}

pub struct QueryRoot;

// Base GraphQL query model
#[Object]
impl QueryRoot {
    /// Test query to verify service is running without attempting
    /// to communicate with the underlying subsystem
    async fn ping(&self) -> &str {
        "pong"
    }

    /// Returns information on the currently active mode
    async fn active_mode(&self, ctx: &Context<'_>) -> Result<Option<ScheduleMode>> {
        let context = ctx.data::<kubos_service::Context<Scheduler>>()?;
        Ok(get_active_mode(&context.subsystem().scheduler_dir)
            .map_err(|err| async_graphql::Error::new(format!("Failed to get active mode: {}", err)))?)
    }

    /// Returns a list of information on currently available modes
    async fn available_modes(&self, ctx: &Context<'_>, name: Option<String>) -> Result<Vec<ScheduleMode>> {
        let context = ctx.data::<kubos_service::Context<Scheduler>>()?;
        Ok(get_available_modes(&context.subsystem().scheduler_dir, name)
            .map_err(|err| async_graphql::Error::new(format!("Failed to get available modes: {}", err)))?)
    }
}

pub struct MutationRoot;

// Base GraphQL mutation model
#[Object]
impl MutationRoot {
    /// Creates a new mode
    async fn create_mode(&self, ctx: &Context<'_>, name: String) -> Result<GenericResponse> {
        let context = ctx.data::<kubos_service::Context<Scheduler>>()?;
        Ok(match create_mode(&context.subsystem().scheduler_dir, &name) {
            Ok(_) => {
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }

    /// Removes an existing mode
    async fn remove_mode(&self, ctx: &Context<'_>, name: String) -> Result<GenericResponse> {
        let context = ctx.data::<kubos_service::Context<Scheduler>>()?;
        Ok(match remove_mode(&context.subsystem().scheduler_dir, &name) {
            Ok(_) => {
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }

    /// Activates a mode
    async fn activate_mode(&self, ctx: &Context<'_>, name: String) -> Result<GenericResponse> {
        if name == SAFE_MODE {
            return Ok(GenericResponse { success: false, errors: "Must use safeMode to activate safe".to_owned() });
        }
        
        let context = ctx.data::<kubos_service::Context<Scheduler>>()?;
        Ok(match activate_mode(&context.subsystem().scheduler_dir, &name)
        .and_then(|_| context.subsystem().stop())
        .and_then(|_| context.subsystem().start()) {
            Ok(_) => {
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }

    /// Activates the safe mode
    async fn safe_mode(&self, ctx: &Context<'_>) -> Result<GenericResponse> {
        let context = ctx.data::<kubos_service::Context<Scheduler>>()?;
        Ok(match activate_mode(&context.subsystem().scheduler_dir, SAFE_MODE)
        .and_then(|_| context.subsystem().stop())
        .and_then(|_| context.subsystem().start()) {
            Ok(_) => {
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }

    /// Imports a new task list into a mode
    async fn import_task_list(&self, ctx: &Context<'_>, name: String, path: String, mode: String) -> Result<GenericResponse> {
        let context = ctx.data::<kubos_service::Context<Scheduler>>()?;
        Ok(match import_task_list(&context.subsystem().scheduler_dir, &name, &path, &mode)
        .and_then(|_| context.subsystem().check_stop_task_list(&name, &mode))
        .and_then(|_| context.subsystem().check_start_task_list(&name, &mode)) {
            Ok(_) => GenericResponse { success: true, errors: "".to_owned() },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }

    /// Removes a task list from a mode
    async fn remove_task_list(&self, ctx: &Context<'_>, name: String, mode: String) -> Result<GenericResponse> {
        let context = ctx.data::<kubos_service::Context<Scheduler>>()?;
        Ok(match remove_task_list(&context.subsystem().scheduler_dir, &name, &mode)
        .and_then(|_| context.subsystem().check_stop_task_list(&name, &mode)) {
            Ok(_) => {
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }

    /// Imports a raw task list into a mode
    async fn import_raw_task_list(&self, ctx: &Context<'_>, name: String, mode: String, json: String) -> Result<GenericResponse> {
        let context = ctx.data::<kubos_service::Context<Scheduler>>()?;
        Ok(match import_raw_task_list(&context.subsystem().scheduler_dir, &name, &mode, &json)
        .and_then(|_| context.subsystem().check_stop_task_list(&name, &mode))
        .and_then(|_| context.subsystem().check_start_task_list(&name, &mode)) {
            Ok(_) => GenericResponse { success: true, errors: "".to_owned() },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }
}
