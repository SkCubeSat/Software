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

use crate::app_entry;
use crate::monitor;
use async_graphql::*;

/// Common response fields structure for requests
/// which don't return any specific data
#[derive(SimpleObject)]
pub struct GenericResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
}

/// Response fields for the `register` mutation
#[derive(SimpleObject)]
pub struct RegisterResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// The new registry entry created after successfully registration
    pub entry: Option<KAppRegistryEntry>,
}

/// Response fields for the `startApp` mutation
#[derive(SimpleObject)]
pub struct StartResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// PID of the started process
    pub pid: Option<i32>,
}

#[derive(SimpleObject)]
#[graphql(name = "App")]
pub struct KApp {
    /// Name
    pub name: String,
    /// Version
    pub version: String,
    /// Author
    pub author: String,
    /// Absolute Path to Executable
    pub executable: String,
    /// Configuration File Path
    pub config: String,
}

impl From<app_entry::App> for KApp {
    fn from(app: app_entry::App) -> Self {
        KApp {
            name: app.name,
            version: app.version,
            author: app.author,
            executable: app.executable,
            config: app.config,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(name = "AppRegistryEntry")]
pub struct KAppRegistryEntry {
    /// App
    pub app: KApp,
    /// Active
    pub active: bool,
}

impl From<app_entry::AppRegistryEntry> for KAppRegistryEntry {
    fn from(entry: app_entry::AppRegistryEntry) -> Self {
        KAppRegistryEntry {
            app: KApp::from(entry.app),
            active: entry.active_version,
        }
    }
}

/// GraphQL-compatible version of MonitorEntry
#[derive(SimpleObject)]
#[graphql(name = "MonitorEntry")]
pub struct KMonitorEntry {
    pub name: String,
    pub version: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub running: bool,
    pub pid: Option<i32>,
    pub last_rc: Option<i32>,
    pub last_signal: Option<i32>,
    pub args: Option<Vec<String>>,
    pub config: String,
}

impl From<monitor::MonitorEntry> for KMonitorEntry {
    fn from(entry: monitor::MonitorEntry) -> Self {
        KMonitorEntry {
            name: entry.name,
            version: entry.version,
            start_time: entry.start_time.to_rfc3339(),
            end_time: entry.end_time.map(|dt| dt.to_rfc3339()),
            running: entry.running,
            pid: entry.pid,
            last_rc: entry.last_rc,
            last_signal: entry.last_signal,
            args: entry.args,
            config: entry.config,
        }
    }
}
