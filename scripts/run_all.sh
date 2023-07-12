#!/usr/bin/env bash
set -e

echo "Running rust tests"
# export RUST_TEST_THREADS=1
# cargo test -- --nocapture --test-threads=1
cargo test -- --nocapture

# examples
cargo run --example read-file-plus-code-dml-generation
cargo run --example read-yaml
cargo run --example generate-dmls
