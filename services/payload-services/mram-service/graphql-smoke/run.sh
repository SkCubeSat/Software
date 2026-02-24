#!/bin/sh
set -eu

URL="${1:-http://127.0.0.1:8090/graphql}"
DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REQ="$DIR/requests"

run_req() {
  name="$1"
  file="$REQ/$2"
  echo "\n=== $name ==="
  echo "POST $URL"
  curl -sS "$URL" \
    -H 'content-type: application/json' \
    --data @"$file"
  echo
}

run_req "Ping" 00_ping.json
run_req "Storage" 01_storage.json
run_req "Write note.txt" 10_write_note.json
run_req "List files" 11_files.json
run_req "File metadata" 12_file_note.json
run_req "Read file" 13_read_note.json
run_req "Delete file" 14_delete_note.json
run_req "Format reject (confirm=false)" 20_format_reject.json
# Destructive; disabled by default. Uncomment if you want to wipe MRAM filesystem.
# run_req "Format filesystem" 21_format_confirm.json
