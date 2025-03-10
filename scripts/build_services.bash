#!/bin/bash

# Script to build Kubos services
# Builds monitor-service, telemetry-service, app-service, and scheduler-service

# Kubos directory
KUBOS_DIR="/home/vagrant/kubos"

# Service directories
MONITOR_SERVICE_DIR="$KUBOS_DIR/services/monitor-service"
TELEMETRY_SERVICE_DIR="$KUBOS_DIR/services/telemetry-service"
APP_SERVICE_DIR="$KUBOS_DIR/services/app-service"
SCHEDULER_SERVICE_DIR="$KUBOS_DIR/services/scheduler-service"

# Function to build a service
build_service() {
    local service_dir=$1
    local service_name=$2
    
    echo "Building $service_name..."
    
    if [ ! -d "$service_dir" ]; then
        echo "Error: Service directory not found: $service_dir"
        return 1
    fi
    
    # Navigate to service directory and build
    cd "$service_dir" || return 1
    cargo build
    
    if [ $? -ne 0 ]; then
        echo "Error: Failed to build $service_name"
        return 1
    fi
    
    echo "$service_name built successfully"
    return 0
}

# Check if Kubos directory exists
if [ ! -d "$KUBOS_DIR" ]; then
    echo "Error: Kubos directory not found: $KUBOS_DIR"
    exit 1
fi

echo "Starting build process for Kubos services..."

# Build monitor service
build_service "$MONITOR_SERVICE_DIR" "monitor-service" || exit 1

# Build telemetry service
build_service "$TELEMETRY_SERVICE_DIR" "telemetry-service" || exit 1

# Build app service
build_service "$APP_SERVICE_DIR" "app-service" || exit 1

# Build scheduler service
build_service "$SCHEDULER_SERVICE_DIR" "scheduler-service" || exit 1

echo "All services built successfully!"
echo "You can now run the services using ./run_services.bash"

exit 0 