# FSM Governance Engine (NLnet proposal)

Standalone Rust library that implements the core FSM rules described in `GRANT_PROPOSAL_NLNET.md`.

## Highlights

- `Grant` struct with lifecycle helpers (`approve`, `activate`, `disburse`) enforced by `FsmError`.
- `VoteType` + `GrantVote` for recording decision weights.
- JSON Schema (`docs/FSM_schema.json`) describing valid states/transitions.
- Audit trail (`AuditEntry`, `AuditTrail`) with serialization/export guidance.

## Example integrations

### DAO grant workflow

1. Instantiate `Grant::new(...)`, record `AuditEntry::new(...)` each time `approve`, `activate`, or `disburse` is called.
2. Persist trail via `serde_json::to_string(&trail.entries())` and attach to governance report.

### Treasury allocation

1. Replace `GrantStatus` with treasury-specific enums while reusing `AuditTrail`.
2. Each audit entry becomes an immutable proof for regulators (with metadata describing approvals).

### Compliance pipeline

1. Build a `ComplianceReport` struct like `Grant`, derive Borsh/Serde.
2. Use `AuditTrail` helpers to verify state before moving to the next phase.

## Example scripts

- `cargo run --example dao_grant_flow` — walkthrough: create grant, log audit entries, simulate vote, disburse and verify audit trail.

## Next steps

1. Refer to `docs/API.md` for usage.
2. Use `docs/AuditTrail.md` when integrating compliance logging.
3. Validate custom FSM definitions with `docs/FSM_schema.json`.
4. Follow `docs/RELEASE.md` to publish a crate release once testing/test coverage is complete.

## Philosophical foundations

The civilization model that guides IndrasNet—its MVP → Audit → Post-grant evolution, the deprecated PDA memory layer, and the cryptographic constitution—is laid out in `INDRASNET_CIVILIZATION_FOUNDATION.md`. That document makes explicit how an ontology-driven constitution, AI jurisdiction, PDA reality, and CI-based constitutional control form a single self-correcting civilization.
