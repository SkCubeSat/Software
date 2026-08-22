#!/bin/sh

# Source this file from the packaged antenna-deployment directory:
#   . ./setup-bench.sh
#
# These defaults are for the standard OBC test installation. Override
# ANTENNA_TEST_DIR or CFG before sourcing if either path is different.
ANTENNA_TEST_DIR="${ANTENNA_TEST_DIR:-/home/kubos/antenna-tests/antenna-deployment}"
CFG="${CFG:-/home/kubos/fram-tests/fram-service/config/fram-hw.toml}"
APP="$ANTENNA_TEST_DIR/bin/antenna-deployment"
CONFIG="$CFG"
ANTENNA_FRAM_ONLY=1

export ANTENNA_TEST_DIR APP CFG CONFIG ANTENNA_FRAM_ONLY

# Prepare only the active-low antenna sense inputs. Do not export or configure
# GPIO 115/117 here: those are the deployment outputs and must remain under the
# guarded hardware-test runner's control.
setup_sense_gpio() {
  pin="$1"
  gpio_dir="/sys/class/gpio/gpio$pin"

  if [ ! -d "$gpio_dir" ]; then
    if ! echo "$pin" > /sys/class/gpio/export; then
      echo "WARNING: failed to export sense GPIO $pin" >&2
      return 1
    fi

    # sysfs may take a moment to create the GPIO attributes after export.
    attempts=0
    while [ ! -e "$gpio_dir/direction" ] && [ "$attempts" -lt 5 ]; do
      sleep 1
      attempts=$((attempts + 1))
    done
  fi

  if [ ! -e "$gpio_dir/direction" ]; then
    echo "WARNING: sense GPIO $pin did not appear after export" >&2
    return 1
  fi

  if ! echo in > "$gpio_dir/direction"; then
    echo "WARNING: failed to configure sense GPIO $pin as an input" >&2
    return 1
  fi

  value=$(cat "$gpio_dir/value" 2>/dev/null) || {
    echo "WARNING: failed to read sense GPIO $pin" >&2
    return 1
  }
  echo "  GPIO $pin raw value=$value (active-low; 0=confirmed, 1=inactive)"
}

echo "Antenna bench environment configured:"
echo "  APP=$APP"
echo "  CFG=$CFG"
echo "  ANTENNA_FRAM_ONLY=$ANTENNA_FRAM_ONLY"
echo "Antenna sense inputs:"
setup_sense_gpio 66 || true
setup_sense_gpio 7 || true
echo "WARNING: run-hardware.sh can activate real antenna deployment GPIO outputs."
