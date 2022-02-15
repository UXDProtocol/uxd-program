#!/bin/sh

AMOUNT_SOL=15

solana transfer --keypair "./bank-keypair.json" --commitment processed $1 $AMOUNT_SOL