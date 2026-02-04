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
use std::thread;
use std::time::Duration;
use util::SchedulerFixture;
use utils::testing::ServiceListener;

#[test]
fn run_init_no_tasks() {
    let listener = ServiceListener::spawn("127.0.0.1", 9020);
    let _fixture = SchedulerFixture::spawn("127.0.0.1", 8020);

    // With no tasks registered, no requests should be received
    assert_eq!(listener.wait_for_request(Duration::from_millis(500), None), None);
}

#[tokio::test]
async fn run_init_single_no_delay() {
    let listener = ServiceListener::spawn("127.0.0.1", 9021);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    fixture.create_mode("init").await;

    // Create some schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
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

    // Check if the task actually ran - use polling with timeout for CI reliability
    let request = listener.expect_request(Duration::from_secs(5), "startApp mutation (basic-app)");
    assert_eq!(request, query.to_owned())
}

#[tokio::test]
async fn run_init_single_with_delay() {
    let listener = ServiceListener::spawn("127.0.0.1", 9022);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);
    fixture.create_mode("init").await;

    // Create some schedule with an init task
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

    // This task should not have run immediately - wait a short time and verify no request
    assert_eq!(listener.wait_for_request(Duration::from_millis(500), None), None);

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task actually ran after the delay - use polling with timeout for CI reliability
    let request = listener.expect_request(Duration::from_secs(5), "startApp mutation (basic-app) after delay");
    assert_eq!(request, query.to_owned())
}

#[tokio::test]
async fn run_init_two_tasks() {
    let listener = ServiceListener::spawn("127.0.0.1", 9023);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8023);
    fixture.create_mode("init").await;

    // Create some schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "description": "second-task",
                "delay": "1s",
                "app": {
                    "name": "other-app"
                }
            },
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("two", &schedule_path, "init").await;
    fixture.activate_mode("init").await;

    // Check if first task ran - use polling with timeout for CI reliability
    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;
    let request = listener.expect_request(Duration::from_secs(5), "first startApp mutation (basic-app)");
    assert_eq!(request, query.to_owned());

    // Check if second app ran in order - it has a 1s delay, so allow extra time
    let query = r#"{"query":"mutation { startApp(name: \"other-app\") { success, errors } }"}"#;
    let request = listener.expect_request(Duration::from_secs(5), "second startApp mutation (other-app)");
    assert_eq!(request, query.to_owned());
}

#[tokio::test]
async fn run_init_single_args() {
    let listener = ServiceListener::spawn("127.0.0.1", 9024);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8024);
    fixture.create_mode("init").await;

    // Create some schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "basic-app",
                    "args": ["-l", "-h"]
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("imaging", &schedule_path, "init").await;
    fixture.activate_mode("init").await;

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\", args: [\"-l\",\"-h\"]) { success, errors } }"}"#;

    // Check if the task actually ran - use polling with timeout for CI reliability
    let request = listener.expect_request(Duration::from_secs(5), "startApp mutation with args");
    assert_eq!(request, query.to_owned())
}

#[tokio::test]
async fn run_init_single_custom_task_list() {
    let listener = ServiceListener::spawn("127.0.0.1", 9025);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8025);
    fixture.create_mode("init").await;

    // Create some schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "basic-app",
                    "config": "path/to/custom.toml"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("imaging", &schedule_path, "init").await;
    fixture.activate_mode("init").await;

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\", config: \"path/to/custom.toml\") { success, errors } }"}"#;

    // Check if the task actually ran - use polling with timeout for CI reliability
    let request = listener.expect_request(Duration::from_secs(5), "startApp mutation with custom config");
    assert_eq!(request, query.to_owned())
}

#[tokio::test]
async fn run_init_two_schedules_one_mode() {
    let listener = ServiceListener::spawn("127.0.0.1", 9027);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8027);
    fixture.create_mode("init").await;

    // Create first schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "first-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("first", &schedule_path, "init").await;

    // Create second schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "1s",
                "app": {
                    "name": "second-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("second", &schedule_path, "init").await;

    // Activate first schedule
    fixture.activate_mode("init").await;

    // Check if the first task ran - use polling with timeout for CI reliability
    let query = r#"{"query":"mutation { startApp(name: \"first-app\") { success, errors } }"}"#;
    let request = listener.expect_request(Duration::from_secs(5), "startApp mutation (first-app)");
    assert_eq!(request, query.to_owned());

    // Check if the second task ran - it has 1s delay, so allow extra time
    let query = r#"{"query":"mutation { startApp(name: \"second-app\") { success, errors } }"}"#;
    let request = listener.expect_request(Duration::from_secs(5), "startApp mutation (second-app)");
    assert_eq!(request, query.to_owned())
}

#[tokio::test]
async fn run_init_two_modes() {
    let listener = ServiceListener::spawn("127.0.0.1", 9028);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8028);
    fixture.create_mode("init").await;
    fixture.create_mode("secondary").await;

    // Create first schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "first-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("first", &schedule_path, "init").await;

    // Create second schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "second-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("second", &schedule_path, "secondary").await;

    // Activate first schedule
    fixture.activate_mode("init").await;

    // Check if the task ran - use polling with timeout for CI reliability
    let query = r#"{"query":"mutation { startApp(name: \"first-app\") { success, errors } }"}"#;
    let request = listener.expect_request(Duration::from_secs(5), "startApp mutation (first-app)");
    assert_eq!(request, query.to_owned());

    // Activate second schedule
    fixture.activate_mode("secondary").await;

    // Check if the task ran - use polling with timeout for CI reliability
    let query = r#"{"query":"mutation { startApp(name: \"second-app\") { success, errors } }"}"#;
    let request = listener.expect_request(Duration::from_secs(5), "startApp mutation (second-app)");
    assert_eq!(request, query.to_owned())
}

#[tokio::test]
async fn run_init_two_modes_check_stop() {
    let listener = ServiceListener::spawn("127.0.0.1", 9029);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8029);
    fixture.create_mode("init").await;
    fixture.create_mode("secondary").await;

    // Register first schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "1s",
                "app": {
                    "name": "delay-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("first", &schedule_path, "init").await;

    // Register second schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "second-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("second", &schedule_path, "secondary").await;

    // Activate first schedule (with delayed task)
    fixture.activate_mode("init").await;

    // Quickly activate second schedule before the first task runs
    // (first task has 1s delay, so we need to switch modes before that)
    thread::sleep(Duration::from_millis(100));
    fixture.activate_mode("secondary").await;

    // Check if the second task ran - use polling with timeout for CI reliability
    let query = r#"{"query":"mutation { startApp(name: \"second-app\") { success, errors } }"}"#;
    let request = listener.expect_request(Duration::from_secs(5), "startApp mutation (second-app)");
    assert_eq!(request, query.to_owned());

    // Give the scheduler time to run (or not) delayed task from first schedule
    // The first schedule's task should NOT run because we switched modes
    assert_eq!(listener.wait_for_request(Duration::from_secs(2), None), None)
}

#[tokio::test]
async fn run_init_after_import() {
    let listener = ServiceListener::spawn("127.0.0.1", 9030);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8030);
    fixture.create_mode("init").await;

    // Register first schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "first-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));

    // Activate mode
    fixture.activate_mode("init").await;

    // Mode is empty, so no task should have run
    assert_eq!(listener.wait_for_request(Duration::from_millis(500), None), None);

    // Import task_list then confirm task is run afterwards
    fixture.import_task_list("first", &schedule_path, "init").await;

    // Check if the task ran - use polling with timeout for CI reliability
    let query = r#"{"query":"mutation { startApp(name: \"first-app\") { success, errors } }"}"#;
    let request = listener.expect_request(Duration::from_secs(5), "startApp mutation (first-app) after import");
    assert_eq!(request, query.to_owned());
}

#[tokio::test]
async fn run_init_check_remove() {
    let listener = ServiceListener::spawn("127.0.0.1", 9031);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8031);
    fixture.create_mode("init").await;

    // Register task_list with a delayed task
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "3s",
                "app": {
                    "name": "delay-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("first", &schedule_path, "init").await;

    // Activate mode
    fixture.activate_mode("init").await;

    // Task is delayed so it shouldn't have run yet
    assert_eq!(listener.wait_for_request(Duration::from_millis(500), None), None);

    // Remove the task_list before the delayed task runs
    fixture.remove_task_list("first", "init").await;

    // Verify task did not run even after the delay would have passed
    assert_eq!(listener.wait_for_request(Duration::from_secs(4), None), None);
}

#[tokio::test]
async fn run_init_import_twice() {
    let listener = ServiceListener::spawn("127.0.0.1", 9032);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8032);

    fixture.create_mode("init").await;

    // Create some schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
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

    // Check if the task actually ran - use polling with timeout for CI reliability
    let request = listener.expect_request(Duration::from_secs(5), "first startApp mutation (basic-app)");
    assert_eq!(request, query.to_owned());

    // Import task list again and verify task ran again
    fixture.import_task_list("imaging", &schedule_path, "init").await;

    // Check if the task actually ran again - use polling with timeout for CI reliability
    let request = listener.expect_request(Duration::from_secs(5), "second startApp mutation (basic-app) after re-import");
    assert_eq!(request, query.to_owned())
}
