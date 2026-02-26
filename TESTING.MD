# Default Testing Procedure

## Standard Test Flow

Use this as the default testing process for services in this repository.

### 1. Run local tests first (Rust default test runner)

From the service directory you are working on, run the Rust test suite locally:

```bash
cargo test
```

This should be the first step whenever possible. It is fast and catches most logic errors before trying hardware-specific tests.

### 2. Only move to OBC testing when hardware is required

Local testing only covers what the current machine can access. If a test depends on hardware, flight configuration, or services running on the actual KubOS OBC, run those tests on-target instead.

## OBC / KubOS Service Testing

### Recommended approach

Write tests that:

- send `curl` commands to the service
- check for expected JSON responses

This works well for service-level integration testing and is easy to run repeatedly.

### Transfer test files to the OBC

Prepare your test scripts/files on the PowerEdge computer, then transfer them to the OBC using:

```bash
transfer <folder-or-file>
```

You can transfer either a single file or a whole test folder.

### Start the KubOS service on the OBC

Run the service with the appropriate config file and put it in the background:

```bash
kubos<service-name> -c <config_file.toml> &
```

Make sure the service is running before executing the tests.

### Run the tests on the OBC

After transferring the test assets, run them directly on the OBC.

The easiest pattern is usually:

- create GraphQL request JSON files
- create a small shell script that runs them in sequence

Python-based tests are also fine if that is more convenient.

### Changing the Baud Rate on Kubos
We initially had the default baud rate of 115200 configured in UBOOT. But later changed in a PR.

The baud rate we use now is **921600**.

So this might not be needed anymore. But in case you ever need to change the baud rate, you can use the following command:
```
stty -F /dev/ttyS0 <baudrate>
```
