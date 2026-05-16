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

#![deny(missing_docs)]
#![deny(warnings)]

//! Kubos Service for interacting with a
//! [NovAtel OEM7 High Precision GNSS Receiver](https://novatel.com/products/receivers/oem7-receivers)
//!
//! # Configuration
//!
//! The service can be configured in the `/etc/kubos-config.toml` with the following fields:
//!
//! - `bus` - Specifies the UART bus the OEM7 is connected to
//! - `baud` - Specifies the serial baud rate (default: 9600)
//! - `ip` - Specifies the service's IP address
//! - `port` - Specifies the port on which the service will be listening for UDP packets
//!
//! For example:
//!
//! ```toml
//! [novatel-oem7-service]
//! bus = "/dev/ttyS4"
//! baud = 9600
//!
//! [novatel-oem7-service.addr]
//! ip = "0.0.0.0"
//! port = 8130
//! ```
//!
//! # Starting the Service
//!
//! The service should be started automatically by its init script, but may also be started manually:
//!
//! ```bash
//! $ novatel-oem7-service
//! Kubos OEM7 service started
//! Listening on: 0.0.0.0:8130
//! ```

mod model;
mod objects;
mod schema;

use crate::model::Subsystem;
use crate::schema::{MutationRoot, QueryRoot};
use kubos_service::{Config, Logger, Service};
use log::error;

fn main() {
    Logger::init("novatel-oem7-service").unwrap();

    let config = Config::new("novatel-oem7-service")
        .map_err(|err| {
            error!("Failed to load service config: {:?}", err);
            err
        })
        .unwrap();

    let bus = config
        .get("bus")
        .ok_or_else(|| {
            error!("No 'bus' value found in 'novatel-oem7-service' section of config");
            "No 'bus' value found in 'novatel-oem7-service' section of config"
        })
        .unwrap();
    let bus = bus
        .as_str()
        .ok_or_else(|| {
            error!("Failed to parse 'bus' config value");
            "Failed to parse 'bus' config value"
        })
        .unwrap();

    let baud: u32 = config
        .get("baud")
        .and_then(|v| v.as_integer())
        .map(|v| v as u32)
        .unwrap_or(9600);

    let subsystem = Subsystem::new(bus, baud).unwrap_or_else(|e| {
        error!("Failed to initialize OEM7 subsystem: {}", e);
        panic!("Failed to initialize OEM7 subsystem: {}", e);
    });

    Service::new(config, subsystem, QueryRoot, MutationRoot).start();
}
