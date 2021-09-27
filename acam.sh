#!/bin/bash

set -euxo pipefail

cd ~/Development/UXD/solana-usds

# stupid rust doesnt handle sigpip
spl-token create-token > /tmp/spl-mint
COIN_MINT=$(head -1 /tmp/spl-mint | cut -d " " -f 3)

spl-token create-account "$COIN_MINT"
spl-token mint "$COIN_MINT" 100

node app/index.js "$COIN_MINT"
