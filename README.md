# FSM Governance Engine

[![DOI](https://zenodo.org/badge/1120650150.svg)](https://doi.org/10.5281/zenodo.18394740)

Before making any changes, read `CODEX_INSTRUCTIONS.md` and follow it strictly.

Standalone Rust library for deterministic validation of integrity-critical, human-driven and system-driven processes.
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

## Quickstart

```bash
cargo test
cargo run --example fsm_definition_loader
```

## Quick verification (smoke checks)

Run the full test suite:

```bash
cargo test
```

Validate a declarative FSM definition against the schema (strict mode):

```bash
cargo run --bin fsm_validate -- docs/example_fsm_definition.json --schema docs/FSM_schema.json --strict
```

Expected: OK (strict validation).

Load and parse the example definition (definition loader example):

```bash
cargo run --example fsm_definition_loader
# Expected: "Loaded 4 states and 4 transitions."
```

## Security notes

- Validation-only: the library does not execute actions or enforce outcomes.
- Deterministic transitions and explicit contracts; invalid transitions are rejected.
- Declarative definitions are schema-validated before use.
- Audit records are append-only and deterministically ordered within the validation scope (where applicable).

## Explicit non-goals

- No execution or orchestration of workflows.
- No automation of decisions or governance outcomes.
- No authorization, identity, or key management.
- No storage, networking, or external side effects.

## Declarative layer

- Schema: `docs/FSM_schema.json`
- Schema overview: `docs/FSM_schema.md`
- Example: `docs/example_fsm_definition.json`
- Invariant semantics: `docs/Invariants.md`

## Documentation

- API: `docs/API.md`
- Audit trail: `docs/AuditTrail.md`
- Audit model: `docs/AuditModel.md`
- Contributing: `CONTRIBUTING.md`
- Governance: `GOVERNANCE.md`
- Code of Conduct: `CODE_OF_CONDUCT.md`

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

- `cargo run --example dao_grant_flow` — walkthrough: create grant, log audit entries, simulate vote, disburse and verify audit trail. Expected: `Audit JSONL` and `Audit trail entries: 2`.
- `cargo run --example fsm_definition_loader` — load and validate `docs/example_fsm_definition.json`. Expected: `Loaded 4 states and 4 transitions.`.
- `cargo run --example governance_lifecycle` — proposal lifecycle (draft → executed). Expected: `Proposal lifecycle complete: Executed`.
- `cargo run --example voting` — governance voting metadata initialization. Expected: `Voting initialized for proposal 200`.
- `cargo run --example treasury_flow` — treasury operation validation. Expected: `Treasury operation validated: Transfer`.
- `cargo run --example audit_only_validation` — audit trail validation without execution. Expected: `Audit JSONL` and `Audit-only validation complete: 1 entries`.

## CI

GitHub Actions runs fmt, clippy, and tests on stable and nightly.
For CI validation steps, see `docs/CI_validation.md`.

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

Security note: This library validates declared process constraints and transition rules without asserting process correctness, and does not execute actions, manage assets, or replace human decision-making.

IndrasNet® is a registered trademark of IndrasNet OÜ.
This project is not affiliated with or endorsed by IndrasNet OÜ unless explicitly stated.
