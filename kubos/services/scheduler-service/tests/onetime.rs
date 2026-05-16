//
// Copyright (C) 2019 Kubos Corporation
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

mod util;

use chrono::prelude::*;
use chrono::Utc;
use serde_json::json;
use std::time::Duration;
use util::SchedulerFixture;
use utils::testing::ServiceListener;

#[tokio::test]
async fn run_onetime_future() {
    let listener = ServiceListener::spawn("127.0.0.1", 9021);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    fixture.create_mode("init").await;

    // Create some schedule with a task starting now
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "1s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("imaging", &schedule_path, "init").await;
    fixture.activate_mode("init").await;

    // Task has 1s delay, shouldn't run immediately
    assert_eq!(listener.wait_for_request(Duration::from_millis(500), None), None);

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task actually ran after delay - use polling with timeout for CI reliability
    let request = listener.expect_request(Duration::from_secs(5), "startApp mutation (basic-app) after delay");
    assert_eq!(request, query.to_owned())
}

#[tokio::test]
async fn run_onetime_past() {
    let listener = ServiceListener::spawn("127.0.0.1", 9022);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

    fixture.create_mode("init").await;

    let now_time: DateTime<Utc> = Utc::now()
        .checked_sub_signed(chrono::Duration::seconds(100))
        .unwrap();
    let now_time = now_time.format("%Y-%m-%d %H:%M:%S").to_string();

    // Create some schedule with a task starting now
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "time": now_time,
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("imaging", &schedule_path, "init").await;
    fixture.activate_mode("init").await;

    // Task is in the past, so it should not run - wait and verify no request
    assert_eq!(listener.wait_for_request(Duration::from_secs(2), None), None);
}
