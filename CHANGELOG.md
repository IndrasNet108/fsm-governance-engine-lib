# Changelog

## [Unreleased]
- Added audit trail module (`AuditEntry`, `AuditTrail`) with Borsh/Serde serialization and verification helpers.
- Documented JSON Schema (`docs/FSM_schema.json`), API (`docs/API.md`), audit trail specification (`docs/AuditTrail.md`) and release checklist.
- Introduced `examples/dao_grant_flow.rs` to demonstrate DAO grant handling and audit logging.
- Ensured FSM enums derive serialization traits and expanded unit tests for audit history + JSON roundtrips.
- Added CI workflow covering `cargo fmt` and `cargo test`.
