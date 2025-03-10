#!/bin/bash

# Script to run Kubos services
# Runs monitor-service, telemetry-service, app-service, and scheduler-service
# Attaches to app-service logs while running the rest in the background
# When the script is terminated, all services are stopped

# Function to clean up services when the script exits
cleanup() {
    echo "Stopping all services..."
    
    # Kill all background processes in the process group
    kill $(jobs -p) 2>/dev/null
    
    echo "All services stopped."
    exit 0
}

# Set up trap to catch script termination
trap cleanup EXIT INT TERM

# Service executables in production environment
PROD_MONITOR_SERVICE="/usr/sbin/monitor-service"
PROD_TELEMETRY_SERVICE="/usr/sbin/telemetry-service"
PROD_APP_SERVICE="/usr/sbin/kubos-app-service"
PROD_SCHEDULER_SERVICE="/usr/sbin/scheduler-service"

# Service executables in development environment
DEV_CONFIG_DIR="/home/vagrant/configs"
DEV_KUBOS_DIR="/home/vagrant/kubos"
DEV_TARGET_DIR="$DEV_KUBOS_DIR/target/debug"
DEV_MONITOR_SERVICE="$DEV_TARGET_DIR/monitor-service"
DEV_TELEMETRY_SERVICE="$DEV_TARGET_DIR/telemetry-service"
DEV_APP_SERVICE="$DEV_TARGET_DIR/kubos-app-service"
DEV_SCHEDULER_SERVICE="$DEV_TARGET_DIR/scheduler-service"

# Config file location
CONFIG_FILE="$DEV_CONFIG_DIR/dev_config.toml"
PROD_CONFIG_FILE="/etc/kubos-config.toml"

# Check if we're in development environment and adjust paths
if [ -f "$DEV_MONITOR_SERVICE" ]; then
    echo "Development environment detected, using binaries from $DEV_TARGET_DIR..."
    
    # Start monitor service
    echo "Starting monitor service..."
    $DEV_MONITOR_SERVICE -c $CONFIG_FILE &
    MONITOR_PID=$!
    echo "Monitor service started with PID: $MONITOR_PID"
    
    # Start telemetry service
    echo "Starting telemetry service..."
    $DEV_TELEMETRY_SERVICE -c $CONFIG_FILE &
    TELEMETRY_PID=$!
    echo "Telemetry service started with PID: $TELEMETRY_PID"
    
    # Start scheduler service
    echo "Starting scheduler service..."
    $DEV_SCHEDULER_SERVICE -c $CONFIG_FILE &
    SCHEDULER_PID=$!
    echo "Scheduler service started with PID: $SCHEDULER_PID"
    
    # Start app service in foreground
    echo "Starting app service in foreground..."
    $DEV_APP_SERVICE -c $CONFIG_FILE
elif [ -f "$PROD_MONITOR_SERVICE" ]; then
    # Production environment
    echo "Production environment detected..."
    
    # Start monitor service
    echo "Starting monitor service..."
    $PROD_MONITOR_SERVICE -c $PROD_CONFIG_FILE &
    MONITOR_PID=$!
    echo "Monitor service started with PID: $MONITOR_PID"
    
    # Start telemetry service
    echo "Starting telemetry service..."
    $PROD_TELEMETRY_SERVICE -c $PROD_CONFIG_FILE &
    TELEMETRY_PID=$!
    echo "Telemetry service started with PID: $TELEMETRY_PID"
    
    # Start scheduler service
    echo "Starting scheduler service..."
    $PROD_SCHEDULER_SERVICE -c $PROD_CONFIG_FILE &
    SCHEDULER_PID=$!
    echo "Scheduler service started with PID: $SCHEDULER_PID"
    
    # Start app service in foreground
    echo "Starting app service in foreground..."
    $PROD_APP_SERVICE -c $PROD_CONFIG_FILE
else
    echo "Error: Could not find service executables in either development or production locations."
    echo "Please run build_services.bash first to build the services."
    exit 1
fi

# The script will continue running until the app service exits or the script is terminated
# When either happens, the cleanup function will be called to stop all services
