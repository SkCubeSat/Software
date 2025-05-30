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

use juniper::{graphql_object, FieldError, FieldResult};

use crate::loadavg;
use crate::meminfo;
use crate::objects::*;
use crate::process;

type Context = kubos_service::Context<()>;

pub struct QueryRoot;

// Base GraphQL query model
#[graphql_object(context = Context)]
impl QueryRoot {
    fn ping() -> FieldResult<String> {
        Ok(String::from("pong"))
    }

    fn mem_info(&self, _context: &Context) -> FieldResult<MemInfoResponse> {
        meminfo::MemInfo::from_proc()
            .map(|info| MemInfoResponse { info })
            .map_err(|err| FieldError::new(err, juniper::Value::null()))
    }

    fn ps(&self, _context: &Context, pids: Option<Vec<i32>>) -> FieldResult<Vec<PSResponse>> {
        let pids_vec: Vec<i32> = match pids {
            Some(vec) => vec,
            None => process::running_pids()?,
        };

        Ok(pids_vec.into_iter().map(PSResponse::new).collect())
    }

    fn load_avg(&self, _context: &Context) -> FieldResult<LoadAvgResponse> {
        loadavg::LoadAvg::from_proc()
            .map(|avgs| LoadAvgResponse { avgs })
            .map_err(|err| FieldError::new(err, juniper::Value::null()))
    }
}

pub struct MutationRoot;

// Base GraphQL mutation model
#[graphql_object(context = Context)]
impl MutationRoot {
    fn noop(&self, _context: &Context) -> FieldResult<bool> {
        Ok(true)
    }
}
