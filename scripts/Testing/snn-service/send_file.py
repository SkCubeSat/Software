import serial
import sys
import time
import zlib

PORT = "/dev/ttyUSB1"
BAUD = 115200

if len(sys.argv) != 3:
    print(f"usage: {sys.argv[0]} <frame_id> <file>")
    sys.exit(1)

frame_id = int(sys.argv[1])
path = sys.argv[2]

with open(path, "rb") as f:
    data = f.read()

size = len(data)
crc = zlib.crc32(data) & 0xffffffff

ser = serial.Serial(PORT, BAUD, timeout=2)
time.sleep(0.5)

# Drain any startup text already waiting
time.sleep(0.2)
banner = ser.read_all()
if banner:
    print("banner:")
    print(banner.decode(errors="replace"))

cmd = f"SEND {frame_id} {size} {crc:08X}\n".encode()
print("send cmd:", cmd.decode().strip())
ser.write(cmd)

resp = ser.readline().decode(errors="replace").strip()
print("resp:", resp)
if resp != "READY":
    print("did not receive READY")
    sys.exit(2)

ser.write(data)
ser.flush()

resp = ser.readline().decode(errors="replace").strip()
print("final:", resp)