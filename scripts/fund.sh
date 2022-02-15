#!/bin/sh

AMOUNT_SOL=15
RECIPIENT_PUBKEY=`solana-keygen pubkey $1`

solana transfer --keypair "./scripts/bank-keypair.json" --commitment processed $RECIPIENT_PUBKEY $AMOUNT_SOL --allow-unfunded-recipient