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

#![deny(missing_docs)]
#![deny(warnings)]

//! Kubos Service for interacting with the telemetry database.
//!
//! # Configuration
//!
//! The service can be configured in the `/etc/kubos-config.toml` with the following fields:
//!
//! ```
//! [telemetry-service]
//! database = "/var/lib/telemetry.db"
//!
//! [telemetry-service.addr]
//! ip = "127.0.0.1"
//! port = 8020
//! ```
//!
//! Where `database` specifies the path to the telemetry database file, `ip` specifies the
//! service's IP address, and `port` specifies the port on which the service will be
//! listening for UDP packets.
//!
//! # Starting the Service
//!
//! The service should be started automatically by its init script, but may also be started manually:
//!
//! ```
//! $ telemetry-service
//! Listening on: 127.0.0.1:8020
//! ```
//!
//! # Panics
//!
//! Attempts to grab database path from Configuration and will `panic!` if not found.
//! Attempts to connect to database at provided path and will `panic!` if connection fails.
//! Attempts to create telemetry table and will `panic!` if table creation fails.
//!
//! # GraphQL Schema
//!
//! ```graphql
//! type Entry {
//!   timestamp: Integer!
//!   subsystem: String!
//!   parameter: String!
//!   value: Float!
//! }
//!
//! query ping: "pong"
//! query telemetry(timestampGe: Integer, timestampLe: Integer, subsystem: String, parameter: String, parameters: [String]): Entry
//! query routedTelemetry(timestampGe: Integer, timestampLe: Integer, subsystem: String, parameter: String, parameters: [String], output: String!, compress: Boolean = true): String!
//!
//! mutation insert(timestamp: Integer, subsystem: String!, parameter: String!, value: String!):{ success: Boolean!, errors: String! }
//! ```
//!
//! # Example Queries
//!
//! ## Select all attributes of all telemetry entries
//! ```graphql
//! {
//!   telemetry {
//!     timestamp,
//!     subsystem,
//!     parameter,
//!     value
//!   }
//! }
//! ```
//!
//! ## Select all attributes of all telemetry entries for the eps subsystem
//! ```graphql
//! {
//!   telemetry(subsystem: "eps") {
//!     timestamp,
//!     subsystem,
//!     parameter,
//!     value
//!   }
//! }
//! ```
//!
//! ## Select all attributes of all telemetry entries for the voltage parameter of the eps subsystem
//! ```graphql
//! {
//!   telemetry(subsystem: "eps", parameter: "voltage") {
//!     timestamp,
//!     subsystem,
//!     parameter,
//!     value
//!   }
//! }
//! ```
//!
//! ## Select all attributes of all telemetry entries for the voltage and current parameters of the eps subsystem
//! ```graphql
//! {
//!   telemetry(subsystem: "eps", parameters: ["voltage", "current"]) {
//!     timestamp,
//!     subsystem,
//!     parameter,
//!     value
//!   }
//! }
//! ```
//!
//! ## Select all attributes of all telemetry entries occurring between the timestamps 100 and 200
//! ```graphql
//! {
//!   telemetry(timestampGe: 101, timestampLe: 199) {
//!     timestamp,
//!     subsystem,
//!     parameter,
//!     value
//!   }
//! }
//! ```
//!
//! ## Select all attributes of all telemetry entries occurring at the timestamp 101
//! ```graphql
//! {
//!   telemetry(timestampGe: 101, timestampLe: 101) {
//!     timestamp,
//!     subsystem,
//!     parameter,
//!     value
//!   }
//! }
//! ```
//!
//! ## Select ten entries occurring on or after the timestamp 1008
//! ```graphql
//! {
//!   telemetry(limit: 10, timestampGe: 1008) {
//!     timestamp,
//!     subsystem,
//!     parameter,
//!     value
//!   }
//! }
//! ```
//!
//! ## Repeat the previous query, but route the output to compressed file `/home/system/recent_telem.tar.gz`
//! ```graphql
//! {
//!   telemetry(limit: 10, timestampGe: 1008, output: "/home/system/recent_telem")
//! }
//! ```
//!
//! ## Repeat the previous query, but route the output to uncompressed file `/home/system/recent_telem`
//! ```graphql
//! {
//!   telemetry(limit: 10, timestampGe: 1008, output: "/home/system/recent_telem", compress: false)
//! }
//! ```
//!
//! # Example Mutations
//!
//! ## Insert a new entry, allowing the service to generate the timestamp
//! ```graphql
//! mutation {
//!     insert(subsystem: "eps", parameter: "voltage", value: "4.0") {
//!         success,
//!         errors
//!     }
//! }
//! ```
//!
//! ## Insert a new entry with a custom timestamp
//! ```graphql
//! mutation {
//!     insert(timestamp: 533, subsystem: "eps", parameter: "voltage", value: "5.1") {
//!         success,
//!         errors
//!     }
//! }
//!
//! ```
//!
//! ## Delete all entries from the EPS subsystem occuring before timestamp 1003
//! ```graphql
//! mutation {
//!     delete(subsystem: "eps", timestampLe: 1004) {
//!         success,
//!         errors,
//!         entriesDeleted
//!     }
//! }
//! ```

use env_logger;
use kubos_service::Config;
use kubos_service::Service;
use log;

mod schema;
mod udp;

use crate::schema::{MutationRoot, QueryRoot, Subsystem};
use kubos_telemetry_db::Database;

fn main() {
    // Initialize logging
    env_logger::init();

    // Get configuration first so we can read the database path
    let config = Config::new("telemetry-service").unwrap_or_else(|err| {
        log::error!("Failed to load service config: {:?}", err);
        std::process::exit(1);
    });

    // Set up database connection using path from config
    let db_path = config
        .get("database")
        .map(|v| v.as_str().unwrap_or("telemetry.db").to_string())
        .unwrap_or_else(|| "telemetry.db".to_string());
    
    let mut database = Database::new(&db_path);
    database.setup();

    // Determine if we should set up a UDP connection for passively receiving
    // telemetry for insertion
    let direct_udp = config.get("direct_port").and_then(|port| {
        let port_num = port.as_integer()?;
        let host = config.hosturl().unwrap_or_else(|| {
            log::error!("Failed to load service URL");
            std::process::exit(1);
        });
        let mut host_parts = host.split(':').map(|val| val.to_owned());
        let host_ip = host_parts.next().unwrap_or_else(|| {
            log::error!("Failed to parse service IP address");
            std::process::exit(1);
        });

        Some(format!("{}:{}", host_ip, port_num))
    });

    // Create and start the service
    Service::new(
        config,
        Subsystem::new(database, direct_udp),
        QueryRoot,
        MutationRoot,
    )
    .start();
}
