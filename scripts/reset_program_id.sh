#!/bin/sh

# Get current keypair's Pubkey
OLD_PUBKEY=`solana-keygen pubkey ./target/deploy/uxd-keypair.json`
echo $OLD_PUBKEY

# Reset the keypair
solana-keygen new -o ./target/deploy/uxd-keypair.json --force --no-bip39-passphrase

# Get the new keypair's Pubkey
NEW_PUBKEY=`solana-keygen pubkey ./target/deploy/uxd-keypair.json`
echo $NEW_PUBKEY

# Replace
sed -i.bak "s/$OLD_PUBKEY/$NEW_PUBKEY/g" ./Anchor.toml
sed -i.bak "s/$OLD_PUBKEY/$NEW_PUBKEY/g" ./programs/uxd/src/lib.rs
sed -i.bak "s/$OLD_PUBKEY/$NEW_PUBKEY/g" ./tests/constants.ts
