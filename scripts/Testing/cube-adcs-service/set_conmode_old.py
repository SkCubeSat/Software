#!/usr/bin/env python3

# -----------------------------------------------------------------------------#
# Script to test Cube ADCS connected to OBC, without service usage. 
# Script sends a command to set the control mode on the ADCS.
# -----------------------------------------------------------------------------#

import socket
import struct
import sys
import time

# ============================================================
# CubeSpace Message Types
# ============================================================

MSG_TYPE_TC               = 1
MSG_TYPE_TC_ACK           = 2
MSG_TYPE_TC_NACK          = 3

CAN_EFF_FLAG = 0x80000000
CAN_EFF_MASK = 0x1FFFFFFF

CAN_FRAME_FORMAT = "=IB3x8s"
CAN_FRAME_SIZE = struct.calcsize(CAN_FRAME_FORMAT)

# ============================================================
# Telecommand Configuration
# ============================================================

TC_ID = 58

SRC_ADDRESS = 1
DST_ADDRESS = 4

# Safe initial test value
CONTROL_MODE = 0

# Seconds before ADCS exits this mode
# Should set to 0 if you want to set the control mode permanently 
CONTROL_TIMEOUT = 0

# ============================================================

def build_can_id(msg_type, tctlm_id, src_addr, dst_addr):

    return (
        ((msg_type & 0x1F) << 24) |
        ((tctlm_id & 0xFF) << 16) |
        ((src_addr & 0xFF) << 8) |
        (dst_addr & 0xFF)
    )

# ============================================================

def decode_can_id(can_id):

    return {
        "msg_type": (can_id >> 24) & 0x1F,
        "tctlm_id": (can_id >> 16) & 0xFF,
        "src_addr": (can_id >> 8) & 0xFF,
        "dst_addr": can_id & 0xFF
    }

# ============================================================

def main():

    # --------------------------------------------------------
    # Open CAN socket
    # --------------------------------------------------------

    try:

        s = socket.socket(socket.AF_CAN, socket.SOCK_RAW, socket.CAN_RAW)

        s.bind(("can0",))

        s.settimeout(5.0)

    except OSError as e:

        print("Failed to open CAN socket:", e)
        sys.exit(1)

    # --------------------------------------------------------
    # Build telecommand payload
    # --------------------------------------------------------

    payload = struct.pack(
        "<BH",
        CONTROL_MODE,
        CONTROL_TIMEOUT
    )

    dlc = len(payload)

    payload_padded = payload.ljust(8, b'\x00')

    # --------------------------------------------------------
    # Build CAN ID
    # --------------------------------------------------------

    can_id = build_can_id(
        MSG_TYPE_TC,
        TC_ID,
        SRC_ADDRESS,
        DST_ADDRESS
    )

    frame_id = can_id | CAN_EFF_FLAG

    frame = struct.pack(
        CAN_FRAME_FORMAT,
        frame_id,
        dlc,
        payload_padded
    )

    # --------------------------------------------------------
    # Print TX info
    # --------------------------------------------------------

    print("\nSending Control Mode Telecommand")
    print("--------------------------------")
    print("TC ID          :", TC_ID)
    print("Control Mode   :", CONTROL_MODE)
    print("Timeout        :", CONTROL_TIMEOUT)
    print("CAN ID         : 0x%08X" % can_id)
    print("Payload        :", payload.hex())
    print("--------------------------------\n")

    # --------------------------------------------------------
    # Send
    # --------------------------------------------------------

    try:

        s.send(frame)

    except OSError as e:

        print("Send failed:", e)
        sys.exit(1)

    # --------------------------------------------------------
    # Wait for ACK/NACK
    # --------------------------------------------------------

    print("Waiting for ACK/NACK...\n")

    start = time.time()

    while time.time() - start < 5.0:

        try:

            packet = s.recv(CAN_FRAME_SIZE)

            raw_can_id, dlc, data = struct.unpack(
                CAN_FRAME_FORMAT,
                packet
            )

            can_id = raw_can_id & CAN_EFF_MASK

            fields = decode_can_id(can_id)

            payload = data[:dlc]

            print("--------------------------------")
            print("RX CAN ID : 0x%08X" % can_id)
            print("MSG TYPE : %d" % fields["msg_type"])
            print("TCTLM ID : %d" % fields["tctlm_id"])
            print("SRC ADDR : %d" % fields["src_addr"])
            print("DST ADDR : %d" % fields["dst_addr"])
            print("DLC      : %d" % dlc)
            print("DATA     :", payload.hex())
            print("--------------------------------")

            # Verify message is from ADCS
            if (
                fields["src_addr"] != DST_ADDRESS or
                fields["dst_addr"] != SRC_ADDRESS
            ):
                continue

            # ACK
            if fields["msg_type"] == MSG_TYPE_TC_ACK:

                print("\nTELECOMMAND ACKNOWLEDGED\n")
                return

            # NACK
            elif fields["msg_type"] == MSG_TYPE_TC_NACK:

                if dlc > 0:
                    print("\nTELECOMMAND REJECTED")
                    print("Error Code:", payload[0])
                else:
                    print("\nTELECOMMAND REJECTED")

                return

        except socket.timeout:
            continue

    print("\nTIMEOUT waiting for ACK/NACK\n")

# ============================================================

if __name__ == "__main__":
    main()