use std::{fmt, process::ExitCode, str::FromStr};

use clap::{Parser, Subcommand, ValueEnum};
use serde_json::{Map, Value, json};

#[derive(Parser, Debug)]
#[command(about = "Typed GraphQL client for the RADSAT communications service")]
struct Cli {
    /// Communications service GraphQL endpoint.
    #[arg(long, default_value = "http://127.0.0.1:8150/graphql")]
    url: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run a Needronix Management Protocol command.
    Nmp {
        /// Configured radio to target.
        #[arg(value_enum, ignore_case = true)]
        role: RadioRole,

        /// NMP user or superuser key, as an unsigned decimal integer.
        key: u32,

        #[command(subcommand)]
        command: NmpCommand,
    },
}

#[derive(Subcommand, Debug)]
enum NmpCommand {
    /// Read the radio CSP address.
    GetCspAddress,
    /// Read all 32 CSP route-table entries.
    GetRouteTable,
    /// Read whether packets heard on RS-485 are routed.
    GetRoutingFromRs485,
    /// Read the operating frequency in Hz.
    GetFrequency,
    /// Read the configured uplink/downlink role.
    GetLinkType,
    /// Read whether RF transmission is enabled.
    GetTxEnable,
    /// Read the HDLC preamble size.
    GetPreambleSize,
    /// Read whether normal power (rather than micropower) is selected.
    GetNormalPower,
    /// Read the ground-segment receive-to-transmit delay in milliseconds.
    GetGsRxTxDelay,
    /// Read whether the ham-radio digipeater is enabled.
    GetDigipeaterEnable,
    /// Read the six-byte AX.25 callsign.
    GetCallsign,
    /// Read the custom Morse message.
    GetMorseCustomMessage,
    /// Read the four-byte custom Morse identification.
    GetMorseCustomIdent,
    /// Read first-start-on-orbit inhibit status and timers.
    GetFsoo,
    /// Read housekeeping RSSI diagnostics.
    GetHkRssi,
    /// Read the RSSI moving-average contribution ratio.
    GetRssiContributionRatio,
    /// Read interface, radio, AX.25, digipeater, Morse, and ADC configuration.
    GetConfig1,
    /// Read CSP routes and identification configuration.
    GetConfig2,
    /// Read the communication watchdog reset period in hours.
    GetCheckCommResetPeriod,
    /// Read the AX.25 status and Morse telemetry periods.
    GetSystemStatusAndMorsePeriod,
    /// Read firmware and constants CRC-32 values.
    GetFwCrc,
    /// Read the full radio hostname.
    GetHostname,
    /// Unlock protected NMP writes for ten minutes.
    Unlock,
    /// Change the CSP/I2C address. This can make the current route unreachable.
    SetCspAddress { csp_address: u8 },
    /// Clear all CSP routes. This is a high-risk operation.
    ClearRouteTable,
    /// Set one or more CSP routes in one transaction.
    SetRoutes {
        /// One or more CSP_ADDRESS:DESTINATION_INTERFACE:NEXT_HOP triplets.
        #[arg(required = true, value_name = "CSP:INTERFACE:NEXT_HOP")]
        routes: Vec<Route>,
    },
    /// Enable or disable routing of packets heard on RS-485.
    SetRoutingFromRs485 { enabled: bool },
    /// Set the operating frequency in Hz.
    SetFrequency { frequency_hz: u32 },
    /// Configure the radio as an uplink or downlink.
    SetLinkType {
        #[arg(value_enum, ignore_case = true)]
        link_type: RadioLinkType,
    },
    /// Enable or disable RF transmission.
    SetTxEnable { enabled: bool },
    /// Set the HDLC preamble size (25 through 300).
    SetPreambleSize { size: u16 },
    /// Select normal power (true) or micropower (false).
    SetNormalPower { normal_power: bool },
    /// Set the ground-segment receive-to-transmit delay in milliseconds.
    SetGsRxTxDelay { delay_ms: u16 },
    /// Enable or disable the ham-radio digipeater.
    SetDigipeaterEnable { enabled: bool },
    /// Set the fixed six-byte AX.25 callsign.
    SetCallsign {
        callsign: String,
        #[arg(long, value_enum, ignore_case = true, default_value = "text")]
        format: DataFormat,
    },
    /// Set the custom Morse message (up to 20 ASCII bytes).
    SetMorseCustomMessage { message: String },
    /// Set the fixed four-byte custom Morse identification.
    SetMorseCustomIdent {
        ident: String,
        #[arg(long, value_enum, ignore_case = true, default_value = "text")]
        format: DataFormat,
    },
    /// Replace the NMP user key.
    SetUserKey { new_user_key: u32 },
    /// Replace the NMP superuser key.
    SetSuperuserKey { new_superuser_key: u32 },
    /// Replace the fixed five-byte ITU shutdown key.
    SetItuKey {
        itu_key: String,
        #[arg(long, value_enum, ignore_case = true, default_value = "text")]
        format: DataFormat,
    },
    /// Restore the saved factory/backup profile into the active profile.
    CopyFactoryToActive,
    /// Save the active profile into the factory/backup profile.
    CopyActiveToFactory,
    /// Generate the one-second reset-output signal.
    GenerateResetSignal,
    /// Configure first-start-on-orbit transmission inhibit.
    SetFsoo { inhibit: bool, period_ms: u64 },
    /// Clear and return the persistent and last reset-cause bytes.
    ResetStatusBytes,
    /// Set the RSSI moving-average contribution ratio (1 through 99 percent).
    SetRssiContributionRatio { ratio: u8 },
    /// Set the communication watchdog reset period (12 through 96 hours).
    SetCheckCommResetPeriod { hours: u16 },
    /// Set the AX.25 status and Morse telemetry periods in milliseconds.
    SetSystemStatusAndMorsePeriod {
        ax25_system_status_period_ms: u32,
        morse_task_period_ms: u64,
    },
    /// Set the writable six-byte user portion of the hostname.
    SetHostnameUserPart {
        user_part: String,
        #[arg(long, value_enum, ignore_case = true, default_value = "text")]
        format: DataFormat,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum RadioRole {
    Uplink,
    Downlink,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum RadioLinkType {
    Uplink,
    Downlink,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum DataFormat {
    Text,
    Hex,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Route {
    csp_address: u8,
    destination_interface: u8,
    next_hop: u8,
}

struct GraphqlRequest {
    query: &'static str,
    variables: Value,
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let request = match cli.command {
        Command::Nmp { role, key, command } => command.graphql_request(role, key),
    };

    post_graphql(&cli.url, request)
}

fn post_graphql(url: &str, request: GraphqlRequest) -> Result<(), String> {
    let body = json!({
        "query": request.query,
        "variables": request.variables,
    })
    .to_string();

    let response = match ureq::post(url)
        .set("content-type", "application/json")
        .send_string(&body)
    {
        Ok(response) => response,
        Err(ureq::Error::Status(status, response)) => {
            let body = response
                .into_string()
                .unwrap_or_else(|_| "<unreadable response body>".to_string());
            return Err(format!("HTTP {status} from {url}: {body}"));
        }
        Err(ureq::Error::Transport(err)) => {
            return Err(format!("could not POST to {url}: {err}"));
        }
    };

    let body = response
        .into_string()
        .map_err(|err| format!("could not read response from {url}: {err}"))?;
    let value: Value = serde_json::from_str(&body)
        .map_err(|err| format!("service returned invalid JSON: {err}; body: {body}"))?;
    println!(
        "{}",
        serde_json::to_string_pretty(&value)
            .map_err(|err| format!("could not format response JSON: {err}"))?
    );

    if value
        .get("errors")
        .and_then(Value::as_array)
        .is_some_and(|errors| !errors.is_empty())
    {
        return Err("GraphQL command failed".to_string());
    }

    Ok(())
}

impl NmpCommand {
    fn graphql_request(self, role: RadioRole, key: u32) -> GraphqlRequest {
        match self {
            Self::GetCspAddress => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetCspAddress(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::GetRouteTable => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetRouteTable(role: $role, key: $key) { cspAddress destinationInterface nextHop } }",
                role,
                key,
                [],
            ),
            Self::GetRoutingFromRs485 => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetRoutingFromRs485(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::GetFrequency => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetFrequency(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::GetLinkType => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetLinkType(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::GetTxEnable => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetTxEnable(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::GetPreambleSize => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetPreambleSize(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::GetNormalPower => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetNormalPower(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::GetGsRxTxDelay => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetGsRxTxDelay(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::GetDigipeaterEnable => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetDigipeaterEnable(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::GetCallsign => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetCallsign(role: $role, key: $key) { text hex } }",
                role,
                key,
                [],
            ),
            Self::GetMorseCustomMessage => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetMorseCustomMessage(role: $role, key: $key) { text hex } }",
                role,
                key,
                [],
            ),
            Self::GetMorseCustomIdent => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetMorseCustomIdent(role: $role, key: $key) { text hex } }",
                role,
                key,
                [],
            ),
            Self::GetFsoo => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetFsoo(role: $role, key: $key) { inhibitEnabled timerEnabled periodMs timeLeftMs backupTimerEnabled backupUptimeToFireS } }",
                role,
                key,
                [],
            ),
            Self::GetHkRssi => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetHkRssi(role: $role, key: $key) { rssiRxImmediate rssiRxAvg rssiRxMax rssiBackgroundImmediate rssiBackgroundAvg rssiBackgroundMax } }",
                role,
                key,
                [],
            ),
            Self::GetRssiContributionRatio => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetRssiContributionRatio(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::GetConfig1 => request(
                concat!(
                    "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetConfig1(role: $role, key: $key) { ",
                    "interface { cspInterfaceTxTimeout i2c0RxToTxBusRecoveryDelay i2c2RxToTxBusRecoveryDelay i2cTxTimeout rs485RxToTxBusRecoveryDelay rs485TxTimeout adfDigitalTxTimeout adfMorseTxTimeout groundSegmentTxRxReconfigDelay } ",
                    "radio { frequencyHz lowestConfigurableFrequencyHz highestConfigurableFrequencyHz linkType hdlcPreambleSize txEnabled normalOutputPower firstStartOnOrbitInhibitPeriodMs communicationCheckResetPeriodHours } ",
                    "ax25 { callsign { text hex } ssid systemStatusTimerPeriodMs externalBeaconInhibitPeriodMs } ",
                    "digipeater { enabled messageRetransmissionCount messageRetransmissionIntervalMs needronixEnabled customerEnabled } ",
                    "morse { taskPeriodAfterStartupMs taskPeriodMs inhibitPeriodAfterStartupMs inhibitPeriodMs taskTxAttemptTimeoutMs customIdent { text hex } maxStringLength customMessage { text hex } } ",
                    "adc { rssiNewSampleContributionRatio } } }",
                ),
                role,
                key,
                [],
            ),
            Self::GetConfig2 => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetConfig2(role: $role, key: $key) { cspAddress routeTable { cspAddress destinationInterface nextHop } routingFromRs485Enabled identification { text hex } } }",
                role,
                key,
                [],
            ),
            Self::GetCheckCommResetPeriod => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetCheckCommResetPeriod(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::GetSystemStatusAndMorsePeriod => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetSystemStatusAndMorsePeriod(role: $role, key: $key) { ax25SystemStatusPeriodMs morseTaskPeriodMs } }",
                role,
                key,
                [],
            ),
            Self::GetFwCrc => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetFwCrc(role: $role, key: $key) { firmwareCrc32 constantsCrc32 firmwareCrc32Hex constantsCrc32Hex } }",
                role,
                key,
                [],
            ),
            Self::GetHostname => request(
                "query Nmp($role: RadioRole!, $key: Int!) { radioNmpGetHostname(role: $role, key: $key) { text hex } }",
                role,
                key,
                [],
            ),
            Self::Unlock => request(
                "mutation Nmp($role: RadioRole!, $key: Int!) { radioNmpUnlock(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::SetCspAddress { csp_address } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $cspAddress: Int!) { radioNmpSetCspAddress(role: $role, key: $key, cspAddress: $cspAddress) }",
                role,
                key,
                [("cspAddress", json!(csp_address))],
            ),
            Self::ClearRouteTable => request(
                "mutation Nmp($role: RadioRole!, $key: Int!) { radioNmpClearRouteTable(role: $role, key: $key) { cspAddress destinationInterface nextHop } }",
                role,
                key,
                [],
            ),
            Self::SetRoutes { routes } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $routes: [NmpRouteInput!]!) { radioNmpSetRoutes(role: $role, key: $key, routes: $routes) { cspAddress destinationInterface nextHop } }",
                role,
                key,
                [(
                    "routes",
                    Value::Array(routes.into_iter().map(Route::graphql_value).collect()),
                )],
            ),
            Self::SetRoutingFromRs485 { enabled } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $enabled: Boolean!) { radioNmpSetRoutingFromRs485(role: $role, key: $key, enabled: $enabled) }",
                role,
                key,
                [("enabled", json!(enabled))],
            ),
            Self::SetFrequency { frequency_hz } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $frequencyHz: Int!) { radioNmpSetFrequency(role: $role, key: $key, frequencyHz: $frequencyHz) }",
                role,
                key,
                [("frequencyHz", json!(frequency_hz))],
            ),
            Self::SetLinkType { link_type } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $linkType: NmpRadioLinkType!) { radioNmpSetLinkType(role: $role, key: $key, linkType: $linkType) }",
                role,
                key,
                [("linkType", json!(link_type.graphql_name()))],
            ),
            Self::SetTxEnable { enabled } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $enabled: Boolean!) { radioNmpSetTxEnable(role: $role, key: $key, enabled: $enabled) }",
                role,
                key,
                [("enabled", json!(enabled))],
            ),
            Self::SetPreambleSize { size } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $size: Int!) { radioNmpSetPreambleSize(role: $role, key: $key, size: $size) }",
                role,
                key,
                [("size", json!(size))],
            ),
            Self::SetNormalPower { normal_power } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $normalPower: Boolean!) { radioNmpSetNormalPower(role: $role, key: $key, normalPower: $normalPower) }",
                role,
                key,
                [("normalPower", json!(normal_power))],
            ),
            Self::SetGsRxTxDelay { delay_ms } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $delayMs: Int!) { radioNmpSetGsRxTxDelay(role: $role, key: $key, delayMs: $delayMs) }",
                role,
                key,
                [("delayMs", json!(delay_ms))],
            ),
            Self::SetDigipeaterEnable { enabled } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $enabled: Boolean!) { radioNmpSetDigipeaterEnable(role: $role, key: $key, enabled: $enabled) }",
                role,
                key,
                [("enabled", json!(enabled))],
            ),
            Self::SetCallsign { callsign, format } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $callsign: String!, $format: NmpDataFormat!) { radioNmpSetCallsign(role: $role, key: $key, callsign: $callsign, format: $format) { text hex } }",
                role,
                key,
                [
                    ("callsign", json!(callsign)),
                    ("format", json!(format.graphql_name())),
                ],
            ),
            Self::SetMorseCustomMessage { message } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $message: String!) { radioNmpSetMorseCustomMessage(role: $role, key: $key, message: $message) { text hex } }",
                role,
                key,
                [("message", json!(message))],
            ),
            Self::SetMorseCustomIdent { ident, format } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $ident: String!, $format: NmpDataFormat!) { radioNmpSetMorseCustomIdent(role: $role, key: $key, ident: $ident, format: $format) { text hex } }",
                role,
                key,
                [
                    ("ident", json!(ident)),
                    ("format", json!(format.graphql_name())),
                ],
            ),
            Self::SetUserKey { new_user_key } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $newUserKey: Int!) { radioNmpSetUserKey(role: $role, key: $key, newUserKey: $newUserKey) }",
                role,
                key,
                [("newUserKey", json!(new_user_key))],
            ),
            Self::SetSuperuserKey { new_superuser_key } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $newSuperuserKey: Int!) { radioNmpSetSuperuserKey(role: $role, key: $key, newSuperuserKey: $newSuperuserKey) }",
                role,
                key,
                [("newSuperuserKey", json!(new_superuser_key))],
            ),
            Self::SetItuKey { itu_key, format } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $ituKey: String!, $format: NmpDataFormat!) { radioNmpSetItuKey(role: $role, key: $key, ituKey: $ituKey, format: $format) }",
                role,
                key,
                [
                    ("ituKey", json!(itu_key)),
                    ("format", json!(format.graphql_name())),
                ],
            ),
            Self::CopyFactoryToActive => request(
                "mutation Nmp($role: RadioRole!, $key: Int!) { radioNmpCopyFactoryToActive(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::CopyActiveToFactory => request(
                "mutation Nmp($role: RadioRole!, $key: Int!) { radioNmpCopyActiveToFactory(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::GenerateResetSignal => request(
                "mutation Nmp($role: RadioRole!, $key: Int!) { radioNmpGenerateResetSignal(role: $role, key: $key) }",
                role,
                key,
                [],
            ),
            Self::SetFsoo { inhibit, period_ms } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $inhibit: Boolean!, $periodMs: Int!) { radioNmpSetFsoo(role: $role, key: $key, inhibit: $inhibit, periodMs: $periodMs) { inhibitEnabled timerEnabled periodMs timeLeftMs backupTimerEnabled backupUptimeToFireS } }",
                role,
                key,
                [("inhibit", json!(inhibit)), ("periodMs", json!(period_ms))],
            ),
            Self::ResetStatusBytes => request(
                "mutation Nmp($role: RadioRole!, $key: Int!) { radioNmpResetStatusBytes(role: $role, key: $key) { persistent last persistentHex lastHex } }",
                role,
                key,
                [],
            ),
            Self::SetRssiContributionRatio { ratio } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $ratio: Int!) { radioNmpSetRssiContributionRatio(role: $role, key: $key, ratio: $ratio) }",
                role,
                key,
                [("ratio", json!(ratio))],
            ),
            Self::SetCheckCommResetPeriod { hours } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $hours: Int!) { radioNmpSetCheckCommResetPeriod(role: $role, key: $key, hours: $hours) }",
                role,
                key,
                [("hours", json!(hours))],
            ),
            Self::SetSystemStatusAndMorsePeriod {
                ax25_system_status_period_ms,
                morse_task_period_ms,
            } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $ax25PeriodMs: Int!, $morsePeriodMs: Int!) { radioNmpSetSystemStatusAndMorsePeriod(role: $role, key: $key, ax25SystemStatusPeriodMs: $ax25PeriodMs, morseTaskPeriodMs: $morsePeriodMs) { ax25SystemStatusPeriodMs morseTaskPeriodMs } }",
                role,
                key,
                [
                    ("ax25PeriodMs", json!(ax25_system_status_period_ms)),
                    ("morsePeriodMs", json!(morse_task_period_ms)),
                ],
            ),
            Self::SetHostnameUserPart { user_part, format } => request(
                "mutation Nmp($role: RadioRole!, $key: Int!, $userPart: String!, $format: NmpDataFormat!) { radioNmpSetHostnameUserPart(role: $role, key: $key, userPart: $userPart, format: $format) { text hex } }",
                role,
                key,
                [
                    ("userPart", json!(user_part)),
                    ("format", json!(format.graphql_name())),
                ],
            ),
        }
    }
}

fn request<const N: usize>(
    query: &'static str,
    role: RadioRole,
    key: u32,
    extra_variables: [(&'static str, Value); N],
) -> GraphqlRequest {
    let mut variables = Map::new();
    variables.insert("role".to_string(), json!(role.graphql_name()));
    variables.insert("key".to_string(), json!(key));
    for (name, value) in extra_variables {
        variables.insert(name.to_string(), value);
    }

    GraphqlRequest {
        query,
        variables: Value::Object(variables),
    }
}

impl RadioRole {
    fn graphql_name(self) -> &'static str {
        match self {
            Self::Uplink => "UPLINK",
            Self::Downlink => "DOWNLINK",
        }
    }
}

impl RadioLinkType {
    fn graphql_name(self) -> &'static str {
        match self {
            Self::Uplink => "UPLINK",
            Self::Downlink => "DOWNLINK",
        }
    }
}

impl DataFormat {
    fn graphql_name(self) -> &'static str {
        match self {
            Self::Text => "TEXT",
            Self::Hex => "HEX",
        }
    }
}

impl Route {
    fn graphql_value(self) -> Value {
        json!({
            "cspAddress": self.csp_address,
            "destinationInterface": self.destination_interface,
            "nextHop": self.next_hop,
        })
    }
}

impl FromStr for Route {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split(':');
        let csp_address = parse_route_byte(parts.next(), "CSP address", value)?;
        let destination_interface = parse_route_byte(parts.next(), "destination interface", value)?;
        let next_hop = parse_route_byte(parts.next(), "next hop", value)?;
        if parts.next().is_some() {
            return Err(format!(
                "route `{value}` must contain exactly three colon-separated bytes"
            ));
        }

        Ok(Self {
            csp_address,
            destination_interface,
            next_hop,
        })
    }
}

fn parse_route_byte(value: Option<&str>, name: &str, route: &str) -> Result<u8, String> {
    value
        .ok_or_else(|| format!("route `{route}` is missing its {name}"))?
        .parse()
        .map_err(|_| format!("route `{route}` has an invalid {name}"))
}

impl fmt::Display for Route {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}:{}",
            self.csp_address, self.destination_interface, self.next_hop
        )
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptySubscription, Request, Schema, Variables};
    use clap::CommandFactory;
    use comms_services::schema::{MutationRoot, QueryRoot};

    use super::*;

    #[test]
    fn clap_exposes_all_49_nmp_commands() {
        let command = Cli::command();
        let nmp = command
            .get_subcommands()
            .find(|command| command.get_name() == "nmp")
            .unwrap();
        assert_eq!(nmp.get_subcommands().count(), 49);
    }

    #[test]
    fn parses_nmp_read_command_case_insensitively() {
        let cli = Cli::try_parse_from([
            "comms-cli",
            "--url",
            "http://localhost/graphql",
            "nmp",
            "DOWNLINK",
            "123",
            "get-frequency",
        ])
        .unwrap();

        match cli.command {
            Command::Nmp {
                role: RadioRole::Downlink,
                key: 123,
                command: NmpCommand::GetFrequency,
            } => {}
            command => panic!("unexpected command: {command:?}"),
        }
    }

    #[test]
    fn parses_routes_and_builds_graphql_variables() {
        let route: Route = "2:1:8".parse().unwrap();
        assert_eq!(
            route,
            Route {
                csp_address: 2,
                destination_interface: 1,
                next_hop: 8,
            }
        );

        let request = NmpCommand::SetRoutes {
            routes: vec![route],
        }
        .graphql_request(RadioRole::Uplink, 42);
        assert_eq!(request.variables["role"], "UPLINK");
        assert_eq!(request.variables["key"], 42);
        assert_eq!(request.variables["routes"][0]["nextHop"], 8);
    }

    #[test]
    fn rejects_malformed_routes() {
        assert!("2:1".parse::<Route>().is_err());
        assert!("2:1:8:9".parse::<Route>().is_err());
        assert!("256:1:8".parse::<Route>().is_err());
    }

    #[test]
    fn uses_variables_for_string_and_enum_inputs() {
        let request = NmpCommand::SetMorseCustomIdent {
            ident: "52 41 44 31".to_string(),
            format: DataFormat::Hex,
        }
        .graphql_request(RadioRole::Downlink, 7);

        assert_eq!(request.variables["ident"], "52 41 44 31");
        assert_eq!(request.variables["format"], "HEX");
        assert!(request.query.contains("$ident: String!"));
    }

    #[test]
    fn every_cli_operation_validates_against_the_service_schema() {
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
        let commands = vec![
            NmpCommand::GetCspAddress,
            NmpCommand::GetRouteTable,
            NmpCommand::GetRoutingFromRs485,
            NmpCommand::GetFrequency,
            NmpCommand::GetLinkType,
            NmpCommand::GetTxEnable,
            NmpCommand::GetPreambleSize,
            NmpCommand::GetNormalPower,
            NmpCommand::GetGsRxTxDelay,
            NmpCommand::GetDigipeaterEnable,
            NmpCommand::GetCallsign,
            NmpCommand::GetMorseCustomMessage,
            NmpCommand::GetMorseCustomIdent,
            NmpCommand::GetFsoo,
            NmpCommand::GetHkRssi,
            NmpCommand::GetRssiContributionRatio,
            NmpCommand::GetConfig1,
            NmpCommand::GetConfig2,
            NmpCommand::GetCheckCommResetPeriod,
            NmpCommand::GetSystemStatusAndMorsePeriod,
            NmpCommand::GetFwCrc,
            NmpCommand::GetHostname,
            NmpCommand::Unlock,
            NmpCommand::SetCspAddress { csp_address: 8 },
            NmpCommand::ClearRouteTable,
            NmpCommand::SetRoutes {
                routes: vec![Route {
                    csp_address: 2,
                    destination_interface: 1,
                    next_hop: 2,
                }],
            },
            NmpCommand::SetRoutingFromRs485 { enabled: true },
            NmpCommand::SetFrequency {
                frequency_hz: 437_000_000,
            },
            NmpCommand::SetLinkType {
                link_type: RadioLinkType::Downlink,
            },
            NmpCommand::SetTxEnable { enabled: true },
            NmpCommand::SetPreambleSize { size: 100 },
            NmpCommand::SetNormalPower { normal_power: true },
            NmpCommand::SetGsRxTxDelay { delay_ms: 100 },
            NmpCommand::SetDigipeaterEnable { enabled: false },
            NmpCommand::SetCallsign {
                callsign: "RADSAT".to_string(),
                format: DataFormat::Text,
            },
            NmpCommand::SetMorseCustomMessage {
                message: "RADSAT TEST".to_string(),
            },
            NmpCommand::SetMorseCustomIdent {
                ident: "RAD1".to_string(),
                format: DataFormat::Text,
            },
            NmpCommand::SetUserKey { new_user_key: 2 },
            NmpCommand::SetSuperuserKey {
                new_superuser_key: 3,
            },
            NmpCommand::SetItuKey {
                itu_key: "12345".to_string(),
                format: DataFormat::Text,
            },
            NmpCommand::CopyFactoryToActive,
            NmpCommand::CopyActiveToFactory,
            NmpCommand::GenerateResetSignal,
            NmpCommand::SetFsoo {
                inhibit: true,
                period_ms: 2_040_000,
            },
            NmpCommand::ResetStatusBytes,
            NmpCommand::SetRssiContributionRatio { ratio: 10 },
            NmpCommand::SetCheckCommResetPeriod { hours: 24 },
            NmpCommand::SetSystemStatusAndMorsePeriod {
                ax25_system_status_period_ms: 30_000,
                morse_task_period_ms: 90_000,
            },
            NmpCommand::SetHostnameUserPart {
                user_part: "RADSAT".to_string(),
                format: DataFormat::Text,
            },
        ];
        assert_eq!(commands.len(), 49);

        for command in commands {
            let request = command.graphql_request(RadioRole::Downlink, 1);
            let response = futures::executor::block_on(
                schema.execute(
                    Request::new(request.query)
                        .variables(Variables::from_json(request.variables.clone())),
                ),
            );
            assert_eq!(
                response.errors.len(),
                1,
                "operation did not reach its resolver: {}\n{:?}",
                request.query,
                response.errors
            );
            assert!(
                response.errors[0].message.contains("does not exist"),
                "operation failed schema validation: {}\n{:?}",
                request.query,
                response.errors
            );
        }
    }
}
