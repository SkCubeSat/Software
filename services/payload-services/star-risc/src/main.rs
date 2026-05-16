use async_graphql::{Context, EmptyMutation, Object};
use kubos_service::{Config, Service};
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_serial::SerialPortBuilderExt;
use tokio::io::AsyncReadExt;

// Define our data structure for storing UART readings
#[derive(Clone)]
struct UartReading {
    data: Vec<u8>,
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

// Our subsystem - this is what kubos-service will manage
#[derive(Clone)]
pub struct StarRiscSubsystem {
    state: Arc<RwLock<AppState>>,
}

impl StarRiscSubsystem {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AppState::new(1000))),
        }
    }

    pub async fn add_reading(&self, data: Vec<u8>) {
        let mut state = self.state.write().await;
        state.add_reading(data);
    }

    pub async fn get_uart_readings(&self) -> Vec<u8> {
        let state = self.state.read().await;
        state.get_readings()
            .iter()
            .flat_map(|reading| reading.data.clone())
            .collect()
    }
}

// Define our Query type using async-graphql
#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self) -> &str {
        "pong"
    }

    async fn uart_readings(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<u8>> {
        let subsystem_ctx = ctx.data::<kubos_service::Context<StarRiscSubsystem>>()?;
        Ok(subsystem_ctx.subsystem().get_uart_readings().await)
    }
}

// UART reading task
async fn uart_reading_task(subsystem: StarRiscSubsystem) {
    // For simulation, we'll use a PTY (pseudo-terminal)
    // You can create one using: socat -d -d pty,raw,echo=0 pty,raw,echo=0
    // This will give you two device paths like /dev/pts/X and /dev/pts/Y
    // Use one end to write data and the other to read
    
    // Try to open the UART, but don't panic if it fails (for demo purposes)
    let uart_result = tokio_serial::new("/dev/pts/11", 115200)
        .data_bits(tokio_serial::DataBits::Eight)
        .flow_control(tokio_serial::FlowControl::None)
        .parity(tokio_serial::Parity::None)
        .stop_bits(tokio_serial::StopBits::One)
        .timeout(Duration::from_millis(100))
        .open_native_async();
    
    if let Ok(mut uart) = uart_result {
        let mut buffer = [0u8; 1024];
        loop {
            match uart.read(&mut buffer).await {
                Ok(bytes_read) if bytes_read > 0 => {
                    let data = buffer[..bytes_read].to_vec();
                    subsystem.add_reading(data).await;
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
    } else {
        println!("Could not open UART device, running in simulation mode");
        // Generate some simulated data
        let mut counter = 0u8;
        loop {
            let data = vec![counter, counter.wrapping_add(1), counter.wrapping_add(2)];
            subsystem.add_reading(data).await;
            counter = counter.wrapping_add(1);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize the logger
    kubos_service::Logger::init("star-risc-service").unwrap();

    // Load the service configuration
    let config = Config::new("star-risc")
        .map_err(|err| {
            eprintln!("Failed to load service config: {:?}", err);
            err
        })
        .unwrap();

    // Create our subsystem
    let subsystem = StarRiscSubsystem::new();

    // Spawn the UART reading task
    let subsystem_clone = subsystem.clone();
    tokio::spawn(async move {
        uart_reading_task(subsystem_clone).await;
    });

    // Create and start the service using kubos-service
    let service = Service::new(
        config,
        subsystem,
        QueryRoot::default(),
        EmptyMutation,
    );

    println!("Star RISC service starting...");
    service.start_async().await;
}
