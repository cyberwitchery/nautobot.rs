#!/usr/bin/env bash
set -euo pipefail

root_dir=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$root_dir"

version=$(rg -m 1 '^version = ' Cargo.toml | sed -E 's/version = "([^"]+)"/\1/')

echo "release version: ${version}"

if git status --porcelain | rg . >/dev/null 2>&1; then
  echo "warning: git working tree is dirty"
fi

echo "running fmt"
cargo fmt --all

echo "running clippy"
cargo clippy --all-targets --all-features

echo "running tests"
cargo test

if [[ "${SKIP_COVERAGE:-}" != "1" ]]; then
  echo "running coverage"
  cargo llvm-cov --workspace --all-features --ignore-filename-regex 'crates/nautobot-openapi|smoke_local.rs'
else
  echo "skipping coverage (SKIP_COVERAGE=1)"
fi

echo "building release"
cargo build --release

echo "packaging nautobot-openapi"
cargo package -p nautobot-openapi

echo "packaging nautobot (no-verify to allow local path dependency)"
cargo package -p nautobot --no-verify

echo "packaging nautobot-cli (no-verify to allow local path dependency)"
cargo package -p nautobot-cli --no-verify

echo "done"
echo "next: publish in order:"
echo "  cargo publish -p nautobot-openapi"
echo "  cargo publish -p nautobot"
echo "  cargo publish -p nautobot-cli"
