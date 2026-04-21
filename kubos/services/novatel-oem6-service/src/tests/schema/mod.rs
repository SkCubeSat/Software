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

use self::test_data::*;
use super::*;

use crate::model::*;
use crate::schema::*;
use async_graphql::Request;
use serde_json::json;
use std::sync::mpsc::sync_channel;

/// Send a GraphQL request to an in-process schema and return the Response.
macro_rules! request {
    ($schema:expr, $query:expr) => {{
        let request = Request::new($query.replace("\n", ""));
        futures::executor::block_on($schema.execute(request))
    }};
}

/// Extract the data body of a Response as a serde_json::Value.
macro_rules! response_data {
    ($res:expr) => {{
        serde_json::from_str::<serde_json::Value>(&$res.data.into_json().unwrap().to_string())
            .unwrap()
    }};
}

/// Assert that a GraphQL query returns the expected JSON data.
macro_rules! test {
    ($schema:ident, $query:ident, $expected:ident) => {{
        let res = request!($schema, $query);
        assert!(res.errors.is_empty(), "GraphQL errors: {:?}", res.errors);
        let data = response_data!(res);
        assert_eq!(data, $expected);
    }};
}

mod mutations;
mod queries;
mod test_data;

#[test]
fn ping() {
    let mut mock = MockStream::default();

    let schema = service_new!(mock);

    let query = r#"{
            ping
        }"#;

    let expected = json!({
            "ping": "pong"
    });

    test!(schema, query, expected);
}
