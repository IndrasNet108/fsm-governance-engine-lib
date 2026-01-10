# FSM Governance Engine

Standalone Rust library for deterministic validation of integrity-critical processes.
This project implements a validation-only FSM core. It does not execute actions, automate decisions, or enforce outcomes.

## Highlights

- Validation-only FSM core for deterministic process verification.
- `Grant` struct with lifecycle helpers (`approve`, `activate`, `disburse`) enforced by `FsmError`.
- `VoteType` + `GrantVote` for recording decision metadata.
- JSON Schema (`docs/FSM_schema.json`) describing valid states/transitions.
- Audit trail (`AuditEntry`, `AuditTrail`) with serialization/export guidance.

## Example integrations

### Governance-related process validation example

1. Instantiate `Grant::new(...)`, record `AuditEntry::new(...)` each time `approve`, `activate`, or `disburse` is called.
2. Persist trail via `serde_json::to_string(&trail.entries())` and attach to governance report.

### Financial-process-adjacent validation example

1. Replace `GrantStatus` with domain-specific enums while reusing `AuditTrail`.
2. Each audit entry serves as an immutable validation record with structured metadata.

### Compliance validation pipeline

1. Build a `ComplianceReport` - like struct , derive Borsh/Serde.
2. Use `AuditTrail` helpers to validate process state before transitioning phases.

## Example scripts

- `cargo run --example dao_grant_flow` — walkthrough: create grant, log audit entries, simulate vote, disburse and verify audit trail.

## Next steps

1. Refer to `docs/API.md` for usage.
2. Use `docs/AuditTrail.md` when integrating compliance logging.
3. Validate custom FSM definitions with `docs/FSM_schema.json`.
4. Follow `docs/RELEASE.md` to publish a crate release once testing/test coverage is complete.

## Philosophical foundations (optional context)

The following document provides broader conceptual background and is not required to use or understand the FSM Governance Engine.

The civilization model that guides IndrasNet—its MVP → Audit → Post-grant evolution, the deprecated PDA memory layer, and the cryptographic constitution—is laid out in `INDRASNET_CIVILIZATION_FOUNDATION.md`. That document makes explicit how an ontology-driven constitution, AI jurisdiction, PDA reality, and CI-based constitutional control form a single self-correcting civilization.

Security note: This library validates process correctness but does not execute actions, manage assets, or replace human decision-making.
