# NovAtel OEM7 Service

Kubos Service for interacting with a [NovAtel OEM7 High Precision GNSS Receiver](https://novatel.com/products/receivers/oem7-receivers)

This service uses a hybrid ASCII/binary protocol:
- **Commands** are sent as ASCII text (simple, no CRC computation needed)
- **Log data** is received in compact binary format (bandwidth efficient for high-frequency polling)

# Configuration

The service can be configured in the `/etc/kubos-config.toml` with the following fields:

- `bus` - Specifies the UART bus the OEM7 is connected to
- `baud` - Specifies the serial baud rate (default: 9600)
- `ip` - Specifies the service's IP address
- `port` - Specifies the port on which the service will be listening for UDP packets

For example:

```toml
[novatel-oem7-service]
bus = "/dev/ttyS4"
baud = 9600

[novatel-oem7-service.addr]
ip = "0.0.0.0"
port = 8130
```

# Starting the Service

The service should be started automatically by its init script, but may also be started manually:

```bash
$ novatel-oem7-service
Kubos OEM7 service started
Listening on: 0.0.0.0:8130
```

If no config file is specified, then the service will look at `/etc/kubos-config.toml`.
An alternative config file may be specified on the command line at run time:

```bash
$ novatel-oem7-service -c config.toml
```

# Queries

## Ping

Test query to verify service is running without attempting
to communicate with the underlying subsystem

```json
{
    ping: "pong"
}
```

## ACK

Get the last run mutation

```json
{
    ack: AckCommand
}
```

## Errors

Get all errors encountered since the last time this field was queried

```json
{
    errors: [String]
}
```

## Power Status

Get the current power state of the system

Note: `uptime` is included as an available field in order to conform to
      the Kubos Service Outline, but cannot be implemented for this device,
      so the value will be 1 if the device is on and 0 if the device is off

```json
{
    power {
        state: PowerState,
        uptime: Int
    }
}
```

## Configuration

Get the current configuration of the system

```json
{
    config: "Not Implemented"
}
```

## Test Results

Get the test results of the last run test

```json
{
    testResults{
        success,
        telemetryNominal{...},
        telemetryDebug{...}
    }
}
```

## System Status

Get the current system status and errors.
Requests the RXSTATUS binary log from the receiver and parses the
receiver status word into OEM7-specific flag names.

```json
{
    systemStatus {
       errors: Vec<String>,
       status: Vec<String>
    }
}
```

## Lock Status

Get current status of position information gathering.
Requests the BESTXYZ binary log from the receiver.

```json
{
    lockStatus {
        positionStatus: SolutionStatus,
          positionType: PosVelType,
          time {
            ms: Int,
              week: Int
          },
        timeStatus: RefTimeStatus,
        velocityStatus: SolutionStatus,
          velocityType: PosVelType
    }
}
```

## Lock Information

Get the last known good position information.
Requests the BESTXYZ binary log from the receiver.

Position and velocity are in ECEF (Earth-Centered Earth-Fixed) coordinates.

```json
{
    lockInfo {
       position: Vec<Float>,
       time {
           ms: Int,
           week: Int
       },
       velocity: Vec<Float>
    }
}
```

## Telemetry

Get current telemetry information for the system.
Combines nominal telemetry (position, status) with debug telemetry (version info).

```json
{
    telemetry{
        debug {
            components: [{
                bootVersion: String,
                compType: Int,
                compileDate: String,
                compileTime: String,
                hwVersion: String,
                model: String,
                serialNum: String,
                swVersion: String,
            }],
            numComponents: Int
        },
        nominal{
            lockInfo {...},
            lockStatus {...},
            systemStatus: {
               errors: Vec<String>,
               status: Vec<String>
            }
        }
    }
}
```

# Mutations

## Errors

Get all errors encountered while processing this GraphQL request

Note: This will only return errors thrown by fields which have
already been processed, so it is recommended that this field be specified last.

```json
mutation {
    errors: [String]
}
```

## No-Op

Execute a trivial command against the system (requests VERSION log as connectivity test)

```json
mutation {
    noop {
        errors: String,
        success: Boolean
   }
}
```

## Set Power State

Control the power state of the system

Note: Power control of the GPS device will be done by the GPSRM service

```json
mutation {
    controlPower: "Not Implemented"
}
```

## Configuration

Configure the system by sending LOG/UNLOG commands to the receiver.

- config: Vector of configuration requests (ConfigStruct)
  - option: Configuration operation which should be performed
  - hold: For `LOG_*` requests, specifies whether this request should be excluded
          from removal by future 'UNLOG_ALL' requests.
          For `UNLOG_ALL` requests, specifies whether the 'hold' value in previous
          `LOG_*` requests should be ignored.
  - interval: Interval at which log messages should be generated.
              Note: Only applies to `LOG_POSITION_DATA` requests. Ignored otherwise
  - offset: Offset of interval at which log messages should be generated.
            Note: Only applies to `LOG_POSITION_DATA` requests. Ignored otherwise

```json
mutation {
    configureHardware(config: [{option: ConfigOption, hold: Boolean, interval: Float, offset: Float},...]) {
        config: String
        errors: String,
        success: Boolean,
    }
}
```

## System Self-Test

Run a system self-test

- test: Type of self-test to perform

```json
mutation {
    testHardware(test: TestType) {
        ... on IntegrationTestResults {
            errors: String,
            success: Boolean,
            telemetryNominal{...},
            telemetryDebug{...}
        }
        ... on HardwareTestResults {
            errors: "Not Implemented",
            success: true,
            data: Empty
        }
   }
}
```

## Passthrough

Pass a raw ASCII command through to the receiver and return the response.

Unlike the OEM6 service which sends hex-encoded binary bytes, this service
sends the command string **as-is** in ASCII format. This allows sending any
NovAtel ASCII command directly.

- command: ASCII command string to send (e.g., `"LOG VERSIONA ONCE"`, `"UNLOGALL TRUE"`)

```json
mutation {
    issueRawCommand(command: String) {
        errors: String,
        success: Boolean,
        response: String
    }
}
```
