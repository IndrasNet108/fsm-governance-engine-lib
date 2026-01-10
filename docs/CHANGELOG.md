# Changelog

## [Unreleased]
- Added audit trail module (`AuditEntry`, `AuditTrail`) with Borsh/Serde serialization and verification helpers.
- Documented JSON Schema (`docs/FSM_schema.json`), API (`docs/API.md`), audit trail specification (`docs/AuditTrail.md`) and release checklist.
- Introduced `examples/dao_grant_flow.rs` to demonstrate DAO grant handling and audit logging.
- Ensured FSM enums derive serialization traits and expanded unit tests for audit history + JSON roundtrips.
- Added CI workflow covering `cargo fmt` and `cargo test`.
- Added declarative FSM definitions (`FsmDefinition`) with structural validation and invariant metadata.
- Expanded schema and docs for invariants (`docs/Invariants.md`) plus example definition JSON.
- Added CLI validator (`fsm_validate`) and example loader for JSON definitions.
- Added `--schema` and `--strict` flags to the CLI validator.
