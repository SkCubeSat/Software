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

use crate::udp::*;
use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::RunQueryDsl;
use flate2::write::GzEncoder;
use flate2::Compression;
use juniper::{graphql_object, FieldError, FieldResult, GraphQLInputObject, GraphQLObject, Value};
use serde_derive::Serialize;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

type Context = kubos_service::Context<Subsystem>;

#[derive(Clone)]
pub struct Subsystem {
    pub database: Arc<Mutex<kubos_telemetry_db::Database>>,
}

impl Subsystem {
    pub fn new(database: kubos_telemetry_db::Database, direct_udp: Option<String>) -> Self {
        let db = Arc::new(Mutex::new(database));

        if let Some(udp_url) = direct_udp {
            let udp = DirectUdp::new(db.clone());
            spawn(move || udp.start(udp_url.to_owned()));
        }

        Subsystem { database: db }
    }
}

// Define our own Entry struct to be compatible with diesel 2.0
#[derive(Debug, Serialize, Queryable, QueryableByName)]
pub struct Entry {
    #[diesel(sql_type = Double)]
    pub timestamp: f64,
    #[diesel(sql_type = Text)]
    pub subsystem: String,
    #[diesel(sql_type = Text)]
    pub parameter: String,
    #[diesel(sql_type = Text)]
    pub value: String,
}

// Updated to use the attribute-based syntax
#[graphql_object(context = ())]
impl Entry {
    /// Timestamp
    fn timestamp(&self) -> f64 {
        self.timestamp
    }

    /// Subsystem name
    fn subsystem(&self) -> &str {
        &self.subsystem
    }

    /// Telemetry parameter
    fn parameter(&self) -> &str {
        &self.parameter
    }

    /// Telemetry value
    fn value(&self) -> &str {
        &self.value
    }
}

fn query_db(
    database: &Arc<Mutex<kubos_telemetry_db::Database>>,
    timestamp_ge: Option<f64>,
    timestamp_le: Option<f64>,
    subsystem: Option<String>,
    parameters: Option<Vec<String>>,
    limit: Option<i32>,
) -> FieldResult<Vec<Entry>> {
    // Build query using raw SQL to avoid Diesel version conflicts
    let mut conditions = Vec::new();

    if let Some(sub) = subsystem {
        conditions.push(format!("subsystem = '{}'", sub.replace('\'', "''")));
    }

    if let Some(params) = parameters {
        if !params.is_empty() {
            let param_list: Vec<String> = params
                .iter()
                .map(|p| format!("'{}'", p.replace('\'', "''")))
                .collect();
            conditions.push(format!("parameter IN ({})", param_list.join(", ")));
        }
    }

    if let Some(time_ge) = timestamp_ge {
        conditions.push(format!("timestamp >= {}", time_ge));
    }

    if let Some(time_le) = timestamp_le {
        conditions.push(format!("timestamp <= {}", time_le));
    }

    let query = if conditions.is_empty() {
        "SELECT timestamp, subsystem, parameter, value FROM telemetry ORDER BY timestamp DESC"
            .to_string()
    } else {
        format!(
            "SELECT timestamp, subsystem, parameter, value FROM telemetry WHERE {} ORDER BY timestamp DESC",
            conditions.join(" AND ")
        )
    };

    let query = if let Some(l) = limit {
        format!("{} LIMIT {}", query, l)
    } else {
        query
    };

    let mut db_lock = database.lock().map_err(|err| {
        log::error!("Failed to get lock on database: {:?}", err);
        err
    })?;

    // Use diesel::sql_query for compatibility
    let entries = diesel::sql_query(query)
        .load::<Entry>(&mut db_lock.connection)
        .map_err(|err| {
            log::error!("Failed to load database entries: {:?}", err);
            err
        })?;

    Ok(entries)
}

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    /// Test service query
    fn ping() -> FieldResult<String> {
        Ok(String::from("pong"))
    }

    /// Telemetry entries in database
    fn telemetry(
        &self,
        context: &Context,
        timestamp_ge: Option<f64>,
        timestamp_le: Option<f64>,
        subsystem: Option<String>,
        parameter: Option<String>,
        parameters: Option<Vec<String>>,
        limit: Option<i32>,
    ) -> FieldResult<Vec<Entry>> {
        if parameter.is_some() && parameters.is_some() {
            return Err(FieldError::new(
                "The `parameter` and `parameters` input fields are mutually exclusive",
                Value::null(),
            ));
        }

        if let Some(param) = parameter {
            query_db(
                &context.subsystem.database,
                timestamp_ge,
                timestamp_le,
                subsystem,
                Some(vec![param]),
                limit,
            )
        } else {
            query_db(
                &context.subsystem.database,
                timestamp_ge,
                timestamp_le,
                subsystem,
                parameters,
                limit,
            )
        }
    }

    /// Telemetry entries in database
    fn routed_telemetry(
        &self,
        context: &Context,
        timestamp_ge: Option<f64>,
        timestamp_le: Option<f64>,
        subsystem: Option<String>,
        parameter: Option<String>,
        parameters: Option<Vec<String>>,
        limit: Option<i32>,
        output: String,
        compress: Option<bool>,
    ) -> FieldResult<String> {
        let compress = compress.unwrap_or(true);

        if parameter.is_some() && parameters.is_some() {
            return Err(FieldError::new(
                "The `parameter` and `parameters` input fields are mutually exclusive",
                Value::null(),
            ));
        }

        let entries = if let Some(param) = parameter {
            query_db(
                &context.subsystem.database,
                timestamp_ge,
                timestamp_le,
                subsystem,
                Some(vec![param]),
                limit,
            )?
        } else {
            query_db(
                &context.subsystem.database,
                timestamp_ge,
                timestamp_le,
                subsystem,
                parameters,
                limit,
            )?
        };

        let entries = serde_json::to_vec(&entries)?;

        let output_str = output.clone();
        let output_path = Path::new(&output_str);

        let file_name_raw = output_path
            .file_name()
            .ok_or_else(|| FieldError::new("Unable to parse output file name", Value::null()))?;
        let file_name = file_name_raw.to_str().ok_or_else(|| {
            FieldError::new("Unable to parse output file name to string", Value::null())
        })?;

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        {
            let mut output_file = File::create(output_path)?;
            output_file.write_all(&entries)?;
        }

        if compress {
            let tar_path = format!("{}.tar.gz", output_str);
            let tar_file = File::create(&tar_path)?;
            let encoder = GzEncoder::new(tar_file, Compression::default());
            let mut tar = tar::Builder::new(encoder);
            tar.append_file(file_name, &mut File::open(output_path)?)?;
            tar.finish()?;

            fs::remove_file(output_path)?;

            Ok(tar_path)
        } else {
            Ok(output)
        }
    }
}

pub struct MutationRoot;

#[derive(GraphQLObject)]
struct InsertResponse {
    success: bool,
    errors: String,
}

#[derive(GraphQLObject)]
struct DeleteResponse {
    success: bool,
    errors: String,
    entries_deleted: Option<i32>,
}

#[derive(GraphQLInputObject)]
struct InsertEntry {
    timestamp: Option<f64>,
    subsystem: String,
    parameter: String,
    value: String,
}

#[graphql_object(context = Context)]
impl MutationRoot {
    fn insert(
        &self,
        context: &Context,
        timestamp: Option<f64>,
        subsystem: String,
        parameter: String,
        value: String,
    ) -> FieldResult<InsertResponse> {
        let result = match timestamp {
            Some(time) => {
                let mut db_lock = context.subsystem.database.lock().map_err(|err| {
                    log::error!("insert - Failed to get lock on database: {:?}", err);
                    err
                })?;
                db_lock.insert(time, &subsystem, &parameter, &value)
            }
            None => {
                let mut db_lock = context.subsystem.database.lock().map_err(|err| {
                    log::error!("insert - Failed to get lock on database: {:?}", err);
                    err
                })?;
                db_lock.insert_systime(&subsystem, &parameter, &value)
            }
        };

        Ok(InsertResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => format!("{}", err),
            },
        })
    }

    fn insert_bulk(
        &self,
        context: &Context,
        timestamp: Option<f64>,
        entries: Vec<InsertEntry>,
    ) -> FieldResult<InsertResponse> {
        let time = time::now_utc().to_timespec();
        let systime = time.sec as f64 + (f64::from(time.nsec) / 1_000_000_000.0);

        let mut new_entries: Vec<kubos_telemetry_db::Entry> = Vec::new();
        for entry in entries {
            let ts = entry.timestamp.or(timestamp).unwrap_or(systime);

            new_entries.push(kubos_telemetry_db::Entry {
                timestamp: ts,
                subsystem: entry.subsystem,
                parameter: entry.parameter,
                value: entry.value,
            });
        }

        let mut db_lock = context.subsystem.database.lock().map_err(|err| {
            log::error!("insert_bulk - Failed to get lock on database: {:?}", err);
            err
        })?;

        let result = db_lock.insert_bulk(new_entries);

        Ok(InsertResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => format!("{}", err),
            },
        })
    }

    fn delete(
        &self,
        context: &Context,
        timestamp_ge: Option<f64>,
        timestamp_le: Option<f64>,
        subsystem: Option<String>,
        parameter: Option<String>,
    ) -> FieldResult<DeleteResponse> {
        // We'll use a direct SQL query for delete to avoid diesel compatibility issues
        let mut conditions = Vec::new();

        if let Some(sub) = subsystem {
            conditions.push(format!("subsystem = '{}'", sub.replace('\'', "''")));
        }

        if let Some(param) = parameter {
            conditions.push(format!("parameter = '{}'", param.replace('\'', "''")));
        }

        if let Some(time_ge) = timestamp_ge {
            conditions.push(format!("timestamp >= {}", time_ge));
        }

        if let Some(time_le) = timestamp_le {
            conditions.push(format!("timestamp <= {}", time_le));
        }

        let mut db_lock = context.subsystem.database.lock().map_err(|err| {
            log::error!("delete - Failed to get lock on database: {:?}", err);
            err
        })?;

        let query = if conditions.is_empty() {
            "DELETE FROM telemetry".to_string()
        } else {
            format!("DELETE FROM telemetry WHERE {}", conditions.join(" AND "))
        };

        let result = diesel::sql_query(query).execute(&mut db_lock.connection);

        match result {
            Ok(num) => Ok(DeleteResponse {
                success: true,
                errors: "".to_owned(),
                entries_deleted: Some(num as i32),
            }),
            Err(err) => Ok(DeleteResponse {
                success: false,
                errors: format!("{}", err),
                entries_deleted: None,
            }),
        }
    }
}
