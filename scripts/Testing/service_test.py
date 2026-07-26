#!/usr/bin/env python3

import argparse

import kubos_app as app_api

import sys

import json



def main():

    # Setup logging for the application

    logger = app_api.logging_setup("arb-service-client")



    parser = argparse.ArgumentParser(description="Client script for Any Service")



    # Allow specifying a custom config file (useful for local testing or non-standard paths)

    parser.add_argument('--config', '-c', help='Path to config.toml file')



    # The query string is required input from the command line

    parser.add_argument('--query', '-q', help='GraphQL query to execute', required=True)



    # Allow overriding the service name if needed, default to standard OEM-6 service name

    parser.add_argument('--service', '-s', help='Service name to query', default='novatel-oem6-service')



    args = parser.parse_args()



    # Initialize the Services API with the provided or default config

    if args.config is not None:

        SERVICES = app_api.Services(args.config)

    else:

        SERVICES = app_api.Services()



    request = args.query

    logger.info(f"Sending query to {args.service}: {request}")



    try:

        # Execute the query against the specified service

        response = SERVICES.query(service=args.service, query=request)



        # Check if the service returned errors in the GraphQL response

        if "errors" in response:

            logger.error("Service returned errors: " + str(response["errors"]))

            print(json.dumps(response, indent=2))

            sys.exit(1)



        # Output the successful data formatted as JSON

        print(json.dumps(response, indent=2))

        logger.info("Query completed successfully")



    except Exception as e:

        logger.error("Something went wrong during execution: " + str(e))

        sys.exit(1)



if __name__ == "__main__":

    main()