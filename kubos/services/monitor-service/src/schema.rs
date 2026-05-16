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

use async_graphql::{Context, Object, Result};

use crate::loadavg;
use crate::meminfo;
use crate::objects::*;
use crate::process;

pub struct QueryRoot;

// Base GraphQL query model
#[Object]
impl QueryRoot {
    async fn ping(&self) -> &str {
        "pong"
    }

    async fn mem_info(&self, _ctx: &Context<'_>) -> Result<MemInfoResponse> {
        meminfo::MemInfo::from_proc()
            .map(|info| MemInfoResponse { info })
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn ps(&self, _ctx: &Context<'_>, pids: Option<Vec<i32>>) -> Result<Vec<PSResponse>> {
        let pids_vec: Vec<i32> = match pids {
            Some(vec) => vec,
            None => process::running_pids()
                .map_err(|err| async_graphql::Error::new(err.to_string()))?,
        };

        Ok(pids_vec.into_iter().map(PSResponse::new).collect())
    }

    async fn load_avg(&self, _ctx: &Context<'_>) -> Result<LoadAvgResponse> {
        loadavg::LoadAvg::from_proc()
            .map(|avgs| LoadAvgResponse { avgs })
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}

pub struct MutationRoot;

// Base GraphQL mutation model
#[Object]
impl MutationRoot {
    async fn noop(&self, _ctx: &Context<'_>) -> bool {
        true
    }
}
