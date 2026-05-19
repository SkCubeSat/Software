#!/usr/bin/env python3

# -----------------------------------------------------------------------------#
# Script to test Analog IC connected to OBC, without service usage. 
# Requests for telemetry from the board.
# -----------------------------------------------------------------------------#

import time
import os
import fcntl

# --- Configuration ---
I2C_BUS = 1
SLAVE_ADDR = 0x13

# Linux I2C device control constant (from linux/i2c-dev.h)
I2C_SLAVE = 0x0703

# Command definitions
I2C_CMD_START = 99
I2C_CMD_SEND_DATA = 197

def send_command(fd, command):
    """Writes a single command byte to the STM32."""
    try:
        os.write(fd, bytes([command]))
        print(f"Command 0x{command:02X} sent.")
    except Exception as e:
        print(f"Error writing command: {e}")

def data_request_matrix(fd, command, rows=5, cols=10):
    """
    Send command, wait for STM32 to prep buffer, then read back 
    rows*cols 16-bit values plus a 24-byte ASCII timestamp.
    """
    data_bytes = rows * cols * 2
    ts_bytes = 24                # length of "TS:YYYY-MM-DD hh:mm:ss\r\n"
    total_bytes = data_bytes + ts_bytes

    # 1) Send the command
    send_command(fd, command)

    # 2) CRITICAL: Give the STM32 time to prepare its buffer
    print("Waiting 500ms for STM32 buffer...")
    time.sleep(0.5)

    # 3) Read exactly that many bytes directly from the device file
    try:
        raw_bytes = os.read(fd, total_bytes)
        raw = list(raw_bytes)
    except Exception as e:
        print(f"Error reading data (Error 121 likely happens here): {e}")
        return None, None

    # Verify we got the expected amount of data
    if len(raw) != total_bytes:
        print(f"Warning: Expected {total_bytes} bytes, but got {len(raw)} bytes.")

    # 4) Split into data vs timestamp
    data_raw = raw[:data_bytes]
    ts_raw = raw[data_bytes:]

    # 5) Unpack into list of little-endian uint16
    vals = [data_raw[i] | (data_raw[i+1] << 8) for i in range(0, len(data_raw), 2)]

    # 6) Reshape into matrix
    matrix = []
    for r in range(rows):
        start = r * cols
        matrix.append(vals[start:start + cols])

    # 7) Decode timestamp (ignoring null or garbage bytes if they exist)
    timestamp = ''.join(chr(b) for b in ts_raw if b < 128)

    return matrix, timestamp

def main():
    print("Testing STM32 communication on KubOS using direct OS calls...")
    
    bus_path = f"/dev/i2c-{I2C_BUS}"
    
    try:
        # Open the I2C bus as a standard Linux file
        fd = os.open(bus_path, os.O_RDWR)
        
        # Tell the Linux kernel which I2C address we want to talk to
        fcntl.ioctl(fd, I2C_SLAVE, SLAVE_ADDR)
        
        # Start the testing routine
        send_command(fd, I2C_CMD_START)
        
        # Give the test a moment to run before requesting data
        time.sleep(1)
        
        # Request the matrix data
        matrix, ts = data_request_matrix(fd, I2C_CMD_SEND_DATA, rows=5, cols=10)
        
        if matrix:
            print("\nReceived data matrix (5x10):")
            for row in matrix:
                print("  ", row)
            print(f"Timestamp: {ts.strip()}")
            
        os.close(fd)
            
    except FileNotFoundError:
        print(f"Error: I2C bus {bus_path} not found.")
    except PermissionError:
        print("Error: Permission denied. Try running with 'sudo'.")
    except Exception as e:
        print(f"Unexpected error: {e}")

if __name__ == '__main__':
    main()