#!/bin/bash
set -euxo pipefail

cd /home/hana/work/soteria/solana-usds

anchor build --program-name depository
anchor build --program-name controller

anchor deploy --program-name depository
anchor deploy --program-name controller

# stupid rust doesnt handle sigpip
spl-token create-token > /tmp/hana-spl-mint
COIN_MINT=$(head -1 /tmp/hana-spl-mint | cut -d " " -f 3)

spl-token create-account "$COIN_MINT"
spl-token mint "$COIN_MINT" 100

node app/index.js "$COIN_MINT"
