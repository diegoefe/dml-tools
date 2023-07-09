#!/usr/bin/env bash
set -e

echo "Running rust tests"
cargo test -- --nocapture

# examples
cargo run --example gen-dmls
