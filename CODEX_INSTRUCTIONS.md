# Custom Instructions for Codex

## Role and boundaries

- You work on FSM Governance Engine in Rust.
- Goal is hardening and formalization of the existing validation only library.
- Do not build a platform, service, user interface, network layer, or storage layer.
- Do not add authentication, authorization, keys, signatures, or blockchain integrations.
- Do not expand scope without an explicit request. Mark any nice to have as OUT-OF-SCOPE.

## Core principles

- Deterministic validation only. The library returns OK or error and no automatic actions.
- Fail closed. If data is missing or uncertain, return an error.

# Roadmap tasks

## Phase one: specification and test vectors

Deliverables:

1. docs/spec/FSM_Definition_Spec_v1.md
   - JSON format for FSM definition
   - Semantics for states, transitions, invariants
   - Error model with codes and conditions

2. docs/spec/Schema_Evolution_Policy.md
   - Schema versioning
   - Backward and forward compatibility

3. tests/vectors/ with at least twenty test vectors
   - Each vector includes input JSON and expected result
   - Expected result is OK or ERR with error code

4. scripts/repro_check.sh or cargo xtask repro-check
   - Verifies reproducibility on reference data

Requirements:

- Each test vector runs with one command in CI.
- Any behavior change requires spec updates and expected results updates.

## Phase two: core hardening

Deliverables:

1. At least thirty property based or negative tests
   - Use proptest or equivalent in a PR
   - Focus on invariants, transitions, loader, edge cases

2. Fuzzing
   - Use cargo fuzz and libFuzzer
   - Targets include loader and validation
   - Add a fixed nightly budget in CI or in documented run instructions

3. docs/security/FAIL_CLOSED.md
   - Describe failure modes clearly

4. Triage
   - docs/process/TRIAGE.md
   - Issue template with reproduction steps, minimization, severity

Requirements:

- Any critical defect requires a test, fix, and changelog entry.

## Phase three: release engineering and artifacts

Deliverables:

1. v0.1.0 release
   - Cargo.toml version bump
   - Git tag and release notes
   - crates.io publish

2. SBOM
   - SPDX or CycloneDX with stated rationale
   - Artifact stored in release or release assets

3. Reproducible release process
   - docs/release/RELEASE.md
   - Build commands, hash checks, environment

4. Changelog policy
   - docs/release/CHANGELOG_POLICY.md
   - SemVer and change categories

Requirements:

- Any release is reproducible by the documented steps.
- Artifacts for external review include spec, test vectors, SBOM, and instructions.

## Phase four: two reference integrations

Deliverables:

1. GitHub Actions step
   - workflow .github/workflows/validate-fsm.yml
   - Runs CLI and compares outputs to expected

2. Governance example
   - examples/governance_lifecycle/
   - Validation and audit JSONL output

3. Reproduction docs
   - docs/examples/GOVERNANCE_EXAMPLE.md
   - docs/examples/CI_INTEGRATION.md

Requirements:

- Examples must be copy and run.
- Outputs must be stable and verifiable.

# Work rules and PR format

For each task:

- Provide a minimal plan of change with files and changes.
- Use small PRs where one PR maps to one deliverable.
- After each PR, include a short verification section with commands, expected results, and test vector references.

Forbidden:

- Refactors without a deliverable mapping.
- New modules without a scoped deliverable.
- Nondeterministic outputs.
- Logs instead of structured errors.

# Definition of done

Work is done when:

- Spec v1.0 and schema evolution policy exist.
- At least twenty test vectors with expected outputs exist.
- At least thirty property or negative tests exist.
- Fuzzing targets exist with a documented nightly budget.
- v0.1.0 release exists with SBOM and RELEASE.md.
- Two integrations exist with reproducible outputs.
- CI is green and reproduction commands are documented.

# Response style

- Write short lists.
- If unsure, choose fail closed.
- Mark assumptions as ASSUMPTION.
- Mark out of scope items as OUT-OF-SCOPE.

# Minimal command set

- cargo test
- cargo run -- validate <path_to_json>
- cargo run -- audit <path_to_json> --out audit.jsonl if applicable
- cargo fuzz run <target> if enabled

# Layout note

If the repository uses different commands or structure, first discover the real layout and then adapt without changing goals and deliverables.

# Phrases to avoid scope drift

- No new features, only hardening and formalization.
- Validation only. No execution.
- Deterministic outputs and reproducible commands are the product.
- Fail closed is mandatory.
