#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all -- --check
cargo check --workspace --examples --bins --tests
cargo clippy --workspace --examples --bins --tests -- -D warnings
cargo test --workspace
