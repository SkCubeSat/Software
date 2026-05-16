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

use regex::Regex;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::str::FromStr;

use crate::error::Error;

#[cfg(test)]
use lazy_static::lazy_static;

/// Stats provided by the Linux /proc/<pid>/stat file format
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProcStat {
    pid: i32,                   // %d
    comm: String,               // %s
    state: char,                // %c
    ppid: i32,                  // %d
    pgrp: i32,                  // %d
    session: i32,               // %d
    tty_nr: i32,                // %d
    tpgid: i32,                 // %d
    flags: u32,                 // %u
    minflt: u64,                // %lu
    cminflt: u64,               // %lu
    majflt: u64,                // %lu
    cmajflt: u64,               // %lu
    utime: u64,                 // %lu
    stime: u64,                 // %lu
    cutime: i64,                // %ld
    cstime: i64,                // %ld
    priority: i64,              // %ld
    nice: i64,                  // %ld
    num_threads: i64,           // %ld
    itrealvalue: i64,           // %ld
    starttime: u64,             // %llu
    vsize: u64,                 // %lu
    rss: i64,                   // %ld
    rsslim: u64,                // %lu
    startcode: u64,             // %lu
    endcode: u64,               // %lu
    startstack: u64,            // %lu
    kstkesp: u64,               // %lu
    kstkeip: u64,               // %lu
    signal: u64,                // %lu
    blocked: u64,               // %lu
    sigignore: u64,             // %lu
    sigcatch: u64,              // %lu
    wchan: u64,                 // %lu
    nswap: u64,                 // %lu
    cnswap: u64,                // %lu
    exit_signal: i32,           // %d
    processor: i32,             // %d
    rt_priority: u32,           // %u
    policy: u32,                // %u
    delayacct_blkio_ticks: u64, // %llu
    guest_time: u64,            // %lu
    cguest_time: i64,           // %ld
    start_data: u64,            // %lu
    end_data: u64,              // %lu
    start_brk: u64,             // %lu
    arg_start: u64,             // %lu
    arg_end: u64,               // %lu
    env_start: u64,             // %lu
    env_end: u64,               // %lu
    exit_code: i32,             // %d
}

#[cfg(not(test))]
#[inline]
pub fn root_dir() -> PathBuf {
    PathBuf::from("/")
}

/// Generates a path from parts starting at `/`
#[macro_export]
macro_rules! root_path {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_path = root_dir();
            $(
                temp_path.push($x.to_string());
            )*
            temp_path
        }
    };
}

fn unwrap_optstr<T>(opt: Option<&str>) -> T
where
    T: FromStr,
    T: Default,
{
    opt.map_or_else(T::default, |s| T::from_str(s).unwrap_or_default())
}

impl ProcStat {
    /// Convenience function that parses the stat file for a specific process ID
    /// See ProcStat::parse for more information
    pub fn from_pid(pid: i32) -> Result<Self, Error> {
        let file = File::open(root_path!("proc", pid, "stat"))?;
        Self::parse(BufReader::new(file))
    }

    /// Parse a String with the format of a /proc/pid/stat file
    /// See <http://man7.org/linux/man-pages/man5/proc.5.html> for more information
    pub fn parse<R>(stat: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut ps = Self::default();

        // Order and format of these fields taken from
        // http://man7.org/linux/man-pages/man5/proc.5.html
        let data: Result<Vec<_>, _> = stat.bytes().collect();
        let data_vec = data?;
        let data_str = String::from_utf8_lossy(&data_vec);

        let re = Regex::new(r"(?P<pid>\d+) \((?P<comm>.+)\) (?P<the_rest>.+)")?;
        let caps = re
            .captures(&data_str)
            .ok_or_else(|| Error::Format("Invalid procstat format".into()))?;

        ps.pid = i32::from_str(&caps["pid"]).unwrap_or_default();
        ps.comm = caps["comm"].into();

        let mut iter = caps["the_rest"].split_whitespace();
        ps.state = unwrap_optstr(iter.next());
        ps.ppid = unwrap_optstr(iter.next());
        ps.pgrp = unwrap_optstr(iter.next());
        ps.session = unwrap_optstr(iter.next());
        ps.tty_nr = unwrap_optstr(iter.next());
        ps.tpgid = unwrap_optstr(iter.next());
        ps.flags = unwrap_optstr(iter.next());
        ps.minflt = unwrap_optstr(iter.next());
        ps.cminflt = unwrap_optstr(iter.next());
        ps.majflt = unwrap_optstr(iter.next());
        ps.cmajflt = unwrap_optstr(iter.next());
        ps.utime = unwrap_optstr(iter.next());
        ps.stime = unwrap_optstr(iter.next());
        ps.cutime = unwrap_optstr(iter.next());
        ps.cstime = unwrap_optstr(iter.next());
        ps.priority = unwrap_optstr(iter.next());
        ps.nice = unwrap_optstr(iter.next());
        ps.num_threads = unwrap_optstr(iter.next());
        ps.itrealvalue = unwrap_optstr(iter.next());
        ps.starttime = unwrap_optstr(iter.next());
        ps.vsize = unwrap_optstr(iter.next());
        ps.rss = unwrap_optstr(iter.next());
        ps.rsslim = unwrap_optstr(iter.next());
        ps.startcode = unwrap_optstr(iter.next());
        ps.endcode = unwrap_optstr(iter.next());
        ps.startstack = unwrap_optstr(iter.next());
        ps.kstkesp = unwrap_optstr(iter.next());
        ps.kstkeip = unwrap_optstr(iter.next());
        ps.signal = unwrap_optstr(iter.next());
        ps.blocked = unwrap_optstr(iter.next());
        ps.sigignore = unwrap_optstr(iter.next());
        ps.sigcatch = unwrap_optstr(iter.next());
        ps.wchan = unwrap_optstr(iter.next());
        ps.nswap = unwrap_optstr(iter.next());
        ps.cnswap = unwrap_optstr(iter.next());
        ps.exit_signal = unwrap_optstr(iter.next());
        ps.processor = unwrap_optstr(iter.next());
        ps.rt_priority = unwrap_optstr(iter.next());
        ps.policy = unwrap_optstr(iter.next());
        ps.delayacct_blkio_ticks = unwrap_optstr(iter.next());
        ps.guest_time = unwrap_optstr(iter.next());
        ps.cguest_time = unwrap_optstr(iter.next());
        ps.start_data = unwrap_optstr(iter.next());
        ps.end_data = unwrap_optstr(iter.next());
        ps.start_brk = unwrap_optstr(iter.next());
        ps.arg_start = unwrap_optstr(iter.next());
        ps.arg_end = unwrap_optstr(iter.next());
        ps.env_start = unwrap_optstr(iter.next());
        ps.env_end = unwrap_optstr(iter.next());
        ps.exit_code = unwrap_optstr(iter.next());
        Ok(ps)
    }

    /// One of the following characters, indicating process state:
    ///
    /// * `R` - Running
    /// * `S` - Sleeping in an interruptible wait
    /// * `D` - Waiting in uninterruptible disk sleep
    /// * `Z` - Zombie
    /// * `T` - Stopped (on a signal) or (before Linux 2.6.33) trace stopped
    /// * `t` - Tracing stop (Linux 2.6.33 onward)
    /// * `W` - Paging (only before Linux 2.6.0)
    /// * `X` - Dead (from Linux 2.6.0 onward)
    /// * `x` - Dead (Linux 2.6.33 to 3.13 only)
    /// * `K` - Wakekill (Linux 2.6.33 to 3.13 only)
    /// * `W` - Waking (Linux 2.6.33 to 3.13 only)
    /// * `P` - Parked (Linux 3.9 to 3.13 only)
    pub fn state(&self) -> char {
        self.state
    }

    /// The PID of the parent of this process
    pub fn parent_pid(&self) -> i32 {
        self.ppid
    }

    /// Virtual memory size in bytes
    pub fn mem_usage(&self) -> u64 {
        self.vsize
    }

    /// Resident Set Size: number of pages the process has in real memory.  This is just the pages
    /// which count toward text, data, or stack space.  This does not include pages which have not
    /// been demand-loaded in, or which are swapped out.
    pub fn rss(&self) -> i32 {
        self.rss as i32
    }

    /// Number of threads in this process
    pub fn num_threads(&self) -> i32 {
        self.num_threads as i32
    }

    /// Attempts to read the command line arguments used to execute this process, and falls
    /// back to the raw process name if /proc/pid/cmdline does not exist or is empty
    pub fn cmd(&self) -> Result<Vec<String>, Error> {
        let file = File::open(root_path!("proc", self.pid, "cmdline"))?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        if contents.is_empty() {
            Ok(vec![self.comm.clone()])
        } else {
            let mut argv: Vec<String> = contents.split('\0').map(String::from).collect();
            argv.pop();
            Ok(argv)
        }
    }
}

/// Returns an array of the PIDs of all currently running processes
pub fn running_pids() -> Result<Vec<i32>, Error> {
    let proc = root_path!("proc");
    let entries = fs::read_dir(proc)?;
    let mut pids = Vec::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if let Ok(pid) = i32::from_str(file_name) {
                    pids.push(pid);
                }
            }
        }
    }

    Ok(pids)
}

#[cfg(test)]
pub fn root_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("testroot")
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref PROC_RAW: Vec<u8> = {
            b"12345 (bash) S 12344 12345 12345 34820 12345 4202496 17434 782422 62 28 15 16 103 60 \
              20 0 1 0 287033 56205312 10492 18446744073709551615 94073079275520 94073079994797 \
              140724981622432 0 0 0 65536 3670020 1266777851 0 0 0 17 2 0 0 2 0 0 94073080100768 \
              94073080116368 94073105047552 140724981633650 140724981633657 140724981633657 \
              140724981638583 0"
                .to_vec()
        };
        static ref SD_PAM_RAW: Vec<u8> = {
            b"717 (systemd-journal) S 1 717 717 0 -1 4210688 5079 9284 37 0 77 57 0 0 20 0 1 0 \
              80678 127893504 2966 18446744073709551615 1 1 0 0 0 0 0 4224 0 0 0 0 17 3 0 0 0 0 0 \
              0 0 0 0 0 0 0 0"
                .to_vec()
        };
        static ref CRON_RAW: Vec<u8> = {
            b"1 (cron) S 0 1 1 0 -1 4210944 0 0 0 0 0 0 0 0 20 0 1 0 121784 5603328 285 \
              18446744073709551615 1 1 0 0 0 0 0 0 0 0 0 0 17 3 0 0 0 0 0 0 0 0 0 0 0 0 0"
                .to_vec()
        };
    }

    #[test]
    fn procstat_parse() {
        let ps = ProcStat::parse(&PROC_RAW[..]);

        assert!(ps.is_ok());
        assert_eq!(
            ps.unwrap(),
            ProcStat {
                pid: 12345,
                comm: "bash".to_owned(),
                state: 'S',
                ppid: 12344,
                pgrp: 12345,
                session: 12345,
                tty_nr: 34820,
                tpgid: 12345,
                flags: 4202496,
                minflt: 17434,
                cminflt: 782422,
                majflt: 62,
                cmajflt: 28,
                utime: 15,
                stime: 16,
                cutime: 103,
                cstime: 60,
                priority: 20,
                nice: 0,
                num_threads: 1,
                itrealvalue: 0,
                starttime: 287033,
                vsize: 56205312,
                rss: 10492,
                rsslim: 18446744073709551615,
                startcode: 94073079275520,
                endcode: 94073079994797,
                startstack: 140724981622432,
                kstkesp: 0,
                kstkeip: 0,
                signal: 0,
                blocked: 65536,
                sigignore: 3670020,
                sigcatch: 1266777851,
                wchan: 0,
                nswap: 0,
                cnswap: 0,
                exit_signal: 17,
                processor: 2,
                rt_priority: 0,
                policy: 0,
                delayacct_blkio_ticks: 2,
                guest_time: 0,
                cguest_time: 0,
                start_data: 94073080100768,
                end_data: 94073080116368,
                start_brk: 94073105047552,
                arg_start: 140724981633650,
                arg_end: 140724981633657,
                env_start: 140724981633657,
                env_end: 140724981638583,
                exit_code: 0,
            }
        );
    }
}
