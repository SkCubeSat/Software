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

//! Service for interacting with the Analog IC payload board.
//!
//! The Analog IC board is a payload on the RADSAT-SK2 satellite that tests
//! radiation effects on integrated circuits (ICs). It communicates with the
//! OBC over I2C at address 0x13.
//!
//! # Configuration
//!
//! The service can be configured in `/etc/kubos-config.toml` with the following fields:
//!
//! ```toml
//! [analog-ic-service]
//! bus = "/dev/i2c-1"
//!
//! [analog-ic-service.addr]
//! ip = "127.0.0.1"
//! port = 8002
//! ```
//!
//! Where `bus` specifies the I2C bus the payload is on, `ip` specifies the
//! service's IP address, and `port` specifies the port on which the service
//! will be listening for GraphQL requests.
//!
//! # Running the Service
//!
//! The service should be started automatically by its init script, but may
//! also be started manually:
//!
//! ```bash
//! $ analog-ic-service
//! Listening on: 127.0.0.1:8002
//! ```
//!
//! An alternative config file may be specified:
//!
//! ```bash
//! $ analog-ic-service -c config.toml
//! ```
//!
//! # Panics
//!
//! Attempts to grab `bus` from Configuration and will `panic!` if not found.
//!
//! # GraphQL Schema
//!
//! ## Queries
//!
//! ### Ping
//!
//! Test query to verify service is running.
//!
//! ```json
//! {
//!     ping: "pong"
//! }
//! ```
//!
//! ### ACK
//!
//! Fetch the last mutation which was executed by the service.
//!
//! ```json
//! {
//!     ack: Mutation!
//! }
//! ```
//!
//! ### Errors
//!
//! Fetch all errors encountered by the service since the last time queried.
//!
//! ```json
//! {
//!     errors: [String]
//! }
//! ```
//!
//! ### Telemetry
//!
//! Retrieve the latest payload data (IC readings and timestamp).
//!
//! ```json
//! {
//!     telemetry {
//!         icReadings: [Int!]!
//!         timestampBytes: [Int!]!
//!         rawData: [Int!]!
//!     }
//! }
//! ```
//!
//! ## Mutations
//!
//! ### No-Op
//!
//! ```json
//! mutation {
//!     noop {
//!         success: Boolean!
//!         errors: String!
//!     }
//! }
//! ```
//!
//! ### Reset
//!
//! Reset the Analog IC board.
//!
//! ```json
//! mutation {
//!     reset {
//!         success: Boolean!
//!         errors: String!
//!     }
//! }
//! ```
//!
//! ### Start
//!
//! Force the IC testing loop to run.
//!
//! ```json
//! mutation {
//!     start {
//!         success: Boolean!
//!         errors: String!
//!     }
//! }
//! ```
//!
//! ### Power Saving Mode
//!
//! Enter low-power sleep mode (~50 mW).
//!
//! ```json
//! mutation {
//!     powerSavingMode {
//!         success: Boolean!
//!         errors: String!
//!     }
//! }
//! ```
//!
//! ### Normal Power Mode
//!
//! Resume normal operation (tests every 12 hours).
//!
//! ```json
//! mutation {
//!     normalPowerMode {
//!         success: Boolean!
//!         errors: String!
//!     }
//! }
//! ```
//!
//! ### Set RTC Time
//!
//! Set the board's RTC using OBC system time.
//!
//! ```json
//! mutation {
//!     setRtcTime {
//!         success: Boolean!
//!         errors: String!
//!     }
//! }
//! ```
//!
//! ### Issue Raw Command
//!
//! Pass a custom command through to the board.
//!
//! ```json
//! mutation {
//!     issueRawCommand(command: Int!, data: [Int!]) {
//!         success: Boolean!
//!         errors: String!
//!     }
//! }
//! ```

#![deny(missing_docs)]
#![deny(warnings)]

extern crate kubos_service;

pub mod models;
pub mod schema;

use crate::models::subsystem::Subsystem;
use crate::schema::mutation::MutationRoot;
use crate::schema::query::QueryRoot;
use kubos_service::{Config, Logger, Service};
use log::error;

fn main() {
    Logger::init("analog-ic-service").unwrap();

    let config = Config::new("analog-ic-service")
        .map_err(|err| {
            error!("Failed to load service config: {:?}", err);
            err
        })
        .unwrap();
    println!("Loaded config: {:?}", config);

    let bus = config
        .get("bus")
        .ok_or_else(|| {
            error!("Failed to load 'bus' config value");
            "Failed to load 'bus' config value"
        })
        .unwrap();
    let bus = bus
        .as_str()
        .ok_or_else(|| {
            error!("Failed to parse 'bus' config value");
            "Failed to parse 'bus' config value"
        })
        .unwrap();

    let subsystem: Box<Subsystem> = Box::new(
        Subsystem::from_path(bus)
            .map_err(|err| {
                error!("Failed to create subsystem: {:?}", err);
                err
            })
            .unwrap(),
    );

    Service::new(config, subsystem, QueryRoot, MutationRoot).start();
}
