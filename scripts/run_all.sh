# cargo install cargo-first
#!/usr/bin/env bash
# set -e

cmd="cargo first"
# cmd=cargo
echo "Running rust tests"
export RUST_TEST_THREADS=1
# ${cmd} test -- --nocapture --test-threads=1
${cmd} test -- --nocapture

# examples
${cmd} run --example read-file-plus-code-dml-generation
${cmd} run --example read-yaml
${cmd} run --example generate-dmls
