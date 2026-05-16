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

use serde_json::json;
use std::time::Duration;
use util::SchedulerFixture;
use utils::testing::ServiceListener;

#[tokio::test]
async fn run_recurring_no_delay() {
    let listener = ServiceListener::spawn("127.0.0.1", 9021);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    fixture.create_mode("init").await;

    // Create some schedule with a recurring task starting now
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "period": "1s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("imaging", &schedule_path, "init").await;
    fixture.activate_mode("init").await;

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task was run at least twice - use polling with timeout for CI reliability
    let request = listener.expect_request(Duration::from_secs(5), "first recurring startApp mutation");
    assert_eq!(request, query.to_owned());
    let request = listener.expect_request(Duration::from_secs(5), "second recurring startApp mutation");
    assert_eq!(request, query.to_owned());
    
    // Verify no additional requests immediately available (no third run yet)
    // Use short timeout since we just want to verify there's no immediate third request
    assert_eq!(listener.wait_for_request(Duration::from_millis(100), None), None)
}

#[tokio::test]
async fn run_recurring_delay() {
    let listener = ServiceListener::spawn("127.0.0.1", 9022);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

    fixture.create_mode("init").await;

    // Create some schedule with a recurring task starting now
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "1s",
                "period": "1s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("imaging", &schedule_path, "init").await;
    fixture.activate_mode("init").await;

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task was run at least twice - use polling with timeout for CI reliability
    // First run starts after 1s delay, then repeats every 1s
    let request = listener.expect_request(Duration::from_secs(5), "first recurring startApp mutation after delay");
    assert_eq!(request, query.to_owned());
    let request = listener.expect_request(Duration::from_secs(5), "second recurring startApp mutation");
    assert_eq!(request, query.to_owned());
    
    // Verify no additional requests immediately available (no third run yet)
    assert_eq!(listener.wait_for_request(Duration::from_millis(100), None), None)
}
