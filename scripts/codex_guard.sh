#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail() {
  echo "ERROR: $1" >&2
  exit 1
}

has_rg() {
  command -v rg >/dev/null 2>&1
}

search_file() {
  local pattern="$1"
  local file="$2"
  if has_rg; then
    rg -q "$pattern" "$file"
  else
    grep -qE "$pattern" "$file"
  fi
}

search_paths() {
  local pattern="$1"
  shift
  if has_rg; then
    rg -n "$pattern" "$@"
  else
    grep -nRE "$pattern" "$@"
  fi
}

if [[ ! -f "${root_dir}/CODEX_INSTRUCTIONS.md" ]]; then
  fail "Missing CODEX_INSTRUCTIONS.md"
fi

if [[ ! -f "${root_dir}/README.md" ]]; then
  fail "Missing README.md"
fi

if ! head -n 10 "${root_dir}/README.md" | grep -q "CODEX_INSTRUCTIONS.md"; then
  fail "README.md must reference CODEX_INSTRUCTIONS.md in the first 10 lines"
fi

if [[ ! -f "${root_dir}/.github/PULL_REQUEST_TEMPLATE.md" ]]; then
  fail "Missing .github/PULL_REQUEST_TEMPLATE.md"
fi

if ! search_file "CODEX_INSTRUCTIONS.md" "${root_dir}/.github/PULL_REQUEST_TEMPLATE.md"; then
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
  if search_file "^${dep}\\b" "${root_dir}/Cargo.toml"; then
    fail "Forbidden dependency detected in Cargo.toml: ${dep}"
  fi
  if search_file "\\b${dep}\\b" "${root_dir}/Cargo.toml"; then
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
  if search_paths "${pattern}" "${root_dir}/src" "${root_dir}/examples"; then
    fail "Nondeterministic usage detected: ${pattern}"
  fi
done

echo "codex_guard: OK"
