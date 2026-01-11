#!/usr/bin/env bash
set -euo pipefail

DEFINITION_PATH=${1:-docs/example_fsm_definition.json}
SCHEMA_PATH=${2:-docs/FSM_schema.json}

cargo run --bin fsm_validate -- "$DEFINITION_PATH" --schema "$SCHEMA_PATH" --strict
