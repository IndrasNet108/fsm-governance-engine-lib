# FSM Governance Engine

Standalone Rust library for deterministic validation of integrity-critical, human-driven processes.
This project implements a validation-only FSM core. It does not execute actions, automate decisions, or enforce outcomes.

## Core innovation

Declarative, validation-only FSM for integrity-critical processes, with formally defined invariants
and deterministic auditability.

## Highlights

- Declarative FSM definitions (JSON) with a schema for states, transitions, and invariants.
- Validation-only FSM core for deterministic process verification.
- Invariants as first-class artifacts (terminal rules, forbidden cycles, required transitions).
- Audit trail (`AuditEntry`, `AuditTrail`) with serialization/export guidance.
- Reference implementations for governance (`Grant`, `IdeaStatus`) to show integration patterns.

## Quick verification (smoke checks)

Run the full test suite:

```bash
cargo test
```

Validate a declarative FSM definition against the schema (strict mode):

```bash
cargo run --bin fsm_validate -- docs/example_fsm_definition.json --schema docs/FSM_schema.json --strict
```

Load and parse the example definition (definition loader example):

```bash
cargo run --example fsm_definition_loader
# Expected: "Loaded 4 states and 4 transitions."
```

## Security notes

- Validation-only: the library does not execute actions or enforce outcomes.
- Deterministic transitions and explicit contracts; invalid transitions are rejected.
- Declarative definitions are schema-validated before use.
- Audit records are append-only and deterministically ordered (where applicable).

## Declarative layer

- Schema: `docs/FSM_schema.json`
- Example: `docs/example_fsm_definition.json`
- Invariant semantics: `docs/Invariants.md`

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
- `cargo run --example fsm_definition_loader` — load and validate `docs/example_fsm_definition.json`.

## CLI validator

Use the built-in validator to check JSON definitions:

```bash
cargo run --bin fsm_validate -- docs/example_fsm_definition.json
```

## Next steps

1. Refer to `docs/API.md` for usage.
2. Use `docs/AuditTrail.md` when integrating compliance logging.
3. Validate custom FSM definitions with `docs/FSM_schema.json`.
4. Review invariants guidance in `docs/Invariants.md`.
5. Follow `docs/RELEASE.md` to publish a crate release once testing/test coverage is complete.

## Philosophical foundations (optional context)

The following document provides broader conceptual background and is not required to use or understand the FSM Governance Engine.

The civilization model that guides IndrasNet—its MVP → Audit → Post-grant evolution, the deprecated PDA memory layer, and the cryptographic constitution—is laid out in `INDRASNET_CIVILIZATION_FOUNDATION.md`. That document makes explicit how an ontology-driven constitution, AI jurisdiction, PDA reality, and CI-based constitutional control form a single self-correcting civilization.

Security note: This library validates process correctness but does not execute actions, manage assets, or replace human decision-making.
