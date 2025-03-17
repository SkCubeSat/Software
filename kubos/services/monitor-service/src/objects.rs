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
use crate::loadavg::LoadAvg;
use crate::meminfo::MemInfo;
use crate::process::ProcStat;
use crate::userinfo::UserInfo;
use juniper::graphql_object;

pub struct LoadAvgResponse {
    pub avgs: LoadAvg,
}

#[graphql_object(context = ())]
impl LoadAvgResponse {
    fn load_1m(&self) -> Option<f64> {
        self.avgs.load_1m()
    }

    fn load_5m(&self) -> Option<f64> {
        self.avgs.load_5m()
    }

    fn load_15m(&self) -> Option<f64> {
        self.avgs.load_15m()
    }

    fn processes_active(&self) -> Option<f64> {
        self.avgs.processes_active().map(|v| v as f64)
    }

    fn processes_total(&self) -> Option<f64> {
        self.avgs.processes_total().map(|v| v as f64)
    }

    fn last_pid(&self) -> Option<f64> {
        self.avgs.last_pid().map(|v| v as f64)
    }
}

pub struct MemInfoResponse {
    pub info: MemInfo,
}

#[graphql_object(context = ())]
impl MemInfoResponse {
    fn total(&self) -> Option<i32> {
        self.info.total().map(|v| v as i32)
    }

    fn free(&self) -> Option<i32> {
        self.info.free().map(|v| v as i32)
    }

    fn available(&self) -> Option<i32> {
        self.info.available().map(|v| v as i32)
    }

    fn low_free(&self) -> Option<i32> {
        self.info.low_free().map(|v| v as i32)
    }
}

pub struct PSResponse {
    pub pid: i32,
    pub user: Option<UserInfo>,
    pub stat: Option<ProcStat>,
}

impl PSResponse {
    pub fn new(pid: i32) -> PSResponse {
        PSResponse {
            pid,
            user: UserInfo::from_pid(pid).ok(),
            stat: ProcStat::from_pid(pid).ok(),
        }
    }
}

#[graphql_object(context = ())]
impl PSResponse {
    fn pid(&self) -> i32 {
        self.pid
    }

    fn uid(&self) -> Option<i32> {
        self.user.as_ref().map(|u| u.uid() as i32)
    }

    fn gid(&self) -> Option<i32> {
        self.user.as_ref().map(|u| u.gid() as i32)
    }

    fn usr(&self) -> Option<String> {
        self.user.as_ref().and_then(|u| u.user())
    }

    fn grp(&self) -> Option<String> {
        self.user.as_ref().and_then(|u| u.group())
    }

    fn state(&self) -> Option<String> {
        self.stat.as_ref().map(|stat| stat.state().to_string())
    }

    fn ppid(&self) -> Option<i32> {
        self.stat.as_ref().map(|stat| stat.parent_pid())
    }

    fn mem(&self) -> Option<i32> {
        self.stat.as_ref().map(|stat| stat.mem_usage() as i32)
    }

    fn rss(&self) -> Option<i32> {
        self.stat.as_ref().map(|stat| stat.rss())
    }

    fn threads(&self) -> Option<i32> {
        self.stat.as_ref().map(|stat| stat.num_threads())
    }

    fn cmd(&self) -> Option<String> {
        self.stat
            .as_ref()
            .and_then(|stat| stat.cmd().ok().map(|argv| argv.join(" ")))
    }
}
