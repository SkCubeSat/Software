#!/usr/bin/env python3

import argparse
import kubos_app as app_api
import sys


# ------------------------------------------------------------
# Setup Services
# ------------------------------------------------------------
def setup_services(config=None):
    if config:
        return app_api.Services(config)
    return app_api.Services()


# ------------------------------------------------------------
# Query Helper
# ------------------------------------------------------------
def send_query(services, name, query):
    print(f"\n--- {name} ---")
    try:
        response = services.query(service="clyde-3g-eps-service", query=query)
        print(response)
        return response
    except Exception as e:
        print(f"ERROR during {name}: {e}")
        sys.exit(1)


# ------------------------------------------------------------
# Core Commands
# ------------------------------------------------------------
def set_watchdog(services, period):
    query = f"""
    mutation {{
        setWatchdogPeriod(period: {period}) {{
            success
            errors
        }}
    }}
    """
    send_query(services, "Set Watchdog", query)


def hardware_test(services):
    query = """
    mutation {
        testHardware(test: HARDWARE) {
            success
            errors
        }
    }
    """
    send_query(services, "Hardware Test", query)


def issue_raw(services, command, data):
    data_str = ",".join(str(x) for x in data)

    query = f"""
    mutation {{
        issueRawCommand(command: {command}, data: [{data_str}]) {{
            success
            errors
        }}
    }}
    """
    send_query(services, "Raw Command", query)


# ------------------------------------------------------------
# PDM CONTROL
# ------------------------------------------------------------
def pdm_on(services, ch):
    issue_raw(services, 0x50, [ch])


def pdm_off(services, ch):
    issue_raw(services, 0x51, [ch])

def get_pdm_state(services):
    issue_raw(services, 0x45, [0])

def pdm_all_on(services):
    issue_raw(services, 0x40, [0])


def pdm_all_off(services):
    issue_raw(services, 0x41, [0])


def pdm_set_initial_on(services, ch):
    issue_raw(services, 0x52, [ch])


def pdm_set_initial_off(services, ch):
    issue_raw(services, 0x53, [ch])


def pdm_set_timer(services, ch, limit):
    query = f"""
    mutation {{
        issueRawCommand(command: 96, data: [{ch}, {limit}]) {{
            success
            errors
        }}
    }}
    """
    send_query(services, "Set PDM Timer", query)


# ------------------------------------------------------------
# TELEMETRY
# The MotherboardTelemetry type exposes a single resolver:
#   value(telemetryType: MotherboardTelemetryType): Float
# Use GraphQL aliases to fetch multiple values in one query.
# ------------------------------------------------------------
def telemetry_query(services, group):

    if group == "voltage":
        query = """
        {
            telemetry {
                motherboard {
                    VoltageFeedingBcr1:   value(telemetryType: VOLTAGE_FEEDING_BCR_1)
                    VoltageFeedingBcr2:   value(telemetryType: VOLTAGE_FEEDING_BCR_2)
                    VoltageFeedingBcr3:   value(telemetryType: VOLTAGE_FEEDING_BCR_3)
                    BcrOutputVoltage:     value(telemetryType: BCR_OUTPUT_VOLTAGE)
                    OutputVoltage12V:     value(telemetryType: OUTPUT_VOLTAGE_12V)
                    OutputVoltageBattery: value(telemetryType: OUTPUT_VOLTAGE_BATTERY)
                    OutputVoltage5V:      value(telemetryType: OUTPUT_VOLTAGE_5V)
                    OutputVoltage33V:     value(telemetryType: OUTPUT_VOLTAGE_33V)
                    OutputVoltageSwitch1: value(telemetryType: OUTPUT_VOLTAGE_SWITCH_1)
                    OutputVoltageSwitch2: value(telemetryType: OUTPUT_VOLTAGE_SWITCH_2)
                    OutputVoltageSwitch3: value(telemetryType: OUTPUT_VOLTAGE_SWITCH_3)
                    OutputVoltageSwitch4: value(telemetryType: OUTPUT_VOLTAGE_SWITCH_4)
                    OutputVoltageSwitch5: value(telemetryType: OUTPUT_VOLTAGE_SWITCH_5)
                    OutputVoltageSwitch6: value(telemetryType: OUTPUT_VOLTAGE_SWITCH_6)
                    OutputVoltageSwitch7: value(telemetryType: OUTPUT_VOLTAGE_SWITCH_7)
                    OutputVoltageSwitch8: value(telemetryType: OUTPUT_VOLTAGE_SWITCH_8)
                    OutputVoltageSwitch9: value(telemetryType: OUTPUT_VOLTAGE_SWITCH_9)
                    OutputVoltageSwitch10: value(telemetryType: OUTPUT_VOLTAGE_SWITCH_10)
                }
            }
        }
        """

    elif group == "current":
        query = """
        {
            telemetry {
                motherboard {
                    CurrentBcr1Sa1a:      value(telemetryType: CURRENT_BCR_1_SA_1A)
                    CurrentBcr1Sa1b:      value(telemetryType: CURRENT_BCR_1_SA_1B)
                    CurrentBcr2Sa2a:      value(telemetryType: CURRENT_BCR_2_SA_2A)
                    CurrentBcr2Sa2b:      value(telemetryType: CURRENT_BCR_2_SA_2B)
                    CurrentBcr3Sa3a:      value(telemetryType: CURRENT_BCR_3_SA_3A)
                    CurrentBcr3Sa3b:      value(telemetryType: CURRENT_BCR_3_SA_3B)
                    BcrOutputCurrent:     value(telemetryType: BCR_OUTPUT_CURRENT)
                    CurrentDraw3V3:       value(telemetryType: CURRENT_DRAW_3V3)
                    CurrentDraw5V:        value(telemetryType: CURRENT_DRAW_5V)
                    OutputCurrent12V:     value(telemetryType: OUTPUT_CURRENT_12V)
                    OutputCurrentBattery: value(telemetryType: OUTPUT_CURRENT_BATTERY)
                    OutputCurrent5V:      value(telemetryType: OUTPUT_CURRENT_5V)
                    OutputCurrent33V:     value(telemetryType: OUTPUT_CURRENT_33V)
                    OutputCurrentSwitch1: value(telemetryType: OUTPUT_CURRENT_SWITCH_1)
                    OutputCurrentSwitch2: value(telemetryType: OUTPUT_CURRENT_SWITCH_2)
                    OutputCurrentSwitch3: value(telemetryType: OUTPUT_CURRENT_SWITCH_3)
                    OutputCurrentSwitch4: value(telemetryType: OUTPUT_CURRENT_SWITCH_4)
                    OutputCurrentSwitch5: value(telemetryType: OUTPUT_CURRENT_SWITCH_5)
                    OutputCurrentSwitch6: value(telemetryType: OUTPUT_CURRENT_SWITCH_6)
                    OutputCurrentSwitch7: value(telemetryType: OUTPUT_CURRENT_SWITCH_7)
                    OutputCurrentSwitch8: value(telemetryType: OUTPUT_CURRENT_SWITCH_8)
                    OutputCurrentSwitch9: value(telemetryType: OUTPUT_CURRENT_SWITCH_9)
                    OutputCurrentSwitch10: value(telemetryType: OUTPUT_CURRENT_SWITCH_10)
                }
            }
        }
        """

    elif group == "temperature":
        query = """
        {
            telemetry {
                motherboard {
                    boardTemperature: value(telemetryType: BOARD_TEMPERATURE)
                }
            }
        }
        """

    elif group == "reset":
        # reset sub-fields use snake_case -> camelCase: brown_out -> brownOut, etc.
        query = """
        {
            telemetry {
                reset {
                    brownOut       { motherboard daughterboard }
                    watchdog       { motherboard daughterboard }
                    manual         { motherboard daughterboard }
                    automaticSoftware { motherboard daughterboard }
                }
            }
        }
        """

    elif group == "watchdog":
        # field is watchdog_period in Rust -> watchdogPeriod in GraphQL
        query = """
        {
            telemetry {
                watchdogPeriod
            }
        }
        """

    elif group == "version":
        # firmware_number in Rust -> firmwareNumber in GraphQL
        query = """
        {
            telemetry {
                version {
                    motherboard {
                        revision
                        firmwareNumber
                    }
                    daughterboard {
                        revision
                        firmwareNumber
                    }
                }
            }
        }
        """

    else:
        print("Invalid telemetry group")
        sys.exit(1)

    send_query(services, f"Telemetry ({group})", query)


# ------------------------------------------------------------
# CLI
# ------------------------------------------------------------
def main():
    parser = argparse.ArgumentParser(
        description="""
Clyde Space EPS Control Tool (KubOS)

-------------------------
TELEMETRY COMMANDS
-------------------------

Query motherboard telemetry:

    eps_ctrl.py telemetry voltage
    eps_ctrl.py telemetry current
    eps_ctrl.py telemetry temperature
    eps_ctrl.py telemetry reset
    eps_ctrl.py telemetry watchdog
    eps_ctrl.py telemetry version

-------------------------
PDM CONTROL OVERVIEW
-------------------------

Each PDM (Power Distribution Module) controls a power rail.

Common usage:

Turn ON a rail:
    eps_ctrl.py pdm-on 3

Turn OFF a rail:
    eps_ctrl.py pdm-off 3

Turn ON all rails:
    eps_ctrl.py pdm-all-on

Turn OFF all rails:
    eps_ctrl.py pdm-all-off

-------------------------
ADVANCED PDM FEATURES
-------------------------

Initial State:
    Defines the default state after EPS reset

    eps_ctrl.py pdm-init-on 3
    eps_ctrl.py pdm-init-off 3

Timer Control:
    Automatically turns OFF a rail after timeout

    eps_ctrl.py pdm-timer 3 60

    → Turns ON PDM 3 for 60 units (EPS-defined timebase)

-------------------------
RAW COMMAND
-------------------------

    eps_ctrl.py raw 128 0   # Manual reset

-------------------------

-------------------------
WATCHDOG
-------------------------

Set EPS watchdog timeout:

    eps_ctrl.py set-watchdog 10

Reset watchdog:
    eps_ctrl.py raw 34 0

-------------------------
""",
        formatter_class=argparse.RawTextHelpFormatter
    )

    parser.add_argument('--config', '-c')

    sub = parser.add_subparsers(dest="cmd")

    # Core
    sub.add_parser("noop")
    sub.add_parser("hw-test")

    p_wd = sub.add_parser("set-watchdog")
    p_wd.add_argument("period", type=int)

    # Raw
    p_raw = sub.add_parser("raw")
    p_raw.add_argument("command", type=int)
    p_raw.add_argument("data", nargs="*", type=int)

    # PDM
    p_on = sub.add_parser("pdm-on")
    p_on.add_argument("channel", type=int)

    p_off = sub.add_parser("pdm-off")
    p_off.add_argument("channel", type=int)

    p_init_on = sub.add_parser("pdm-init-on")
    p_init_on.add_argument("channel", type=int)

    p_init_off = sub.add_parser("pdm-init-off")
    p_init_off.add_argument("channel", type=int)

    sub.add_parser("pdm-all-on")
    sub.add_parser("pdm-all-off")
    sub.add_parser("get-pdm-state")

    p_timer = sub.add_parser("pdm-timer")
    p_timer.add_argument("channel", type=int)
    p_timer.add_argument("limit", type=int)

    # Telemetry
    p_tel = sub.add_parser("telemetry")
    p_tel.add_argument(
        "group",
        choices=["voltage", "current", "temperature", "reset", "watchdog", "version"]
    )

    args = parser.parse_args()
    services = setup_services(args.config)

    # Dispatch
    if args.cmd == "noop":
        hardware_test(services)  # noop → hw-test as placeholder

    elif args.cmd == "hw-test":
        hardware_test(services)

    elif args.cmd == "set-watchdog":
        set_watchdog(services, args.period)

    elif args.cmd == "raw":
        data = args.data if args.data else [0]
        issue_raw(services, args.command, data)

    elif args.cmd == "pdm-on":
        pdm_on(services, args.channel)

    elif args.cmd == "pdm-off":
        pdm_off(services, args.channel)

    elif args.cmd == "pdm-init-on":
        pdm_set_initial_on(services, args.channel)

    elif args.cmd == "pdm-init-off":
        pdm_set_initial_off(services, args.channel)

    elif args.cmd == "pdm-all-on":
        pdm_all_on(services)

    elif args.cmd == "pdm-all-off":
        pdm_all_off(services)

    elif args.cmd == "pdm-timer":
        pdm_set_timer(services, args.channel, args.limit)

    elif args.cmd == "telemetry":
        telemetry_query(services, args.group)

    elif args.cmd == "get-pdm-state":
        get_pdm_state(services)

    else:
        parser.print_help()


if __name__ == "__main__":
    main()
