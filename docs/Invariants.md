# Invariants

Invariants define checkable constraints over declared states and transitions. They are
intended to be machine-verifiable and auditor-friendly, without introducing execution logic.

## Why invariants

- Provide deterministic validation outcomes about allowed process behavior.
- Make governance workflows auditable as a set of rules, not just code paths.
- Allow multiple implementations to validate the same process definition.

## Supported invariant kinds

- `terminal_states`: listed states must have no outbound transitions.
- `required_transitions`: listed transitions must exist in the definition.
- `forbidden_transitions`: listed transitions must not exist.
- `forbidden_cycles`: listed states must not be part of any cycle (no path back to self).
- `self_transitions_required`: listed states must include explicit self-transitions. If `states` is empty, applies to all states.

## Example

```json
{
  "kind": "terminal_states",
  "states": ["Archived"],
  "description": "Archived is a terminal state with no outbound transitions."
}
```

Invariants are descriptive by design. Enforcement is performed by validators using the
FSM definition, transition table, and this invariant list.
