#!/usr/bin/env python3

# -----------------------------------------------------------------------------#
# Script to test Analog IC service on the OBC, using GraphQL requests.
# Sends various GraphQL queries to request different types of information from the service. Fully Dynamic.
# -----------------------------------------------------------------------------#

import urllib.request
import json
import sys

# Default configuration based on standard Kubos service setup
SERVICE_URL = "http://0.0.0.0:8002/graphql"

# --- PREDEFINED QUERIES ---
QUERIES = {
    "ping": "{ ping }",
    "ack": "{ ack }",
    "errors": "{ errors }",
    "telemetry": """{
        telemetry {
            icReadings
            timestamp
            rawData
        }
    }""",
    "rtcTime": """{
        rtcTime {
            year
            month
            day
            weekday
            hour
            minute
            second
        }
    }""",
    "powerStatus": """{
        powerStatus {
            mode
            rawValue
        }
    }""",
    "latestTimestamp": """{
        latestTimestamp {
            bytes
        }
    }"""
}

# --- PREDEFINED MUTATIONS ---
MUTATIONS = {
    "noop": """mutation {
        noop {
            success
            errors
        }
    }""",
    "reset": """mutation {
        reset {
            success
            errors
        }
    }""",
    "start": """mutation {
        start {
            success
            errors
        }
    }""",
    "powerSavingMode": """mutation {
        powerSavingMode {
            success
            errors
        }
    }""",
    "normalPowerMode": """mutation {
        normalPowerMode {
            success
            errors
        }
    }""",
    "setRtcTime": """mutation {
        setRtcTime {
            success
            errors
        }
    }""",
    "issueRawCommand": """mutation($command: Int!, $data: [Int!]!) {
        issueRawCommand(command: $command, data: $data) {
            success
            errors
        }
    }"""
}

def send_request(query, variables=None):
    """Sends the GraphQL request to the Analog IC Kubos service."""
    payload = {"query": query}
    if variables:
        payload["variables"] = variables

    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(SERVICE_URL, data=data, headers={"Content-Type": "application/json"})

    try:
        with urllib.request.urlopen(req, timeout=10) as response:
            result = json.loads(response.read().decode("utf-8"))
            print(json.dumps(result, indent=2))
    except urllib.error.URLError as e:
        print(f"\n[!] Network Error: Could not connect to {SERVICE_URL}. Is the service running?")
        print(f"Details: {e}\n")
    except Exception as e:
        print(f"\n[!] Error parsing response: {e}\n")

def print_menu():
    print("\n" + "="*40)
    print(" Analog IC Service GraphQL Client")
    print("="*40)
    print("--- QUERIES ---")
    for q in QUERIES.keys():
        print(f"  {q}")
    print("\n--- MUTATIONS ---")
    for m in MUTATIONS.keys():
        print(f"  {m}")
    print("\nType 'exit' or 'quit' to close.")
    print("="*40)

def main():
    if len(sys.argv) > 1:
        global SERVICE_URL
        SERVICE_URL = sys.argv[1]

    print(f"Targeting service at: {SERVICE_URL}")
    print_menu()

    while True:
        try:
            choice = input("\nEnter query/mutation name (or 'menu'): ").strip()
        except (KeyboardInterrupt, EOFError):
            print("\nExiting...")
            break

        if choice.lower() in ['exit', 'quit']:
            break
        elif choice.lower() == 'menu':
            print_menu()
            continue
        elif not choice:
            continue

        # Handle Queries
        if choice in QUERIES:
            print(f"\nExecuting Query: {choice}...")
            send_request(QUERIES[choice])

        # Handle simple Mutations
        elif choice in ["noop", "reset", "start", "powerSavingMode", "normalPowerMode", "setRtcTime"]:
            print(f"\nExecuting Mutation: {choice}...")
            send_request(MUTATIONS[choice])

        # Handle parameterized Mutation: issueRawCommand
        elif choice == "issueRawCommand":
            try:
                cmd_input = input("Enter command byte (integer, e.g. 97 for 0x61): ").strip()
                command_val = int(cmd_input)

                data_input = input("Enter data bytes as comma-separated integers (e.g. 1, 2, 3) or leave blank for none: ").strip()
                if data_input:
                    data_val = [int(x.strip()) for x in data_input.split(",")]
                else:
                    data_val = [0]

                variables = {
                    "command": command_val,
                    "data": data_val
                }
                print(f"\nExecuting Mutation: {choice}...")
                send_request(MUTATIONS[choice], variables)
            except ValueError:
                print("\n[!] Invalid input. Command and data must be integers.")

        else:
            print("[!] Invalid choice. Type 'menu' to see available options.")

if __name__ == "__main__":
    main()