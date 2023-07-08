#!/usr/bin/env bash

set -e

echo "Running rust tests"
cargo test -- --nocapture
cargo run --example gen-dmls