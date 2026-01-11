# Contributing

Thanks for contributing to FSM Governance Engine.

## Scope

This project is validation-only. Contributions should preserve:

- deterministic behavior,
- explicit transition rules,
- invariant-driven validation,
- no side effects or execution logic.

## Development setup

```bash
cargo test
```

Optional:

```bash
cargo run --bin fsm_validate -- docs/example_fsm_definition.json --schema docs/FSM_schema.json --strict
cargo run --example fsm_definition_loader
```

## Style

- Keep public APIs documented with Rustdoc.
- Prefer explicit error handling over implicit fallbacks.
- Keep examples small and reproducible.

## Submitting changes

1. Create a focused branch.
2. Add tests for new behavior.
3. Update documentation when semantics change.
4. Open a pull request with a clear summary.
