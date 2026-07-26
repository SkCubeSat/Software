#!/usr/bin/env python3

# -----------------------------------------------------------------------------#
# Script to test Novatel OEM7 connected to OBC, without service usage. 
# Collects any buffered data, then send Unlogall command to GNSS
# -----------------------------------------------------------------------------#

import serial
import time
import binascii

# Confirm this matches your setup
SERIAL_PORT = '/dev/ttyS4'
BAUD_RATE = 9600

def main():
    try:
        print(f"--- Opening {SERIAL_PORT} at {BAUD_RATE} ---")
        ser = serial.Serial(SERIAL_PORT, BAUD_RATE, timeout=0.1)

        # TEST 1: PASSIVE LISTENING
        print("\n[Test 1] Listening for 3 seconds (Don't send anything)...")
        start_time = time.time()
        rx_buffer = b""

        while time.time() - start_time < 3:
            if ser.in_waiting:
                chunk = ser.read(ser.in_waiting)
                rx_buffer += chunk
                # Print dots to show activity
                print(".", end="", flush=True)
            time.sleep(0.1)

        print("\n")
        if len(rx_buffer) == 0:
            print("RESULT: No data received. Device is silent or Rx pin disconnected.")
        else:
            print(f"RESULT: Received {len(rx_buffer)} bytes.")
            print(f"Sample (Hex): {binascii.hexlify(rx_buffer[:20])}")
            try:
                print(f"Sample (ASCII): {rx_buffer[:50].decode('ascii', errors='replace')}")
            except:
                pass

        # TEST 2: ACTIVE COMMAND (The 'Tx' Test)
        print("\n[Test 2] Attempting to send UNLOGALL...")
        ser.reset_input_buffer()
        # Send command with proper terminators
        ser.write(b'UNLOGALL TRUE\r\n')

        # Wait for ack
        time.sleep(1.0)
        response = ser.read_all()

        if len(response) == 0:
            print("RESULT: NO RESPONSE. Tx line likely broken/disconnected.")
        else:
            print(f"RESULT: Device Responded! \nRaw: {response}")

        ser.close()

    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()