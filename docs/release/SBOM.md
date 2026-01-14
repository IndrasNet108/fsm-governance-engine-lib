# SBOM

## Scope
This document describes how to produce a software bill of materials.

## Format choice
Use CycloneDX for Rust.
The output is a JSON file stored with the release artifacts.

## Command

- cargo install cyclonedx-bom
- cyclonedx-bom -o release/sbom.json

## Notes

- The SBOM must be generated from the release commit.
- Store the SBOM with the release assets.

