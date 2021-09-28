#!/bin/bash

set -euxo pipefail

# stupid rust doesnt handle sigpip
solana config get
spl-token create-token > /tmp/spl-mint
COIN_MINT=$(head -1 /tmp/spl-mint | cut -d " " -f 3)

spl-token create-account $COIN_MINT
spl-token mint $COIN_MINT 100

export COIN_MINT=$COIN_MINT


# # deploy oracle on devnet (TMP HanaHack)
# solana airdrop 1 --url https://api.testnet.solana.com --keypair ~/.config/solana/id.json   
# anchor deploy --program-name oracle --provider.cluster devnet --provider.wallet ~/.config/solana/devnet.json 

# Run oracle first to have prices
node app/oracle.js 
# PID_ORACLE = $!
#
# npx mocha -t 50000 app/index.js $COIN_MINT

# node app/index.js "$COIN_MINT"
