# Communications CLI

`comms-cli` is a typed command-line client for the communications service's
GraphQL API. It currently exposes every implemented public NXTRX4 NMP command.

```sh
cargo run -p comms-cli -- nmp downlink 0 get-frequency
cargo run -p comms-cli -- nmp downlink 123456 set-tx-enable true
cargo run -p comms-cli -- nmp downlink 123456 set-routes 2:1:2 8:2:8
```

The endpoint defaults to `http://127.0.0.1:8150/graphql` and can be overridden:

```sh
comms-cli --url http://obc:8150/graphql nmp uplink 0 get-config1
```

Use `comms-cli nmp --help` for the complete command list.
