#!/bin/bash

###############################################################################
# One script to run them all
# IF RUN WITH `anchor test --detach` the local validator will keep running
# there is another option in 1.17.0 where you can just setup a local env without 
# runing test.
###############################################################################

export RUST_LOG=solana_runtime::system_instruction_processor=trace,solana_runtime::message_processor=info,solana_bpf_loader=debug,solana_rbpf=debug

set -euo pipefail


# Cluster parameter. This should be inferred from the current ANCHOR_ENV (TODO)
###############################################################################

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

# Builds handles by anchor
###############################################################################

# Deployments and the targets are handled by anchor - run w/ desired cluster
###############################################################################

# Setup fake BTC - local test env, new each run for fresh tests
###############################################################################
spl-token create-token --decimals 6 $SOLANA_NET | tee /tmp/fakebtc-spl-mint
FAKE_BTC_MINT=$(head -1 /tmp/fakebtc-spl-mint | cut -d " " -f 3)

spl-token create-account $SOLANA_NET "$FAKE_BTC_MINT"
spl-token mint $SOLANA_NET "$FAKE_BTC_MINT" 100

# Export for JS to access
###############################################################################
export FAKE_BTC_MINT=$FAKE_BTC_MINT
export NETWORK=$NETWORK


echo "###############################################################################"
echo "# Running SETUP ORACLES"
echo "###############################################################################\n"

# Setup localnet oracles pulling data from the testnet populated accounts
###############################################################################
node app/setup_oracles.js -v

echo "###############################################################################"
echo "# Running index.js"
echo "###############################################################################\n"

# Run tests
###############################################################################
node app/index.js -v




# stash
###############################################################################
# npx ts-mocha -p ./tsconfig.json -t 100000 test/smb_mp.ts

# npx mocha -t 50_000 app/index.js -v

# npx mocha -t 50_000 app/apiTest.js -v