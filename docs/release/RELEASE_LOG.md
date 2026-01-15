# Release Log

## v0.1.0 — release complete

Date: 2026-01-15
Commit: 58b30f2
Tag: v0.1.0

Status:
- master merged
- GitHub Release published
- crates.io published
- SBOM attached to GitHub Release (CycloneDX 1.3)

Verification:
- cargo test --locked
- governance_lifecycle example deterministic (JSONL diff empty)
- CI green

Notes:
- Validation-only library
- No execution or side effects
- SBOM generated via cargo-cyclonedx
