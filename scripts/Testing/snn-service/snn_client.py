#!/usr/bin/env python3

# -----------------------------------------------------------------------------#
# Script to test SNN service on the OBC, using GraphQL requests.
# Sends various GraphQL queries to request different types of information from the service. Fully Dynamic.
# -----------------------------------------------------------------------------#

import urllib.request
import json
import sys

# Default configuration based on standard Kubos service setup
SERVICE_URL = "http://127.0.0.1:8092/graphql"

# --- PREDEFINED QUERIES ---
QUERIES = {
    "ping": "{ ping }",
    "health": """{ 
        health { 
            uartBus 
            uartBaud 
            phase 
            queueDepth 
            queueCapacity 
            jobsCompleted 
            jobsFailed 
            lastError 
        } 
    }""",
    "state": """{ 
        state { 
            phase 
            currentImageId 
            queueDepth 
            queueCapacity 
            queuedImageIds 
            lastError 
        } 
    }""",
    "inferenceStatus": """query($imageId: Int!) { 
        inferenceStatus(imageId: $imageId) { 
            imageId 
            phase 
            queuePosition 
            error 
        } 
    }""",
    "getResult": """query($imageId: Int!) { 
        getResult(imageId: $imageId) { 
            imageId 
            sizeBytes 
            crc32 
            bitmapBase64 
            phase 
        } 
    }"""
}

# --- PREDEFINED MUTATIONS ---
MUTATIONS = {
    "submitImage": """mutation($imageBase64: String!) { 
        submitImage(imageBase64: $imageBase64) { 
            success 
            accepted 
            imageId 
            queuePosition 
            queueDepth 
            error 
        } 
    }""",
    "infer": """mutation($imageBase64: String!) { 
        infer(imageBase64: $imageBase64) { 
            success 
            imageId 
            sizeBytes 
            crc32 
            bitmapBase64 
            error 
        } 
    }""",
    "cancel": """mutation($imageId: Int!) { 
        cancel(imageId: $imageId) { 
            success 
            cancelled 
            error 
        } 
    }"""
}

def send_request(query, variables=None):
    """Sends the GraphQL request to the SNN Kubos service."""
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
    print(" SNN Service GraphQL Client")
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

        # Handle simple Queries
        if choice in ["ping", "health", "state"]:
            print(f"\nExecuting Query: {choice}...")
            send_request(QUERIES[choice])
        
        # Handle parameterized Queries
        elif choice in ["inferenceStatus", "getResult"]:
            try:
                img_id_input = input("Enter Image ID (integer): ").strip()
                image_id = int(img_id_input)
                variables = {"imageId": image_id}
                print(f"\nExecuting Query: {choice}...")
                send_request(QUERIES[choice], variables)
            except ValueError:
                print("\n[!] Invalid input. Image ID must be an integer.")

        # Handle parameterized Mutations for Submitting/Inferring Base64 Images
        elif choice in ["submitImage", "infer"]:
            img_b64 = input("Enter Base64 Image String: ").strip()
            if img_b64:
                variables = {"imageBase64": img_b64}
                print(f"\nExecuting Mutation: {choice}...")
                send_request(MUTATIONS[choice], variables)
            else:
                print("Command aborted. Empty Base64 string provided.")

        # Handle Mutation for Cancelling Job
        elif choice == "cancel":
            try:
                img_id_input = input("Enter Image ID to cancel (integer): ").strip()
                image_id = int(img_id_input)
                variables = {"imageId": image_id}
                print(f"\nExecuting Mutation: {choice}...")
                send_request(MUTATIONS[choice], variables)
            except ValueError:
                print("\n[!] Invalid input. Image ID must be an integer.")

        else:
            print("[!] Invalid choice. Type 'menu' to see available options.")

if __name__ == "__main__":
    main()