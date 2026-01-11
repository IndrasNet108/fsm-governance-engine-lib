# FSM Schema

This document describes the declarative FSM schema used by the engine.
The authoritative schema file is `docs/FSM_schema.json`.

## Top-level fields

- `states`: array of unique state identifiers.
- `transitions`: array of allowed transitions.
- `defaults`: optional defaults (currently supports `initialState`).
- `invariants`: optional list of invariant declarations.

## Transition fields

Each transition entry contains:

- `from`: source state.
- `to`: target state.
- `action`: symbolic action name.
- `guard`: optional guard expression (string, for external evaluation).
- `metadata`: optional description and role hints.

## Invariants

Invariants are declarative constraints checked by validators. Supported kinds are
listed in `docs/Invariants.md`.

## Validation

Schema validation:

```bash
cargo run --bin fsm_validate -- docs/example_fsm_definition.json --schema docs/FSM_schema.json
```

Strict validation (schema + structure + invariants):

```bash
cargo run --bin fsm_validate -- docs/example_fsm_definition.json --schema docs/FSM_schema.json --strict
```

## Example

See `docs/example_fsm_definition.json` for a complete definition.
