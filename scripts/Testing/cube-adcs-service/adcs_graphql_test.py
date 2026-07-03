#!/usr/bin/env python3
"""Dynamic GraphQL tester for the CubeSpace ADCS service.

The script introspects the running cube-adcs-service and builds GraphQL queries
from the live schema, so it stays useful as generated telemetry/command structs
change. By default it only performs safe discovery and telemetry reads. Command
mutations require an explicit opt-in flag.
"""

import argparse
import json
import re
import sys
import time
import urllib.error
import urllib.request


DEFAULT_URL = "http://127.0.0.1:8000/graphql"

INTROSPECTION_QUERY = """
query AdcsIntrospection {
  __schema {
    queryType { name }
    mutationType { name }
  }
  queryType: __type(name: "QueryRoot") {
    fields {
      name
      description
      args { name type { ...TypeRef } defaultValue }
      type { ...TypeRef }
    }
  }
  mutationType: __type(name: "MutationRoot") {
    fields {
      name
      description
      args { name type { ...TypeRef } defaultValue }
      type { ...TypeRef }
    }
  }
  types: __schema {
    types {
      kind
      name
      fields {
        name
        description
        args { name type { ...TypeRef } defaultValue }
        type { ...TypeRef }
      }
      inputFields {
        name
        description
        defaultValue
        type { ...TypeRef }
      }
      enumValues { name description }
    }
  }
}

fragment TypeRef on __Type {
  kind
  name
  ofType {
    kind
    name
    ofType {
      kind
      name
      ofType {
        kind
        name
        ofType {
          kind
          name
          ofType {
            kind
            name
          }
        }
      }
    }
  }
}
"""

DEFINITION_QUERY = """
query AdcsDefinitions {
  telemetryDefinitions {
    id
    name
    purpose
    lengthBytes
    fields {
      name
      offsetBits
      lengthBits
      dataType
      description
      scale
      unit
      enumTable
    }
  }
  commandDefinitions {
    id
    name
    purpose
    lengthBytes
    fields {
      name
      offsetBits
      lengthBits
      dataType
      description
      scale
      unit
      enumTable
    }
  }
}
"""

COMMAND_RESPONSE_SELECTION = "success errors commandId acknowledged errorCode payloadHex"


class GraphqlError(RuntimeError):
    pass


def post_graphql(url, query, variables=None, timeout=10.0):
    body = json.dumps({"query": query, "variables": variables or {}}).encode("utf-8")
    request = urllib.request.Request(
        url,
        data=body,
        headers={"Content-Type": "application/json", "Accept": "application/json"},
        method="POST",
    )

    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            payload = response.read().decode("utf-8")
    except urllib.error.URLError as err:
        raise GraphqlError("failed to contact {}: {}".format(url, err)) from err

    try:
        decoded = json.loads(payload)
    except json.JSONDecodeError as err:
        raise GraphqlError("service returned non-JSON response: {}".format(payload)) from err

    return decoded


def unwrap_type(type_ref):
    required = False
    is_list = False
    current = type_ref
    while current:
        kind = current.get("kind")
        if kind == "NON_NULL":
            required = True
            current = current.get("ofType")
            continue
        if kind == "LIST":
            is_list = True
            current = current.get("ofType")
            continue
        return {
            "kind": kind,
            "name": current.get("name"),
            "required": required,
            "is_list": is_list,
        }
    return {"kind": None, "name": None, "required": required, "is_list": is_list}


def type_name(type_ref):
    return unwrap_type(type_ref)["name"]


def type_kind(type_ref):
    return unwrap_type(type_ref)["kind"]


def index_types(introspection):
    return {
        item["name"]: item
        for item in introspection["data"]["types"]["types"]
        if item.get("name")
    }


def schema_fields(introspection, root_name):
    root = introspection["data"].get(root_name) or {}
    return root.get("fields") or []


def parse_id(description, label):
    if not description:
        return None
    match = re.search(r"{} ID\s+(\d+)".format(label), description, re.IGNORECASE)
    return int(match.group(1)) if match else None


def is_builtin_meta_field(name):
    return name.startswith("__")


def build_selection(type_ref, types, depth=0, seen=None):
    """Build a valid GraphQL selection set for an output type."""
    seen = set(seen or [])
    unwrapped = unwrap_type(type_ref)
    kind = unwrapped["kind"]
    name = unwrapped["name"]

    if kind in ("SCALAR", "ENUM") or not name:
        return ""
    if depth > 4 or name in seen:
        return "__typename"

    type_def = types.get(name) or {}
    fields = type_def.get("fields") or []
    parts = []
    for field in fields:
        field_name = field["name"]
        if is_builtin_meta_field(field_name) or field.get("args"):
            continue

        nested = build_selection(field["type"], types, depth + 1, seen | {name})
        if nested:
            parts.append("{} {{ {} }}".format(field_name, nested))
        else:
            parts.append(field_name)

    return " ".join(parts) if parts else "__typename"


def graphql_literal(value):
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, (int, float)):
        return str(value)
    if value is None:
        return "null"
    if isinstance(value, list):
        return "[{}]".format(", ".join(graphql_literal(item) for item in value))
    if isinstance(value, dict):
        return "{{ {} }}".format(
            ", ".join("{}: {}".format(key, graphql_literal(val)) for key, val in value.items())
        )
    return json.dumps(value)


def default_input_value(type_ref, types):
    unwrapped = unwrap_type(type_ref)
    kind = unwrapped["kind"]
    name = unwrapped["name"]

    if unwrapped["is_list"]:
        return []
    if kind == "SCALAR":
        if name in ("Int", "Long", "I32", "I64", "U32", "U64"):
            return 0
        if name in ("Float", "Double"):
            return 0.0
        if name == "Boolean":
            return False
        return ""
    if kind == "ENUM":
        enum_values = (types.get(name) or {}).get("enumValues") or []
        return enum_values[0]["name"] if enum_values else ""
    if kind == "INPUT_OBJECT":
        return default_input_object(name, types)
    return None


def default_input_object(name, types):
    type_def = types.get(name) or {}
    result = {}
    for field in type_def.get("inputFields") or []:
        result[field["name"]] = default_input_value(field["type"], types)
    return result


def operation_args(args, inputs, types):
    chunks = []
    for arg in args:
        name = arg["name"]
        if name in inputs:
            value = inputs[name]
        else:
            value = default_input_value(arg["type"], types)
        chunks.append("{}: {}".format(name, graphql_literal(value)))
    return "({})".format(", ".join(chunks)) if chunks else ""


def telemetry_field_map(introspection):
    fields = schema_fields(introspection, "queryType")
    result = {}
    for field in fields:
        telemetry_id = parse_id(field.get("description"), "telemetry")
        if telemetry_id is not None:
            result[telemetry_id] = field
    return result


def command_field_map(introspection):
    fields = schema_fields(introspection, "mutationType")
    result = {}
    for field in fields:
        command_id = parse_id(field.get("description"), "telecommand")
        if command_id is not None:
            result[command_id] = field
    return result


def load_json(path):
    if not path:
        return {}
    with open(path, "r", encoding="utf-8") as stream:
        return json.load(stream)


def write_json(path, value):
    with open(path, "w", encoding="utf-8") as stream:
        json.dump(value, stream, indent=2, sort_keys=True)
        stream.write("\n")


def selected_items(items, selected_ids):
    if not selected_ids:
        return items
    wanted = set(selected_ids)
    return {item_id: item for item_id, item in items.items() if item_id in wanted}


def parse_ids(values):
    ids = []
    for value in values or []:
        for chunk in value.split(","):
            chunk = chunk.strip()
            if chunk:
                ids.append(int(chunk, 0))
    return ids


def run_discovery(args, introspection):
    definitions = post_graphql(args.url, DEFINITION_QUERY, timeout=args.timeout)
    if definitions.get("errors"):
        raise GraphqlError(json.dumps(definitions["errors"], indent=2))

    telemetry = definitions["data"]["telemetryDefinitions"]
    commands = definitions["data"]["commandDefinitions"]
    telemetry_fields = telemetry_field_map(introspection)
    command_fields = command_field_map(introspection)

    print("Telemetry definitions: {}".format(len(telemetry)))
    for item in telemetry:
        field = telemetry_fields.get(item["id"])
        print(
            "  {:3d}  {:36s}  GraphQL: {}".format(
                item["id"], item["name"][:36], field["name"] if field else "MISSING"
            )
        )

    print("\nCommand definitions: {}".format(len(commands)))
    for item in commands:
        field = command_fields.get(item["id"])
        print(
            "  {:3d}  {:36s}  GraphQL: {}".format(
                item["id"], item["name"][:36], field["name"] if field else "MISSING"
            )
        )


def run_telemetry(args, introspection):
    types = index_types(introspection)
    fields = selected_items(telemetry_field_map(introspection), parse_ids(args.ids))
    failures = 0

    for telemetry_id in sorted(fields):
        field = fields[telemetry_id]
        selection = build_selection(field["type"], types)
        query = "query {{ {} {{ {} }} }}".format(field["name"], selection)
        started = time.time()
        response = post_graphql(args.url, query, timeout=args.timeout)
        elapsed_ms = int((time.time() - started) * 1000)
        ok = not response.get("errors")
        failures += 0 if ok else 1
        print_result("telemetry", telemetry_id, field["name"], ok, elapsed_ms, response, args.verbose)

        if args.delay:
            time.sleep(args.delay)

    if failures:
        raise SystemExit(1)


def command_templates(introspection):
    types = index_types(introspection)
    fields = command_field_map(introspection)
    templates = {}
    for command_id in sorted(fields):
        field = fields[command_id]
        args = field.get("args") or []
        if not args:
            templates[field["name"]] = {}
            continue
        templates[field["name"]] = {
            arg["name"]: default_input_value(arg["type"], types) for arg in args
        }
    return templates


def run_commands(args, introspection):
    if args.write_template:
        write_json(args.write_template, command_templates(introspection))
        print("Wrote command input template to {}".format(args.write_template))
        return

    if not args.include_mutating_commands:
        print("Command mutations were not sent.")
        print("Use --write-template inputs.json to create an input file.")
        print("Use --include-mutating-commands --inputs inputs.json to send commands.")
        return

    types = index_types(introspection)
    fields = selected_items(command_field_map(introspection), parse_ids(args.ids))
    provided_inputs = load_json(args.inputs)
    failures = 0

    for command_id in sorted(fields):
        field = fields[command_id]
        field_inputs = provided_inputs.get(field["name"], {})
        args_text = operation_args(field.get("args") or [], field_inputs, types)
        mutation = "mutation {{ {}{} {{ {} }} }}".format(
            field["name"], args_text, COMMAND_RESPONSE_SELECTION
        )
        started = time.time()
        response = post_graphql(args.url, mutation, timeout=args.timeout)
        elapsed_ms = int((time.time() - started) * 1000)

        errors = response.get("errors")
        data = (response.get("data") or {}).get(field["name"]) or {}
        ok = not errors and data.get("success") is True
        failures += 0 if ok else 1
        print_result("command", command_id, field["name"], ok, elapsed_ms, response, args.verbose)

        if args.delay:
            time.sleep(args.delay)

    if failures:
        raise SystemExit(1)


def print_result(kind, item_id, name, ok, elapsed_ms, response, verbose):
    status = "PASS" if ok else "FAIL"
    print("{} {:3d} {:36s} {:4s} {} ms".format(kind, item_id, name[:36], status, elapsed_ms))
    if verbose or not ok:
        print(json.dumps(response, indent=2, sort_keys=True))


def build_parser():
    parser = argparse.ArgumentParser(
        description="Dynamically test CubeSpace ADCS GraphQL commands and telemetry."
    )
    parser.add_argument("--url", default=DEFAULT_URL, help="GraphQL endpoint URL")
    parser.add_argument("--timeout", type=float, default=10.0, help="HTTP timeout in seconds")
    parser.add_argument("--verbose", action="store_true", help="Print full GraphQL responses")

    subparsers = parser.add_subparsers(dest="mode", required=True)

    discover = subparsers.add_parser("discover", help="List command/telemetry definitions")
    discover.set_defaults(func=run_discovery)

    telemetry = subparsers.add_parser("telemetry", help="Execute telemetry queries")
    telemetry.add_argument("--ids", action="append", help="Telemetry IDs, comma-separated or repeated")
    telemetry.add_argument("--delay", type=float, default=0.0, help="Delay between requests")
    telemetry.set_defaults(func=run_telemetry)

    commands = subparsers.add_parser("commands", help="Prepare or execute telecommand mutations")
    commands.add_argument("--ids", action="append", help="Command IDs, comma-separated or repeated")
    commands.add_argument("--inputs", help="JSON file containing command inputs")
    commands.add_argument("--write-template", help="Write a JSON input template and exit")
    commands.add_argument("--delay", type=float, default=0.0, help="Delay between commands")
    commands.add_argument(
        "--include-mutating-commands",
        action="store_true",
        help="Actually send command mutations to the ADCS",
    )
    commands.set_defaults(func=run_commands)

    return parser


def main(argv):
    parser = build_parser()
    args = parser.parse_args(argv)
    introspection = post_graphql(args.url, INTROSPECTION_QUERY, timeout=args.timeout)
    if introspection.get("errors"):
        raise GraphqlError(json.dumps(introspection["errors"], indent=2))
    args.func(args, introspection)


if __name__ == "__main__":
    try:
        main(sys.argv[1:])
    except GraphqlError as err:
        print("ERROR: {}".format(err), file=sys.stderr)
        raise SystemExit(2)
