# Hardening status

## Scope
This document summarizes hardening work for validation-only behavior.

## Property-based tests

- tests/definition_proptest.rs
  - prop_valid_structure
  - prop_whitespace_action_invalid
  - prop_unknown_from_invalid

## Negative tests

- tests/definition_negative.rs
  - Covers invalid structure, invalid transitions, and invariant violations.
  - More than thirty negative cases included.

## Fuzzing targets

- fuzz/fuzz_targets/definition_parser.rs
- fuzz/fuzz_targets/definition_validate.rs

## Fuzzing budget

- .github/workflows/fuzz.yml
- Ten minutes per target per nightly run.

## Failure handling

- Fail closed on invalid inputs and invariant violations.
- Errors are returned to callers with no execution.

