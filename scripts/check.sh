#!/usr/bin/env bash
# Single quality gate: run locally and by CI. Fails on the first failing step.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

step() { printf '\n==> %s\n' "$*"; }

if ! cargo deny --version >/dev/null 2>&1; then
  echo "error: cargo-deny is not installed." >&2
  echo "install it with: cargo install cargo-deny --version 0.19.0 --locked" >&2
  exit 1
fi

step "cargo fmt --check"
cargo fmt --check

step "cargo clippy --all-targets -- -D warnings"
cargo clippy --all-targets -- -D warnings

step "cargo test"
cargo test

step "cargo doc --no-deps (warnings denied)"
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

step "cargo deny check"
cargo deny check

printf '\nAll checks passed.\n'
