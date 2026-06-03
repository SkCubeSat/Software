#!/bin/sh
set -e

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH= cd -- "$DIR/../../.." && pwd)
TARGET="${TARGET:-armv7-unknown-linux-gnueabihf}"
OUT="${OUT:-$ROOT/target/obc-tests/comms-services}"
INCLUDE_SERVICE_BIN="${INCLUDE_SERVICE_BIN:-1}"

case "$OUT" in
  "$ROOT"/target/obc-tests/*) ;;
  *)
    echo "Refusing to package outside $ROOT/target/obc-tests: $OUT" >&2
    exit 2
    ;;
esac

if [ "$INCLUDE_SERVICE_BIN" = "1" ] && [ "${SKIP_BUILD:-0}" != "1" ]; then
  "$DIR/build.sh"
fi

rm -rf "$OUT"
mkdir -p "$OUT/bin" "$OUT/config" "$OUT/requests"

if [ "$INCLUDE_SERVICE_BIN" = "1" ]; then
  cp "$ROOT/target/$TARGET/release/comms-services" "$OUT/bin/"
  chmod +x "$OUT/bin/comms-services"
fi

cp "$DIR/config/comms-hw.toml" "$OUT/config/"
cp "$DIR"/requests/*.json "$OUT/requests/"
cp "$DIR/run.sh" "$OUT/"
cp "$DIR/scan_csp_addresses.sh" "$OUT/"
cp "$DIR/README.md" "$OUT/"

chmod +x "$OUT/run.sh"
chmod +x "$OUT/scan_csp_addresses.sh"

echo "Packaged OBC comms tests at $OUT"
echo "Transfer with:"
echo "  transfer -- -d /home/kubos/comms-tests $OUT"
