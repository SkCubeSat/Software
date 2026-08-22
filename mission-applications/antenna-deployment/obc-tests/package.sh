#!/bin/sh
set -e

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH= cd -- "$DIR/../../.." && pwd)
TARGET="${TARGET:-armv7-unknown-linux-gnueabihf}"
OUT="${OUT:-$ROOT/target/obc-tests/antenna-deployment}"

case "$OUT" in
  "$ROOT"/target/obc-tests/*) ;;
  *)
    echo "Refusing to package outside $ROOT/target/obc-tests: $OUT" >&2
    exit 2
    ;;
esac

if [ "${SKIP_BUILD:-0}" != "1" ]; then
  # Calling through sh also works when a Windows checkout did not preserve
  # the executable bit on build.sh.
  sh "$DIR/build.sh"
fi

rm -rf "$OUT"
mkdir -p "$OUT/bin"

cp "$ROOT/target/$TARGET/release/antenna-deployment" "$OUT/bin/"
cp "$DIR/run.sh" "$OUT/"
cp "$DIR/run-hardware.sh" "$OUT/"
cp "$DIR/setup-bench.sh" "$OUT/"
cp "$DIR/test-gpio.sh" "$OUT/"

chmod +x "$OUT/run.sh" "$OUT/run-hardware.sh" "$OUT/test-gpio.sh" \
  "$OUT/bin/antenna-deployment"

echo "Packaged OBC antenna-deployment tests at $OUT"
echo "Transfer with:"
echo "  transfer -p /dev/ttyUSB0 -d /home/kubos/antenna-tests $OUT"
