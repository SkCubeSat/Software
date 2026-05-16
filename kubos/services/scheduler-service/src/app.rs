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
//! Definitions and functions for dealing with scheduled app execution
//!

use crate::error::SchedulerError;
use crate::schema::GenericResponse;
use async_graphql::SimpleObject;
use log::{debug, error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct StartAppResponse {
    #[serde(rename = "startApp")]
    pub start_app: GenericResponse,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct StartAppGraphQL {
    pub data: StartAppResponse,
}

// Helper function for sending query to app service using modern async reqwest
pub async fn service_query(query: &str, hosturl: &str) -> Result<StartAppGraphQL, SchedulerError> {
    debug!("service query {}, url: {}", query, hosturl);
    
    // The app service will wait 300ms to see if an app completes before returning its response to us
    let client = Client::builder()
        .timeout(Duration::from_millis(350))
        .build()
        .map_err(|e| SchedulerError::QueryError {
            err: format!("Error building client: {}", e),
        })?;
    
    let mut map = HashMap::new();
    map.insert("query", query);
    let url = format!("http://{}", hosturl);

    let res = client
        .post(&url)
        .json(&map)
        .send()
        .await
        .map_err(|e| SchedulerError::QueryError {
            err: format!("Error posting query: {}", e),
        })?;

    let text = res.text().await.map_err(|e| SchedulerError::QueryError {
        err: format!("Error extracting response: {}", e),
    })?;
    
    debug!("service response: {}", text);

    from_str(&text).map_err(|e| SchedulerError::QueryError {
        err: format!("Error parsing response as JSON: {}", e),
    })
}

// Configuration used for execution of an app
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub args: Option<Vec<String>>,
    pub config: Option<String>,
}

impl App {
    /// Execute this app by sending a GraphQL mutation to the app service
    pub async fn execute(&self, service_url: &str) -> Result<(), SchedulerError> {
        info!("Start app {}", self.name);
        let mut query_args = format!("name: \"{}\"", self.name);
        if let Some(config) = &self.config {
            query_args.push_str(&format!(", config: \"{}\"", config));
        }
        if let Some(args) = &self.args {
            let app_args: Vec<String> = args.iter().map(|x| format!("\"{}\"", x)).collect();

            let app_args = app_args.join(",");
            query_args.push_str(&format!(", args: [{}]", app_args));
        }
        let query = format!(
            r#"mutation {{ startApp({}) {{ success, errors }} }}"#,
            query_args
        );
        
        match service_query(&query, service_url).await {
            Err(e) => {
                error!("Failed to send start app query: {}", e);
                Err(e)
            }
            Ok(resp) => {
                if !resp.data.start_app.success {
                    let err = format!("Failed to start scheduled app: {}", resp.data.start_app.errors);
                    error!("{}", err);
                    Err(SchedulerError::GenericError { err })
                } else {
                    info!("Successfully started app {}", self.name);
                    Ok(())
                }
            }
        }
    }
}
