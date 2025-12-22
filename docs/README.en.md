# FSM Governance Engine (NLnet proposal)

The standalone Rust crate implements the FSM governance rules outlined in `GRANT_PROPOSAL_NLNET.md`.

## Highlights

1. `Grant` lifecycle helpers (`approve`, `activate`, `disburse`) validated through `FsmError`.
2. `AuditEntry`/`AuditTrail` log every state transition, ready for compliance exports (Borsh + JSON).
3. Example schema (`docs/FSM_schema.json`) describes states/transitions and can be validated with any JSON Schema tool.
4. Publication checklist (`docs/RELEASE.md`) guides you through `cargo package` → `cargo publish`.

## Example usage

- Run `cargo run --example dao_grant_flow` to take a grant through approval, activate, vote logging, disbursement, and audit verification.
- Extend with your own domain (treasury, compliance). Reuse `AuditTrail` and FSM enums.

## Next steps

1. Review `docs/API.md` + `docs/AuditTrail.md` for detailed interfaces.
2. Apply the schema in `docs/FSM_schema.json` to your configuration files.
3. Execute `docs/RELEASE.md` checklist before publishing to crates.io.
