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

use log::info;
use serde_json::json;
use std::time::Duration;
use util::{BasicAppResponder, SchedulerFixture};
use utils::testing::ServiceListener;

#[tokio::test]
async fn import_raw_tasks() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8020);

    fixture.create_mode("operational").await;

    // Create some schedule with an init task
    let schedule: String = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    })
    .to_string()
    .escape_default()
    .collect();
    assert_eq!(
        fixture.import_raw_task_list("first", "operational", &schedule).await,
        json!({
            "data" : {
                "importRawTaskList": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active, schedule { filename } } }"#).await,
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false,
                        "schedule": [
                            {
                                "filename": "first"
                            }
                        ]
                    },
                    {
                        "name": "safe",
                        "active": true,
                        "schedule": [ ]
                    }
                ]
            }
        })
    );
}

#[tokio::test]
async fn import_raw_run_delay() {
    let listener = ServiceListener::spawn("127.0.0.1", 9021);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    fixture.create_mode("init").await;

    // Create some schedule with an init task
    let schedule: String = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    })
    .to_string()
    .escape_default()
    .collect();
    fixture.import_raw_task_list("imaging", "init", &schedule).await;
    fixture.activate_mode("init").await;

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task actually ran - use polling with timeout for CI reliability
    let request = listener.expect_request(Duration::from_secs(5), "startApp mutation (basic-app)");
    assert_eq!(request, query.to_owned())
}

#[tokio::test]
async fn import_raw_run_two_tasks() {
    let listener = ServiceListener::spawn("127.0.0.1", 9022);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);
    fixture.create_mode("init").await;

    // Create some schedule with an init task
    let schedule: String = json!({
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
    })
    .to_string()
    .escape_default()
    .collect();
    fixture.import_raw_task_list("two", "init", &schedule).await;
    fixture.activate_mode("init").await;

    // Check if first task ran - use polling with timeout for CI reliability
    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;
    let request = listener.expect_request(Duration::from_secs(5), "first startApp mutation (basic-app)");
    assert_eq!(request, query.to_owned());

    // Check if second app ran in order - it has a 1s delay
    let query = r#"{"query":"mutation { startApp(name: \"other-app\") { success, errors } }"}"#;
    let request = listener.expect_request(Duration::from_secs(5), "second startApp mutation (other-app)");
    assert_eq!(request, query.to_owned());
}

#[tokio::test]
async fn import_raw_run_onetime_future() {
    utils::init_logger();

    let listener = ServiceListener::spawn_with_responder("127.0.0.1", 9023, BasicAppResponder);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8023);

    fixture.create_mode("init").await;
    fixture.activate_mode("init").await;

    // Create some schedule with a task starting now
    let schedule: String = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "2s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    })
    .to_string()
    .escape_default()
    .collect();

    let res = fixture.import_raw_task_list("imaging", "init", &schedule).await;
    info!("import result: {}", res);

    // Task has 2s delay, so shouldn't run immediately
    assert_eq!(listener.wait_for_request(Duration::from_millis(500), None), None);

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task actually ran after delay - use polling with timeout for CI reliability
    let request = listener.expect_request(Duration::from_secs(5), "startApp mutation (basic-app) after delay");
    assert_eq!(request, query.to_owned())
}

#[tokio::test]
async fn import_raw_run_recurring_no_delay() {
    let listener = ServiceListener::spawn_with_responder("127.0.0.1", 9024, BasicAppResponder);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8024);

    fixture.create_mode("init").await;

    // Create some schedule with a recurring task starting now
    let schedule: String = json!({
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
    })
    .to_string()
    .escape_default()
    .collect();
    fixture.import_raw_task_list("imaging", "init", &schedule).await;
    fixture.activate_mode("init").await;

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task was run at least twice - use polling with timeout for CI reliability
    let request = listener.expect_request(Duration::from_secs(5), "first recurring startApp mutation");
    assert_eq!(request, query.to_owned());
    let request = listener.expect_request(Duration::from_secs(5), "second recurring startApp mutation");
    assert_eq!(request, query.to_owned());
}

#[tokio::test]
async fn import_raw_bad_json() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8025);

    fixture.create_mode("operational").await;

    // Create some schedule with an init task
    let schedule = "this is not json";
    assert_eq!(
        fixture.import_raw_task_list("first", "operational", schedule).await,
        json!({
            "data" : {
                "importRawTaskList": {
                    "errors": "Failed to parse task list 'first': Failed to parse json: expected ident at line 1 column 2",
                    "success": false
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active, schedule { filename } } }"#).await,
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false,
                        "schedule": [ ]
                    },
                    {
                        "name": "safe",
                        "active": true,
                        "schedule": [ ]
                    }
                ]
            }
        })
    );
}

#[tokio::test]
async fn import_raw_run_delay_duplicate() {
    let listener = ServiceListener::spawn("127.0.0.1", 9026);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8026);

    fixture.create_mode("init").await;

    // Create some schedule with an init task
    let schedule: String = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    })
    .to_string()
    .escape_default()
    .collect();
    fixture.import_raw_task_list("imaging", "init", &schedule).await;
    fixture.activate_mode("init").await;

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task actually ran - use polling with timeout for CI reliability
    let request = listener.expect_request(Duration::from_secs(5), "first startApp mutation (basic-app)");
    assert_eq!(request, query.to_owned());

    fixture.import_raw_task_list("imaging", "init", &schedule).await;

    // Check if the task actually ran again - use polling with timeout for CI reliability
    let request = listener.expect_request(Duration::from_secs(5), "second startApp mutation (basic-app) after re-import");
    assert_eq!(request, query.to_owned())
}
