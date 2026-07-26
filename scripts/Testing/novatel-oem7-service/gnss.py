#!/usr/bin/env python3

# -----------------------------------------------------------------------------#
# Script to test Novatel OEM7 connected to OBC, without service usage. 
# Requests best position (BESTXYZ), rxstatus, and hwmonitor logs for telemetry.
# -----------------------------------------------------------------------------#

import serial
import time

PORT = '/dev/ttyS4'
BAUD = 9600

def get_telemetry():
    telemetry = {
        "systemStatus": {"errors": False, "status_hex": ""},
        "power": {"state": "OFF", "uptime_sec": 0, "voltage_3v3": 0.0},
        "lockInfo": {"position_status": "UNKNOWN", "x": 0.0, "y": 0.0, "z": 0.0}
    }

    try:
        ser = serial.Serial(PORT, BAUD, timeout=1, rtscts=False)
        time.sleep(0.2)
        ser.reset_input_buffer()

        # Request our 3 core telemetry logs
        ser.write(b'LOG RXSTATUSA ONCE\r\n')
        ser.write(b'LOG HWMONITORA ONCE\r\n')
        ser.write(b'LOG BESTXYZA ONCE\r\n')
        time.sleep(1) # Wait for processing and 9600 baud transmission

        # Parse the incoming lines
        while ser.in_waiting > 0:
            line = ser.readline().decode('ascii', errors='ignore').strip()

            if line.startswith("#RXSTATUSA"):
                # Example: ...;00000000,5,024c0000...
                data_payload = line.split(';')[1]
                fields = data_payload.split(',')
                telemetry["systemStatus"]["errors"] = (fields[0] != "00000000")
                telemetry["systemStatus"]["status_hex"] = fields[2]

            elif line.startswith("#HWMONITORA"):
                # Example: ...;6,44.32,100,3.18...
                data_payload = line.split(';')[1]
                fields = data_payload.split(',')
                telemetry["power"]["state"] = "ON"
                # Grab internal temp and one of the 3.3V rail readings
                telemetry["power"]["temp_c"] = float(fields[1])
                telemetry["power"]["voltage_3v3"] = float(fields[3])

            elif line.startswith("<UPTIME"):
                fields = line.split()
                telemetry["power"]["uptime_sec"] = float(fields[7])

            elif line.startswith("#BESTXYZA"):
                # Example: ...;INSUFFICIENT_OBS,NONE,0.0000,0.0000,0.0000...
                data_payload = line.split(';')[1]
                fields = data_payload.split(',')
                telemetry["lockInfo"]["position_status"] = fields[0]
                telemetry["lockInfo"]["x"] = float(fields[2])
                telemetry["lockInfo"]["y"] = float(fields[3])
                telemetry["lockInfo"]["z"] = float(fields[4])

        ser.close()
        return telemetry

    except Exception as e:
        print(f"Error communicating with GNSS: {e}")
        return None

if __name__ == "__main__":
    print("Fetching GNSS Telemetry...\n")
    data = get_telemetry()

    if data:
        import json
        print(json.dumps(data, indent=2))