#!/usr/bin/env bash
set -e

echo "Running rust tests"
cargo test -- --nocapture

if [ "$1" == "ex" ]
then
    cargo run --example gen-dmls
fi
