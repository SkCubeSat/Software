#!/usr/bin/env python3

# -----------------------------------------------------------------------------#
# Script to test Dosimeter connected to OBC, without service usage. 
# Requests for telemetry readings for all 8 sensors.
# -----------------------------------------------------------------------------#

import sys
import math
import time
import os
import fcntl

# Constants -----------------------------
I2C_BUS = 1
I2C_SLAVE_CTL = 0x0703                  # Linux I2C device control constant
DOSIMETER_I2C_SLAVE_ADDR = 0x4B         # I2C Address

DOSIMETER_COMMAND_LENGTH = 1
DOSIMETER_RESPONSE_LENGTH = 2
DOSIMETER_RESPONSE_HIGH_BYTE_MASK = 0x0F

ADC_BIT_RESOLUTION = 12
MAX_ADC_VALUE = 4096.0 - 1
SAMPLES_TO_AVERAGE = 10

TIMER_DELAY = 1
V_REF = 3328 # (mV) - Note: Update this if your internal VREF is not 3.328V!
#---------------------------------------

# USING INTERNAL REFERENCE COMMAND CODES
dosimeter_list = [
    0x8C, # channel 0 -> u2 ->
    0xCC, # channel 1 -> u3 -> 50
    0x9C, # channel 2 -> u4 -> 100
    0xDC, # channel 3 -> u5 -> 200
    0xAC, # channel 4 -> u6 -> 20
    0xEC, # channel 5 -> u7 -> 300 mil
    0xBC, # channel 6 -> u8 -> 300 mil
    0xFC  # channel 7 -> u9 -> 300 mil
]

hex_to_chip = {
    dosimeter_list[0] : "SENSOR1 (U2)",
    dosimeter_list[1] : "SENSOR2 (U3)",
    dosimeter_list[2] : "SENSOR3 (U4)",
    dosimeter_list[3] : "SENSOR4 (U5)",
    dosimeter_list[4] : "SENSOR5 (U6)",
    dosimeter_list[5] : "SENSOR6 (U7)",
    dosimeter_list[6] : "SENSOR7 (U8)",
    dosimeter_list[7] : "SENSOR8 (U9)"
}
    
str_to_print = ""

sensor_history = {addr: [] for addr in dosimeter_list}

def addStr(entry):
    global str_to_print
    str_to_print += entry
    str_to_print += ", "

def printString():
    global str_to_print
    print(str_to_print)
    str_to_print = ""

def switch_sensor(fd, sensor_code):
    try:
        os.write(fd, bytes([sensor_code]))
    except Exception as e:
        print(f"Error switching sensor {hex(sensor_code)}: {e}")

def read_raw(fd, addr):
    try:
        # Write the command byte
        os.write(fd, bytes([addr]))
        # Read 2 bytes back
        raw_bytes = os.read(fd, DOSIMETER_RESPONSE_LENGTH)
        
        # Mask out 4 leading zeros as shown in Figure 17
        return ((raw_bytes[0] & DOSIMETER_RESPONSE_HIGH_BYTE_MASK) << 8) | raw_bytes[1]
    except Exception as e:
        print(f"Error reading raw data for {hex(addr)}: {e}")
        return 0

def get_averaged_voltage(fd, cmd):
    new_val = read_raw(fd, cmd)
    sensor_history[cmd].append(new_val)
    
    if len(sensor_history[cmd]) > SAMPLES_TO_AVERAGE:
        sensor_history[cmd].pop(0)
    
    avg_raw = sum(sensor_history[cmd]) / len(sensor_history[cmd])
    
    voltage = (avg_raw / MAX_ADC_VALUE) * V_REF
    return avg_raw, voltage

def read_dosimeter(fd, dos_code):
    global str_to_print
    switch_sensor(fd, dos_code)
    
    addStr(f"{time.time()}")
    addStr(hex(dos_code).upper())
    addStr(hex_to_chip[dos_code])
    
    reading = get_averaged_voltage(fd, dos_code)
    addStr(f"{reading[0]:.2f}")
    
    # Prevent Trailing Comma
    str_to_print += f"{reading[1]:.2f}"

    printString()

def loop_sensors(fd):
    global dosimeter_list
    for sensor in dosimeter_list:
        read_dosimeter(fd, sensor)
    
    time.sleep(TIMER_DELAY) # Delay between sensors
    return 0

def main():
    bus_path = f"/dev/i2c-{I2C_BUS}"
    fd = None
    
    try:
        # Open the I2C bus file descriptor
        fd = os.open(bus_path, os.O_RDWR)
        
        # Bind the file descriptor to the Dosimeter I2C slave address
        fcntl.ioctl(fd, I2C_SLAVE_CTL, DOSIMETER_I2C_SLAVE_ADDR)
        print(f"Successfully connected to dosimeter at {hex(DOSIMETER_I2C_SLAVE_ADDR)} on {bus_path}")
        print("Using INTERNAL voltage reference.")
        print("Timestamp, Hex_Code, Sensor, Avg_Raw, Voltage(mV)")
        print("-" * 65)
        
        # Collect Data forever
        while True:
            loop_sensors(fd)
            
    except FileNotFoundError:
        print(f"Error: I2C bus {bus_path} not found.")
    except PermissionError:
        print("Error: Permission denied. Try running with 'sudo'.")
    except KeyboardInterrupt:
        print("\nExiting data collection...")
    except Exception as e:
        print(f"Unexpected error: {e}")
    finally:
        if fd is not None:
            try:
                os.close(fd)
            except OSError:
                pass

if __name__ == "__main__":
    main()