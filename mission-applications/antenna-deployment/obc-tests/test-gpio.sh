#!/bin/sh
set -eu

CONFIRM="${ANTENNA_GPIO_TEST_CONFIRM:-}"
PULSE_SECONDS="${ANTENNA_GPIO_TEST_PULSE_SECONDS:-1}"
GPIO_ROOT=/sys/class/gpio

export_gpio() {
  pin="$1"
  gpio_dir="$GPIO_ROOT/gpio$pin"

  if [ ! -d "$gpio_dir" ]; then
    echo "$pin" > "$GPIO_ROOT/export" || return 1
    attempts=0
    while [ ! -e "$gpio_dir/direction" ] && [ "$attempts" -lt 5 ]; do
      sleep 1
      attempts=$((attempts + 1))
    done
  fi

  [ -e "$gpio_dir/direction" ]
}

set_output_low() {
  pin="$1"
  gpio_dir="$GPIO_ROOT/gpio$pin"
  [ -e "$gpio_dir/direction" ] || return 0

  # Writing "low" configures the output and establishes its initial value
  # atomically, avoiding a transient high when changing direction.
  echo low > "$gpio_dir/direction" 2>/dev/null || {
    echo out > "$gpio_dir/direction" 2>/dev/null || return 1
    echo 0 > "$gpio_dir/value" 2>/dev/null || return 1
  }
}

cleanup() {
  set_output_low 117 || echo "WARNING: failed to return GPIO 117 low" >&2
  set_output_low 115 || echo "WARNING: failed to return GPIO 115 low" >&2
}

trap cleanup EXIT
trap 'exit 130' INT TERM HUP

case "$PULSE_SECONDS" in
  ''|*[!0-9]*)
    echo "ANTENNA_GPIO_TEST_PULSE_SECONDS must be a positive integer" >&2
    exit 2
    ;;
  0)
    echo "ANTENNA_GPIO_TEST_PULSE_SECONDS must be greater than zero" >&2
    exit 2
    ;;
esac

echo "Antenna GPIO bench test"
echo "WARNING: GPIO 117 and GPIO 115 will each be driven HIGH for ${PULSE_SECONDS}s."
echo "Disconnect all deployment loads and connect only safe test equipment."

if [ "$CONFIRM" != "TEST_ALL_GPIOS" ]; then
  if [ -t 0 ]; then
    printf "Type TEST_ALL_GPIOS to continue: "
    read -r CONFIRM
  fi
fi

if [ "$CONFIRM" != "TEST_ALL_GPIOS" ]; then
  echo "GPIO test cancelled; required confirmation was not provided." >&2
  exit 2
fi

for pin in 66 7 117 115; do
  if ! export_gpio "$pin"; then
    echo "Failed to export GPIO $pin" >&2
    exit 1
  fi
done

echo in > "$GPIO_ROOT/gpio66/direction"
echo in > "$GPIO_ROOT/gpio7/direction"
set_output_low 117
set_output_low 115

echo "Initial values (active-low sense: 0=confirmed, 1=inactive):"
echo "  VHF sense GPIO 66: $(cat "$GPIO_ROOT/gpio66/value")"
echo "  UHF sense GPIO 7:  $(cat "$GPIO_ROOT/gpio7/value")"
echo "  VHF output GPIO 117: $(cat "$GPIO_ROOT/gpio117/value")"
echo "  UHF output GPIO 115: $(cat "$GPIO_ROOT/gpio115/value")"

echo "Pulsing VHF output GPIO 117 HIGH for ${PULSE_SECONDS}s"
echo 1 > "$GPIO_ROOT/gpio117/value"
echo "  GPIO 117 during pulse: $(cat "$GPIO_ROOT/gpio117/value")"
sleep "$PULSE_SECONDS"
echo 0 > "$GPIO_ROOT/gpio117/value"
echo "  GPIO 117 after pulse: $(cat "$GPIO_ROOT/gpio117/value")"
echo "  VHF sense GPIO 66: $(cat "$GPIO_ROOT/gpio66/value")"

echo "Pulsing UHF output GPIO 115 HIGH for ${PULSE_SECONDS}s"
echo 1 > "$GPIO_ROOT/gpio115/value"
echo "  GPIO 115 during pulse: $(cat "$GPIO_ROOT/gpio115/value")"
sleep "$PULSE_SECONDS"
echo 0 > "$GPIO_ROOT/gpio115/value"
echo "  GPIO 115 after pulse: $(cat "$GPIO_ROOT/gpio115/value")"
echo "  UHF sense GPIO 7: $(cat "$GPIO_ROOT/gpio7/value")"

echo "PASS: all four GPIO sysfs interfaces were configured and read successfully."
