#!/usr/bin/env bash
set -euo pipefail

cargo test --test vector_validation
cargo test --test definitions_validation
