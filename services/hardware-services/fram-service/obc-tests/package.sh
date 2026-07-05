#!/bin/sh
set -e

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH= cd -- "$DIR/../../../.." && pwd)
TARGET="${TARGET:-armv7-unknown-linux-gnueabihf}"
OUT="${OUT:-$ROOT/target/obc-tests/fram-service}"

case "$OUT" in
  "$ROOT"/target/obc-tests/*) ;;
  *)
    echo "Refusing to package outside $ROOT/target/obc-tests: $OUT" >&2
    exit 2
    ;;
esac

if [ "${SKIP_BUILD:-0}" != "1" ]; then
  "$DIR/build.sh"
fi

rm -rf "$OUT"
mkdir -p "$OUT/bin" "$OUT/config" "$OUT/requests"

cp "$ROOT/target/$TARGET/release/fram-service" "$OUT/bin/"
cp "$ROOT/target/$TARGET/release/fram-obc-tests" "$OUT/bin/"
cp "$DIR/config/fram-hw.toml" "$OUT/config/"
cp "$DIR"/requests/*.json "$OUT/requests/"
cp "$DIR/run.sh" "$OUT/"
cp "$DIR/README.md" "$OUT/"

chmod +x "$OUT/run.sh" "$OUT/bin/fram-service" "$OUT/bin/fram-obc-tests"

echo "Packaged OBC FRAM tests at $OUT"
echo "Transfer with:"
echo "  transfer -- -d /home/kubos/fram-tests $OUT"
