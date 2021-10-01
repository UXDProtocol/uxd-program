#!/bin/bash

# TO be ran by anchor test, who take care of building and deploying to localnet

# IF RUN WITH `anchor test --detach` the local validator will keep running 
# with all the right stuff set for front end testing

export RUST_LOG=solana_runtime::system_instruction_processor=trace,solana_runtime::message_processor=info,solana_bpf_loader=debug,solana_rbpf=debug

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

# stupid rust doesnt handle sigpipe
spl-token create-token $SOLANA_NET > /tmp/hana-spl-mint
COIN_MINT=$(head -1 /tmp/hana-spl-mint | cut -d " " -f 3)

spl-token create-account $SOLANA_NET "$COIN_MINT"
spl-token mint $SOLANA_NET "$COIN_MINT" 100

export COIN_MINT=$COIN_MINT
export NETWORK=$NETWORK

# Deploy data to the oracle -- Do once if using devnet
# anchor deploy --provider.cluster devnet
# if fails `solana deploy ~/Development/UXD/solana-usds/target/deploy/oracle.so tmp.json`

node app/oracle.js -v

# Run tests
npx mocha -t 50_000 app/index.js -v

npx mocha -t 50_000 app/apiTest.js -v

# npx ts-mocha -p ./tsconfig.json -t 100000 test/smb_mp.ts


