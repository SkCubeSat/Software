#!/bin/sh
set -u

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
APP="${APP:-$DIR/bin/antenna-deployment}"
CONFIG="${CONFIG:-/home/kubos/fram-tests/fram-service/config/fram-hw.toml}"
STATE_WRITE="${ANTENNA_TEST_STATE_WRITE:-0}"

PASS=0
FAIL=0
RESTORE_NEEDED=0
ORIGINAL_DETUMBLING=""
ORIGINAL_DEPLOYED=""

pass() {
  echo "PASS: $1"
  PASS=$((PASS + 1))
}

fail() {
  echo "FAIL: $1" >&2
  echo "  expected output containing: $2" >&2
  echo "  actual output: $3" >&2
  FAIL=$((FAIL + 1))
}

run_app() {
  "$APP" "$@" -c "$CONFIG" --stdout 2>&1
}

state_value() {
  key="$1"
  text="$2"
  printf '%s\n' "$text" | sed -n "s/.*${key} = //p" | tail -n 1
}

expect_output() {
  name="$1"
  expected="$2"
  shift 2

  output=$(run_app "$@")
  status=$?
  printf '%s\n' "$output"

  if [ "$status" -eq 0 ] && printf '%s\n' "$output" | grep -F "$expected" >/dev/null; then
    pass "$name"
  else
    fail "$name" "$expected" "$output"
  fi
}

restore_state() {
  if [ "$RESTORE_NEEDED" != "1" ]; then
    return
  fi

  echo "Restoring original mission state"
  run_app set-flag detumbling_complete "$ORIGINAL_DETUMBLING" >/dev/null || \
    echo "WARNING: failed to restore detumbling_complete" >&2
  run_app set-flag deployed "$ORIGINAL_DEPLOYED" >/dev/null || \
    echo "WARNING: failed to restore deployed" >&2
  RESTORE_NEEDED=0
}

trap restore_state EXIT INT TERM

if [ ! -x "$APP" ]; then
  echo "antenna-deployment binary not executable: $APP" >&2
  exit 2
fi

echo "Antenna deployment OBC tests"
echo "Application: $APP"
echo "KubOS config: $CONFIG"

if [ ! -f "$CONFIG" ]; then
  echo "KubOS config not found: $CONFIG" >&2
  echo "Set CONFIG to the file used by antenna-deployment." >&2
  exit 2
fi

initial=$(run_app show-state)
initial_status=$?
printf '%s\n' "$initial"
if [ "$initial_status" -eq 0 ] && printf '%s\n' "$initial" | grep -F "deployed = " >/dev/null; then
  pass "read mission state from FRAM service"
else
  fail "read mission state from FRAM service" "deployed = " "$initial"
fi

expect_output \
  "reconcile dry run" \
  "reconcile complete (dry_run = true)" \
  reconcile dry-run

if [ "$STATE_WRITE" != "1" ]; then
  echo "State write/restore tests are disabled."
  echo "Set ANTENNA_TEST_STATE_WRITE=1 to enable them on non-flight test hardware."
else
  ORIGINAL_DETUMBLING=$(state_value detumbling_complete "$initial")
  ORIGINAL_DEPLOYED=$(state_value deployed "$initial")

  case "$ORIGINAL_DETUMBLING:$ORIGINAL_DEPLOYED" in
    true:true|true:false|false:true|false:false) ;;
    *)
      echo "Could not parse original mission state; refusing to write." >&2
      exit 2
      ;;
  esac

  RESTORE_NEEDED=1
  if [ "$ORIGINAL_DETUMBLING" = "true" ]; then
    TEST_DETUMBLING=false
  else
    TEST_DETUMBLING=true
  fi

  expect_output \
    "write detumbling_complete" \
    "set-flag detumbling_complete = $TEST_DETUMBLING" \
    set-flag detumbling_complete "$TEST_DETUMBLING"
  expect_output \
    "read back detumbling_complete" \
    "detumbling_complete = $TEST_DETUMBLING" \
    show-state

  # deployed=true makes run-once return before GPIO initialization or pulses.
  expect_output \
    "set safe already-deployed state" \
    "set-flag deployed = true" \
    set-flag deployed true
  expect_output \
    "already-deployed path performs no deployment" \
    "deployed=true, nothing to do" \
    run-once

  restore_state

  restored=$(run_app show-state)
  if printf '%s\n' "$restored" | grep -F "detumbling_complete = $ORIGINAL_DETUMBLING" >/dev/null && \
     printf '%s\n' "$restored" | grep -F "deployed = $ORIGINAL_DEPLOYED" >/dev/null; then
    pass "restore original mission state"
  else
    fail "restore original mission state" \
      "detumbling_complete = $ORIGINAL_DETUMBLING and deployed = $ORIGINAL_DEPLOYED" \
      "$restored"
  fi
fi

echo
echo "Results: $PASS passed, $FAIL failed"
if [ "$FAIL" -ne 0 ]; then
  exit 1
fi
