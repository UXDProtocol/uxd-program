#!/usr/bin/env bash

set -e
cd "$(dirname "$0")/.."

source ./ci/rust-version.sh stable
source ./ci/solana-version.sh
source ./ci/anchor-version.sh

export RUSTFLAGS="-D warnings"
export RUSTBACKTRACE=1

set -x

# Build all C examples
# make -C examples/c # COMMENTED

# Build/test all host crates
cargo +"$rust_stable" build
cargo +"$rust_stable" test -- --nocapture

# run cargo test
cargo +"$rust_stable" test

# run integration tests
anchor test --skip-local-validator

exit 0