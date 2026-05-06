use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process;
use std::time::Duration;

use fram_service::backend::{ByteStorage, I2cFramBackend};

const DEFAULT_URL: &str = "http://127.0.0.1:8091/graphql";
const DEFAULT_BUS: &str = "/dev/i2c-2";
const DEFAULT_ADDR: u8 = 0x50;
const DEFAULT_CAPACITY: u32 = 8192;
const DEFAULT_ADDRESS_WIDTH: u8 = 2;
const DEFAULT_MAX_TRANSFER: usize = 32;
const DEFAULT_SCRATCH_OFFSET: u32 = 4096;
const SCRATCH_LEN: usize = 32;

#[derive(Debug)]
struct Options {
    url: String,
    i2c_bus: String,
    i2c_addr: u8,
    capacity: u32,
    address_width: u8,
    max_transfer: usize,
    scratch_offset: u32,
    scan_only: bool,
    skip_graphql: bool,
    skip_direct: bool,
    mission_write: bool,
}

fn main() {
    let options = match Options::parse() {
        Ok(options) => options,
        Err(err) => {
            eprintln!("{err}");
            process::exit(2);
        }
    };

    let mut failures = Vec::new();

    if options.scan_only {
        run(
            "scan FM24CL64B address range",
            || scan_fram_candidates(&options),
            &mut failures,
        );
        finish(failures);
        return;
    }

    if !options.skip_direct {
        run(
            "direct FRAM scratch roundtrip",
            || direct_fram_test(&options),
            &mut failures,
        );
    }

    if !options.skip_graphql {
        run("GraphQL ping", || graphql_ping(&options.url), &mut failures);
        run(
            "GraphQL health",
            || graphql_health(&options.url),
            &mut failures,
        );
        run(
            "GraphQL mission state read",
            || graphql_mission_state(&options.url),
            &mut failures,
        );

        if options.mission_write {
            run(
                "GraphQL mission flag write/restore",
                || graphql_mission_write_restore(&options.url),
                &mut failures,
            );
        }
    }

    finish(failures);
}

fn finish(failures: Vec<String>) {
    if failures.is_empty() {
        println!("PASS fram-obc-tests");
    } else {
        eprintln!("FAIL fram-obc-tests");
        for failure in failures {
            eprintln!("- {failure}");
        }
        process::exit(1);
    }
}

fn run<F>(name: &str, test: F, failures: &mut Vec<String>)
where
    F: FnOnce() -> Result<(), String>,
{
    print!("{name} ... ");
    if let Err(err) = std::io::stdout().flush() {
        failures.push(format!("{name}: failed to flush stdout: {err}"));
        return;
    }

    match test() {
        Ok(()) => println!("ok"),
        Err(err) => {
            println!("FAILED");
            failures.push(format!("{name}: {err}"));
        }
    }
}

impl Options {
    fn parse() -> Result<Self, String> {
        let mut options = Self {
            url: env::var("FRAM_TEST_URL").unwrap_or_else(|_| DEFAULT_URL.to_string()),
            i2c_bus: env::var("FRAM_TEST_I2C_BUS").unwrap_or_else(|_| DEFAULT_BUS.to_string()),
            i2c_addr: env::var("FRAM_TEST_I2C_ADDR")
                .ok()
                .map(|value| parse_u8(&value))
                .transpose()?
                .unwrap_or(DEFAULT_ADDR),
            capacity: env::var("FRAM_TEST_CAPACITY")
                .ok()
                .map(|value| parse_u32(&value))
                .transpose()?
                .unwrap_or(DEFAULT_CAPACITY),
            address_width: env::var("FRAM_TEST_ADDRESS_WIDTH")
                .ok()
                .map(|value| parse_u8(&value))
                .transpose()?
                .unwrap_or(DEFAULT_ADDRESS_WIDTH),
            max_transfer: env::var("FRAM_TEST_MAX_TRANSFER")
                .ok()
                .map(|value| {
                    value
                        .parse::<usize>()
                        .map_err(|err| format!("invalid FRAM_TEST_MAX_TRANSFER: {err}"))
                })
                .transpose()?
                .unwrap_or(DEFAULT_MAX_TRANSFER),
            scratch_offset: env::var("FRAM_TEST_SCRATCH_OFFSET")
                .ok()
                .map(|value| parse_u32(&value))
                .transpose()?
                .unwrap_or(DEFAULT_SCRATCH_OFFSET),
            scan_only: env::var("FRAM_TEST_SCAN_ONLY").ok().as_deref() == Some("1"),
            skip_graphql: env::var("FRAM_TEST_SKIP_GRAPHQL").ok().as_deref() == Some("1"),
            skip_direct: env::var("FRAM_TEST_SKIP_DIRECT").ok().as_deref() == Some("1"),
            mission_write: env::var("FRAM_TEST_MISSION_WRITE").ok().as_deref() == Some("1"),
        };

        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--url" => options.url = require_arg(&mut args, "--url")?,
                "--i2c-bus" => options.i2c_bus = require_arg(&mut args, "--i2c-bus")?,
                "--i2c-addr" => {
                    options.i2c_addr = parse_u8(&require_arg(&mut args, "--i2c-addr")?)?
                }
                "--capacity" => {
                    options.capacity = parse_u32(&require_arg(&mut args, "--capacity")?)?
                }
                "--address-width" => {
                    options.address_width = parse_u8(&require_arg(&mut args, "--address-width")?)?
                }
                "--max-transfer" => {
                    options.max_transfer = require_arg(&mut args, "--max-transfer")?
                        .parse::<usize>()
                        .map_err(|err| format!("invalid --max-transfer: {err}"))?
                }
                "--scratch-offset" => {
                    options.scratch_offset =
                        parse_u32(&require_arg(&mut args, "--scratch-offset")?)?
                }
                "--scan-only" => options.scan_only = true,
                "--skip-graphql" => options.skip_graphql = true,
                "--skip-direct" => options.skip_direct = true,
                "--mission-write" => options.mission_write = true,
                "--help" | "-h" => {
                    print_usage();
                    process::exit(0);
                }
                other => return Err(format!("unknown argument '{other}'")),
            }
        }

        if options.scratch_offset < 2048 {
            return Err(
                "scratch offset must be >= 2048 to stay out of mission record slots".to_string(),
            );
        }

        let scratch_len = u32::try_from(SCRATCH_LEN).map_err(|err| err.to_string())?;
        if options
            .scratch_offset
            .checked_add(scratch_len)
            .filter(|end| *end <= options.capacity)
            .is_none()
        {
            return Err(format!(
                "scratch offset {} plus {} bytes exceeds capacity {}",
                options.scratch_offset, SCRATCH_LEN, options.capacity
            ));
        }

        Ok(options)
    }
}

fn print_usage() {
    println!(
        "fram-obc-tests [options]\n\
         \n\
         Options:\n\
           --url URL                  GraphQL URL (default {DEFAULT_URL})\n\
           --i2c-bus PATH             I2C bus path (default {DEFAULT_BUS})\n\
           --i2c-addr ADDR            FRAM I2C address, decimal or 0xNN (default 0x50)\n\
           --capacity BYTES           FRAM capacity (default 8192)\n\
           --address-width BYTES      FRAM address width (default 2)\n\
           --max-transfer BYTES       I2C chunk size (default 32)\n\
           --scratch-offset OFFSET    Direct test scratch offset (default 4096)\n\
           --scan-only                Read-probe 0x50..0x57 and exit without writes\n\
           --skip-graphql             Do not call the GraphQL service\n\
           --skip-direct              Do not run direct I2C scratch test\n\
           --mission-write            Toggle and restore detumbling_complete through GraphQL\n"
    );
}

fn require_arg(args: &mut impl Iterator<Item = String>, name: &str) -> Result<String, String> {
    args.next()
        .ok_or_else(|| format!("{name} requires a value"))
}

fn parse_u8(value: &str) -> Result<u8, String> {
    parse_u32(value).and_then(|value| {
        u8::try_from(value).map_err(|_| format!("value '{value}' does not fit in u8"))
    })
}

fn parse_u32(value: &str) -> Result<u32, String> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        u32::from_str_radix(hex, 16).map_err(|err| format!("invalid hex value '{value}': {err}"))
    } else {
        trimmed
            .parse::<u32>()
            .map_err(|err| format!("invalid integer value '{value}': {err}"))
    }
}

fn direct_fram_test(options: &Options) -> Result<(), String> {
    let mut backend = I2cFramBackend::new(
        &options.i2c_bus,
        options.i2c_addr,
        options.capacity,
        options.address_width,
        options.max_transfer,
    )
    .map_err(|err| err.to_string())?;

    let mut original = [0u8; SCRATCH_LEN];
    backend
        .read(options.scratch_offset, &mut original)
        .map_err(|err| format!("scratch read failed: {err}"))?;

    let mut pattern = [0u8; SCRATCH_LEN];
    let label = b"FRAM-OBC-TEST-v1";
    pattern[..label.len()].copy_from_slice(label);
    pattern[label.len()..label.len() + 4].copy_from_slice(&process::id().to_le_bytes());
    pattern[SCRATCH_LEN - 4..].copy_from_slice(&0xF00D_CAFE_u32.to_le_bytes());

    backend
        .write(options.scratch_offset, &pattern)
        .map_err(|err| format!("scratch write failed: {err}"))?;

    let mut readback = [0u8; SCRATCH_LEN];
    backend
        .read(options.scratch_offset, &mut readback)
        .map_err(|err| format!("scratch readback failed: {err}"))?;

    let mut restore_error = None;
    if let Err(err) = backend.write(options.scratch_offset, &original) {
        restore_error = Some(format!("failed to restore original scratch bytes: {err}"));
    }

    if readback != pattern {
        return Err("scratch readback did not match written pattern".to_string());
    }

    if let Some(err) = restore_error {
        return Err(err);
    }

    let mut restored = [0u8; SCRATCH_LEN];
    backend
        .read(options.scratch_offset, &mut restored)
        .map_err(|err| format!("scratch restore readback failed: {err}"))?;
    if restored != original {
        return Err("scratch restore verification failed".to_string());
    }

    Ok(())
}

fn scan_fram_candidates(options: &Options) -> Result<(), String> {
    use embedded_hal::i2c::I2c;

    let mut found = Vec::new();
    let mut i2c =
        linux_embedded_hal::I2cdev::new(&options.i2c_bus).map_err(|err| err.to_string())?;

    for addr in 0x50..=0x57 {
        let mut byte = [0u8; 1];
        match i2c.read(addr, &mut byte) {
            Ok(()) => {
                println!("0x{addr:02X}: ACK, current-read byte=0x{:02X}", byte[0]);
                found.push(addr);
            }
            Err(err) => println!("0x{addr:02X}: no read response ({err})"),
        }
    }

    if found.is_empty() {
        Err(format!(
            "no FM24CL64B-style responders found on {}",
            options.i2c_bus
        ))
    } else {
        println!("Candidates: {}", format_addr_list(&found));
        Ok(())
    }
}

fn format_addr_list(addrs: &[u8]) -> String {
    addrs
        .iter()
        .map(|addr| format!("0x{addr:02X}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn graphql_ping(url: &str) -> Result<(), String> {
    let body = graphql(url, "{ ping }")?;
    expect_contains(&body, "\"ping\":\"pong\"")
}

fn graphql_health(url: &str) -> Result<(), String> {
    let body = graphql(
        url,
        "{ health { backend capacityBytes framReachable lastError } }",
    )?;
    expect_contains(&body, "\"backend\":\"i2c\"")?;
    expect_contains(&body, "\"capacityBytes\":8192")?;
    expect_contains(&body, "\"framReachable\":true")
}

fn graphql_mission_state(url: &str) -> Result<(), String> {
    let body = graphql(
        url,
        "{ missionState { removeBeforeFlight deployed deployStart detumblingComplete } }",
    )?;
    expect_contains(&body, "\"missionState\"")
}

fn graphql_mission_write_restore(url: &str) -> Result<(), String> {
    let original = query_detumbling(url)?;
    let test_value = !original;

    set_detumbling(url, test_value)?;
    let observed = query_detumbling(url)?;
    if observed != test_value {
        let restore_result = set_detumbling(url, original);
        return match restore_result {
            Ok(()) => Err(format!(
                "detumbling_complete was {observed}, expected {test_value}; original restored"
            )),
            Err(err) => Err(format!(
                "detumbling_complete was {observed}, expected {test_value}; restore also failed: {err}"
            )),
        };
    }

    set_detumbling(url, original)?;

    let restored = query_detumbling(url)?;
    if restored != original {
        return Err(format!(
            "detumbling_complete restore readback was {restored}, expected {original}"
        ));
    }

    Ok(())
}

fn query_detumbling(url: &str) -> Result<bool, String> {
    let body = graphql(url, "{ missionState { detumblingComplete } }")?;
    json_bool(&body, "detumblingComplete")
        .ok_or_else(|| format!("could not find detumblingComplete bool in response: {body}"))
}

fn set_detumbling(url: &str, value: bool) -> Result<(), String> {
    let query = format!(
        "mutation {{ setMissionFlag(key: DETUMBLING_COMPLETE, value: {value}, mirrorToEnv: false) {{ success errors state {{ detumblingComplete }} }} }}"
    );
    let body = graphql(url, &query)?;
    expect_contains(&body, "\"success\":true")?;
    let observed = json_bool(&body, "detumblingComplete")
        .ok_or_else(|| format!("could not find detumblingComplete bool in response: {body}"))?;
    if observed == value {
        Ok(())
    } else {
        Err(format!(
            "detumblingComplete write returned {observed}, expected {value}"
        ))
    }
}

fn graphql(url: &str, query: &str) -> Result<String, String> {
    let endpoint = Endpoint::parse(url)?;
    let json = format!("{{\"query\":\"{}\"}}", json_escape(query));
    let mut stream = TcpStream::connect((endpoint.host.as_str(), endpoint.port))
        .map_err(|err| format!("connect {}:{} failed: {err}", endpoint.host, endpoint.port))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .map_err(|err| format!("set read timeout failed: {err}"))?;
    stream
        .set_write_timeout(Some(Duration::from_secs(10)))
        .map_err(|err| format!("set write timeout failed: {err}"))?;

    let request = format!(
        "POST {} HTTP/1.1\r\n\
         Host: {}\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        endpoint.path,
        endpoint.host,
        json.len(),
        json
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|err| format!("HTTP write failed: {err}"))?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|err| format!("HTTP read failed: {err}"))?;

    let (headers, body) = response
        .split_once("\r\n\r\n")
        .ok_or_else(|| format!("invalid HTTP response: {response}"))?;
    if !headers.starts_with("HTTP/1.1 200") && !headers.starts_with("HTTP/1.0 200") {
        return Err(format!("non-200 response: {headers}\n{body}"));
    }
    if body.contains("\"errors\"") {
        return Err(format!("GraphQL returned errors: {body}"));
    }

    Ok(body.to_string())
}

fn expect_contains(body: &str, needle: &str) -> Result<(), String> {
    if body.contains(needle) {
        Ok(())
    } else {
        Err(format!("response did not contain {needle}: {body}"))
    }
}

fn json_bool(body: &str, field: &str) -> Option<bool> {
    let true_pattern = format!("\"{field}\":true");
    let false_pattern = format!("\"{field}\":false");
    if body.contains(&true_pattern) {
        Some(true)
    } else if body.contains(&false_pattern) {
        Some(false)
    } else {
        None
    }
}

fn json_escape(value: &str) -> String {
    let mut out = String::new();
    for c in value.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out
}

#[derive(Debug)]
struct Endpoint {
    host: String,
    port: u16,
    path: String,
}

impl Endpoint {
    fn parse(url: &str) -> Result<Self, String> {
        let rest = url
            .strip_prefix("http://")
            .ok_or_else(|| "only http:// URLs are supported".to_string())?;
        let (host_port, path) = rest.split_once('/').unwrap_or((rest, "graphql"));
        let (host, port) = match host_port.rsplit_once(':') {
            Some((host, port)) => (
                host.to_string(),
                port.parse::<u16>()
                    .map_err(|err| format!("invalid URL port '{port}': {err}"))?,
            ),
            None => (host_port.to_string(), 80),
        };
        if host.is_empty() {
            return Err("URL host is empty".to_string());
        }
        Ok(Self {
            host,
            port,
            path: format!("/{path}"),
        })
    }
}
