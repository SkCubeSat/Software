#!/usr/bin/env python3

# -----------------------------------------------------------------------------#
# Script to test Novatel OEM7 service on the OBC, using GraphQL requests.
# Sends various GraphQL queries to request different types of information from the service. Fully Dynamic.
# -----------------------------------------------------------------------------#

import urllib.request
import json
import sys

# Default configuration based on standard Kubos service setup
SERVICE_URL = "http://0.0.0.0:8130/graphql"

# --- PREDEFINED QUERIES ---
QUERIES = {
    "ping": "{ ping }",
    "ack": "{ ack }",
    "errors": "{ errors }",
    "power": "{ power { state, uptime } }",
    "config": "{ config }",
    "testResults": """{ 
        testResults { 
            success 
            telemetryNominal { 
                systemStatus { status } 
            } 
        } 
    }""",
    "systemStatus": """{ 
        systemStatus { 
            errors 
            status 
        } 
    }""",
    "lockStatus": """{ 
        lockStatus { 
            positionStatus 
            positionType 
            time { ms, week } 
            timeStatus 
            velocityStatus 
            velocityType 
        } 
    }""",
    "lockInfo": """{ 
        lockInfo { 
            position 
            time { ms, week } 
            velocity 
        } 
    }""",
    "telemetry": """{ 
        telemetry { 
            debug { 
                components { bootVersion, compType, compileDate, compileTime, hwVersion, model, serialNum, swVersion }
                numComponents 
            } 
            nominal { 
                lockInfo { position, time { ms, week }, velocity }
                lockStatus { positionStatus, positionType, time { ms, week }, timeStatus, velocityStatus, velocityType }
                systemStatus { errors, status }
            } 
        } 
    }"""
}

# --- PREDEFINED MUTATIONS ---
MUTATIONS = {
    "mut_errors": "mutation { errors }",
    "noop": """mutation { 
        noop { 
            errors 
            success 
        } 
    }""",
    "controlPower": "mutation { controlPower }",
    "configureHardware": """mutation($config: [ConfigStruct!]) { 
        configureHardware(config: $config) { 
            config 
            errors 
            success 
        } 
    }""",
    "testHardware": """mutation($testType: TestType!) { 
        testHardware(test: $testType) { 
            ... on IntegrationTestResults { errors, success } 
            ... on HardwareTestResults { errors, success } 
        } 
    }""",
    "issueRawCommand": """mutation($cmd: String!) { 
        issueRawCommand(command: $cmd) { 
            errors 
            success 
            response 
        } 
    }"""
}

def send_request(query, variables=None):
    """Sends the GraphQL request to the OEM7 Kubos service."""
    payload = {"query": query}
    if variables:
        payload["variables"] = variables

    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(SERVICE_URL, data=data, headers={"Content-Type": "application/json"})
    
    try:
        with urllib.request.urlopen(req, timeout=5) as response:
            result = json.loads(response.read().decode("utf-8"))
            print(json.dumps(result, indent=2))
    except urllib.error.URLError as e:
        print(f"\n[!] Network Error: Could not connect to {SERVICE_URL}. Is the service running?")
        print(f"Details: {e}\n")
    except Exception as e:
        print(f"\n[!] Error parsing response: {e}\n")

def print_menu():
    print("\n" + "="*40)
    print(" NovAtel OEM7 GraphQL Client")
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
        elif choice in ["mut_errors", "noop", "controlPower"]:
            print(f"\nExecuting Mutation: {choice}...")
            send_request(MUTATIONS[choice])
        
        # Handle Dynamic Mutations (Requires User Input)
        elif choice == "issueRawCommand":
            cmd = input("Enter the ASCII command to send (e.g. LOG VERSIONA ONCE): ").strip()
            if cmd:
                variables = {"cmd": cmd}
                send_request(MUTATIONS["issueRawCommand"], variables)
            else:
                print("Command aborted.")

        elif choice == "testHardware":
            print("Available Test Types: INTEGRATION, HARDWARE")
            test_type = input("Enter test type (default: INTEGRATION): ").strip() or "INTEGRATION"
            variables = {"testType": test_type}
            send_request(MUTATIONS["testHardware"], variables)

        elif choice == "configureHardware":
            print("Creating a simple LOG_POSITION_DATA configuration.")
            interval = input("Enter interval in seconds (default: 1.0): ").strip()
            interval = float(interval) if interval else 1.0
            
            variables = {
                "config": [{
                    "option": "LOG_POSITION_DATA",
                    "hold": False,
                    "interval": interval,
                    "offset": 0.0
                }]
            }
            send_request(MUTATIONS["configureHardware"], variables)

        else:
            print("[!] Invalid choice. Type 'menu' to see available options.")

if __name__ == "__main__":
    main()