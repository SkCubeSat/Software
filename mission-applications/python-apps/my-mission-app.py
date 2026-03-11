#!/usr/bin/env python3

import argparse
from kubos import app
import sys

def main():

    logger = app.logging_setup("my-mission-app")

    parser = argparse.ArgumentParser()

    parser.add_argument('--config', '-c')

    args = parser.parse_args()

    if args.config is not None:
        global SERVICES
        SERVICES = app.Services(args.config)
    else:
        SERVICES = app.Services()

    args = parser.parse_args()

    request = '\{memInfo\{available\}\}'

    try:
        response = SERVICES.query(service="monitor-service", query=request)
    except Exception as e: 
        logger.error("Something went wrong: " + str(e))
        sys.exit(1)

    data = response["memInfo"]
    available = data["available"]

    logger.info("Current available memory: %s kB" % (available))

    request = '''
        mutation \{
            insert(subsystem: "OBC", parameter: "available_mem", value: "%s") \{
                success,
                errors
            \}
        \}
        ''' % (available)

    try:
        response = SERVICES.query(service="telemetry-service", query=request)
    except Exception as e: 
        logger.error("Something went wrong: " + str(e))
        sys.exit(1)

    data = response["insert"]
    success = data["success"]
    errors = data["errors"]

    if success == False:
        logger.error("Telemetry insert encountered errors: " + str(errors))
        sys.exit(1)
    else:
        logger.info("Telemetry insert completed successfully")

if __name__ == "__main__":
    main()