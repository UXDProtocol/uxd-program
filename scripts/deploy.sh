#!/bin/bash
set -euo pipefail

usage() { echo "USAGE: deploy.sh [-u <devnet>] [-v]"; exit 1; }

NETWORK=
while getopts ':u:v' opt; do
    case "${opt}" in
        u) NETWORK="$OPTARG" ;;
        v) set -x ;;
        *) usage
    esac
done

if [ -n "$NETWORK" ]; then
    ANCHOR_NET="--provider.cluster $NETWORK"
    SOLANA_NET="-u $NETWORK"
else
    ANCHOR_NET=
    SOLANA_NET=
fi

anchor build --program-name depository
anchor build --program-name controller

anchor deploy --program-name depository $ANCHOR_NET
anchor deploy --program-name controller $ANCHOR_NET

# stupid rust doesnt handle sigpipe
spl-token create-token $SOLANA_NET > /tmp/hana-spl-mint
COIN_MINT=$(head -1 /tmp/hana-spl-mint | cut -d " " -f 3)

spl-token create-account $SOLANA_NET "$COIN_MINT"
spl-token mint $SOLANA_NET "$COIN_MINT" 100

export COIN_MINT=$COIN_MINT
export NETWORK=$NETWORK

node app/index.js
