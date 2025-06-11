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
use util::SchedulerFixture;

#[tokio::test]
async fn create_new_mode() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8020);

    assert_eq!(
        fixture.create_mode("operational").await,
        json!({
            "data" : {
                "createMode": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active } }"#).await,
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false
                    },
                    {
                        "name": "safe",
                        "active": true,
                    }
                ]
            }
        })
    );
}

#[tokio::test]
async fn create_duplicate_mode() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    assert_eq!(
        fixture.create_mode("operational").await,
        json!({
            "data" : {
                "createMode": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.create_mode("operational").await,
        json!({
            "data" : {
                "createMode": {
                    "errors": "Failed to create 'operational': File exists (os error 17)",
                    "success": false
                }
            }
        })
    );
}

#[tokio::test]
async fn create_two_modes() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

    assert_eq!(
        fixture.create_mode("operational").await,
        json!({
            "data" : {
                "createMode": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.create_mode("low_power").await,
        json!({
            "data" : {
                "createMode": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active } }"#).await,
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "low_power",
                        "active": false
                    },
                    {
                        "name": "operational",
                        "active": false
                    },
                    {
                        "name": "safe",
                        "active": true
                    }
                ]
            }
        })
    );
}

#[tokio::test]
async fn create_modes_name_filter() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8023);

    assert_eq!(
        fixture.create_mode("operational").await,
        json!({
            "data" : {
                "createMode": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.create_mode("low_power").await,
        json!({
            "data" : {
                "createMode": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes(name: "operational") { name, active } }"#).await,
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false
                    }
                ]
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes(name: "low_power") { name, active } }"#).await,
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "low_power",
                        "active": false
                    }
                ]
            }
        })
    );
}
