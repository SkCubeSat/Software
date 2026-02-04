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

use async_graphql::{Context, Object, Result};

#[derive(Clone)]
pub struct Subsystem;

pub struct QueryRoot;

// Base GraphQL query model
#[Object]
impl QueryRoot {
    async fn ping(&self, _ctx: &Context<'_>, #[graphql(default = false)] fail: bool) -> Result<String> {
        if fail {
            Err(async_graphql::Error::new("Query failed"))
        } else {
            Ok(String::from("query"))
        }
    }
}

pub struct MutationRoot;

// Base GraphQL mutation model
#[Object]
impl MutationRoot {
    async fn ping(&self, _ctx: &Context<'_>) -> Result<String> {
        Ok(String::from("mutation"))
    }
}
