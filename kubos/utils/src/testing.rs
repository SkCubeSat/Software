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

extern crate tempfile;

use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    routing::post,
    Router,
};
use log::debug;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::Write;
use std::net::SocketAddr;
use std::process;
use std::process::{Command, Stdio};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tempfile::tempdir;
use tokio::sync::oneshot;

pub struct TestCommand {
    command: String,
    args: Vec<&'static str>,
    child_handle: RefCell<Box<Option<process::Child>>>,
}

impl TestCommand {
    pub fn new(command: &str, args: Vec<&'static str>) -> TestCommand {
        TestCommand {
            command: String::from(command),
            args,
            child_handle: RefCell::new(Box::new(None)),
        }
    }

    /// Ask Cargo to run the command.
    /// This is *not* a blocking function. The command is
    /// spawned in the background, allowing the test
    /// to continue on.
    pub fn spawn(&self) {
        let child = Command::new(&self.command)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start");

        let mut child_handle = self.child_handle.borrow_mut();
        *child_handle = Box::new(Some(child));
    }
}

/// This structure allows the creation of an instance
/// of an actual service/binary crate for use in
/// integration tests within the same crate.
pub struct TestService {
    config_path: String,
    // Keep around so the temp file stays alive
    _config_file: File,
    // Keep around so the temp dir stays alive
    _tmp_dir: tempfile::TempDir,
    name: String,
    child_handle: RefCell<Box<Option<process::Child>>>,
}

impl TestService {
    /// Create config for TestService and return basic struct
    pub fn new(name: &str, ip: &str, port: u16) -> TestService {
        let mut config = Vec::new();
        writeln!(&mut config, "[{}.addr]", name).unwrap();
        writeln!(&mut config, "ip = \"{}\"", ip).unwrap();
        writeln!(&mut config, "port = {}", port).unwrap();
        let config_str = String::from_utf8(config).unwrap();

        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");
        let mut config_file = File::create(config_path.clone()).unwrap();
        writeln!(config_file, "{}", config_str).unwrap();

        TestService {
            config_path: config_path.to_str().unwrap().to_owned(),
            _config_file: config_file,
            _tmp_dir: dir,
            name: String::from(name),
            child_handle: RefCell::new(Box::new(None)),
        }
    }

    /// Appends additional configuration data to service's config
    pub fn config(&mut self, config_data: &str) {
        self._config_file.seek(SeekFrom::End(0)).unwrap();
        self._config_file.write_all(config_data.as_bytes()).unwrap();
    }

    /// Ask Cargo to build the service binary.
    /// This is a *blocking* function. We know when it returns
    /// that the service is ready to be run.
    pub fn build(&self) {
        Command::new("cargo")
            .arg("build")
            .arg("--package")
            .arg(&self.name)
            .output()
            .expect("Failed to build service");
    }

    /// Ask Cargo to run the service binary.
    /// This is *not* a blocking function. The service is
    /// spawned in the background, allowing the test
    /// to continue on.
    pub fn spawn(&self) {
        let mut cmd = Command::new("cargo");
        cmd.arg("run")
            .arg("--package")
            .arg(self.name.clone())
            .arg("--")
            .arg("-c")
            .arg(self.config_path.clone());
        if log::log_enabled!(log::Level::Info) {
            cmd.arg("--stdout")
                .arg("-l")
                .arg(log::max_level().as_str().to_ascii_lowercase());
        }
        let child = cmd.spawn().expect("Failed to start");

        let mut child_handle = self.child_handle.borrow_mut();
        *child_handle = Box::new(Some(child));
    }

    /// Kill the running process.
    pub fn kill(&self) {
        let mut borrowed_child = self.child_handle.borrow_mut();
        if let Some(mut handle) = borrowed_child.take() {
            handle.kill().unwrap();
        }
    }
}

/// Implement custom drop functionality which
/// will retrieve handle to child process and kill it.
impl Drop for TestService {
    fn drop(&mut self) {
        let mut borrowed_child = self.child_handle.borrow_mut();
        if let Some(mut handle) = borrowed_child.take() {
            handle.kill().unwrap();
        }
    }
}

pub async fn service_query(query: &str, ip: &str, port: u16) -> Value {
    // Use async client for modern tokio compatibility
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(250))
        .build()
        .unwrap();
    
    let mut map = ::std::collections::HashMap::new();
    map.insert("query", query);
    
    for attempt in 0..10 {
        match client
            .post(&format!("http://{}:{}/graphql", ip, port))
            .json(&map)
            .send()
            .await
        {
            Ok(result) => {
                match result.text().await {
                    Ok(text) => {
                        if !text.is_empty() {
                            return serde_json::from_str(&text).unwrap_or_else(|e| {
                                panic!("Failed to parse JSON response '{}': {}", text, e);
                            });
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get response text: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Request failed (attempt {}): {}", attempt + 1, e);
            }
        }
        // Increase wait time for each retry with async sleep
        tokio::time::sleep(Duration::from_millis(200 + attempt * 100)).await;
    }

    panic!("Service query failed after 10 attempts - {}:{}/graphql", ip, port);
}

pub trait ServiceResponder: Send + Sync + 'static {
    fn respond(&self, body: &str) -> String;
}

#[derive(Clone)]
struct DefaultServiceResponder;

impl ServiceResponder for DefaultServiceResponder {
    fn respond(&self, _body: &str) -> String {
        "{}".to_string()
    }
}

/// Shared state for axum handlers
struct AppState<R: ServiceResponder> {
    requests: Arc<Mutex<VecDeque<String>>>,
    responder: R,
}

/// Handler for POST requests
async fn handle_post<R: ServiceResponder>(
    State(state): State<Arc<AppState<R>>>,
    body: Bytes,
) -> (StatusCode, String) {
    let body_str = String::from_utf8_lossy(&body).to_string();
    debug!("service_listener, body: {}", body_str);
    let response = state.responder.respond(&body_str);
    state.requests.lock().unwrap().push_back(body_str);
    (StatusCode::OK, response)
}

pub struct ServiceListener {
    requests: Arc<Mutex<VecDeque<String>>>,
    shutdown_tx: Option<oneshot::Sender<()>>,
    server_thread: Option<thread::JoinHandle<()>>,
}

impl ServiceListener {
    /// Spawns a new dummy service listener
    /// This listener will just listen for posts and save off
    /// the request bodies for examination in tests
    pub fn spawn(_ip: &str, port: u16) -> Self {
        Self::spawn_with_responder(_ip, port, DefaultServiceResponder)
    }

    pub fn spawn_with_responder<R>(_ip: &str, port: u16, responder: R) -> ServiceListener
    where
        R: ServiceResponder + Clone,
    {
        let requests = Arc::new(Mutex::new(VecDeque::<String>::new()));
        let req_handle = requests.clone();

        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        let server_thread = thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime");

            rt.block_on(async move {
                let state = Arc::new(AppState {
                    requests: req_handle,
                    responder,
                });

                let app = Router::new()
                    .route("/", post(handle_post::<R>))
                    .fallback(post(handle_post::<R>))
                    .with_state(state);

                let addr = SocketAddr::from(([127, 0, 0, 1], port));
                let listener = tokio::net::TcpListener::bind(addr)
                    .await
                    .expect("Failed to bind");

                axum::serve(listener, app)
                    .with_graceful_shutdown(async {
                        let _ = shutdown_rx.await;
                    })
                    .await
                    .expect("Server error");
            });
        });

        // Give the server a moment to start
        thread::sleep(Duration::from_millis(10));

        ServiceListener {
            requests,
            shutdown_tx: Some(shutdown_tx),
            server_thread: Some(server_thread),
        }
    }

    /// Pops a request body from the collected queue
    /// These are popped off in FIFO fashion
    pub fn get_request(&self) -> Option<String> {
        self.requests.lock().unwrap().pop_front()
    }

    /// Waits for a request to arrive with polling and timeout.
    /// Returns the request body if one arrives within the timeout, or None if it times out.
    /// This is more robust than using sleep + get_request for CI environments
    /// where timing can be unpredictable.
    ///
    /// # Arguments
    /// * `timeout` - Maximum time to wait for a request
    /// * `poll_interval` - How often to check for new requests (default: 50ms if None)
    pub fn wait_for_request(
        &self,
        timeout: Duration,
        poll_interval: Option<Duration>,
    ) -> Option<String> {
        let interval = poll_interval.unwrap_or(Duration::from_millis(50));
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            if let Some(request) = self.get_request() {
                return Some(request);
            }
            thread::sleep(interval);
        }

        // One final check after timeout
        self.get_request()
    }

    /// Waits for a request to arrive, panicking with a helpful message if it doesn't.
    /// Use this when you expect a request to definitely arrive.
    ///
    /// # Arguments
    /// * `timeout` - Maximum time to wait for a request
    /// * `expected_desc` - Description of expected request for error message
    pub fn expect_request(&self, timeout: Duration, expected_desc: &str) -> String {
        self.wait_for_request(timeout, None).unwrap_or_else(|| {
            panic!(
                "Timed out after {:?} waiting for request: {}",
                timeout, expected_desc
            )
        })
    }
}

impl Drop for ServiceListener {
    fn drop(&mut self) {
        // Send shutdown signal
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        // Wait for server thread to finish
        if let Some(handle) = self.server_thread.take() {
            let _ = handle.join();
        }
    }
}
