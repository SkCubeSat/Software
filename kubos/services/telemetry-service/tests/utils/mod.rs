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
use std::env;
use std::fs::File;
use std::io::*;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use std::{thread, thread::JoinHandle};

static UP_SQL: &str = r"CREATE TABLE telemetry (
    timestamp INTEGER NOT NULL,
    subsystem VARCHAR(255) NOT NULL,
    parameter VARCHAR(255) NOT NULL,
    value VARCHAR(255) NOT NULL,
    PRIMARY KEY (timestamp, subsystem, parameter))";

static DOWN_SQL: &str = r"DROP TABLE telemetry;";

fn setup_db(db: &str, sql: Option<&str>) {
    Command::new("sqlite3")
        .arg(db)
        .arg(DOWN_SQL)
        .output()
        .expect("SQL cmd failed");

    Command::new("sqlite3")
        .arg(db)
        .arg(UP_SQL)
        .output()
        .expect("SQL cmd failed");

    if let Some(sql) = sql {
        Command::new("sqlite3")
            .arg(db)
            .arg(sql)
            .output()
            .expect("SQL cmd failed");
    }
}

fn start_telemetry(config: String) -> (JoinHandle<()>, Sender<bool>) {
    let mut telem_path = env::current_exe().unwrap();
    telem_path.pop();
    telem_path.set_file_name("telemetry-service");

    let (tx, rx): (Sender<bool>, Receiver<bool>) = channel();
    let telem_thread = thread::spawn(move || {
        let mut telem_proc = Command::new(telem_path)
            .arg("-c")
            .arg(config)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut run = true;
        while run {
            thread::sleep(Duration::from_millis(100));
            if rx.try_recv().is_ok() {
                telem_proc.kill().unwrap();
                run = false;
            }
        }
    });

    // Give the process more time to actually start (increased from 300ms)
    thread::sleep(Duration::from_millis(1000));
    (telem_thread, tx)
}

pub struct TelemetryServiceFixture {
    join_handle: Option<JoinHandle<()>>,
    sender: Option<Sender<bool>>,
}

impl TelemetryServiceFixture {
    pub fn setup(
        db: &str,
        service_port: Option<u16>,
        udp_port: Option<u16>,
        sql: Option<&str>,
    ) -> Self {
        // Use port 0 for OS-assigned ports when None is provided
        // This prevents port conflicts between parallel tests
        let service_port = service_port.unwrap_or(8111);
        let udp_port = udp_port.unwrap_or(9111);

        setup_db(db, sql);

        let config_dir = match Path::new(db).parent() {
            Some(dir) => dir,
            None => Path::new(""),
        };
        let config_path = config_dir.join("config.toml");

        let config = format!(
            r#"
            [telemetry-service]
            database = "{}"
            direct_port = {}
            
            [telemetry-service.addr]
            ip = "127.0.0.1"
            port = {}
            "#,
            db, udp_port, service_port
        );

        let mut config_file = File::create(config_path.clone()).unwrap();
        config_file.write_all(config.as_bytes()).unwrap();

        let (join_handle, sender) = start_telemetry(config_path.to_str().unwrap().to_owned());

        Self {
            join_handle: Some(join_handle),
            sender: Some(sender),
        }
    }
}

impl Drop for TelemetryServiceFixture {
    fn drop(&mut self) {
        self.sender.take().unwrap().send(true).unwrap();
        self.join_handle.take().unwrap().join().unwrap();
    }
}

pub fn do_query(service_port: Option<u16>, query: &str) -> serde_json::Value {
    let port = service_port.unwrap_or(8111);  // Must match default in TelemetryServiceFixture::setup

    let client = reqwest::Client::new();

    let uri = format!("http://127.0.0.1:{}/graphql", port);

    let mut map = ::std::collections::HashMap::new();
    map.insert("query", query);

    // Retry with exponential backoff for connection issues
    let max_retries = 5;
    let mut delay = Duration::from_millis(200);

    for attempt in 0..max_retries {
        match client.post(&uri).json(&map).send() {
            Ok(mut response) => {
                return response.json().expect("Couldn't deserialize response");
            }
            Err(e) => {
                if attempt < max_retries - 1 {
                    thread::sleep(delay);
                    delay *= 2; // Exponential backoff
                } else {
                    panic!("Couldn't send request after {} attempts: {:?}", max_retries, e);
                }
            }
        }
    }

    unreachable!()
}
