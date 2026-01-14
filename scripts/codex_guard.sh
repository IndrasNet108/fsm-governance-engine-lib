#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail() {
  echo "ERROR: $1" >&2
  exit 1
}

if [[ ! -f "${root_dir}/CODEX_INSTRUCTIONS.md" ]]; then
  fail "Missing CODEX_INSTRUCTIONS.md"
fi

if [[ ! -f "${root_dir}/README.md" ]]; then
  fail "Missing README.md"
fi

if ! head -n 10 "${root_dir}/README.md" | rg -q "CODEX_INSTRUCTIONS.md"; then
  fail "README.md must reference CODEX_INSTRUCTIONS.md in the first 10 lines"
fi

if [[ ! -f "${root_dir}/.github/PULL_REQUEST_TEMPLATE.md" ]]; then
  fail "Missing .github/PULL_REQUEST_TEMPLATE.md"
fi

if ! rg -q "CODEX_INSTRUCTIONS.md" "${root_dir}/.github/PULL_REQUEST_TEMPLATE.md"; then
  fail "PR template must reference CODEX_INSTRUCTIONS.md"
fi

forbidden_deps=(
  "tokio"
  "async-std"
  "reqwest"
  "hyper"
  "tonic"
  "actix"
  "warp"
  "axum"
  "rocket"
  "sqlx"
  "diesel"
  "rusqlite"
  "sled"
)

for dep in "${forbidden_deps[@]}"; do
  if rg -q "^${dep}\b" "${root_dir}/Cargo.toml"; then
    fail "Forbidden dependency detected in Cargo.toml: ${dep}"
  fi
  if rg -q "\b${dep}\b" "${root_dir}/Cargo.toml"; then
    fail "Forbidden dependency detected in Cargo.toml: ${dep}"
  fi
done

if [[ -d "${root_dir}/src/bin" ]]; then
  while IFS= read -r file; do
    if [[ "$(basename "$file")" != "fsm_validate.rs" ]]; then
      fail "Unexpected binary target: ${file}"
    fi
  done < <(find "${root_dir}/src/bin" -type f -name "*.rs" | sort)
fi

nondeterministic_patterns=(
  "SystemTime::now"
  "Utc::now"
  "Local::now"
  "OffsetDateTime::now"
  "thread_rng"
  "rand::random"
)

for pattern in "${nondeterministic_patterns[@]}"; do
  if rg -n "${pattern}" "${root_dir}/src" "${root_dir}/examples"; then
    fail "Nondeterministic usage detected: ${pattern}"
  fi
done

echo "codex_guard: OK"
