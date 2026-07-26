#!/usr/bin/env python3
import argparse
import sys
import time
import zlib
import serial


def crc32_bytes(data):
    return zlib.crc32(data) & 0xFFFFFFFF


def write_line(ser, line):
    print(f"send: {line}")
    ser.write((line + "\n").encode("ascii"))
    ser.flush()


def read_until_match(ser, predicates, timeout_s=10.0, print_prefix="recv"):
    deadline = time.monotonic() + timeout_s

    while time.monotonic() < deadline:
        remaining = max(0.1, deadline - time.monotonic())
        old_timeout = ser.timeout
        ser.timeout = min(1.0, remaining)
        try:
            raw = ser.readline()
        finally:
            ser.timeout = old_timeout

        if not raw:
            continue

        line = raw.decode(errors="replace").strip()
        if not line:
            continue

        print(f"{print_prefix}: {line}")

        for name, pred in predicates:
            if pred(line):
                return name, line

    raise TimeoutError("timed out waiting for expected response")


def parse_result_info(line, frame_id):
    # RESULT_INFO 42 READY 12288 A1B2C3D4
    parts = line.split()
    if len(parts) != 5 or parts[0] != "RESULT_INFO":
        raise ValueError(f"bad RESULT_INFO format: {line}")

    got_id = int(parts[1])
    if got_id != frame_id:
        raise ValueError(f"wrong frame id in RESULT_INFO: got {got_id}, expected {frame_id}")

    status = parts[2]
    size = int(parts[3])
    crc = int(parts[4], 16)
    return status, size, crc


def parse_result_ready(line, frame_id):
    # RESULT_READY 42 12288 A1B2C3D4
    parts = line.split()
    if len(parts) != 4 or parts[0] != "RESULT_READY":
        raise ValueError(f"bad RESULT_READY format: {line}")

    got_id = int(parts[1])
    if got_id != frame_id:
        raise ValueError(f"wrong frame id in RESULT_READY: got {got_id}, expected {frame_id}")

    size = int(parts[2])
    crc = int(parts[3], 16)
    return size, crc


def receive_exact(ser, size, timeout_s=20.0):
    old_timeout = ser.timeout
    ser.timeout = timeout_s
    try:
        data = ser.read(size)
    finally:
        ser.timeout = old_timeout

    if len(data) != size:
        raise TimeoutError(f"expected {size} result bytes, got {len(data)}")

    return data


def main():
    parser = argparse.ArgumentParser(
        description="Test TE0724 payload UART full JPEG receive + dummy SNN result chain"
    )
    parser.add_argument("frame_id", type=int)
    parser.add_argument("image")
    parser.add_argument("--port", default="/dev/ttyUSB1")
    parser.add_argument("--baud", type=int, default=115200)
    parser.add_argument("--timeout", type=float, default=10.0)
    parser.add_argument("--chunk-size", type=int, default=4096)
    parser.add_argument("--save-result", default=None,
                        help="Optional path to save returned result mask/bin")
    parser.add_argument("--skip-status", action="store_true",
                        help="Skip STATUS/IDLE handshake and send SEND immediately")
    parser.add_argument("--no-result", action="store_true",
                        help="Only send image and wait for RESULT_READY; do not request result bytes")
    args = parser.parse_args()

    with open(args.image, "rb") as f:
        image_data = f.read()

    image_size = len(image_data)
    image_crc = crc32_bytes(image_data)

    print(f"image: {args.image}")
    print(f"frame_id: {args.frame_id}")
    print(f"size: {image_size}")
    print(f"crc32: {image_crc:08X}")
    print(f"port: {args.port}")
    print(f"baud: {args.baud}")

    with serial.Serial(args.port, args.baud, timeout=1) as ser:
        time.sleep(0.3)
        ser.reset_input_buffer()
        ser.reset_output_buffer()

        # The daemon may have sent PAYLOAD_READY before this script opened the port,
        # so actively test liveness instead of depending only on the startup banner.
        write_line(ser, "PING")
        name, line = read_until_match(
            ser,
            [
                ("PONG", lambda line: line == "PONG"),
                ("PAYLOAD_READY", lambda line: line == "PAYLOAD_READY"),
            ],
            timeout_s=args.timeout,
        )

        # If PAYLOAD_READY was consumed first, ask again for PONG.
        if name != "PONG":
            write_line(ser, "PING")
            read_until_match(
                ser,
                [("PONG", lambda line: line == "PONG")],
                timeout_s=args.timeout,
            )

        if not args.skip_status:
            write_line(ser, "STATUS")
            read_until_match(
                ser,
                [("IDLE", lambda line: line.startswith("IDLE"))],
                timeout_s=args.timeout,
            )

        write_line(ser, f"SEND {args.frame_id} {image_size} {image_crc:08X}")

        read_until_match(
            ser,
            [("READY", lambda line: line == "READY")],
            timeout_s=args.timeout,
        )

        print(f"send raw image bytes: {image_size}")
        for off in range(0, image_size, args.chunk_size):
            ser.write(image_data[off:off + args.chunk_size])
        ser.flush()

        read_until_match(
            ser,
            [
                ("RX_OK", lambda line: line == f"RX_OK {args.frame_id}"),
                ("RX_FAIL", lambda line: line.startswith(f"RX_FAIL {args.frame_id}")),
            ],
            timeout_s=args.timeout,
        )

        # New structured daemon sends PROCESSING and then RESULT_READY.
        name, line = read_until_match(
            ser,
            [
                ("PROCESSING", lambda line: line == f"PROCESSING {args.frame_id}"),
                ("RESULT_READY", lambda line: line == f"RESULT_READY {args.frame_id}"),
                ("PROCESS_FAIL", lambda line: line.startswith(f"PROCESS_FAIL {args.frame_id}")),
            ],
            timeout_s=args.timeout,
        )

        if name == "PROCESSING":
            read_until_match(
                ser,
                [
                    ("RESULT_READY", lambda line: line == f"RESULT_READY {args.frame_id}"),
                    ("PROCESS_FAIL", lambda line: line.startswith(f"PROCESS_FAIL {args.frame_id}")),
                ],
                timeout_s=args.timeout,
            )
        elif name == "PROCESS_FAIL":
            raise RuntimeError(line)

        if args.no_result:
            print("image receive + processing test complete")
            return

        write_line(ser, f"GET_RESULT_INFO {args.frame_id}")
        _, info_line = read_until_match(
            ser,
            [("RESULT_INFO", lambda line: line.startswith(f"RESULT_INFO {args.frame_id} "))],
            timeout_s=args.timeout,
        )

        status, result_size, result_crc = parse_result_info(info_line, args.frame_id)
        if status != "READY":
            raise RuntimeError(f"result not ready: {info_line}")

        print(f"result info: size={result_size} crc32={result_crc:08X}")

        write_line(ser, f"GET_RESULT {args.frame_id}")
        _, ready_line = read_until_match(
            ser,
            [("RESULT_READY_WITH_SIZE", lambda line: line.startswith(f"RESULT_READY {args.frame_id} "))],
            timeout_s=args.timeout,
        )

        tx_size, tx_crc = parse_result_ready(ready_line, args.frame_id)
        if tx_size != result_size or tx_crc != result_crc:
            raise RuntimeError(
                f"result header mismatch: info=({result_size},{result_crc:08X}) "
                f"ready=({tx_size},{tx_crc:08X})"
            )

        write_line(ser, "READY")

        print(f"receive raw result bytes: {result_size}")
        result_data = receive_exact(ser, result_size, timeout_s=args.timeout)
        actual_result_crc = crc32_bytes(result_data)

        print(f"received result crc32: {actual_result_crc:08X}")
        if actual_result_crc != result_crc:
            write_line(ser, f"RESULT_RX_FAIL {args.frame_id} CRC")
            raise RuntimeError(
                f"result CRC mismatch: expected {result_crc:08X}, got {actual_result_crc:08X}"
            )

        if args.save_result:
            with open(args.save_result, "wb") as f:
                f.write(result_data)
            print(f"saved result: {args.save_result}")

        write_line(ser, f"RESULT_RX_OK {args.frame_id}")

        # Optional final state line from daemon.
        try:
            read_until_match(
                ser,
                [("IDLE", lambda line: line.startswith("IDLE"))],
                timeout_s=2.0,
            )
        except TimeoutError:
            pass

        print("full chain OK")


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"ERROR: {e}", file=sys.stderr)
        sys.exit(1)