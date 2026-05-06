#!/bin/sh
set -e

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH= cd -- "$DIR/../../../.." && pwd)
TARGET="${TARGET:-armv7-unknown-linux-gnueabihf}"
CROSS="${CROSS:-cross}"
STRIP="${STRIP:-arm-linux-gnueabihf-strip}"

cd "$ROOT"

echo "Building fram-service for $TARGET"
"$CROSS" build \
  --target "$TARGET" \
  -p fram-service \
  --release \
  --features i2c

echo "Building fram-obc-tests for $TARGET"
"$CROSS" build \
  --target "$TARGET" \
  -p fram-service \
  --release \
  --features i2c,obc-tests \
  --bin fram-obc-tests

strip_binary() {
  bin="$1"

  if [ "${NO_STRIP:-0}" = "1" ]; then
    echo "Skipping strip for $bin because NO_STRIP=1"
    return 0
  fi

  if command -v "$STRIP" >/dev/null 2>&1; then
    echo "Stripping $bin with $STRIP"
    "$STRIP" "$bin"
  elif command -v llvm-strip >/dev/null 2>&1; then
    echo "Stripping $bin with llvm-strip"
    llvm-strip "$bin"
  else
    echo "WARNING: no strip tool found. Set STRIP=arm-linux-gnueabihf-strip or NO_STRIP=1." >&2
  fi
}

strip_binary "$ROOT/target/$TARGET/release/fram-service"
strip_binary "$ROOT/target/$TARGET/release/fram-obc-tests"
