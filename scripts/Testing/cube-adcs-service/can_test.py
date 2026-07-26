#!/usr/bin/env python3

# -----------------------------------------------------------------------------#
# Script to test Cube ADCS connected to OBC, without service usage. 
# Script sends specified tctlm command and reads response from ADCS.
# Script will only read and output one frame from the ADCS. This is used for a high level check that the ADCS is alive. 
# -----------------------------------------------------------------------------#

import socket
import struct
import time
import sys

# ============================================================
# CubeSpace CAN Protocol Constants
# ============================================================

MSG_TYPE_TC            = 1
MSG_TYPE_TC_ACK        = 2
MSG_TYPE_TC_NACK       = 3
MSG_TYPE_TLM_REQ       = 4
MSG_TYPE_TLM_RESP      = 5
MSG_TYPE_TLM_NACK      = 6

# Test telemetry request
TCTLM_ID = 128

# Linux SocketCAN
CAN_EFF_FLAG = 0x80000000
CAN_EFF_MASK = 0x1FFFFFFF

CAN_FRAME_FORMAT = "=IB3x8s"
CAN_FRAME_SIZE = struct.calcsize(CAN_FRAME_FORMAT)

# ============================================================
# Build CubeSpace 29-bit CAN ID
# ============================================================

def build_can_id(msg_type, tctlm_id, src_addr, dst_addr):

    return (
        ((msg_type & 0x1F) << 24) |
        ((tctlm_id & 0xFF) << 16) |
        ((src_addr & 0xFF) << 8) |
        (dst_addr & 0xFF)
    )

# ============================================================
# Decode CubeSpace CAN ID
# ============================================================

def decode_can_id(can_id):

    return {
        "msg_type": (can_id >> 24) & 0x1F,
        "tctlm_id": (can_id >> 16) & 0xFF,
        "src_addr": (can_id >> 8) & 0xFF,
        "dst_addr": can_id & 0xFF
    }

# ============================================================
# Main
# ============================================================

def main():

    src_address = 1
    dst_address = 4

    # --------------------------------------------------------
    # Open CAN socket
    # --------------------------------------------------------

    try:
        s = socket.socket(socket.AF_CAN, socket.SOCK_RAW, socket.CAN_RAW)
        s.bind(("can0",))
        s.settimeout(5.0)

    except OSError as e:

        print("Failed to open can0:", e)
        print("")
        print("Bring CAN up with:")
        print("ip link set can0 up type can bitrate 1000000")
        sys.exit(1)

    # --------------------------------------------------------
    # Build telemetry request
    # --------------------------------------------------------

    can_id = build_can_id(
        MSG_TYPE_TLM_REQ,
        TCTLM_ID,
        src_address,
        dst_address
    )

    frame_id = can_id | CAN_EFF_FLAG

    frame = struct.pack(
        CAN_FRAME_FORMAT,
        frame_id,
        0,
        b'\x00' * 8
    )

    print("")
    print("Sending telemetry request...")
    print("CAN ID: 0x%08X" % can_id)
    print("")

    # --------------------------------------------------------
    # Send frame
    # --------------------------------------------------------

    try:
        s.send(frame)

    except OSError as e:

        print("CAN send failed:", e)
        sys.exit(1)

    # --------------------------------------------------------
    # Wait for response
    # --------------------------------------------------------

    print("Waiting for response...\n")

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

            # Check for response from ADCS
            if (
                fields["src_addr"] == dst_address and
                fields["dst_addr"] == src_address
            ):

                print("\nVALID RESPONSE FROM ADCS RECEIVED\n")
                return

        except socket.timeout:
            continue

        except OSError as e:

            print("CAN receive error:", e)
            sys.exit(1)

    print("\nTIMEOUT: No response received\n")

if __name__ == "__main__":
    main()