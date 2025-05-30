use async_graphql::{EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    response::{self, IntoResponse},
    routing::get,
};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use std::time::{Duration};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio_serial::{SerialPortBuilderExt};
use tokio::io::{AsyncReadExt};

// Define our data structure for storing UART readings
#[derive(Clone)]
struct UartReading {
    data: Vec<u8>,
}

#[async_graphql::Object]
impl UartReading {
    async fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

// Define our shared state
#[derive(Default)]
struct AppState {
    readings: VecDeque<UartReading>,
    max_readings: usize,
}

impl AppState {
    fn new(max_readings: usize) -> Self {
        Self {
            readings: VecDeque::new(),
            max_readings,
        }
    }

    fn add_reading(&mut self, data: Vec<u8>) {
        self.readings.push_back(UartReading { data });
        
        // Remove old readings if we exceed the maximum size
        while self.readings.len() > self.max_readings {
            self.readings.pop_front();
        }
    }

    fn get_readings(&self) -> Vec<UartReading> {
        self.readings.iter().cloned().collect()
    }
}

// Define our Query type
#[derive(Default)]
struct Query {
    state: Arc<RwLock<AppState>>,
}

#[async_graphql::Object]
impl Query {
    async fn uart_readings(&self) -> Vec<u8> {
        let state = self.state.read().await;
        state.get_readings()
            .iter()
            .flat_map(|reading| reading.data.clone())
            .collect()
    }
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

// UART reading task
async fn uart_reading_task(state: Arc<RwLock<AppState>>) {
    // For simulation, we'll use a PTY (pseudo-terminal)
    // You can create one using: socat -d -d pty,raw,echo=0 pty,raw,echo=0
    // This will give you two device paths like /dev/pts/X and /dev/pts/Y
    // Use one end to write data and the other to read
    let mut uart = tokio_serial::new("/dev/pts/8", 115200)
        .data_bits(tokio_serial::DataBits::Eight)
        .flow_control(tokio_serial::FlowControl::None)
        .parity(tokio_serial::Parity::None)
        .stop_bits(tokio_serial::StopBits::One)
        .timeout(Duration::from_millis(100))
        .open_native_async()
        .expect("Failed to open UART");
    
    let mut buffer = [0u8; 1024];
    loop {
        match uart.read(&mut buffer).await {
            Ok(bytes_read) if bytes_read > 0 => {
                let data = buffer[..bytes_read].to_vec();
                let mut state = state.write().await;
                state.add_reading(data);
            }
            Ok(_) => {
                // No data read, continue
            }
            Err(e) => {
                eprintln!("UART read error: {}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Create shared state with a maximum of 1000 readings
    let state = Arc::new(RwLock::new(AppState::new(1000)));
    
    // Spawn the UART reading task
    let state_clone = state.clone();
    tokio::spawn(async move {
        uart_reading_task(state_clone).await;
    });

    // Build our GraphQL schema
    let schema = Schema::build(Query { state }, EmptyMutation, EmptySubscription)
        .finish();

    // Create our application with routes
    let app = Router::new()
        .route("/", get(graphiql).post_service(GraphQL::new(schema)));

    println!("GraphiQL IDE: http://localhost:8000");

    // Start the server
    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
