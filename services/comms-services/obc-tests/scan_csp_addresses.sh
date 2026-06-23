#!/bin/sh

LC_ALL=C
export LC_ALL

DIR="$(CDPATH= cd "$(dirname "$0")" && pwd)"
cd "$DIR"

CONFIG_FILE="${CONFIG:-config/comms-hw.toml}"
RUN_SCRIPT="${RUN_SCRIPT:-./run.sh}"
URL="${URL:-http://127.0.0.1:8150/graphql}"
REQ_DIR="${REQ_DIR:-requests}"
DELAY_MS="${DELAY_MS:-1000}"
RESTORE_CONFIG="${RESTORE_CONFIG:-1}"
SHOW_OUTPUT="${SHOW_OUTPUT:-0}"
ALLOW_EXISTING_SERVICE="${ALLOW_EXISTING_SERVICE:-0}"
CSP_MIN_ADDRESS=0
CSP_MAX_ADDRESS=31
START_ADDRESS="${START_ADDRESS:-$CSP_MIN_ADDRESS}"
END_ADDRESS="${END_ADDRESS:-$CSP_MAX_ADDRESS}"
CONTROLLER_TIMEOUT_RETRIES="${CONTROLLER_TIMEOUT_RETRIES:-1}"
SCAN_OPERATION="${SCAN_OPERATION:-ping}"
MORSE_SOURCE="${MORSE_SOURCE:-SAT1}"
MORSE_TEXT="${MORSE_TEXT:-sixseven}"

usage() {
  cat <<'EOF'
Usage:
  ./scan_csp_addresses.sh [delay-ms]
  ./scan_csp_addresses.sh --delay-ms 250
  ./scan_csp_addresses.sh --start 4 --end 12
  ./scan_csp_addresses.sh --range 4-12

Scans CSP v1 node addresses 0 through 31 by rewriting:
  config/comms-hw.toml -> [comms-services.radios.uplink].csp_node

For each address it runs:
  ./run.sh ping UPLINK 2

Environment:
  DELAY_MS=1000        Delay between runs, in milliseconds.
  START_ADDRESS=0      First CSP v1 address to scan.
  END_ADDRESS=31       Last CSP v1 address to scan.
  CONTROLLER_TIMEOUT_RETRIES=1
                       Retry the same address after this many controller timeouts.
                       If dmesg is unavailable, retry unsuccessful attempts because
                       controller timeout messages may not be visible to the script.
  CONFIG=path          Config file to rewrite. Defaults to config/comms-hw.toml.
  RUN_SCRIPT=path      Runner to execute. Defaults to ./run.sh.
  URL=url              GraphQL URL used by run.sh. Defaults to port 8150.
  REQ_DIR=path         Request fixture directory. Defaults to requests.
  RESTORE_CONFIG=0     Leave the config at the last scanned address.
  SHOW_OUTPUT=1        Print full run.sh output for every address.
  ALLOW_EXISTING_SERVICE=1
                       Skip the guard for an already-running service.

Internal scanner selection:
  SCAN_OPERATION=ping|morse-text
                       Defaults to ping. csp_scan_with_morse.sh selects
                       morse-text and supplies MORSE_SOURCE and MORSE_TEXT.
EOF
}

is_uint() {
  case "$1" in
    ''|*[!0123456789]*) return 1 ;;
    *) return 0 ;;
  esac
}

normalize_uint() {
  value="$1"
  while [ "${value#0}" != "$value" ] && [ "$value" != "0" ]; do
    value="${value#0}"
  done
  printf '%s\n' "$value"
}

set_address_range() {
  address_range="$1"
  case "$address_range" in
    *-*) ;;
    *)
      echo "--range requires START-END" >&2
      exit 2
      ;;
  esac
  START_ADDRESS="${address_range%-*}"
  END_ADDRESS="${address_range#*-}"
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    -h|--help)
      usage
      exit 0
      ;;
    --delay-ms)
      [ "$#" -ge 2 ] || { echo "--delay-ms requires a value" >&2; exit 2; }
      DELAY_MS="$2"
      shift 2
      ;;
    --delay-ms=*)
      DELAY_MS="${1#*=}"
      shift
      ;;
    --start)
      [ "$#" -ge 2 ] || { echo "--start requires a value" >&2; exit 2; }
      START_ADDRESS="$2"
      shift 2
      ;;
    --start=*)
      START_ADDRESS="${1#*=}"
      shift
      ;;
    --end)
      [ "$#" -ge 2 ] || { echo "--end requires a value" >&2; exit 2; }
      END_ADDRESS="$2"
      shift 2
      ;;
    --end=*)
      END_ADDRESS="${1#*=}"
      shift
      ;;
    --range)
      [ "$#" -ge 2 ] || { echo "--range requires a value" >&2; exit 2; }
      set_address_range "$2"
      shift 2
      ;;
    --range=*)
      set_address_range "${1#*=}"
      shift
      ;;
    *)
      if [ "${delay_arg_seen:-0}" = "1" ]; then
        usage >&2
        exit 2
      fi
      DELAY_MS="$1"
      delay_arg_seen=1
      shift
      ;;
  esac
done

if ! is_uint "$DELAY_MS"; then
  echo "delay must be a non-negative integer number of milliseconds: $DELAY_MS" >&2
  exit 2
fi
DELAY_MS="$(normalize_uint "$DELAY_MS")"

if ! is_uint "$START_ADDRESS"; then
  echo "start address must be a CSP v1 address ($CSP_MIN_ADDRESS-$CSP_MAX_ADDRESS): $START_ADDRESS" >&2
  exit 2
fi
START_ADDRESS="$(normalize_uint "$START_ADDRESS")"
if [ "$START_ADDRESS" -lt "$CSP_MIN_ADDRESS" ] || [ "$START_ADDRESS" -gt "$CSP_MAX_ADDRESS" ]; then
  echo "start address must be within CSP v1 bounds ($CSP_MIN_ADDRESS-$CSP_MAX_ADDRESS): $START_ADDRESS" >&2
  exit 2
fi

if ! is_uint "$END_ADDRESS"; then
  echo "end address must be a CSP v1 address ($CSP_MIN_ADDRESS-$CSP_MAX_ADDRESS): $END_ADDRESS" >&2
  exit 2
fi
END_ADDRESS="$(normalize_uint "$END_ADDRESS")"
if [ "$END_ADDRESS" -lt "$CSP_MIN_ADDRESS" ] || [ "$END_ADDRESS" -gt "$CSP_MAX_ADDRESS" ]; then
  echo "end address must be within CSP v1 bounds ($CSP_MIN_ADDRESS-$CSP_MAX_ADDRESS): $END_ADDRESS" >&2
  exit 2
fi

if [ "$START_ADDRESS" -gt "$END_ADDRESS" ]; then
  echo "start address must be less than or equal to end address: $START_ADDRESS > $END_ADDRESS" >&2
  exit 2
fi

if ! is_uint "$CONTROLLER_TIMEOUT_RETRIES"; then
  echo "controller timeout retries must be a non-negative integer: $CONTROLLER_TIMEOUT_RETRIES" >&2
  exit 2
fi
CONTROLLER_TIMEOUT_RETRIES="$(normalize_uint "$CONTROLLER_TIMEOUT_RETRIES")"

case "$SCAN_OPERATION" in
  ping) ;;
  morse-text)
    if [ "${#MORSE_SOURCE}" -ne 4 ] ||
       ! printf '%s\n' "$MORSE_SOURCE" | grep '^[ -~][ -~][ -~][ -~]$' >/dev/null 2>&1; then
      echo "Morse source identification must be exactly 4 printable ASCII bytes: $MORSE_SOURCE" >&2
      exit 2
    fi
    if [ "${#MORSE_TEXT}" -gt 20 ] ||
       ! printf '%s\n' "$MORSE_TEXT" | grep '^[ -~]*$' >/dev/null 2>&1; then
      echo "Morse text must contain at most 20 printable ASCII bytes" >&2
      exit 2
    fi
    ;;
  *)
    echo "unsupported scan operation: $SCAN_OPERATION" >&2
    exit 2
    ;;
esac

if [ ! -f "$CONFIG_FILE" ]; then
  echo "config file not found: $CONFIG_FILE" >&2
  exit 2
fi

if [ ! -x "$RUN_SCRIPT" ]; then
  echo "runner is not executable: $RUN_SCRIPT" >&2
  exit 2
fi

service_ready() {
  [ -f "$REQ_DIR/00_ping.json" ] || return 1
  curl -fsS "$URL" \
    -H 'content-type: application/json' \
    --data @"$REQ_DIR/00_ping.json" >/dev/null 2>&1
}

if [ "$ALLOW_EXISTING_SERVICE" != "1" ] && service_ready; then
  echo "a comms service already responds at $URL" >&2
  echo "stop it before scanning so each run loads the rewritten config, or set ALLOW_EXISTING_SERVICE=1" >&2
  exit 2
fi

TMP_ROOT="${TMPDIR:-/tmp}/scan-csp-addresses.$$"
if ! mkdir "$TMP_ROOT"; then
  echo "failed to create temporary directory: $TMP_ROOT" >&2
  exit 1
fi

CONFIG_BACKUP="$TMP_ROOT/comms-hw.toml.backup"
CONFIG_TMP="$TMP_ROOT/comms-hw.toml.tmp"
if ! cp "$CONFIG_FILE" "$CONFIG_BACKUP"; then
  echo "failed to back up config file: $CONFIG_FILE" >&2
  rm -f "$CONFIG_BACKUP"
  rmdir "$TMP_ROOT" 2>/dev/null || true
  exit 1
fi

cleanup() {
  status=$?
  if [ "$RESTORE_CONFIG" = "1" ] && [ -f "$CONFIG_BACKUP" ]; then
    cp "$CONFIG_BACKUP" "$CONFIG_FILE"
  fi
  rm -f "$CONFIG_BACKUP" "$CONFIG_TMP"
  rmdir "$TMP_ROOT" 2>/dev/null || true
  return "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

set_uplink_csp_node() {
  address="$1"
  tmp_file="$CONFIG_TMP"

  if ! awk -v new_node="$address" '
    /^\[comms-services\.radios\.uplink\][[:space:]]*$/ {
      in_uplink = 1
      seen_uplink = 1
      print
      next
    }

    /^\[[^]]+\][[:space:]]*$/ {
      in_uplink = 0
    }

    in_uplink && /^[[:space:]]*csp_node[[:space:]]*=/ {
      sub(/[[:space:]]*=[[:space:]]*[0-9][0-9]*/, " = " new_node)
      updated = 1
    }

    { print }

    END {
      if (!seen_uplink || !updated) {
        exit 42
      }
    }
  ' "$CONFIG_FILE" > "$tmp_file"; then
    rm -f "$tmp_file"
    echo "failed to update [comms-services.radios.uplink].csp_node in $CONFIG_FILE" >&2
    exit 1
  fi

  if ! cp "$tmp_file" "$CONFIG_FILE"; then
    rm -f "$tmp_file"
    echo "failed to write updated config file: $CONFIG_FILE" >&2
    exit 1
  fi
  rm -f "$tmp_file"
}

sleep_ms() {
  milliseconds="$1"
  [ "$milliseconds" -eq 0 ] && return 0

  if command -v usleep >/dev/null 2>&1; then
    usleep "$((milliseconds * 1000))"
  else
    seconds=$((milliseconds / 1000))
    if [ $((milliseconds % 1000)) -ne 0 ]; then
      seconds=$((seconds + 1))
    fi
    sleep "$seconds"
  fi
}

is_successful_interaction() {
  output="$1"
  printf '%s\n' "$output" | grep '"errors"[[:space:]]*:' >/dev/null 2>&1 && return 1

  case "$SCAN_OPERATION" in
    ping)
      printf '%s\n' "$output" | grep '"roundTripMs"[[:space:]]*:[[:space:]]*[0-9][0-9]*' >/dev/null 2>&1
      ;;
    morse-text)
      printf '%s\n' "$output" | grep '"success"[[:space:]]*:[[:space:]]*true' >/dev/null 2>&1
      ;;
  esac
}

run_scan_command() {
  case "$SCAN_OPERATION" in
    ping)
      START_SERVICE=1 CONFIG="$CONFIG_FILE" "$RUN_SCRIPT" ping UPLINK 2
      ;;
    morse-text)
      START_SERVICE=1 CONFIG="$CONFIG_FILE" "$RUN_SCRIPT" \
        morse-text UPLINK "$MORSE_SOURCE" "$MORSE_TEXT"
      ;;
  esac
}

is_controller_timeout() {
  output="$1"
  printf '%s\n' "$output" | grep -i 'controller timed out' >/dev/null 2>&1
}

can_check_controller_timeout_log() {
  [ "${CAN_CHECK_CONTROLLER_TIMEOUT_LOG:-0}" = "1" ]
}

controller_timeout_count() {
  if can_check_controller_timeout_log; then
    dmesg 2>/dev/null | grep -i -c 'controller timed out'
  else
    printf '0\n'
  fi
}

successful_addresses=""
successful_count=0

CAN_CHECK_CONTROLLER_TIMEOUT_LOG=0
if command -v dmesg >/dev/null 2>&1 && dmesg >/dev/null 2>&1; then
  CAN_CHECK_CONTROLLER_TIMEOUT_LOG=1
fi

echo "Scanning UPLINK CSP v1 node addresses $START_ADDRESS through $END_ADDRESS"
echo "Config: $CONFIG_FILE"
echo "URL: $URL"
case "$SCAN_OPERATION" in
  ping)
    echo "Command: ./run.sh ping UPLINK 2"
    ;;
  morse-text)
    echo "Command: ./run.sh morse-text UPLINK $MORSE_SOURCE \"$MORSE_TEXT\""
    ;;
esac
echo "Delay between runs: ${DELAY_MS}ms"
echo "Controller timeout retries: $CONTROLLER_TIMEOUT_RETRIES"
if [ "$CONTROLLER_TIMEOUT_RETRIES" -gt 0 ] && ! can_check_controller_timeout_log; then
  echo "dmesg not found; unsuccessful attempts will be retried because controller timeouts may not be visible."
fi
if [ "$DELAY_MS" -gt 0 ] && ! command -v usleep >/dev/null 2>&1; then
  echo "usleep not found; sub-second delays will be rounded up for BusyBox sleep."
fi
if [ "$RESTORE_CONFIG" = "1" ]; then
  echo "Config will be restored when the scan exits."
fi
echo

address="$START_ADDRESS"
while [ "$address" -le "$END_ADDRESS" ]; do
  case "$SCAN_OPERATION" in
    ping)
      printf '[%02d] ./run.sh ping UPLINK 2 ... ' "$address"
      ;;
    morse-text)
      printf '[%02d] ./run.sh morse-text UPLINK %s "%s" ... ' \
        "$address" "$MORSE_SOURCE" "$MORSE_TEXT"
      ;;
  esac
  set_uplink_csp_node "$address"

  attempt=0
  while :; do
    if [ "$attempt" -gt 0 ]; then
      printf 'retry %d/%d ... ' "$attempt" "$CONTROLLER_TIMEOUT_RETRIES"
    fi

    controller_timeout_before="$(controller_timeout_count)"
    if output=$(run_scan_command 2>&1); then
      run_status=0
    else
      run_status=$?
    fi
    controller_timeout_after="$(controller_timeout_count)"
    controller_timed_out=0
    if is_controller_timeout "$output"; then
      controller_timed_out=1
    elif [ "$controller_timeout_after" -gt "$controller_timeout_before" ]; then
      controller_timed_out=1
    fi
    retry_after_failure=0
    if [ "$controller_timed_out" = "1" ] || ! can_check_controller_timeout_log; then
      retry_after_failure=1
    fi

    if is_successful_interaction "$output"; then
      break
    fi

    if [ "$retry_after_failure" = "1" ] && [ "$attempt" -lt "$CONTROLLER_TIMEOUT_RETRIES" ]; then
      attempt=$((attempt + 1))
      if [ "$controller_timed_out" = "1" ]; then
        printf 'controller timed out; '
      else
        printf 'retrying unsuccessful attempt; '
      fi
      if [ "$DELAY_MS" -gt 0 ]; then
        sleep_ms "$DELAY_MS"
      fi
      continue
    fi

    break
  done

  if is_successful_interaction "$output"; then
    if [ "$successful_count" -eq 0 ]; then
      successful_addresses="$address"
    else
      successful_addresses="$successful_addresses $address"
    fi
    successful_count=$((successful_count + 1))
    printf 'SUCCESS\n'
    printf '%s\n' "$output" | sed 's/^/  /'
  else
    if [ "${controller_timed_out:-0}" = "1" ]; then
      printf 'controller timed out; '
    fi
    printf 'no successful interaction'
    if [ "$run_status" -ne 0 ]; then
      printf ' (exit %d)' "$run_status"
    fi
    printf '\n'

    if [ "$SHOW_OUTPUT" = "1" ]; then
      printf '%s\n' "$output" | sed 's/^/  /'
    fi
  fi

  if [ "$address" -lt "$END_ADDRESS" ]; then
    sleep_ms "$DELAY_MS"
  fi

  address=$((address + 1))
done

echo
if [ "$successful_count" -eq 0 ]; then
  case "$SCAN_OPERATION" in
    ping)
      echo "No CSP address in $START_ADDRESS through $END_ADDRESS produced a successful UPLINK ping interaction."
      ;;
    morse-text)
      echo "No CSP address in $START_ADDRESS through $END_ADDRESS accepted the UPLINK Morse text command."
      ;;
  esac
else
  case "$SCAN_OPERATION" in
    ping)
      echo "Successful UPLINK CSP address(es): $successful_addresses"
      ;;
    morse-text)
      echo "CSP address(es) that accepted the UPLINK Morse text command: $successful_addresses"
      ;;
  esac
fi
