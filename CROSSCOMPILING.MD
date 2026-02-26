# Cross Compiling Procedure

## Purpose

Cross compiling is the process used when you want to run a service on the KubOS OBC itself.

In most cases, you should already have debugged the code locally as much as possible before this step. Because of that, a `release` build is usually preferred (smaller binary, faster transfer, better runtime performance).

## 1. Get the files onto the PowerEdge PC

Before cross compiling, make sure your service source code is available on the PowerEdge machine.

You can do this by either:

- using SFTP to copy the files
- pulling your branch directly on the PowerEdge PC

## 2. Cross compile the service for KubOS (ARM)

Run the following command from the repository on the PowerEdge PC:

```bash
cross build --target armv7-unknown-linux-gnueabihf -p <service-name> --release
```

This builds a release binary for the ARM target used by the KubOS system.

If you want verbose output while building, add `-vv`:

```bash
cross build --target armv7-unknown-linux-gnueabihf -p <service-name> --release -vv
```

## 3. (Optional) Strip the binary to reduce size

To further reduce the binary size before transfer, strip symbols from the compiled binary:

```bash
arm-linux-gnueabihf-strip <service-name>
```

Note: run this in the directory containing the compiled binary, or provide the full path to the binary.

## 4. Find the compiled binary

The compiled release binary will be located under:

```bash
target/armv7-unknown-linux-gnueabihf/release
```

