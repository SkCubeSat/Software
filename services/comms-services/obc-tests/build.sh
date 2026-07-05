#!/bin/sh
set -e

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH= cd -- "$DIR/../../.." && pwd)
TARGET="${TARGET:-armv7-unknown-linux-gnueabihf}"
CROSS="${CROSS:-cross}"
STRIP="${STRIP:-arm-linux-gnueabihf-strip}"

cd "$ROOT"

echo "Building comms-services and comms-cli for $TARGET"
"$CROSS" build \
  --target "$TARGET" \
  -p comms-services \
  -p comms-cli \
  --release

strip_binary() {
  bin="$1"
  if [ "${NO_STRIP:-0}" = "1" ]; then
    echo "Skipping strip for $bin because NO_STRIP=1"
  elif command -v "$STRIP" >/dev/null 2>&1; then
    echo "Stripping $bin with $STRIP"
    "$STRIP" "$bin"
  elif command -v llvm-strip >/dev/null 2>&1; then
    echo "Stripping $bin with llvm-strip"
    llvm-strip "$bin"
  else
    echo "WARNING: no strip tool found. Set STRIP=arm-linux-gnueabihf-strip or NO_STRIP=1." >&2
  fi
}

strip_binary "$ROOT/target/$TARGET/release/comms-services"
strip_binary "$ROOT/target/$TARGET/release/comms-cli"
