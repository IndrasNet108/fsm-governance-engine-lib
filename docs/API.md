# FSM Governance Engine API

## Core primitives

### `FsmDefinition`

Declarative FSM definition used for validation-only workflows. It is designed to be loaded from
JSON and validated before integration.

- `FsmDefinition { states, transitions, defaults, invariants }` (JSON uses `initialState` in defaults)
- `FsmDefinition::validate_structure()` – ensures transitions reference declared states and use non-empty actions.
- `FsmDefinition::validate_invariants()` – enforces supported invariants (see `docs/Invariants.md`).
- `FsmDefinition::validate()` – runs structure + invariant validation.

Use `docs/FSM_schema.json` for schema validation, `docs/example_fsm_definition.json` for a concrete
example, and `docs/Invariants.md` for invariant semantics.

See `docs/FSM_schema.md` for a schema overview and `docs/AuditModel.md` for audit record semantics.
See `docs/CI_validation.md` for CI/CD integration examples.

### `Grant`

- `Grant::new(id, idea_id, mesh_group_id, category, grant_type, disbursement_type, base_amount, reputation_bonus, created_at)` – creates validated grant state and calculates `total_amount`.
- `Grant::approve()` – moves from `Pending` to `Approved`. Fails with `FsmError::InvalidState` if invoked at another state.
- `Grant::activate()` – moves from `Approved` to `Active`.
- `Grant::disburse(amount)` – increments `disbursed_amount`, preventing overflow and enforcing `total_amount`. Moves to `Completed` automatically when fully disbursed.

Each method returns `Result<(), FsmError>` to make integration with higher-level workflows easy.

### `IdeaStatus`

Use `IdeaStatus::validate_transition(target)` to check permitted transitions before applying them. Invalid transitions raise `FsmError::InvalidStateTransition`.

### `AuditTrail`

See `docs/AuditTrail.md`. The trail can be recorded alongside every transition and exported for audits.

## Serialization

All domain structs derive both `BorshSerialize/BorshDeserialize` and `Serialize/Deserialize`, allowing:

- binary persistence (`Borsh`) for on-chain or embedded use,
- JSON export (`serde_json`) for audit reports.

## Extending to other domains

1. Define new states/enums (following patterns in `grant::types`).
2. Implement a struct similar to `Grant`, deriving serialization traits.
3. Record transitions via `AuditTrail` to reuse existing verification helpers.

## Concrete use cases

- **DAO grants**: Use `Grant` + `AuditTrail` to validate transition attempts (`pending→approved→active`), attach `GrantVote` results, and log disbursements (urgent/escrow).  
- **Treasury workflow**: Treat `GrantStatus` as `TreasuryStatus` (`Proposed`, `Allocated`, `Settled`) and reuse `AuditTrail` entries as compliance-relevant validation artifacts.  
- **Compliance pipeline**: Replace `Grant` with a `Report` type; each transition logs reviewer, decision, and metadata for regulators.

This makes the module reusable for treasury workflows, compliance pipelines, or regulator-managed processes.

## CLI validator

The CLI validates declarative FSM JSON files.

```bash
cargo run --bin fsm_validate -- docs/example_fsm_definition.json
```

Options:

- `--schema <path>`: validate against a JSON Schema (for example `docs/FSM_schema.json`).
- `--strict`: requires `defaults.initialState` and at least one invariant.
