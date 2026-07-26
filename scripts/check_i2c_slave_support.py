#!/usr/bin/env python3
"""Check whether Linux I2C adapters advertise target/slave support.

Run this on the OBC, not on the development machine. It checks the same
I2C_FUNCS ioctl used by i2c-tools and prints nearby sysfs/kernel clues that
help identify the bus driver.
"""

from __future__ import annotations

import argparse
import array
import fcntl
import gzip
import os
from pathlib import Path
from typing import Iterable


I2C_FUNCS = 0x0705

I2C_FUNC_I2C = 0x00000001
I2C_FUNC_10BIT_ADDR = 0x00000002
I2C_FUNC_PROTOCOL_MANGLING = 0x00000004
I2C_FUNC_SMBUS_PEC = 0x00000008
I2C_FUNC_NOSTART = 0x00000010
I2C_FUNC_SLAVE = 0x00000020
I2C_FUNC_SMBUS_BLOCK_PROC_CALL = 0x00008000
I2C_FUNC_SMBUS_QUICK = 0x00010000
I2C_FUNC_SMBUS_READ_BYTE = 0x00020000
I2C_FUNC_SMBUS_WRITE_BYTE = 0x00040000
I2C_FUNC_SMBUS_READ_BYTE_DATA = 0x00080000
I2C_FUNC_SMBUS_WRITE_BYTE_DATA = 0x00100000
I2C_FUNC_SMBUS_READ_WORD_DATA = 0x00200000
I2C_FUNC_SMBUS_WRITE_WORD_DATA = 0x00400000
I2C_FUNC_SMBUS_PROC_CALL = 0x00800000
I2C_FUNC_SMBUS_READ_BLOCK_DATA = 0x01000000
I2C_FUNC_SMBUS_WRITE_BLOCK_DATA = 0x02000000
I2C_FUNC_SMBUS_READ_I2C_BLOCK = 0x04000000
I2C_FUNC_SMBUS_WRITE_I2C_BLOCK = 0x08000000

FUNCTION_BITS = [
    ("I2C_FUNC_I2C", I2C_FUNC_I2C),
    ("I2C_FUNC_10BIT_ADDR", I2C_FUNC_10BIT_ADDR),
    ("I2C_FUNC_PROTOCOL_MANGLING", I2C_FUNC_PROTOCOL_MANGLING),
    ("I2C_FUNC_SMBUS_PEC", I2C_FUNC_SMBUS_PEC),
    ("I2C_FUNC_NOSTART", I2C_FUNC_NOSTART),
    ("I2C_FUNC_SLAVE", I2C_FUNC_SLAVE),
    ("I2C_FUNC_SMBUS_BLOCK_PROC_CALL", I2C_FUNC_SMBUS_BLOCK_PROC_CALL),
    ("I2C_FUNC_SMBUS_QUICK", I2C_FUNC_SMBUS_QUICK),
    ("I2C_FUNC_SMBUS_READ_BYTE", I2C_FUNC_SMBUS_READ_BYTE),
    ("I2C_FUNC_SMBUS_WRITE_BYTE", I2C_FUNC_SMBUS_WRITE_BYTE),
    ("I2C_FUNC_SMBUS_READ_BYTE_DATA", I2C_FUNC_SMBUS_READ_BYTE_DATA),
    ("I2C_FUNC_SMBUS_WRITE_BYTE_DATA", I2C_FUNC_SMBUS_WRITE_BYTE_DATA),
    ("I2C_FUNC_SMBUS_READ_WORD_DATA", I2C_FUNC_SMBUS_READ_WORD_DATA),
    ("I2C_FUNC_SMBUS_WRITE_WORD_DATA", I2C_FUNC_SMBUS_WRITE_WORD_DATA),
    ("I2C_FUNC_SMBUS_PROC_CALL", I2C_FUNC_SMBUS_PROC_CALL),
    ("I2C_FUNC_SMBUS_READ_BLOCK_DATA", I2C_FUNC_SMBUS_READ_BLOCK_DATA),
    ("I2C_FUNC_SMBUS_WRITE_BLOCK_DATA", I2C_FUNC_SMBUS_WRITE_BLOCK_DATA),
    ("I2C_FUNC_SMBUS_READ_I2C_BLOCK", I2C_FUNC_SMBUS_READ_I2C_BLOCK),
    ("I2C_FUNC_SMBUS_WRITE_I2C_BLOCK", I2C_FUNC_SMBUS_WRITE_I2C_BLOCK),
]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Check Linux I2C adapter functionality flags."
    )
    parser.add_argument(
        "devices",
        nargs="*",
        help="I2C device paths to check, for example /dev/i2c-1. Defaults to all /dev/i2c-*.",
    )
    parser.add_argument(
        "--all-functions",
        action="store_true",
        help="Print every decoded I2C/SMBus capability bit.",
    )
    return parser.parse_args()


def sorted_i2c_devices() -> list[Path]:
    def key(path: Path) -> tuple[int, str]:
        try:
            return (int(path.name.rsplit("-", 1)[1]), path.name)
        except (IndexError, ValueError):
            return (10_000, path.name)

    return sorted(Path("/dev").glob("i2c-*"), key=key)


def read_text(path: Path) -> str | None:
    try:
        return path.read_text(encoding="utf-8").strip()
    except OSError:
        return None


def read_link(path: Path) -> str | None:
    try:
        return str(path.resolve())
    except OSError:
        return None


def bus_number(device: Path) -> str | None:
    if not device.name.startswith("i2c-"):
        return None
    return device.name[len("i2c-") :]


def adapter_sysfs_path(device: Path) -> Path | None:
    bus = bus_number(device)
    if bus is None:
        return None
    path = Path("/sys/class/i2c-dev") / f"i2c-{bus}" / "device"
    return path if path.exists() else None


def get_funcs(device: Path) -> int:
    funcs = array.array("L", [0])
    with device.open("rb", buffering=0) as handle:
        fcntl.ioctl(handle.fileno(), I2C_FUNCS, funcs, True)
    return int(funcs[0])


def kernel_config_value(name: str) -> str | None:
    candidates = [
        Path("/proc/config.gz"),
        Path("/boot") / f"config-{os.uname().release}",
    ]

    for path in candidates:
        if not path.exists():
            continue

        try:
            if path.suffix == ".gz":
                with gzip.open(path, "rt", encoding="utf-8", errors="replace") as file:
                    lines = file.readlines()
            else:
                lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
        except OSError:
            continue

        prefix = f"{name}="
        for line in lines:
            if line.startswith(prefix):
                return line.split("=", 1)[1]
            if line == f"# {name} is not set":
                return "not set"

    return None


def slave_backend_modules() -> list[Path]:
    modules = Path("/lib/modules") / os.uname().release
    if not modules.exists():
        return []
    return sorted(path for path in modules.rglob("*slave*") if "i2c" in path.name.lower())


def print_adapter_info(device: Path) -> None:
    sysfs = adapter_sysfs_path(device)
    print(f"device: {device}")
    if sysfs is None:
        print("  sysfs: not found")
        return

    name = read_text(sysfs / "name")
    driver = read_link(sysfs / "driver")
    resolved_device = read_link(sysfs)

    print(f"  adapter name: {name or 'unknown'}")
    print(f"  sysfs device: {resolved_device or 'unknown'}")
    print(f"  driver: {driver or 'unknown'}")


def print_functions(funcs: int, all_functions: bool) -> None:
    print(f"  funcs: 0x{funcs:x}")
    print(f"  I2C_FUNC_I2C: {'yes' if funcs & I2C_FUNC_I2C else 'no'}")
    print(f"  I2C_FUNC_SLAVE: {'yes' if funcs & I2C_FUNC_SLAVE else 'no'}")

    if all_functions:
        print("  decoded functions:")
        for name, bit in FUNCTION_BITS:
            print(f"    {name}: {'yes' if funcs & bit else 'no'}")


def check_devices(devices: Iterable[Path], all_functions: bool) -> int:
    checked = 0
    slave_capable = []

    for device in devices:
        checked += 1
        print_adapter_info(device)

        try:
            funcs = get_funcs(device)
        except PermissionError as error:
            print(f"  error: permission denied: {error}")
        except OSError as error:
            print(f"  error: cannot query I2C_FUNCS: {error}")
        else:
            print_functions(funcs, all_functions)
            if funcs & I2C_FUNC_SLAVE:
                slave_capable.append(str(device))

        print()

    if checked == 0:
        print("No I2C devices found. Expected paths like /dev/i2c-1.")
        return 2

    if slave_capable:
        print("Adapters advertising I2C target/slave support:")
        for device in slave_capable:
            print(f"  {device}")
    else:
        print("No checked adapter advertised I2C_FUNC_SLAVE.")

    return 0


def main() -> int:
    args = parse_args()
    devices = [Path(device) for device in args.devices] if args.devices else sorted_i2c_devices()

    print(f"kernel: {os.uname().sysname} {os.uname().release} {os.uname().machine}")
    config_value = kernel_config_value("CONFIG_I2C_SLAVE")
    print(f"CONFIG_I2C_SLAVE: {config_value or 'unknown'}")

    backends = slave_backend_modules()
    if backends:
        print("I2C slave backend module candidates:")
        for path in backends:
            print(f"  {path}")
    else:
        print("I2C slave backend module candidates: none found")

    print()
    return check_devices(devices, args.all_functions)


if __name__ == "__main__":
    raise SystemExit(main())
