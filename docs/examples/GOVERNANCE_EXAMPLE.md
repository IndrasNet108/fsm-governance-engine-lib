# Governance example

## Scope
This example demonstrates validation of a governance lifecycle definition and audit JSONL output.

## Commands

- cargo run --example governance_lifecycle > /tmp/governance_lifecycle.out
- diff -u examples/expected/governance_lifecycle.stdout /tmp/governance_lifecycle.out

## Expected output

- Audit JSONL output is printed to stdout
- Proposal lifecycle complete: Executed

## Artifacts

- examples/expected/governance_lifecycle.stdout
