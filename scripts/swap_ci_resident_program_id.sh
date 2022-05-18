#!/bin/sh

# The CI uses a constant program 55NneSZjuFv6cVDQxYKZ1UF99JoximnzP9aY65fJ4JT9
# This script swaps program id before running the jobs related to it

# Get current keypair's Pubkey
OLD_PUBKEY=`solana-keygen pubkey ./target/deploy/uxd-keypair.json`
echo $OLD_PUBKEY

# Get the CI Resident program keypair's Pubkey
CI_RESIDENT_PROGRAM_PUBKEY=`solana-keygen pubkey ./target/deploy/ci-resident-keypair.json`
echo $CI_RESIDENT_PROGRAM_PUBKEY

# Replace
sed -i.bak "s/$OLD_PUBKEY/$CI_RESIDENT_PROGRAM_PUBKEY/g" ./Anchor.toml
sed -i.bak "s/$OLD_PUBKEY/$CI_RESIDENT_PROGRAM_PUBKEY/g" ./programs/uxd/src/lib.rs
sed -i.bak "s/$OLD_PUBKEY/$CI_RESIDENT_PROGRAM_PUBKEY/g" ./target/idl/uxd.json
sed -i.bak "s/jsonIdl\[\"metadata\"\]\[\"address\"\]/\"$CI_RESIDENT_PROGRAM_PUBKEY\"/g" ./tests/constants.ts

