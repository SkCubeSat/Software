use async_graphql::{Context, EmptyMutation, Object, SimpleObject};
use kubos_service::{Config, Service};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::{broadcast, RwLock};
use tokio_serial::SerialPortBuilderExt;

struct UartConfig {
    bus: String,
    baud: u32,
}

#[derive(SimpleObject, Clone)]
pub struct Counters {
    pub seu: i32,
    pub deu: i32,
}

// Define our shared state
#[derive(Default)]
struct AppState {
    last_counters: Option<Counters>,
}

impl AppState {
    fn new() -> Self {
        Self { last_counters: None }
    }
}

// Our subsystem - this is what kubos-service will manage
#[derive(Clone)]
pub struct StarRiscSubsystem {
    state: Arc<RwLock<AppState>>,
    tx: Arc<broadcast::Sender<String>>,
}

impl StarRiscSubsystem {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            state: Arc::new(RwLock::new(AppState::new())),
            tx: Arc::new(tx),
        }
    }

    pub async fn publish_line(&self, line: String) {
        let _ = self.tx.send(line);
    }

    pub async fn wait_for_counters(&self) -> Counters {
        let mut rx = self.tx.subscribe();
        let mut seu = None;
        let mut deu = None;

        while let Ok(line) = rx.recv().await {
            if line.contains("SEU counter:") {
                if let Some(val_str) = line.split("SEU counter:").nth(1) {
                    if let Ok(val) = val_str.trim().parse::<i32>() {
                        seu = Some(val);
                    }
                }
            } else if line.contains("DEU counter:") {
                if let Some(val_str) = line.split("DEU counter:").nth(1) {
                    if let Ok(val) = val_str.trim().parse::<i32>() {
                        deu = Some(val);
                    }
                }
            }

            if let (Some(s), Some(d)) = (seu, deu) {
                let counters = Counters { seu: s, deu: d };
                let mut state = self.state.write().await;
                state.last_counters = Some(counters.clone());
                return counters;
            }
        }

        // Default fallback if channel drops
        Counters { seu: -1, deu: -1 }
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

    async fn counters(&self, ctx: &Context<'_>) -> async_graphql::Result<Counters> {
        let subsystem_ctx = ctx.data::<kubos_service::Context<StarRiscSubsystem>>()?;
        Ok(subsystem_ctx.subsystem().wait_for_counters().await)
    }
}

// UART reading task
async fn uart_reading_task(subsystem: StarRiscSubsystem, uart_config: UartConfig) {
    // Try to open the UART, but don't panic if it fails (for demo purposes)
    let uart_result = tokio_serial::new(uart_config.bus.clone(), uart_config.baud)
        .data_bits(tokio_serial::DataBits::Eight)
        .flow_control(tokio_serial::FlowControl::None)
        .parity(tokio_serial::Parity::None)
        .stop_bits(tokio_serial::StopBits::One)
        .timeout(Duration::from_millis(100))
        .open_native_async();

    if let Ok(uart) = uart_result {
        let mut reader = BufReader::new(uart);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(bytes_read) if bytes_read > 0 => {
                    subsystem.publish_line(line.clone()).await;
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
        let mut counter = 0;
        loop {
            subsystem.publish_line(format!("123.45 | SEU counter: {}\n", counter)).await;
            subsystem.publish_line(format!("123.45 | DEU counter: {}\n", counter)).await;
            counter += 1;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

fn load_uart_config(config: &Config) -> UartConfig {
    let bus = config
        .get("uart_bus")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "/dev/pts/11".to_string());
    let baud = config
        .get("uart_baud")
        .and_then(|v| v.as_integer())
        .map(|v| v as u32)
        .unwrap_or(115200);

    UartConfig { bus, baud }
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

    let uart_config = load_uart_config(&config);

    // Create our subsystem
    let subsystem = StarRiscSubsystem::new();

    // Spawn the UART reading task
    let subsystem_clone = subsystem.clone();
    tokio::spawn(async move {
        uart_reading_task(subsystem_clone, uart_config).await;
    });

    // Create and start the service using kubos-service
    let service = Service::new(config, subsystem, QueryRoot::default(), EmptyMutation);

    println!("Star RISC service starting...");
    service.start_async().await;
}
