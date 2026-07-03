# Cube ADCS GraphQL Test Script

`adcs_graphql_test.py` dynamically introspects the running `cube-adcs-service`
GraphQL schema and builds tests from the live command and telemetry fields.

Start the service first, then run one of these commands from the repository root:

```bash
python3 scripts/Testing/cube-adcs-service/adcs_graphql_test.py \
  --url http://127.0.0.1:8000/graphql discover
```

Run telemetry queries only:

```bash
python3 scripts/Testing/cube-adcs-service/adcs_graphql_test.py \
  --url http://127.0.0.1:8000/graphql telemetry
```

Run selected telemetry IDs:

```bash
python3 scripts/Testing/cube-adcs-service/adcs_graphql_test.py \
  --url http://127.0.0.1:8000/graphql telemetry --ids 136,170,243
```

Create a command input template:

```bash
python3 scripts/Testing/cube-adcs-service/adcs_graphql_test.py \
  --url http://127.0.0.1:8000/graphql commands --write-template adcs_command_inputs.json
```

Send selected commands only after reviewing the input JSON:

```bash
python3 scripts/Testing/cube-adcs-service/adcs_graphql_test.py \
  --url http://127.0.0.1:8000/graphql commands \
  --ids 136 \
  --inputs adcs_command_inputs.json \
  --include-mutating-commands
```

Command mutations are intentionally opt-in because many telecommands change ADCS
state.
