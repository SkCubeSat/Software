#!/usr/bin/env python3
"""Sweep 7-bit I2C addresses using write transfers.

Run this on the Linux target with /dev/i2c-* access. Prefer ``--mode zero``
first: it attempts a raw zero-length I2C write, which is the closest thing to
an address-only write probe without SMBus Quick. If the adapter cannot issue a
zero-length transfer, the byte modes send a real byte and can change device
state.
"""

from __future__ import annotations

import argparse
import array
import ctypes
import errno
import fcntl
import os
from dataclasses import dataclass
from pathlib import Path


I2C_SLAVE = 0x0703
I2C_FUNCS = 0x0705
I2C_RDWR = 0x0707
I2C_SMBUS = 0x0720

I2C_FUNC_I2C = 0x00000001
I2C_FUNC_SMBUS_QUICK = 0x00010000
I2C_FUNC_SMBUS_WRITE_BYTE = 0x00040000

I2C_SMBUS_WRITE = 0
I2C_SMBUS_BYTE = 1

SAFE_FIRST_ADDR = 0x08
RESERVED_SCAN_FIRST_ADDR = 0x03
LAST_7BIT_ADDR = 0x77

NACK_ERRNOS = {
    errno.ENXIO,
    errno.EREMOTEIO,
    errno.EIO,
}
UNSUPPORTED_ERRNOS = {
    errno.EINVAL,
    errno.ENOTTY,
    getattr(errno, "EOPNOTSUPP", 95),
}


class I2CMsg(ctypes.Structure):
    _fields_ = [
        ("addr", ctypes.c_uint16),
        ("flags", ctypes.c_uint16),
        ("len", ctypes.c_uint16),
        ("buf", ctypes.POINTER(ctypes.c_uint8)),
    ]


class I2CRdwrIoctlData(ctypes.Structure):
    _fields_ = [
        ("msgs", ctypes.POINTER(I2CMsg)),
        ("nmsgs", ctypes.c_uint32),
    ]


class I2CSmbusData(ctypes.Union):
    _fields_ = [
        ("byte", ctypes.c_uint8),
        ("word", ctypes.c_uint16),
        ("block", ctypes.c_uint8 * 34),
    ]


class I2CSmbusIoctlData(ctypes.Structure):
    _fields_ = [
        ("read_write", ctypes.c_uint8),
        ("command", ctypes.c_uint8),
        ("size", ctypes.c_int),
        ("data", ctypes.POINTER(I2CSmbusData)),
    ]


@dataclass(frozen=True)
class ProbeResult:
    address: int
    status: str
    error: OSError | None = None


def parse_int(value: str) -> int:
    try:
        return int(value, 0)
    except ValueError as error:
        raise argparse.ArgumentTypeError(f"invalid integer: {value}") from error


def parse_address(value: str) -> int:
    address = parse_int(value)
    if not 0 <= address <= 0x7F:
        raise argparse.ArgumentTypeError("address must be a 7-bit value, 0x00-0x7f")
    return address


def parse_byte(value: str) -> int:
    byte = parse_int(value)
    if not 0 <= byte <= 0xFF:
        raise argparse.ArgumentTypeError("byte must be in range 0x00-0xff")
    return byte


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Sweep 7-bit I2C addresses by issuing write transfers.",
        epilog=(
            "Examples:\n"
            "  sudo scripts/i2c_write_sweep.py /dev/i2c-1 --mode zero\n"
            "  sudo scripts/i2c_write_sweep.py 1 --mode byte --byte 0x00 "
            "--i-understand-this-can-write"
        ),
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("device", help="/dev/i2c-N path, or just the bus number N")
    parser.add_argument(
        "--mode",
        choices=("zero", "byte", "smbus-byte"),
        default="zero",
        help=(
            "zero: raw zero-length I2C write; byte: raw one-byte I2C write; "
            "smbus-byte: SMBus write byte"
        ),
    )
    parser.add_argument(
        "--byte",
        type=parse_byte,
        default=0x00,
        help="byte to send in byte/smbus-byte modes, default: 0x00",
    )
    parser.add_argument(
        "--first",
        type=parse_address,
        default=None,
        help="first 7-bit address to scan, default: 0x08 or 0x03 with --include-reserved",
    )
    parser.add_argument(
        "--last",
        type=parse_address,
        default=None,
        help="last 7-bit address to scan, default: 0x77",
    )
    parser.add_argument(
        "--include-reserved",
        action="store_true",
        help="scan from 0x03 by default instead of skipping reserved 0x00-0x07 addresses",
    )
    parser.add_argument(
        "--i-understand-this-can-write",
        action="store_true",
        help="required for byte and smbus-byte modes because they send a real byte",
    )
    parser.add_argument(
        "--verbose",
        action="store_true",
        help="print the errno from each failed probe after the table",
    )

    args = parser.parse_args()
    if args.mode in {"byte", "smbus-byte"} and not args.i_understand_this_can_write:
        parser.error(
            f"--mode {args.mode} sends byte 0x{args.byte:02x} to every scanned address; "
            "add --i-understand-this-can-write to proceed"
        )
    return args


def device_path(value: str) -> Path:
    if value.isdecimal():
        return Path(f"/dev/i2c-{value}")
    return Path(value)


def address_bounds(args: argparse.Namespace) -> tuple[int, int]:
    first = args.first
    if first is None:
        first = RESERVED_SCAN_FIRST_ADDR if args.include_reserved else SAFE_FIRST_ADDR

    last = args.last if args.last is not None else LAST_7BIT_ADDR
    if first > last:
        raise ValueError(f"first address 0x{first:02x} is after last address 0x{last:02x}")
    return first, last


def get_funcs(fd: int) -> int:
    funcs = array.array("L", [0])
    fcntl.ioctl(fd, I2C_FUNCS, funcs, True)
    return int(funcs[0])


def raw_i2c_write(fd: int, address: int, payload: bytes) -> None:
    if payload:
        buffer = (ctypes.c_uint8 * len(payload))(*payload)
        buffer_pointer = ctypes.cast(buffer, ctypes.POINTER(ctypes.c_uint8))
    else:
        buffer = None
        buffer_pointer = ctypes.POINTER(ctypes.c_uint8)()

    message = I2CMsg(
        addr=address,
        flags=0,
        len=len(payload),
        buf=buffer_pointer,
    )
    messages = (I2CMsg * 1)(message)
    ioctl_data = I2CRdwrIoctlData(msgs=messages, nmsgs=1)

    # Keep the payload buffer in scope until the ioctl returns.
    _ = buffer
    fcntl.ioctl(fd, I2C_RDWR, ioctl_data)


def smbus_write_byte(fd: int, address: int, value: int) -> None:
    fcntl.ioctl(fd, I2C_SLAVE, address)
    ioctl_data = I2CSmbusIoctlData(
        read_write=I2C_SMBUS_WRITE,
        command=value,
        size=I2C_SMBUS_BYTE,
        data=ctypes.POINTER(I2CSmbusData)(),
    )
    fcntl.ioctl(fd, I2C_SMBUS, ioctl_data)


def classify_error(error: OSError) -> str:
    if error.errno == errno.EBUSY:
        return "busy"
    if error.errno in UNSUPPORTED_ERRNOS:
        return "unsupported"
    if error.errno in NACK_ERRNOS:
        return "nack"
    return "error"


def probe(fd: int, mode: str, address: int, byte: int) -> ProbeResult:
    try:
        if mode == "zero":
            raw_i2c_write(fd, address, b"")
        elif mode == "byte":
            raw_i2c_write(fd, address, bytes([byte]))
        elif mode == "smbus-byte":
            smbus_write_byte(fd, address, byte)
        else:
            raise AssertionError(f"unhandled mode: {mode}")
    except OSError as error:
        return ProbeResult(address=address, status=classify_error(error), error=error)

    return ProbeResult(address=address, status="ack")


def table_cell(result: ProbeResult | None) -> str:
    if result is None:
        return "  "
    if result.status == "ack":
        return f"{result.address:02x}"
    if result.status == "busy":
        return "UU"
    if result.status == "unsupported":
        return "!!"
    if result.status == "error":
        return "??"
    return "--"


def print_table(results: dict[int, ProbeResult]) -> None:
    print("     " + " ".join(f"{address:x}" for address in range(16)))
    for row in range(0x00, 0x80, 0x10):
        cells = []
        for column in range(16):
            address = row + column
            cells.append(table_cell(results.get(address)))
        print(f"{row:02x}:  " + " ".join(cells))


def errno_name(error_number: int | None) -> str:
    if error_number is None:
        return "unknown"
    return errno.errorcode.get(error_number, str(error_number))


def print_capability_notes(mode: str, funcs: int) -> None:
    print(f"adapter funcs: 0x{funcs:x}")
    print(f"  I2C_FUNC_I2C: {'yes' if funcs & I2C_FUNC_I2C else 'no'}")
    print(f"  I2C_FUNC_SMBUS_QUICK: {'yes' if funcs & I2C_FUNC_SMBUS_QUICK else 'no'}")
    print(
        "  I2C_FUNC_SMBUS_WRITE_BYTE: "
        f"{'yes' if funcs & I2C_FUNC_SMBUS_WRITE_BYTE else 'no'}"
    )

    if mode in {"zero", "byte"} and not funcs & I2C_FUNC_I2C:
        print("warning: adapter does not advertise I2C_FUNC_I2C; raw I2C_RDWR may fail")
    if mode == "smbus-byte" and not funcs & I2C_FUNC_SMBUS_WRITE_BYTE:
        print("warning: adapter does not advertise I2C_FUNC_SMBUS_WRITE_BYTE")


def print_summary(results: dict[int, ProbeResult], verbose: bool) -> None:
    acked = [address for address, result in results.items() if result.status == "ack"]
    busy = [address for address, result in results.items() if result.status == "busy"]
    unsupported = [
        address for address, result in results.items() if result.status == "unsupported"
    ]
    other_errors = [
        result for result in results.values() if result.status not in {"ack", "nack", "busy"}
    ]

    print()
    if acked:
        print("ACKed write transfer at: " + " ".join(f"0x{address:02x}" for address in acked))
    else:
        print("No scanned address completed this write transfer.")

    if busy:
        print("Kernel driver already owns: " + " ".join(f"0x{address:02x}" for address in busy))
    if unsupported:
        print(
            "!! means the adapter rejected this transfer type. "
            "For --mode zero, that usually means zero-length writes are unsupported."
        )

    if verbose:
        print()
        print("failed probes:")
        for result in results.values():
            if result.error is None:
                continue
            name = errno_name(result.error.errno)
            print(
                f"  0x{result.address:02x}: {result.status} "
                f"{name} ({result.error.strerror})"
            )
    elif other_errors:
        print("Use --verbose to show the errno behind ??/!! cells.")


def main() -> int:
    args = parse_args()
    path = device_path(args.device)

    try:
        first, last = address_bounds(args)
    except ValueError as error:
        print(f"error: {error}", file=os.sys.stderr)
        return 2

    if args.mode in {"byte", "smbus-byte"}:
        print(
            f"warning: --mode {args.mode} sends real byte 0x{args.byte:02x}; "
            "this can change device state"
        )

    try:
        fd = os.open(path, os.O_RDWR)
    except OSError as error:
        print(f"error: cannot open {path}: {error}", file=os.sys.stderr)
        return 1

    try:
        funcs = get_funcs(fd)
        print(f"device: {path}")
        print(f"mode: {args.mode}")
        print(f"range: 0x{first:02x}-0x{last:02x}")
        print_capability_notes(args.mode, funcs)
        print()

        results = {
            address: probe(fd, args.mode, address, args.byte)
            for address in range(first, last + 1)
        }
    except OSError as error:
        print(f"error: ioctl failed on {path}: {error}", file=os.sys.stderr)
        return 1
    finally:
        os.close(fd)

    print_table(results)
    print_summary(results, args.verbose)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
