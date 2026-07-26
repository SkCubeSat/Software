#!/bin/sh
set -e

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
URL="${URL:-http://127.0.0.1:8091/graphql}"
SERVICE_BIN="${SERVICE_BIN:-$DIR/bin/fram-service}"
TEST_BIN="${TEST_BIN:-$DIR/bin/fram-obc-tests}"
CONFIG="${CONFIG:-$DIR/config/fram-hw.toml}"
LOG="${LOG:-$DIR/fram-service.log}"
START_SERVICE="${START_SERVICE:-1}"
I2C_BUS="${I2C_BUS:-/dev/i2c-2}"
I2C_ADDR="${I2C_ADDR:-0x50}"
SCRATCH_OFFSET="${SCRATCH_OFFSET:-4096}"
SCAN_ONLY="${FRAM_TEST_SCAN_ONLY:-0}"
MISSION_WRITE="${FRAM_TEST_MISSION_WRITE:-0}"
ENV_WRITE="${FRAM_TEST_ENV_WRITE:-0}"
FW_PRINTENV="${FRAM_TEST_FW_PRINTENV:-/usr/sbin/fw_printenv}"
FW_SETENV="${FRAM_TEST_FW_SETENV:-/usr/sbin/fw_setenv}"

SERVICE_PID=""

cleanup() {
  if [ -n "$SERVICE_PID" ] && kill -0 "$SERVICE_PID" 2>/dev/null; then
    kill "$SERVICE_PID" 2>/dev/null || true
    wait "$SERVICE_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

if [ "$SCAN_ONLY" = "1" ]; then
  if [ ! -x "$TEST_BIN" ]; then
    echo "test binary not executable: $TEST_BIN" >&2
    exit 2
  fi

  "$TEST_BIN" \
    --scan-only \
    --i2c-bus "$I2C_BUS" \
    --i2c-addr "$I2C_ADDR" \
    --scratch-offset "$SCRATCH_OFFSET"
  exit 0
fi

post_request() {
  name="$1"
  file="$2"
  printf '\n=== %s ===\n' "$name"
  curl -fsS "$URL" \
    -H 'content-type: application/json' \
    --data @"$file"
  printf '\n'
}

wait_for_service() {
  i=0
  while [ "$i" -lt 30 ]; do
    if curl -fsS "$URL" \
      -H 'content-type: application/json' \
      --data @"$DIR/requests/00_ping.json" >/dev/null 2>&1; then
      return 0
    fi
    i=$((i + 1))
    sleep 1
  done

  echo "FRAM service did not become ready at $URL" >&2
  if [ -f "$LOG" ]; then
    echo "--- service log ---" >&2
    tail -n 80 "$LOG" >&2 || true
  fi
  return 1
}

if [ "$START_SERVICE" = "1" ]; then
  if [ ! -x "$SERVICE_BIN" ]; then
    echo "service binary not executable: $SERVICE_BIN" >&2
    exit 2
  fi

  echo "Starting FRAM service"
  "$SERVICE_BIN" -c "$CONFIG" >"$LOG" 2>&1 &
  SERVICE_PID="$!"
  wait_for_service
else
  echo "Using already-running FRAM service at $URL"
fi

post_request "Ping" "$DIR/requests/00_ping.json"
post_request "Health" "$DIR/requests/01_health.json"
post_request "Mission state read" "$DIR/requests/02_mission_state.json"
post_request "Reconcile dry run" "$DIR/requests/20_reconcile_dry_run.json"

if [ ! -x "$TEST_BIN" ]; then
  echo "test binary not executable: $TEST_BIN" >&2
  exit 2
fi

TEST_ARGS="--url $URL --i2c-bus $I2C_BUS --i2c-addr $I2C_ADDR --scratch-offset $SCRATCH_OFFSET"
if [ "$MISSION_WRITE" = "1" ]; then
  TEST_ARGS="$TEST_ARGS --mission-write"
else
  echo "Mission flag write/restore is disabled. Set FRAM_TEST_MISSION_WRITE=1 to enable it."
fi
if [ "$ENV_WRITE" = "1" ]; then
  TEST_ARGS="$TEST_ARGS --env-write --fw-printenv $FW_PRINTENV --fw-setenv $FW_SETENV"
else
  echo "U-Boot env mirror write/restore is disabled. Set FRAM_TEST_ENV_WRITE=1 to enable it."
fi

# shellcheck disable=SC2086
"$TEST_BIN" $TEST_ARGS
