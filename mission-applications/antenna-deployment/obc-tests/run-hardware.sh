#!/bin/sh
set -eu

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
APP="${APP:-$DIR/bin/antenna-deployment}"
CONFIG="${CONFIG:-/home/kubos/fram-tests/fram-service/config/fram-hw.toml}"
CONFIRM="${ANTENNA_HARDWARE_TEST_CONFIRM:-}"
ANTENNA_FRAM_ONLY="${ANTENNA_FRAM_ONLY:-0}"
export ANTENNA_FRAM_ONLY

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
echo
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
