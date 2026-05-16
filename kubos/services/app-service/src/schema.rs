/*
 * Copyright (C) 2018 Kubos Corporation
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

use crate::objects::*;
use crate::registry::AppRegistry;
use async_graphql::*;
use kubos_service::Context as ServiceContext;

/// Base GraphQL query model
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Test service query
    async fn ping(&self) -> Result<String> {
        Ok(String::from("pong"))
    }

    /// Kubos Apps Query
    async fn registered_apps(
        &self,
        ctx: &Context<'_>,
        name: Option<String>,
        version: Option<String>,
        active: Option<bool>,
    ) -> Result<Vec<KAppRegistryEntry>> {
        let registry = ctx.data::<ServiceContext<AppRegistry>>()?;
        let entries = registry
            .subsystem()
            .entries
            .lock()
            .map_err(|e| Error::new(format!("Failed to lock entries: {:?}", e)))?;

        let result: Vec<KAppRegistryEntry> = entries
            .iter()
            .filter(|e| {
                if let Some(ref n) = name {
                    if &e.app.name != n {
                        return false;
                    }
                }
                if let Some(ref v) = version {
                    if &e.app.version != v {
                        return false;
                    }
                }
                if let Some(a) = active {
                    if e.active_version != a {
                        return false;
                    }
                }
                true
            })
            .map(|entry| KAppRegistryEntry::from(entry.clone()))
            .collect();

        Ok(result)
    }

    /// App Status Query
    async fn app_status(
        &self,
        ctx: &Context<'_>,
        name: Option<String>,
        version: Option<String>,
        running: Option<bool>,
    ) -> Result<Vec<KMonitorEntry>> {
        let registry = ctx.data::<ServiceContext<AppRegistry>>()?;
        let entries = registry
            .subsystem()
            .monitoring
            .lock()
            .map_err(|e| Error::new(format!("Failed to lock monitoring entries: {:?}", e)))?;

        let result: Vec<KMonitorEntry> = entries
            .iter()
            .filter(|e| {
                if let Some(ref n) = name {
                    if &e.name != n {
                        return false;
                    }
                }
                if let Some(ref v) = version {
                    if &e.version != v {
                        return false;
                    }
                }
                if let Some(r) = running {
                    if e.running != r {
                        return false;
                    }
                }
                true
            })
            .map(|entry| KMonitorEntry::from(entry.clone()))
            .collect();

        Ok(result)
    }
}

/// Base GraphQL mutation model
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Register App
    async fn register(&self, ctx: &Context<'_>, path: String) -> Result<RegisterResponse> {
        let registry = ctx.data::<ServiceContext<AppRegistry>>()?;
        
        match registry.subsystem().register(&path) {
            Ok(app) => Ok(RegisterResponse {
                success: true,
                errors: String::new(),
                entry: Some(KAppRegistryEntry::from(app)),
            }),
            Err(error) => Ok(RegisterResponse {
                success: false,
                errors: error.to_string(),
                entry: None,
            }),
        }
    }

    /// Uninstall App
    async fn uninstall(
        &self,
        ctx: &Context<'_>,
        name: String,
        version: Option<String>,
    ) -> Result<GenericResponse> {
        let registry = ctx.data::<ServiceContext<AppRegistry>>()?;
        
        let result = if let Some(v) = version {
            registry.subsystem().uninstall(&name, &v)
        } else {
            registry.subsystem().uninstall_all(&name)
        };

        match result {
            Ok(_) => Ok(GenericResponse {
                success: true,
                errors: String::new(),
            }),
            Err(error) => Ok(GenericResponse {
                success: false,
                errors: error.to_string(),
            }),
        }
    }

    /// Set App Active Version
    async fn set_version(
        &self,
        ctx: &Context<'_>,
        name: String,
        version: String,
    ) -> Result<GenericResponse> {
        let registry = ctx.data::<ServiceContext<AppRegistry>>()?;
        
        match registry.subsystem().set_version(&name, &version) {
            Ok(_) => Ok(GenericResponse {
                success: true,
                errors: String::new(),
            }),
            Err(error) => Ok(GenericResponse {
                success: false,
                errors: error.to_string(),
            }),
        }
    }

    /// Start App
    async fn start_app(
        &self,
        ctx: &Context<'_>,
        name: String,
        config: Option<String>,
        args: Option<Vec<String>>,
    ) -> Result<StartResponse> {
        let registry = ctx.data::<ServiceContext<AppRegistry>>()?;
        
        match registry.subsystem().start_app(&name, config, args) {
            Ok(pid) => Ok(StartResponse {
                success: true,
                errors: String::new(),
                pid,
            }),
            Err(error) => Ok(StartResponse {
                success: false,
                errors: error.to_string(),
                pid: None,
            }),
        }
    }

    /// Kill Running App
    async fn kill_app(
        &self,
        ctx: &Context<'_>,
        name: String,
        signal: Option<i32>,
    ) -> Result<GenericResponse> {
        let registry = ctx.data::<ServiceContext<AppRegistry>>()?;
        
        match registry.subsystem().kill_app(&name, signal) {
            Ok(_) => Ok(GenericResponse {
                success: true,
                errors: String::new(),
            }),
            Err(error) => Ok(GenericResponse {
                success: false,
                errors: error.to_string(),
            }),
        }
    }
}
