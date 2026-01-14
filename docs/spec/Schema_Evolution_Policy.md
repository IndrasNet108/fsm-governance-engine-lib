# Schema Evolution Policy

## Scope
This policy defines how the FSM definition schema evolves across versions.
The goal is predictable validation and minimal breaking change risk.

## Versioning
Schema versions follow semantic versioning principles.
Major changes may introduce breaking validation.
Minor changes add backward compatible fields or rules.
Patch changes fix errors or clarify rules without changing meaning.

## Backward compatibility
Backward compatible changes include:
- Adding optional fields with defaults.
- Adding new invariant kinds that are optional.
- Clarifying documentation without changing validation behavior.

Breaking changes include:
- Removing fields.
- Renaming fields.
- Changing required fields.
- Changing validation rules that alter pass or fail outcomes.

## Forward compatibility
Definitions should ignore unknown fields only if the schema version explicitly allows it.
The current schema is strict and rejects unknown fields.

## Deprecation
Deprecated fields remain supported for one major version.
Deprecations must be documented with migration guidance.

## Change control
Any change that alters validation outcomes requires:
- A schema version bump.
- Updated spec documentation.
- Updated test vectors and expected results.

