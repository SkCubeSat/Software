#!/bin/sh
set -e

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
APP="${APP:-$DIR/bin/antenna-deployment}"
CONFIG="${CONFIG:-/home/kubos/fram-tests/fram-service/config/fram-hw.toml}"
CONFIRM="${ANTENNA_HARDWARE_TEST_CONFIRM:-}"
MODE="${ANTENNA_HARDWARE_TEST_MODE:-normal}"
ANTENNA_FRAM_ONLY="${ANTENNA_FRAM_ONLY:-0}"
export ANTENNA_FRAM_ONLY

ORIGINAL_STATE=""
RESTORE_NEEDED=0

run_app() {
  "$APP" "$@" -c "$CONFIG" --stdout 2>&1
}

state_value() {
  key="$1"
  text="$2"
  printf '%s\n' "$text" | sed -n "s/.*${key} = //p" | tail -n 1
}

restore_state() {
  if [ "$RESTORE_NEEDED" != "1" ]; then
    return
  fi

  echo "Restoring original mission state"
  for key in deployed vhf_antenna_deployed uhf_antenna_deployed \
    initial_safe_state_complete detumbling_complete
  do
    value=$(state_value "$key" "$ORIGINAL_STATE")
    run_app set-flag "$key" "$value" >/dev/null || \
      echo "WARNING: failed to restore $key" >&2
  done

  deploy_start=$(state_value deploy_start "$ORIGINAL_STATE")
  if [ "$deploy_start" = "null" ]; then
    run_app set-deploy-start clear >/dev/null || \
      echo "WARNING: failed to clear deploy_start" >&2
  else
    run_app set-deploy-start "$deploy_start" >/dev/null || \
      echo "WARNING: failed to restore deploy_start" >&2
  fi
  RESTORE_NEEDED=0
}

trap restore_state EXIT INT TERM

if [ ! -x "$APP" ]; then
  echo "antenna-deployment binary not executable: $APP" >&2
  exit 2
fi

if [ ! -f "$CONFIG" ]; then
  echo "KubOS config not found: $CONFIG" >&2
  echo "Set CONFIG to the file used by antenna-deployment." >&2
  exit 2
fi

echo "Antenna deployment HARDWARE test"
echo "Application: $APP"
echo "KubOS config: $CONFIG"
echo "FRAM-only mode: $ANTENNA_FRAM_ONLY"
echo "Test mode: $MODE"
echo

if [ "$MODE" = "three-attempts" ]; then
  echo "WARNING: This mode can pulse each real deployment output three times."
  echo "Disconnect flight deployment loads and keep both sense inputs inactive."
  echo "It uses the real 90-second delays and takes about 4.5 minutes."
  echo

  if [ "$CONFIRM" != "DEPLOY_3_TIMES" ]; then
    echo "Refusing to run the three-attempt hardware test without confirmation." >&2
    echo "Set ANTENNA_HARDWARE_TEST_CONFIRM=DEPLOY_3_TIMES after making the bench safe." >&2
    exit 2
  fi

  ORIGINAL_STATE=$(run_app show-state)
  for key in deployed vhf_antenna_deployed uhf_antenna_deployed \
    initial_safe_state_complete detumbling_complete
  do
    value=$(state_value "$key" "$ORIGINAL_STATE")
    case "$value" in
      true|false) ;;
      *) echo "Could not read $key; refusing to change state." >&2; exit 2 ;;
    esac
  done
  deploy_start=$(state_value deploy_start "$ORIGINAL_STATE")
  case "$deploy_start" in
    null|[0-9]*) ;;
    *) echo "Could not read deploy_start; refusing to change state." >&2; exit 2 ;;
  esac
  RESTORE_NEEDED=1

  # Keep temporary state changes in FRAM while using real hardware GPIO.
  ANTENNA_FRAM_ONLY=1
  export ANTENNA_FRAM_ONLY
  for key in deployed vhf_antenna_deployed uhf_antenna_deployed \
    initial_safe_state_complete detumbling_complete
  do
    run_app set-flag "$key" false >/dev/null
  done
  run_app set-deploy-start 0 >/dev/null

  echo "Running three-attempt hardware test..."
  set +e
  output=$(run_app run-once)
  status=$?
  set -e
  printf '%s\n' "$output"

  FAIL=0
  check_count() {
    name="$1"
    expected="$2"
    pattern="$3"
    actual=$(printf '%s\n' "$output" | grep -F -c "$pattern" || true)
    if [ "$actual" -eq "$expected" ]; then
      echo "PASS: $name"
    else
      echo "FAIL: $name (expected $expected, got $actual)" >&2
      FAIL=$((FAIL + 1))
    fi
  }

  if [ "$status" -ne 0 ]; then
    echo "FAIL: run-once exited with status $status" >&2
    FAIL=$((FAIL + 1))
  fi
  check_count "three attempt sets" 3 "starting attempt set"
  check_count "three VHF attempts" 3 "attempting VHF deployment"
  check_count "three UHF attempts" 3 "attempting UHF deployment"
  check_count "one check-only cycle" 1 "starting check-only cycle"

  after=$(run_app show-state)
  printf '%s\n' "$after"
  if printf '%s\n' "$after" | grep -F "deployed = false" >/dev/null; then
    echo "PASS: inactive senses did not mark deployment complete"
  else
    echo "FAIL: deployment was unexpectedly marked complete" >&2
    FAIL=$((FAIL + 1))
  fi

  restore_state
  echo "Results: $((5 - FAIL)) passed, $FAIL failed"
  if [ "$FAIL" -ne 0 ]; then
    exit 1
  fi
  exit 0
fi

if [ "$MODE" != "normal" ]; then
  echo "Unknown ANTENNA_HARDWARE_TEST_MODE: $MODE" >&2
  exit 2
fi

echo "Current mission state:"
"$APP" show-state -c "$CONFIG" --stdout
echo
echo "WARNING: This runs the real deployment path."
echo "If the deployment hold timer has elapsed, it can drive GPIO_117 (VHF)"
echo "and GPIO_115 (UHF) HIGH and physically fire connected deployment loads."
echo "The application still enforces its stored mission state and 30-minute hold timer."
if [ "$ANTENNA_FRAM_ONLY" = "1" ]; then
  echo "WARNING: FRAM-only bench mode is enabled; U-Boot state redundancy is disabled."
fi
echo

if [ "$CONFIRM" != "DEPLOY" ]; then
  if [ -t 0 ]; then
    printf "Type DEPLOY to continue: "
    read -r CONFIRM
  else
    echo "Refusing to run without confirmation." >&2
    echo "Set ANTENNA_HARDWARE_TEST_CONFIRM=DEPLOY after verifying the hardware is safe." >&2
    exit 2
  fi
fi

if [ "$CONFIRM" != "DEPLOY" ]; then
  echo "Hardware test cancelled." >&2
  exit 2
fi

echo "Running the real antenna deployment state machine..."
"$APP" run-once -c "$CONFIG" --stdout

echo
echo "Mission state after hardware test:"
"$APP" show-state -c "$CONFIG" --stdout
