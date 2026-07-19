#!/usr/bin/env python3

# -----------------------------------------------------------------------------#
# Script to test Novatel OEM7 connected to OBC, without service usage. 
# Asks for the best position (BESTXYZ).
# -----------------------------------------------------------------------------#

import serial
import time

PORT = '/dev/ttyS1'
BAUD = 9600

def main():
    try:
        print(f"Opening {PORT} at {BAUD} baud...")
        # Open the port without flow control blocking
        ser = serial.Serial(PORT, BAUD, timeout=2, rtscts=False)
        
        # Give the port a moment to stabilize after opening
        time.sleep(0.5)
        
        # Clear any stale data sitting in the receive buffer
        ser.reset_input_buffer()
        
        # Request the BESTXYZ log in ASCII format, just one time
        command = b'LOG BESTXYZA ONCE\r\n'
        print(f"Sending command: {command.decode('ascii').strip()}")
        ser.write(command)
        
        # Give the device time to compute and reply
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