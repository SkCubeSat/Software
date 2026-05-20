#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
PATCH="$ROOT/patches/libcsp-rust-dangerous-implicit-autorefs.patch"
LIBCSP_RUST="$ROOT/vendor/libcsp-rust"

if [ ! -d "$LIBCSP_RUST/.git" ] && [ ! -f "$LIBCSP_RUST/.git" ]; then
    echo "vendor/libcsp-rust is not initialized. Run: git submodule update --init --recursive" >&2
    exit 1
fi

if git -C "$LIBCSP_RUST" apply --reverse --check "$PATCH" >/dev/null 2>&1; then
    echo "libcsp-rust patch is already applied"
    exit 0
fi

git -C "$LIBCSP_RUST" apply "$PATCH"
echo "applied libcsp-rust patch"
