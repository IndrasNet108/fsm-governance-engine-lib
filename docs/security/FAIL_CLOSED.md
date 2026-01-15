# Fail Closed Behavior

## Scope
This document describes the fail closed behavior of the validation layer.
The library validates definitions and transitions and returns OK or error.

## Principles

- Any invalid input produces an error.
- Any unknown invariant kind produces an error.
- Any structural violation produces an error.
- No automatic execution occurs on success or failure.

## Validation behavior

- Structure validation returns an error if states or transitions are missing.
- Transition validation returns an error if from or to states are unknown.
- Invariant validation returns an error if any invariant check fails.
- Schema validation fails if the definition does not match the schema.

## Error handling

- The caller receives an explicit error.
- No implicit recovery is performed.
- The caller decides how to handle the error.

## Out of scope

- Execution and enforcement are outside this library.
- Audit storage and immutability are external concerns.
