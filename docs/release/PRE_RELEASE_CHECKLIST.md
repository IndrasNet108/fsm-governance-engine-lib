# Pre-release checklist

## Scope
This checklist documents steps required before tagging and publishing v0.1.0.

## Steps

- Confirm working tree is clean.
- Confirm Cargo.toml version is v0.1.0.
- Run cargo test.
- Run cargo build --locked.
- Run cargo clippy --all-targets --all-features -- -D warnings.
- Run validate-fsm workflow locally or via CI.
- Run governance lifecycle example twice and confirm identical output.

## Determinism check

- Command:
  - cargo run --example governance_lifecycle > /tmp/governance_lifecycle.out
  - diff -u examples/expected/governance_lifecycle.stdout /tmp/governance_lifecycle.out
- Repeat twice and confirm no differences.

## Notes

- Do not generate SBOM before the release is tagged and published.
- Record the release commit hash in docs/release/RELEASE.md.

