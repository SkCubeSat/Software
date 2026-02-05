/*
 * Copyright (C) 2018 Kubos Corporation
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

macro_rules! mock_service {
    ($registry_dir:ident) => {{
        let registry = AppRegistry::new_from_dir(&$registry_dir.path().to_string_lossy()).unwrap();

        let config = format!(
            r#"
            [app-service]
            registry-dir = "{}"
            [app-service.addr]
            ip = "127.0.0.1"
            port = 9999"#,
            $registry_dir.path().to_str().unwrap(),
        );

        Service::new(
            Config::new_from_str("app-service", &config).unwrap(),
            registry,
            schema::QueryRoot,
            schema::MutationRoot,
        )
    }};
}

macro_rules! request {
    ($service:ident, $query:ident) => {{
        // Execute GraphQL query directly on the schema
        let query_str = $query.replace("\n", " ");
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                $service
                    .schema()
                    .execute(&query_str)
                    .await
            });
        let response_json = serde_json::to_vec(&result).unwrap();
        response_json
    }};
}

macro_rules! wrap {
    ($result:ident) => {{
        &json!({ "data": $result }).to_string()
    }};
}

macro_rules! test {
    ($service:ident, $query:ident, $expected:ident) => {{
        let res = request!($service, $query);
        let res_str = String::from_utf8(res).unwrap();
        let expected_str = wrap!($expected);

        assert_eq!(&res_str, expected_str);
    }};
}

mod register_app;
mod registry_start_app;
mod registry_test;
mod set_version;
mod upgrade_app;

use crate::registry::*;
use crate::schema;
use kubos_service::{Config, Service};
use serde_json::json;
use tempfile::TempDir;

#[test]
fn ping() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let query = r#"{ ping }"#;

    let expected = json!({
            "ping": "pong"
    });

    test!(service, query, expected);
}
