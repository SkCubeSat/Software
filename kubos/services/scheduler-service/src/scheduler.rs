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
//! Structures and functions concerning the actual running of a schedule
//!

use crate::error::SchedulerError;
use crate::mode::{
    activate_mode, create_mode, get_active_mode, get_available_modes, is_mode_active,
};
use crate::task_list::{get_mode_task_lists, validate_task_list, TaskList};
use log::{error, info, warn};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub static DEFAULT_SCHEDULES_DIR: &str = "/home/system/etc/schedules";
pub static SAFE_MODE: &str = "safe";

// Handle to primitives controlling scheduler runtime context
#[derive(Clone)]
pub struct SchedulerHandle {
    // Sender for stopping scheduler runtime/thread
    pub stopper: Sender<()>,
}

#[derive(Clone)]
pub struct Scheduler {
    // Path to directory where schedules/modes are stored
    pub scheduler_dir: String,
    // URL of App Service - for start app queries
    app_service_url: String,
    // Map of active task list names and scheduler handles. This allows us to
    // start/stop tasks associated with individual task lists
    scheduler_map: Arc<Mutex<HashMap<String, SchedulerHandle>>>,
}

impl Scheduler {
    // Create new Scheduler
    pub fn new(sched_dir: &str, app_service_url: &str) -> Result<Scheduler, SchedulerError> {
        // Convert sched_dir to an absolute path
        let sched_dir_path = Path::new(sched_dir);
        let scheduler_dir = if sched_dir_path.is_relative() {
            let cwd = env::current_dir().map_err(|e| SchedulerError::GenericError {
                err: format!("Current working directory invalid: {}", e),
            })?;
            let sched_path = cwd.join(sched_dir_path);

            sched_path
                .to_str()
                .ok_or_else(|| SchedulerError::GenericError {
                    err: format!(
                        "Failed to create absolute schedules_dir path: {:?}",
                        sched_path
                    ),
                })?
                .to_owned()
        } else {
            sched_dir.to_owned()
        };

        Ok(Scheduler {
            scheduler_dir,
            scheduler_map: Arc::new(Mutex::new(HashMap::<String, SchedulerHandle>::new())),
            app_service_url: app_service_url.to_owned(),
        })
    }

    // Ensure that conditions are good for starting the scheduler
    pub fn init(&self) -> Result<(), SchedulerError> {
        if !Path::new(&self.scheduler_dir).is_dir() {
            if let Err(e) = fs::create_dir(&self.scheduler_dir) {
                return Err(SchedulerError::CreateError {
                    err: e.to_string(),
                    path: self.scheduler_dir.to_owned(),
                });
            }
        }

        match get_active_mode(&self.scheduler_dir) {
            // If we get some directory and no error, then do nothing
            Ok(Some(_)) => {}
            // Otherwise if we got an error OR if we found no active directory
            // then attempt to create and/or activate safe mode
            _ => {
                match get_available_modes(&self.scheduler_dir, Some(SAFE_MODE.to_owned())) {
                    // If this list isn't empty then we know safe mode exists
                    Ok(ref list) if !list.is_empty() => {}
                    // If the list is empty OR there was any sort of error retrieving it
                    // then attempt to create the safe mode
                    _ => {
                        create_mode(&self.scheduler_dir, SAFE_MODE)?;
                    }
                }
                activate_mode(&self.scheduler_dir, SAFE_MODE)?;
            }
        }
        Ok(())
    }

    // Checks if task list is in active mode and schedules tasks if needed
    pub fn check_start_task_list(
        &self,
        raw_name: &str,
        raw_mode: &str,
    ) -> Result<(), SchedulerError> {
        let name = raw_name.to_lowercase();
        let mode = raw_mode.to_lowercase();

        if is_mode_active(&self.scheduler_dir, &mode) {
            let list_path = format!("{}/{}/{}.json", self.scheduler_dir, mode, name);
            let list_path = Path::new(&list_path);
            let list = TaskList::from_path(list_path)?;

            Ok(self.start_task_list(list)?)
        } else {
            Ok(())
        }
    }

    // Schedules tasks associated with task list
    fn start_task_list(&self, list: TaskList) -> Result<(), SchedulerError> {
        let mut schedules_map = self.scheduler_map.lock().unwrap();
        let scheduler_handle = list.schedule_tasks(&self.app_service_url)?;
        schedules_map.insert(list.filename, scheduler_handle);
        Ok(())
    }

    // Iterate through the active mode and kick off scheduling tasks
    // Validation and error returning is done here and caught in
    // start() for fail over.
    fn check_start(&self, mode_path: &str) -> Result<(), SchedulerError> {
        for list in get_mode_task_lists(mode_path)? {
            match validate_task_list(&list.path) {
                Err(SchedulerError::TaskTimeError { description, .. }) => warn!(
                    "Found task '{}' in task list '{}' with out of bounds time",
                    description, list.filename
                ),
                Ok(()) => {}
                Err(e) => return Err(e),
            }
            self.start_task_list(list)?;
        }
        Ok(())
    }

    // Iterate through the active mode and kick off scheduling tasks
    pub fn start(&self) -> Result<(), SchedulerError> {
        if let Some(active_mode) = get_active_mode(&self.scheduler_dir)? {
            if let Err(err) = self.check_start(&active_mode.path) {
                if active_mode.name == SAFE_MODE {
                    error!("Failed to start safe mode: {}", err);
                    panic!("Failed to start safe mode: {}", err);
                } else {
                    error!(
                        "Failed to start mode '{}', failing over: {}",
                        active_mode.name, err
                    );
                    activate_mode(&self.scheduler_dir, SAFE_MODE)?;
                    self.start()?;
                }
            }
            Ok(())
        } else {
            error!("Failed to find an active mode");
            panic!("Failed to find an active mode");
        }
    }

    // Stops all running tasks and clears of list of scheduler handles
    pub fn stop(&self) -> Result<(), SchedulerError> {
        let mut schedules_map = self.scheduler_map.lock().unwrap();
        for (name, handle) in schedules_map.drain().take(1) {
            info!("Stopping {}'s tasks", name);
            if let Err(e) = handle.stopper.send(()) {
                error!("Failed to send stop to {}'s tasks: {}", name, e);
            }
        }
        Ok(())
    }

    // Checks if a task list exists in an active mode and stops its scheduler if needed
    pub fn check_stop_task_list(
        &self,
        raw_name: &str,
        raw_mode: &str,
    ) -> Result<(), SchedulerError> {
        let name = raw_name.to_lowercase();
        let mode = raw_mode.to_lowercase();

        if is_mode_active(&self.scheduler_dir, &mode) {
            let mut schedules_map = self.scheduler_map.lock().unwrap();
            if let Some(handle) = schedules_map.remove(&name) {
                info!("Stopping {}'s tasks", name);
                if let Err(e) = handle.stopper.send(()) {
                    error!("Failed to send stop to {}'s tasks: {}", name, e);
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }
}
