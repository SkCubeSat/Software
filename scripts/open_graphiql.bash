#!/bin/bash

# Script to open GraphiQL interfaces for Kubos services in the default browser
# Opens tabs for app-service, telemetry-service, and scheduler-service

# Base IP address
IP_ADDRESS="192.168.56.10"

# Config file location - check both development and production locations
DEV_CONFIG_DIR="../configs"
CONFIG_FILE="$DEV_CONFIG_DIR/dev_config.toml"
PROD_CONFIG_FILE="/etc/kubos-config.toml"

# Determine which config file to use
if [ -f "$CONFIG_FILE" ]; then
    echo "Using development config file: $CONFIG_FILE"
    ACTIVE_CONFIG="$CONFIG_FILE"
elif [ -f "$PROD_CONFIG_FILE" ]; then
    echo "Using production config file: $PROD_CONFIG_FILE"
    ACTIVE_CONFIG="$PROD_CONFIG_FILE"
else
    echo "Error: Could not find config file in either development or production locations."
    echo "Checking current directory for local_config.toml..."
    
    if [ -f "kubos/tools/local_config.toml" ]; then
        echo "Found config file in current workspace."
        ACTIVE_CONFIG="kubos/tools/local_config.toml"
    else
        echo "Error: Could not find config file."
        exit 1
    fi
fi

# Function to extract port from config file
extract_port() {
    local service=$1
    local port=$(grep -A 3 "\\[$service.addr\\]" "$ACTIVE_CONFIG" | grep "port" | awk -F "=" '{print $2}' | tr -d ' ')
    echo "$port"
}

# Extract ports from config file
APP_SERVICE_PORT=$(extract_port "app-service")
TELEMETRY_SERVICE_PORT=$(extract_port "telemetry-service")
SCHEDULER_SERVICE_PORT=$(extract_port "scheduler-service")

# Check if ports were successfully extracted
if [ -z "$APP_SERVICE_PORT" ] || [ -z "$TELEMETRY_SERVICE_PORT" ] || [ -z "$SCHEDULER_SERVICE_PORT" ]; then
    echo "Error: Failed to extract one or more ports from config file."
    echo "Using default ports:"
    [ -z "$APP_SERVICE_PORT" ] && APP_SERVICE_PORT=8000 && echo "- App Service: 8000"
    [ -z "$TELEMETRY_SERVICE_PORT" ] && TELEMETRY_SERVICE_PORT=8020 && echo "- Telemetry Service: 8020"
    [ -z "$SCHEDULER_SERVICE_PORT" ] && SCHEDULER_SERVICE_PORT=8010 && echo "- Scheduler Service: 8010"
else
    echo "Successfully extracted ports from config file:"
    echo "- App Service: $APP_SERVICE_PORT"
    echo "- Telemetry Service: $TELEMETRY_SERVICE_PORT"
    echo "- Scheduler Service: $SCHEDULER_SERVICE_PORT"
fi

# URLs for GraphiQL interfaces
APP_SERVICE_URL="http://${IP_ADDRESS}:${APP_SERVICE_PORT}/graphiql"
TELEMETRY_SERVICE_URL="http://${IP_ADDRESS}:${TELEMETRY_SERVICE_PORT}/graphiql"
SCHEDULER_SERVICE_URL="http://${IP_ADDRESS}:${SCHEDULER_SERVICE_PORT}/graphiql"

echo "Opening GraphiQL interfaces in your default browser..."

# Function to detect the operating system and open URLs accordingly
open_urls() {
    case "$(uname -s)" in
        Linux*)
            # Try different browsers available on Linux
            if command -v xdg-open > /dev/null; then
                xdg-open "$APP_SERVICE_URL"
                sleep 1
                xdg-open "$TELEMETRY_SERVICE_URL"
                sleep 1
                xdg-open "$SCHEDULER_SERVICE_URL"
            elif command -v firefox > /dev/null; then
                firefox "$APP_SERVICE_URL" "$TELEMETRY_SERVICE_URL" "$SCHEDULER_SERVICE_URL"
            elif command -v google-chrome > /dev/null; then
                google-chrome "$APP_SERVICE_URL" "$TELEMETRY_SERVICE_URL" "$SCHEDULER_SERVICE_URL"
            else
                echo "Error: Could not find a suitable browser on your system."
                exit 1
            fi
            ;;
        Darwin*)
            # macOS
            open "$APP_SERVICE_URL" "$TELEMETRY_SERVICE_URL" "$SCHEDULER_SERVICE_URL"
            ;;
        CYGWIN*|MINGW*|MSYS*)
            # Windows
            start "$APP_SERVICE_URL" "$TELEMETRY_SERVICE_URL" "$SCHEDULER_SERVICE_URL"
            ;;
        *)
            echo "Error: Unsupported operating system."
            exit 1
            ;;
    esac
}

# Open the URLs
open_urls

echo "Opened the following GraphiQL interfaces:"
echo "- App Service: $APP_SERVICE_URL"
echo "- Telemetry Service: $TELEMETRY_SERVICE_URL"
echo "- Scheduler Service: $SCHEDULER_SERVICE_URL" 