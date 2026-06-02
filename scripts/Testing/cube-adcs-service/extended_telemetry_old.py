#!/usr/bin/env python3

# -----------------------------------------------------------------------------#
# Script to test Cube ADCS connected to OBC, without service usage. 
# Script sends specified tctlm command and reads response from ADCS.
# Script will read several frames until the expected length is received.
# -----------------------------------------------------------------------------#

import socket
import struct
import time
import sys

# ============================================================
# CubeSpace Message Types
# ============================================================

MSG_TYPE_TLM_REQ          = 4
MSG_TYPE_TLM_RESP_EXT     = 8

# ============================================================

# Modify below values based on command you want to send.

# TCTLM_ID : Type of telemetry requested
# EXPECTED_LENGTH : Expected length of telemetry in bytes
TCTLM_ID = 170
EXPECTED_LENGTH = 33


SRC_ADDRESS = 1
DST_ADDRESS = 4

CAN_EFF_FLAG = 0x80000000
CAN_EFF_MASK = 0x1FFFFFFF

CAN_FRAME_FORMAT = "=IB3x8s"
CAN_FRAME_SIZE = struct.calcsize(CAN_FRAME_FORMAT)

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
# Decode Raw CubeSense Sun Telemetry
# ============================================================

def decode_tlm_170(data):

    print("\n=========== TLM 170 ===========")

    if len(data) != 33:

        print("Unexpected telemetry length:", len(data))
        return

    # --------------------------------------------------------
    # Timestamp
    # --------------------------------------------------------

    sec, nsec = struct.unpack_from("<II", data, 0)

    print("Unix Seconds :", sec)
    print("Nanoseconds  :", nsec)

    # --------------------------------------------------------
    # FSS Data
    # --------------------------------------------------------

    offset = 8

    for i in range(4):

        alpha, beta = struct.unpack_from("<hh", data, offset)

        capture = data[offset + 4]
        detect  = data[offset + 5]

        alpha_deg = alpha * 0.01
        beta_deg  = beta * 0.01

        print("")
        print("FSS%d" % i)
        print("-------------------")
        print("Alpha Angle :", alpha_deg)
        print("Beta Angle  :", beta_deg)
        print("Capture     :", capture)
        print("Detection   :", detect)

        offset += 6

    # --------------------------------------------------------
    # Valid Flags
    # --------------------------------------------------------

    flags = data[32]

    print("")
    print("Validity Flags")
    print("-------------------")
    print("FSS0 :", bool(flags & (1 << 0)))
    print("FSS1 :", bool(flags & (1 << 1)))
    print("FSS2 :", bool(flags & (1 << 2)))
    print("FSS3 :", bool(flags & (1 << 3)))

    print("================================\n")

# ============================================================

def main():

    # --------------------------------------------------------
    # Open CAN
    # --------------------------------------------------------

    s = socket.socket(socket.AF_CAN, socket.SOCK_RAW, socket.CAN_RAW)

    s.bind(("can0",))

    s.settimeout(2.0)

    # --------------------------------------------------------
    # Send telemetry request
    # --------------------------------------------------------

    can_id = build_can_id(
        MSG_TYPE_TLM_REQ,
        TCTLM_ID,
        SRC_ADDRESS,
        DST_ADDRESS
    )

    frame_id = can_id | CAN_EFF_FLAG

    frame = struct.pack(
        CAN_FRAME_FORMAT,
        frame_id,
        0,
        b'\x00' * 8
    )

    print("\nRequesting telemetry 170...\n")

    s.send(frame)

    # --------------------------------------------------------
    # Reassemble telemetry
    # --------------------------------------------------------

    full_payload = bytearray()

    start = time.time()

    while len(full_payload) < EXPECTED_LENGTH:

        if time.time() - start > 5.0:

            print("Timeout waiting for telemetry")
            sys.exit(1)

        try:

            packet = s.recv(CAN_FRAME_SIZE)

            raw_can_id, dlc, data = struct.unpack(
                CAN_FRAME_FORMAT,
                packet
            )

            can_id = raw_can_id & CAN_EFF_MASK

            fields = decode_can_id(can_id)

            payload = data[:dlc]

            # ------------------------------------------------
            # Filter frames
            # ------------------------------------------------

            if (
                fields["msg_type"] != MSG_TYPE_TLM_RESP_EXT or
                fields["tctlm_id"] != TCTLM_ID or
                fields["src_addr"] != DST_ADDRESS or
                fields["dst_addr"] != SRC_ADDRESS
            ):
                continue

            # ------------------------------------------------
            # Append data
            # ------------------------------------------------

            full_payload.extend(payload)

            print(
                "Received frame: %d bytes "
                "(total %d/%d)"
                % (
                    dlc,
                    len(full_payload),
                    EXPECTED_LENGTH
                )
            )

        except socket.timeout:
            continue

    # --------------------------------------------------------
    # Trim excess
    # --------------------------------------------------------

    full_payload = full_payload[:EXPECTED_LENGTH]

    print("\nFULL TELEMETRY:")
    print(full_payload.hex())

    decode_tlm_170(full_payload)

# ============================================================

if __name__ == "__main__":
    main()