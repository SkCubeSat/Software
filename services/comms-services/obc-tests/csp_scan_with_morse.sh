#!/bin/sh

LC_ALL=C
export LC_ALL

DIR="$(CDPATH= cd "$(dirname "$0")" && pwd)"
SOURCE_ID="${MORSE_SOURCE:-SAT1}"
TEXT_MESSAGE="${MORSE_TEXT:-sixseven}"
source_seen=0
text_seen=0

usage() {
  cat <<'EOF'
Usage:
  ./csp_scan_with_morse.sh
  ./csp_scan_with_morse.sh SAT1 sixseven
  ./csp_scan_with_morse.sh SAT1 "message up to 20" --delay-ms 250
  ./csp_scan_with_morse.sh --source SAT1 --text sixseven --range 4-12

Scans UPLINK CSP v1 node addresses and runs this command at each address:
  ./run.sh morse-text UPLINK SOURCE4 TEXT

Morse arguments:
  --source SOURCE4      Exactly 4 printable ASCII bytes. Default: SAT1.
  --text TEXT           At most 20 printable ASCII bytes. Default: sixseven.

The source and text may instead be supplied as the first two positional
arguments. Quote text containing spaces.

Scanner arguments passed through to scan_csp_addresses.sh:
  --delay-ms MS
  --start ADDRESS
  --end ADDRESS
  --range START-END

The same scanner environment variables are supported, including DELAY_MS,
START_ADDRESS, END_ADDRESS, CONTROLLER_TIMEOUT_RETRIES, CONFIG,
RESTORE_CONFIG, SHOW_OUTPUT, and ALLOW_EXISTING_SERVICE.
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    -h|--help)
      usage
      exit 0
      ;;
    --source)
      [ "$#" -ge 2 ] || { echo "--source requires a value" >&2; exit 2; }
      SOURCE_ID="$2"
      source_seen=1
      shift 2
      ;;
    --source=*)
      SOURCE_ID="${1#*=}"
      source_seen=1
      shift
      ;;
    --text)
      [ "$#" -ge 2 ] || { echo "--text requires a value" >&2; exit 2; }
      TEXT_MESSAGE="$2"
      text_seen=1
      shift 2
      ;;
    --text=*)
      TEXT_MESSAGE="${1#*=}"
      text_seen=1
      shift
      ;;
    --)
      shift
      break
      ;;
    -*)
      break
      ;;
    *)
      if [ "$source_seen" = "0" ]; then
        SOURCE_ID="$1"
        source_seen=1
        shift
      elif [ "$text_seen" = "0" ]; then
        TEXT_MESSAGE="$1"
        text_seen=1
        shift
      else
        break
      fi
      ;;
  esac
done

if [ "${#SOURCE_ID}" -ne 4 ] ||
   ! printf '%s\n' "$SOURCE_ID" | grep '^[ -~][ -~][ -~][ -~]$' >/dev/null 2>&1; then
  echo "source identification must be exactly 4 printable ASCII bytes: $SOURCE_ID" >&2
  exit 2
fi

if [ "${#TEXT_MESSAGE}" -gt 20 ] ||
   ! printf '%s\n' "$TEXT_MESSAGE" | grep '^[ -~]*$' >/dev/null 2>&1; then
  echo "text message must contain at most 20 printable ASCII bytes" >&2
  exit 2
fi

SCAN_OPERATION=morse-text \
MORSE_SOURCE="$SOURCE_ID" \
MORSE_TEXT="$TEXT_MESSAGE" \
exec "$DIR/scan_csp_addresses.sh" "$@"
