#!/usr/bin/env bash
set -e

echo "Running rust tests"
# export RUST_TEST_THREADS=1
cargo test -- --nocapture --test-threads=1

# examples
cargo run --example gen-dmls
