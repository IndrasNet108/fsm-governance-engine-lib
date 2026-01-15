# FSM Definition Spec v1.0

## Scope
This spec defines the JSON format and validation semantics for a declarative FSM definition.
The library validates structure and invariants only and never executes actions.

## Determinism
Given the same definition input, validation is deterministic and fail closed.
Invalid definitions produce errors and are rejected.

## Top level object

Required fields:
- states
- transitions

Optional fields:
- defaults
- invariants

Additional properties are not allowed.

## states

Type: array of strings
Rules:
- Must contain at least one entry.
- Each entry is a state name.

## transitions

Type: array of objects
Required fields:
- from
- to
- action

Optional fields:
- guard
- metadata

Rules:
- Must contain at least one entry.
- from, to, and action must be non empty strings.
- from and to must exist in states.

### metadata

Type: object
Optional fields:
- description
- roles

Rules:
- roles is an array of strings.
- Additional properties are not allowed.

## defaults

Type: object
Optional fields:
- initialState

Rules:
- initialState, when present, must be a state in states.

## invariants

Type: array of objects
Default: empty array
Required fields:
- kind

Optional fields:
- states
- transitions
- description

Rules:
- Unknown kinds are invalid.
- states is an array of strings.
- transitions is an array of objects with required fields from and to.

### Supported invariant kinds

- terminal_states
  - Each state in states must have no outbound transitions.

- required_transitions
  - Each transition in transitions must exist in the definition.

- forbidden_transitions
  - Each transition in transitions must not exist in the definition.

- forbidden_cycles
  - Each state in states must not reach itself through any path.

- self_transitions_required
  - Each listed state must have a self transition.
  - If states is empty, this applies to all states.

## Error model

Validation returns an error when any rule above is violated.
The current implementation uses InvalidInput for definition errors.

## Example

See docs/example_fsm_definition.json for a minimal working definition.
