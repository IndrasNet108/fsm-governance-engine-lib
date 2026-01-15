# Release Process

## Scope
This document describes the reproducible release steps for the library.
The release process is validation only and does not introduce execution behavior.

## Preconditions

- Working tree is clean.
- Tests pass.
- Version is updated in Cargo.toml.

## Steps

1. Run tests.
   - cargo test

2. Update version in Cargo.toml.

3. Update docs/CHANGELOG.md with release notes.

4. Build release artifacts.
   - cargo build --release

5. Produce SBOM.
   - Follow docs/release/SBOM.md

6. Tag release.
   - git tag vX.Y.Z

7. Publish to crates.io.
   - cargo publish

## Reproducibility

- Build steps must be identical across machines.
- Use the same Rust toolchain version for the release.
- Record the toolchain version in release notes.

