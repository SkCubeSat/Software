# GraphQL Smoke Requests (File-Based)

These request payloads are intended for running `mram-service` tests with:

```sh
curl -sS http://127.0.0.1:8090/graphql -H 'content-type: application/json' --data @requests/00_ping.json
```

To run the full sequence:

```sh
sh run.sh
# or
sh run.sh http://127.0.0.1:8090/graphql
```

Notes:
- The write request stores `note.txt` with the contents `hello` (`aGVsbG8=`).
- The read request expects that file to exist.
- The delete request removes it again.
- `format` is included in both safe (`confirm=false`) and destructive (`confirm=true`) variants.
