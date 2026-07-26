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
// Contributed by: William Greer (wgreer184@gmail.com) and Sam Justice (sam.justice1@gmail.com)
//

use crate::errors::*;
use std::sync::{Arc, Mutex};

const MAX_ERROR_ENTRIES: usize = 100;

/// Generic telemetry collected by the communication service.
#[derive(Default, GraphQLObject)]
pub struct CommsTelemetry {
    /// Errors that have occured within the communication service.
    pub errors: Vec<String>,
    /// Number of bad uplink packets.
    pub failed_packets_up: i32,
    /// Number of bad downlink packets.
    pub failed_packets_down: i32,
    /// Number of packets successfully uplinked.
    pub packets_up: i32,
    /// Number of packets successfully downlinked.
    pub packets_down: i32,
}

/// Enum used to differentiate types of telemetry collected by the communication service.
pub enum TelemType {
    /// Packets down
    Down,
    /// Packets down that failed
    DownFailed,
    /// Packets up
    Up,
    /// Packets up that failed
    UpFailed,
}

// Function used to obtain a mutex lock and update communication service errors.
pub fn log_error(data: &Arc<Mutex<CommsTelemetry>>, error: String) -> CommsResult<()> {
    let mut telem = data.lock().unwrap_or_else(|err| err.into_inner());
    telem.errors.push(error);
    let excess = telem.errors.len().saturating_sub(MAX_ERROR_ENTRIES);
    if excess > 0 {
        telem.errors.drain(0..excess);
    }

    Ok(())
}

// Function used to obtain a mutex lock and update communcation service telemetry.
pub fn log_telemetry(data: &Arc<Mutex<CommsTelemetry>>, telem_type: &TelemType) -> CommsResult<()> {
    let mut telem = data.lock().unwrap_or_else(|err| err.into_inner());
    match telem_type {
        TelemType::Down => telem.packets_down += 1,
        TelemType::DownFailed => telem.failed_packets_down += 1,
        TelemType::Up => telem.packets_up += 1,
        TelemType::UpFailed => telem.failed_packets_up += 1,
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn log_error_keeps_most_recent_errors() {
        let data = Arc::new(Mutex::new(CommsTelemetry::default()));

        for index in 0..150 {
            log_error(&data, format!("error-{index}")).unwrap();
        }

        let telemetry = data.lock().unwrap();
        assert_eq!(telemetry.errors.len(), MAX_ERROR_ENTRIES);
        assert_eq!(telemetry.errors.first().unwrap(), "error-50");
        assert_eq!(telemetry.errors.last().unwrap(), "error-149");
    }

    #[test]
    fn telemetry_logs_recover_from_poisoned_mutex() {
        let data = Arc::new(Mutex::new(CommsTelemetry::default()));
        let poisoned = data.clone();
        let _ = thread::spawn(move || {
            let _guard = poisoned.lock().unwrap();
            panic!("poison telemetry mutex");
        })
        .join();

        assert!(log_error(&data, "after poison".to_string()).is_ok());
        assert!(log_telemetry(&data, &TelemType::Up).is_ok());

        let telemetry = data.lock().unwrap_or_else(|err| err.into_inner());
        assert_eq!(telemetry.errors, vec!["after poison".to_string()]);
        assert_eq!(telemetry.packets_up, 1);
    }
}
