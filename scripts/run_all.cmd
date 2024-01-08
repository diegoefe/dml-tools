@echo off

echo "Running rust tests"
set RUST_TEST_THREADS=1
rem cargo test -- --nocapture --test-threads=1
cargo test -- --nocapture

rem examples
cargo run --example read-file-plus-code-dml-generation
cargo run --example read-yaml
cargo run --example generate-dmls
