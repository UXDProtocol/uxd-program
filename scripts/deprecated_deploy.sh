#!/bin/bash
set -euo pipefail

usage() { echo "USAGE: deploy.sh [-u <cluster>] [-v]"; exit 1; }

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

anchor build --program-name controller
anchor build --program-name depository

anchor deploy --program-name controller $ANCHOR_NET | tee /tmp/hana-ctrl
anchor deploy --program-name depository $ANCHOR_NET | tee /tmp/hana-dep1
anchor deploy --program-name depository $ANCHOR_NET | tee /tmp/hana-dep2

CONTROLLER=$(sed -n 's/^Program Id: \([a-zA-Z0-9]\+\)/\1/p' /tmp/hana-ctrl)
BTC_DEPOSITORY=$(sed -n 's/^Program Id: \([a-zA-Z0-9]\+\)/\1/p' /tmp/hana-dep1)
SOL_DEPOSITORY=$(sed -n 's/^Program Id: \([a-zA-Z0-9]\+\)/\1/p' /tmp/hana-dep2)

# stupid rust doesnt handle sigpipe
spl-token create-token --decimals 6 $SOLANA_NET | tee /tmp/hana-spl-mint
COIN_MINT=$(head -1 /tmp/hana-spl-mint | cut -d " " -f 3)

spl-token create-account $SOLANA_NET "$COIN_MINT"
spl-token mint $SOLANA_NET "$COIN_MINT" 100


export COIN_MINT=$COIN_MINT
export NETWORK=$NETWORK
export CONTROLLER=$CONTROLLER
export BTC_DEPOSITORY=$BTC_DEPOSITORY
export SOL_DEPOSITORY=$SOL_DEPOSITORY

node app/index.js
