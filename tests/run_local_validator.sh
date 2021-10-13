#!/bin/bash

# clone_account:
#   "3m1y5h2uv7EQL3KaJZehvAJa4yDNvgc5yAdL9KPMKwvk" #: BTC/USD Pyth Account
#   "HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J" #: BTC/USD Pyth Price Account
#   "3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E" #: SOL/USD Pyth Account
#   "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix" #: SOL/USD Pyth Price Account

 # https://api.devnet.solana.com
solana-test-validator \
--url "https://mango.devnet.rpcpool.com" \
--clone 5fP7Z7a87ZEVsKr2tQPApdtq83GcTW4kz919R6ou5h5E # mango
# we could use these instead of having the Pyth/TestWritter, but at the same time the other one allow for more control and we can test with any price
# --clone HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J \ # BTC/USD price account pyth
# --clone J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix  # SOL/USD price account pyth



