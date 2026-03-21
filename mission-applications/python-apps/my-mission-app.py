#!/usr/bin/env python3

import argparse
import sys
# Import exactly what is defined in the kubos_app/__init__.py
from kubos_app import Services, logging_setup

def main():
    # Initialize logger directly from the imported function
    logger = logging_setup("my-mission-app")

    parser = argparse.ArgumentParser()
    parser.add_argument('--config', '-c', help='Path to kubos-config.toml')
    args = parser.parse_args()

    # Initialize Services based on whether a config path was provided
    try:
        if args.config:
            services = Services(args.config)
        else:
            services = Services()
    except Exception as e:
        logger.error(f"Failed to load config: {e}")
        sys.exit(1)

    # Cleaned up GraphQL query (removed unnecessary backslashes)
    request = '{memInfo{available}}'

    try:
        response = services.query(service="monitor-service", query=request)
        available = response["memInfo"]["available"]
        logger.info(f"Current available memory: {available} kB")
    except Exception as e:
        logger.error(f"Monitor query failed: {e}")
        sys.exit(1)

    # Cleaned up Mutation string using a f-string or standard formatting
    # Note: GraphQL mutations use double braces {{ }} if using .format(), 
    # but since we are just doing simple substitution, we'll use a raw string.
    mutation_query = f"""
    mutation {{
        insert(subsystem: "OBC", parameter: "available_mem", value: "{available}") {{
            success,
            errors
        }}
    }}
    """

    try:
        response = services.query(service="telemetry-service", query=mutation_query)
        data = response["insert"]
        
        if not data["success"]:
            logger.error(f"Telemetry insert encountered errors: {data['errors']}")
            sys.exit(1)
        else:
            logger.info("Telemetry insert completed successfully")
            
    except Exception as e:
        logger.error(f"Telemetry service query failed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
