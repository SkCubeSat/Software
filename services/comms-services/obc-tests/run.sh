#!/bin/sh
set -e

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REQ_DIR="${REQ_DIR:-$DIR/requests}"
URL="${URL:-http://127.0.0.1:8150/graphql}"
SERVICE_BIN="${SERVICE_BIN:-$DIR/bin/comms-services}"
CLI_BIN="${CLI_BIN:-$DIR/bin/comms-cli}"
CONFIG="${CONFIG:-$DIR/config/comms-hw.toml}"
LOG="${LOG:-$DIR/comms-services.log}"
START_SERVICE="${START_SERVICE:-auto}"
RUN_MUTATIONS="${RUN_MUTATIONS:-0}"
RUN_REBOOT="${RUN_REBOOT:-0}"

SERVICE_PID=""

cleanup() {
  if [ -n "$SERVICE_PID" ] && kill -0 "$SERVICE_PID" 2>/dev/null; then
    kill "$SERVICE_PID" 2>/dev/null || true
    wait "$SERVICE_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

usage() {
  cat <<'EOF'
Usage:
  ./run.sh [command] [args...]

Default:
  ./run.sh smoke

Commands:
  smoke
      Run non-transmitting service and radio readback requests.

  all
      Run smoke plus mutation fixtures when RUN_MUTATIONS=1.
      Reboot fixtures run only when RUN_REBOOT=1.

  request <file-or-request-name>
      POST one JSON request fixture. Example:
      ./run.sh request 10_radio_ping_uplink.json

  ping <UPLINK|DOWNLINK> [payload-size]
  uptime <UPLINK|DOWNLINK>
  status <UPLINK|DOWNLINK>
  ident <UPLINK|DOWNLINK>
  iface <UPLINK|DOWNLINK> <RADIO|CSP|I2C0|I2C2|RS485>
  stats <UPLINK|DOWNLINK>

  morse-text <UPLINK|DOWNLINK> <SOURCE4> <TEXT>
  morse-compressed <UPLINK|DOWNLINK> <SOURCE4> <num1> <num2> <num3> <num4> <num5> <num6>
  ax25-text <UPLINK|DOWNLINK> <TEXT>
  ax25-hex <UPLINK|DOWNLINK> <HEX>

  reboot <UPLINK|DOWNLINK>
      Requires CONFIRM_REBOOT=1.

  nmp <UPLINK|DOWNLINK> <KEY> <COMMAND> [args...]
      Run a typed NMP command through comms-cli. Use `./run.sh nmp --help`.

Environment:
  URL=http://127.0.0.1:8150/graphql
  START_SERVICE=auto|1|0
  SERVICE_BIN=/path/to/comms-services
  CLI_BIN=/path/to/comms-cli
  CONFIG=/path/to/comms-hw.toml
  RUN_MUTATIONS=1
  RUN_REBOOT=1
EOF
}

json_escape() {
  printf '%s' "$1" | sed 's/\\/\\\\/g; s/"/\\"/g'
}

normalize_role() {
  role=$(printf '%s' "$1" | tr '[:lower:]' '[:upper:]')
  case "$role" in
    UPLINK|DOWNLINK) printf '%s' "$role" ;;
    *)
      echo "invalid role: $1 (expected UPLINK or DOWNLINK)" >&2
      exit 2
      ;;
  esac
}

normalize_interface() {
  interface=$(printf '%s' "$1" | tr '[:lower:]' '[:upper:]')
  case "$interface" in
    RADIO|CSP|I2C0|I2C2|RS485) printf '%s' "$interface" ;;
    *)
      echo "invalid interface: $1 (expected RADIO, CSP, I2C0, I2C2, or RS485)" >&2
      exit 2
      ;;
  esac
}

post_graphql() {
  name="$1"
  query="$2"
  escaped=$(json_escape "$query")

  printf '\n=== %s ===\n' "$name"
  printf 'POST %s\n' "$URL"
  curl -fsS "$URL" \
    -H 'content-type: application/json' \
    --data "{\"query\":\"$escaped\"}"
  printf '\n'
}

post_file() {
  name="$1"
  file="$2"

  printf '\n=== %s ===\n' "$name"
  printf 'POST %s\n' "$URL"
  curl -fsS "$URL" \
    -H 'content-type: application/json' \
    --data @"$file"
  printf '\n'
}

request_file() {
  file="$1"
  if [ -f "$file" ]; then
    printf '%s' "$file"
    return 0
  fi
  if [ -f "$REQ_DIR/$file" ]; then
    printf '%s' "$REQ_DIR/$file"
    return 0
  fi

  echo "request fixture not found: $file" >&2
  exit 2
}

wait_for_service() {
  i=0
  while [ "$i" -lt 30 ]; do
    if service_ready; then
      return 0
    fi
    i=$((i + 1))
    sleep 1
  done

  echo "comms service did not become ready at $URL" >&2
  if [ -f "$LOG" ]; then
    echo "--- service log ---" >&2
    tail -n 80 "$LOG" >&2 || true
  fi
  return 1
}

service_ready() {
  curl -fsS "$URL" \
    -H 'content-type: application/json' \
    --data @"$REQ_DIR/00_ping.json" >/dev/null 2>&1
}

maybe_start_service() {
  case "$START_SERVICE" in
    1) start=1 ;;
    0) start=0 ;;
    auto)
      if service_ready; then
        start=0
      elif [ -x "$SERVICE_BIN" ]; then
        start=1
      else
        start=0
      fi
      ;;
    *)
      echo "START_SERVICE must be auto, 1, or 0" >&2
      exit 2
      ;;
  esac

  if [ "$start" = "1" ]; then
    if [ ! -x "$SERVICE_BIN" ]; then
      echo "service binary not executable: $SERVICE_BIN" >&2
      exit 2
    fi
    if [ ! -f "$CONFIG" ]; then
      echo "service config not found: $CONFIG" >&2
      exit 2
    fi

    echo "Starting comms service"
    "$SERVICE_BIN" -c "$CONFIG" >"$LOG" 2>&1 &
    SERVICE_PID="$!"
  else
    echo "Using already-running comms service at $URL"
  fi

  wait_for_service
}

run_smoke() {
  post_file "Service ping" "$REQ_DIR/00_ping.json"
  post_file "Telemetry" "$REQ_DIR/01_telemetry.json"
  post_file "Health" "$REQ_DIR/02_health.json"
  post_file "Uplink radio health" "$REQ_DIR/03_radio_health_uplink.json"
  post_file "Downlink radio health" "$REQ_DIR/04_radio_health_downlink.json"
  post_file "Uplink radio ping" "$REQ_DIR/10_radio_ping_uplink.json"
  post_file "Downlink radio ping" "$REQ_DIR/11_radio_ping_downlink.json"
  post_file "Uplink uptime" "$REQ_DIR/12_radio_uptime_uplink.json"
  post_file "Downlink uptime" "$REQ_DIR/13_radio_uptime_downlink.json"
  post_file "Uplink status" "$REQ_DIR/14_radio_status_uplink.json"
  post_file "Downlink status" "$REQ_DIR/15_radio_status_downlink.json"
  post_file "Uplink ident" "$REQ_DIR/16_radio_ident_uplink.json"
  post_file "Downlink ident" "$REQ_DIR/17_radio_ident_downlink.json"
  post_file "Uplink RADIO interface stats" "$REQ_DIR/18_radio_interface_uplink_radio.json"
  post_file "Downlink RADIO interface stats" "$REQ_DIR/19_radio_interface_downlink_radio.json"
  post_file "Uplink system stats" "$REQ_DIR/20_radio_system_stats_uplink.json"
  post_file "Downlink system stats" "$REQ_DIR/21_radio_system_stats_downlink.json"
}

run_mutation_fixtures() {
  if [ "$RUN_MUTATIONS" != "1" ]; then
    echo "RF-transmitting mutation fixtures are disabled. Set RUN_MUTATIONS=1 to run them."
    return 0
  fi

  post_file "Send text Morse downlink" "$REQ_DIR/30_send_text_morse_downlink.json"
  post_file "Send compressed Morse downlink" "$REQ_DIR/31_send_compressed_morse_downlink.json"
  post_file "Send AX.25 text downlink" "$REQ_DIR/32_send_ax25_text_downlink.json"
  post_file "Send AX.25 hex downlink" "$REQ_DIR/33_send_ax25_hex_downlink.json"
}

run_reboot_fixtures() {
  if [ "$RUN_REBOOT" != "1" ]; then
    echo "Radio reboot fixtures are disabled. Set RUN_REBOOT=1 to run them."
    return 0
  fi

  post_file "Reboot uplink radio" "$REQ_DIR/90_reboot_uplink.json"
  post_file "Reboot downlink radio" "$REQ_DIR/91_reboot_downlink.json"
}

cmd="${1:-smoke}"
shift || true

case "$cmd" in
  -h|--help|help)
    usage
    exit 0
    ;;
  nmp)
    [ -x "$CLI_BIN" ] || { echo "comms CLI not executable: $CLI_BIN" >&2; exit 2; }
    if [ "${1:-}" = "-h" ] || [ "${1:-}" = "--help" ] || [ "${1:-}" = "help" ]; then
      exec "$CLI_BIN" --url "$URL" nmp --help
    fi
    ;;
esac

maybe_start_service

case "$cmd" in
  smoke)
    run_smoke
    ;;
  all)
    run_smoke
    run_mutation_fixtures
    run_reboot_fixtures
    ;;
  request)
    [ $# -eq 1 ] || { usage; exit 2; }
    file=$(request_file "$1")
    post_file "Request $1" "$file"
    ;;
  ping)
    [ $# -ge 1 ] && [ $# -le 2 ] || { usage; exit 2; }
    role=$(normalize_role "$1")
    payload="${2:-8}"
    post_graphql "Radio ping $role" "{ radioPing(role: $role, payloadSize: $payload) { role payloadSize roundTripMs } }"
    ;;
  uptime)
    [ $# -eq 1 ] || { usage; exit 2; }
    role=$(normalize_role "$1")
    post_graphql "Radio uptime $role" "{ radioUptime(role: $role) { role seconds } }"
    ;;
  status)
    [ $# -eq 1 ] || { usage; exit 2; }
    role=$(normalize_role "$1")
    post_graphql "Radio status $role" "{ radioStatus(role: $role) { role bufferFree } }"
    ;;
  ident)
    [ $# -eq 1 ] || { usage; exit 2; }
    role=$(normalize_role "$1")
    post_graphql "Radio ident $role" "{ radioIdent(role: $role) { role hostname model revision buildDate buildTime } }"
    ;;
  iface)
    [ $# -eq 2 ] || { usage; exit 2; }
    role=$(normalize_role "$1")
    interface=$(normalize_interface "$2")
    post_graphql "Radio interface $role $interface" "{ radioInterfaceStats(role: $role, interface: $interface) { role interface interfaceName txPacketCount rxPacketCount lockedInTxCount rxOverrunCount txLengthError criticalInternalError txPacketCountAfterReset rxPacketCountAfterReset } }"
    ;;
  stats)
    [ $# -eq 1 ] || { usage; exit 2; }
    role=$(normalize_role "$1")
    post_graphql "Radio system stats $role" "{ radioSystemStats(role: $role) { role systemInfo { cpuVoltage10mv cpuTemperatureKelvin totalResetCount } sriStatus { wdtReset borReset porReset rstNmiReset exitFromLpm5 totalUpTime upTime statusBytesHex } adcData { cpuTempKelvin cpuVoltage10mv extVoltage rssiRxImmediate rssiRxAvg rssiRxMax rssiBackgroundImmediate rssiBackgroundAvg rssiBackgroundMax swr paNtc } } }"
    ;;
  morse-text)
    [ $# -eq 3 ] || { usage; exit 2; }
    role=$(normalize_role "$1")
    source=$(json_escape "$2")
    text=$(json_escape "$3")
    post_graphql "Send text Morse $role" "mutation { radioSendTextInMorse(role: $role, sourceIdentification: \"$source\", text: \"$text\") { role success message verbalResponseText verbalResponseHex } }"
    ;;
  morse-compressed)
    [ $# -eq 8 ] || { usage; exit 2; }
    role=$(normalize_role "$1")
    source=$(json_escape "$2")
    post_graphql "Send compressed Morse $role" "mutation { radioSendCompressedMorse(role: $role, sourceIdentification: \"$source\", num1: $3, num2: $4, num3: $5, num4: $6, num5: $7, num6: $8) { role success message verbalResponseText verbalResponseHex } }"
    ;;
  ax25-text)
    [ $# -eq 2 ] || { usage; exit 2; }
    role=$(normalize_role "$1")
    data=$(json_escape "$2")
    post_graphql "Send AX.25 text $role" "mutation { radioSendAx25Message(role: $role, data: \"$data\", format: TEXT) { role success message verbalResponseText verbalResponseHex } }"
    ;;
  ax25-hex)
    [ $# -eq 2 ] || { usage; exit 2; }
    role=$(normalize_role "$1")
    data=$(json_escape "$2")
    post_graphql "Send AX.25 hex $role" "mutation { radioSendAx25Message(role: $role, data: \"$data\", format: HEX) { role success message verbalResponseText verbalResponseHex } }"
    ;;
  reboot)
    [ $# -eq 1 ] || { usage; exit 2; }
    if [ "${CONFIRM_REBOOT:-0}" != "1" ]; then
      echo "Refusing to reboot radio without CONFIRM_REBOOT=1" >&2
      exit 2
    fi
    role=$(normalize_role "$1")
    post_graphql "Reboot radio $role" "mutation { radioReboot(role: $role) { role success message verbalResponseText verbalResponseHex } }"
    ;;
  nmp)
    "$CLI_BIN" --url "$URL" nmp "$@"
    ;;
  *)
    usage
    exit 2
    ;;
esac
