#!/bin/bash

###############################################################################
# One script to run them all
# IF RUN WITH `anchor test --detach` the local validator will keep running
# there is another option in 1.17.0 where you can just setup a local env without 
# runing test.
###############################################################################

export RUST_LOG=solana_runtime::system_instruction_processor=trace,solana_runtime::message_processor=info,solana_bpf_loader=debug,solana_rbpf=debug

set -euo pipefail


# Cluster parameter. ANCHOR_PROVIDER_URL is defined when running anchor
###############################################################################

if [ -n "$ANCHOR_PROVIDER_URL" ]; then
    ANCHOR_NET="--provider.cluster $ANCHOR_PROVIDER_URL"
    SOLANA_NET="-u $ANCHOR_PROVIDER_URL"
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
export NETWORK=$ANCHOR_PROVIDER_URL

echo $ANCHOR_PROVIDER_URL


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

## Here is the new version that should replace this script

npx ts-mocha -p ./tsconfig.json -t 100000 tests/test_*.ts

# stash
###############################################################################
# npx ts-mocha -p ./tsconfig.json -t 100000 test/smb_mp.ts

# npx mocha -t 50_000 app/index.js -v

# npx mocha -t 50_000 app/apiTest.js -v