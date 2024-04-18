#!/bin/bash

set -e

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
PYTHON=$(which python || which python3)

cargo fmt --check -- --config newline_style=Unix

# Install cargo deny first with: cargo install cargo-deny
cargo deny check --hide-inclusion-graph

# Checks enabled everywhere, including tests, benchmarks
cargo clippy
