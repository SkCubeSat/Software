#!/bin/sh

# Source this file from the packaged antenna-deployment directory:
#   . ./setup-bench.sh
#
# These defaults are for the standard OBC test installation. Override
# ANTENNA_TEST_DIR or CFG before sourcing if either path is different.
ANTENNA_TEST_DIR="${ANTENNA_TEST_DIR:-/home/kubos/antenna-tests/antenna-deployment}"
CFG="${CFG:-/home/kubos/fram-tests/fram-service/config/fram-hw.toml}"
APP="$ANTENNA_TEST_DIR/bin/antenna-deployment"
CONFIG="$CFG"
ANTENNA_FRAM_ONLY=1

export ANTENNA_TEST_DIR APP CFG CONFIG ANTENNA_FRAM_ONLY

echo "Antenna bench environment configured:"
echo "  APP=$APP"
echo "  CFG=$CFG"
echo "  ANTENNA_FRAM_ONLY=$ANTENNA_FRAM_ONLY"
echo "WARNING: run-hardware.sh can activate real antenna deployment GPIO outputs."
