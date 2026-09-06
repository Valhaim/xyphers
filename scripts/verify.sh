#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"
python3 scripts/verify-artifacts.py
cargo test --locked --manifest-path research/code/xypher-thermodynamics-proof/Cargo.toml
cargo run --locked --quiet --manifest-path research/code/xypher-thermodynamics-proof/Cargo.toml | diff -u research/code/xypher-thermodynamics-proof/expected-report.txt -
cargo test --locked --manifest-path research/code/molecular-properties/Cargo.toml
cargo run --locked --quiet --manifest-path research/code/molecular-properties/Cargo.toml --example phosphines -- 5
