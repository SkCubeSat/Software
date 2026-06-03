#!/usr/bin/env bash
set -uo pipefail

DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

CONFIG_FILE="${CONFIG:-config/comms-hw.toml}"
RUN_SCRIPT="${RUN_SCRIPT:-./run.sh}"
URL="${URL:-http://127.0.0.1:8150/graphql}"
REQ_DIR="${REQ_DIR:-requests}"
DELAY_MS="${DELAY_MS:-1000}"
RESTORE_CONFIG="${RESTORE_CONFIG:-1}"
SHOW_OUTPUT="${SHOW_OUTPUT:-0}"
ALLOW_EXISTING_SERVICE="${ALLOW_EXISTING_SERVICE:-0}"

usage() {
  cat <<'EOF'
Usage:
  ./scan_csp_addresses.sh [delay-ms]
  ./scan_csp_addresses.sh --delay-ms 250

Scans CSP node addresses 0 through 31 by rewriting:
  config/comms-hw.toml -> [comms-services.radios.uplink].csp_node

For each address it runs:
  ./run.sh ping UPLINK 2

Environment:
  DELAY_MS=1000        Delay between runs, in milliseconds.
  CONFIG=path          Config file to rewrite. Defaults to config/comms-hw.toml.
  RUN_SCRIPT=path      Runner to execute. Defaults to ./run.sh.
  URL=url              GraphQL URL used by run.sh. Defaults to port 8150.
  REQ_DIR=path         Request fixture directory. Defaults to requests.
  RESTORE_CONFIG=0     Leave the config at the last scanned address.
  SHOW_OUTPUT=1        Print full run.sh output for every address.
  ALLOW_EXISTING_SERVICE=1
                       Skip the guard for an already-running service.
EOF
}

is_uint() {
  [[ "$1" =~ ^[0-9]+$ ]]
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

CONFIG_BACKUP="$(mktemp "${TMPDIR:-/tmp}/comms-hw.toml.XXXXXX")" || exit 1
cp "$CONFIG_FILE" "$CONFIG_BACKUP"

cleanup() {
  status=$?
  if [ "$RESTORE_CONFIG" = "1" ] && [ -f "$CONFIG_BACKUP" ]; then
    cp "$CONFIG_BACKUP" "$CONFIG_FILE"
  fi
  rm -f "$CONFIG_BACKUP"
  return "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

set_uplink_csp_node() {
  address="$1"
  tmp_file="$(mktemp "${TMPDIR:-/tmp}/comms-hw-scan.XXXXXX")" || exit 1

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
      sub(/[[:space:]]*=[[:space:]]*[0-9]+/, " = " new_node)
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

  cp "$tmp_file" "$CONFIG_FILE"
  rm -f "$tmp_file"
}

sleep_ms() {
  milliseconds="$1"
  [ "$milliseconds" -eq 0 ] && return 0

  if command -v usleep >/dev/null 2>&1; then
    usleep "$((milliseconds * 1000))"
  else
    sleep "$(awk -v ms="$milliseconds" 'BEGIN { printf "%.3f", ms / 1000 }')"
  fi
}

is_successful_ping() {
  output="$1"
  [[ "$output" =~ \"roundTripMs\"[[:space:]]*:[[:space:]]*[0-9]+ ]] &&
    ! [[ "$output" =~ \"errors\"[[:space:]]*: ]]
}

successful_addresses=()

echo "Scanning UPLINK CSP node addresses 0 through 31"
echo "Config: $CONFIG_FILE"
echo "URL: $URL"
echo "Delay between runs: ${DELAY_MS}ms"
if [ "$RESTORE_CONFIG" = "1" ]; then
  echo "Config will be restored when the scan exits."
fi
echo

for ((address = 0; address <= 31; address++)); do
  printf '[%02d] ./run.sh ping UPLINK 2 ... ' "$address"
  set_uplink_csp_node "$address"

  if output=$(START_SERVICE=1 CONFIG="$CONFIG_FILE" "$RUN_SCRIPT" ping UPLINK 2 2>&1); then
    run_status=0
  else
    run_status=$?
  fi

  if is_successful_ping "$output"; then
    successful_addresses+=("$address")
    printf 'SUCCESS\n'
    printf '%s\n' "$output" | sed 's/^/  /'
  else
    printf 'no successful interaction'
    if [ "$run_status" -ne 0 ]; then
      printf ' (exit %d)' "$run_status"
    fi
    printf '\n'

    if [ "$SHOW_OUTPUT" = "1" ]; then
      printf '%s\n' "$output" | sed 's/^/  /'
    fi
  fi

  if [ "$address" -lt 31 ]; then
    sleep_ms "$DELAY_MS"
  fi
done

echo
if [ "${#successful_addresses[@]}" -eq 0 ]; then
  echo "No CSP address produced a successful UPLINK ping interaction."
else
  echo "Successful UPLINK CSP address(es): ${successful_addresses[*]}"
fi
