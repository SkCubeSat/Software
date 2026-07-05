# Communications CLI

`comms-cli` is a typed command-line client for the communications service's
GraphQL API. It currently exposes every implemented public NXTRX4 NMP command.

```sh
cargo run -p comms-cli -- nmp downlink get-frequency
cargo run -p comms-cli -- nmp downlink unlock
cargo run -p comms-cli -- nmp downlink set-tx-enable true
```

When the service config has no suitable key, provide an explicit decimal or
hexadecimal override before the command:

```sh
comms-cli nmp downlink 0x1234ABCD get-frequency
```

The endpoint defaults to `http://127.0.0.1:8150/graphql` and can be overridden:

```sh
comms-cli --url http://obc:8150/graphql nmp uplink 0 get-config1
```

Use `comms-cli nmp --help` for the complete command list.
