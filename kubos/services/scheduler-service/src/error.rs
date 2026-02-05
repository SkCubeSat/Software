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
//! Scheduler specific errors
//!

use thiserror::Error;

/// Errors which occur when using the scheduler
#[derive(Debug, Error, Eq, PartialEq)]
#[allow(clippy::enum_variant_names)]
#[allow(dead_code)]
pub enum SchedulerError {
    /// An error was raised while activating a mode
    #[error("Failed to activate '{name}': {err}")]
    ActivateError {
        /// The specific error encountered
        err: String,
        /// Mode which failed activation
        name: String,
    },
    // An error was raised when creating a file or directory
    #[error("Failed to create '{path}': {err}")]
    CreateError {
        /// The specific error encountered
        err: String,
        /// Path of file/dir which failed to create
        path: String,
    },
    // An error was raised when parsing a hms field
    #[error("Failed to parse hms field '{field}': {err}")]
    HmsParseError {
        /// Error encountered
        err: String,
        /// Delay or time field parsed
        field: String,
    },
    // An error was raised and scheduler switched to safe mode
    #[error("Scheduler failed over to safe mode due to error: {err}")]
    FailoverError {
        /// Error which caused failover
        err: String,
    },
    // A generic scheduler error
    #[error("Scheduler error encountered: {err}")]
    GenericError {
        /// Generic error encountered
        err: String,
    },
    /// An error was raised while importing a task list
    #[error("Failed to import '{name}': {err}")]
    ImportError {
        /// The specific import error
        err: String,
        // Path of task list which failed to import
        name: String,
    },
    // An error was raised when loading a mode
    #[error("Failed to load mode {path}: {err}")]
    LoadModeError {
        /// The specific error encountered
        err: String,
        /// The path of the mode that failed to load
        path: String,
    },
    // An error was raised when sending a graphql query
    #[error("Scheduler query failed: {err}")]
    #[allow(dead_code)]
    QueryError {
        /// The error encountered
        err: String,
    },
    /// An error was raised when removing a mode or task file
    #[error("Failed to remove '{name}': {err}")]
    RemoveError {
        /// Specific removal error
        err: String,
        /// Name of task or mode removed
        name: String,
    },
    // An error was raised when starting up the scheduler
    #[error("Scheduler failed to start: {err}")]
    StartError {
        /// The error encountered
        err: String,
    },
    // An error was raised when parsing a task list
    #[error("Failed to parse task list '{name}': {err}")]
    TaskListParseError {
        /// The specific parsing error
        err: String,
        /// The name of the task list that failed to parse
        name: String,
    },
    // An error was raised when parsing a task
    #[error("Failed to parse task '{description}': {err}")]
    TaskParseError {
        /// The specific parsing error
        err: String,
        /// The description of task that failed to parse
        description: String,
    },
    // An out of bounds time was found when parsing a task
    #[error("Out of bounds time found in task '{description}': {err}")]
    TaskTimeError {
        /// The specific parsing error
        err: String,
        /// The description of task that failed to parse
        description: String,
    },
}

impl From<String> for SchedulerError {
    fn from(err: String) -> Self {
        SchedulerError::GenericError { err }
    }
}
