@echo off
@REM cargo install cargo-first

@REM set cmd=cargo
set cmd=cargo first
echo "Running rust tests"
set RUST_TEST_THREADS=1
rem %cmd% test -- --nocapture --test-threads=1
%cmd% test -- --nocapture

rem examples
%cmd% run --example read-file-plus-code-dml-generation
%cmd% run --example read-yaml
%cmd% run --example generate-dmls
