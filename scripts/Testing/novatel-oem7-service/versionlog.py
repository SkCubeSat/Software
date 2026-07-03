#!/usr/bin/env python3

# -----------------------------------------------------------------------------#
# Script to test Novatel OEM7 connected to OBC, without service usage. Asks for version info from GNSS.
# -----------------------------------------------------------------------------#

import serial
import time

# UART4 configuration for AM335x based OBC
PORT = '/dev/ttyS4'  # Standard path for UART4 in Kubos/AM335x
BAUD = 9600        # Common default for OEM6; check your hardware docs

def main():
    try:
        print(f"Opening {PORT} at {BAUD} baud...")
        # Open the port. rtscts=False ensures Python doesn't get blocked by flow control pins
        ser = serial.Serial(PORT, BAUD, timeout=2, rtscts=False)

        # Give the port a moment to stabilize after opening
        time.sleep(0.5)

        # Clear any stale data sitting in the receive buffer
        ser.reset_input_buffer()

        # Request the VERSION log in ASCII format, just one time
        command = b'LOG VERSIONA ONCE\r\n'
        print(f"Sending command: {command.decode('ascii').strip()}")
        ser.write(command)

        # Give the device time to process and reply
        time.sleep(1)

        print("\n--- Device Response ---")
        # Read lines until the buffer is empty
        while ser.in_waiting > 0:
            line = ser.readline()
            try:
                # Decode and print the ASCII response
                print(line.decode('ascii', errors='replace').strip())
            except Exception as e:
                print(f"[Raw Data]: {line}")

        print("-----------------------\n")
        ser.close()

    except Exception as e:
        print(f"Serial port error: {e}")

if __name__ == "__main__":
    main()