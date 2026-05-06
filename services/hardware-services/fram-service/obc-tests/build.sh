#!/bin/sh
set -eu

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH= cd -- "$DIR/../../../.." && pwd)
TARGET="${TARGET:-armv7-unknown-linux-gnueabihf}"
CROSS="${CROSS:-cross}"

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
