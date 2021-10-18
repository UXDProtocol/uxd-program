#!/bin/bash

# clone_account:
#   "3m1y5h2uv7EQL3KaJZehvAJa4yDNvgc5yAdL9KPMKwvk" #: BTC/USD Pyth Account
#   "HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J" #: BTC/USD Pyth Price Account
#   "3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E" #: SOL/USD Pyth Account
#   "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix" #: SOL/USD Pyth Price Account

 # https://api.devnet.solana.com
# Reset then configure the local validator to mimi the mango accounts on devnet
# echo "[Setting up local validator with --cloned accounts...]"
solana-test-validator \
 --reset                                                `: # Reset the test-ledger` \
 --url https://mango.devnet.rpcpool.com                 `: # Cluster from which the below program will be cloned` \
 --clone TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA    `: # token program` \
 --clone 8mFQbdXsFXt3R3cu3oSNS3bDZRwJRP18vyzd9J278J9z   `: # cache `\
 --clone ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL   `: # assoc token program` \
 --clone aca3VWxwBeu8FTZowJ9hfSKGzntjX68EXh1N9xpE1PC    `: # Acam wallet` \
 --clone 9KBGW5bbfdLA4zKAXPETwjR5Ub7BR2Zh9gKHsPnrUkUg   `: ## BTC assoc token acc` \
 --clone 7CZfVDqqYueBujgvqxwDp7Y29eoe5zcgdcqvsUW5bcqP   `: ## USDC assoc token acc` \
 --clone 4uv1EwgZNa91tnBbr49KrQmpb29mp95e9ATh77tgxr6B   `: ## WSOL assoc token acc` \
 --clone Eyh77zP5b7arPtPgpnCT8vsGmq9p5Z9HHnBSeQLnAFQi   `: # Test user wallet` \
 --clone C2CmwB1DBgaKUKHUNi9Ua55wKSH12W95wJEe8yMbSzCN   `: ## BTC assoc token acc` \
 --clone gq25cMCGNb9VNquJsqTYNpipUjZbVB6FktQiENvnq7q    `: ## USDC assoc token acc` \
 --clone 92jBUBivpzTu5EEtdVgKXXwmEhfqFMsg91z2RRYyobFQ   `: ## WSOL assoc token acc` \
 --clone 54PcMYTAZd8uRaYyb3Cwgctcfc1LchGMaqVrmxgr3yVs   `: # mango fees vault` \
 --clone 8FRFC6MoGGkMFQwngccyu69VnYbzykGeez7ignHVAFSN   `: # USDC mango test TOKEN MINT followed by mango root/node` \
 --clone HUBX4iwWEUK5VrXXXcB7uhuKrfT4fpu2T9iZbg712JrN \
 --clone J2Lmnc1e4frMnBEJARPoHtfpcohLfN67HdK1inXjTFSM \
 --clone 3UNBZ6o52WTWwjac2kPUb4FyodhU1vFkRJheu1Sh2TvU   `: # BTC TOKEN MINT followed by mango root/node`\
 --clone BeEoyDq1v2DYJCoXDQAJKfmrsoRRvfmV856f2ijkXbtp \
 --clone 4X3nP921qyh6BKJSAohKGNCykSXahFFwg1LxtC993Fai \
 --clone Bb9bsTQa1bGEtQ5KagGkvSHyuLqDWumFUcRqFusFNJWC   `: # Mango TOKEN MINT followed by mango root/node` \
 --clone CY4nMV9huW5KCYFxWChrmoLwGCsZiXoiREeo2PMrBm5o \
 --clone 6rkPNJTXF37X6Pf5ct5Y6E91PozpZpZNNU1AGATomKjD \
 --clone So11111111111111111111111111111111111111112    `: # WSOL TOKEN MINT followed by mango root/node` \
 --clone 8GC81raaLjhTx3yedctxCJW46qdmmSRybH2s1eFYFFxT \
 --clone 7mYqCavd1K24fnL3oKTpX3YM66W5gfikmVHJWM3nrWKe \
 --clone Cu84KB3tDL6SbFgToHMLYVDJJXdJjenNzSKikeAvzmkA   `: # ETH TOKEN MINT followed by mango root/node` \
 --clone AxwY5sgwSq5Uh8GD6A6ZtSzGd5fqvW2hwgGLLgZ4v2eW \
 --clone 3FPjawEtvrwvwtAetaURTbkkucu9BJofxWZUNPGHJtHg \
 --clone AvtB6w9xboLwA145E221vhof5TddhqsChYcx7Fy3xVMH   `: # SRM TOKEN MINT followed by mango root/node` \
 --clone 73W29LAZog2zSyE1uNYivBW8SMZQX3WBX4qfTMrMJxW2 \
 --clone 9wkpWmkSUSn9fitLhVh12cLbiDa5Bbhf6ZBGmPtcdMqN \
 --clone 3YFQ7UYJ7sNGpXTKBxM3bYLVxKpzVudXAe4gLExh5b3n   `: # RAY TOKEN MINT followed by mango root/node` \
 --clone 49S76N83tSBBozugLtNYrMojFqDb3VvYq4wBB6bcAhfV \
 --clone JBHBTED3ttzk5u3U24txdjBFadm4Dnohb7g2pwcxU4rx \
 --clone DAwBSXe6w9g37wdE2tCrFbho3QHKZi4PjuBytQCULap2   `: # USDT TOKEN MINT followed by mango root/node` \
 --clone 7JTHE8C1kvB4h67RVvhdHjDqHXsWkSeoKcBsHV7wVhu \
 --clone ERkKh9yUKzJ3kkHWhMNd3xGaync11TpzQiDFukEatHEQ \
 --clone Ec2enZyoC4nGpEfu2sUNAa2nUGJHWxoUWYSEJ2hNTWTA   `: # MANGO starts - devnet.2 cluster `\
 --clone 4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA   `: # mangoProgramId `\
 --clone 79Rz9FwjTYSGMbpPBbQMT6kEmqhuGvhqpCPEoALJGmsb   `: # vault? `\
 --clone ARXGB9mirFMC5capL5vkYULf4BEzSfbjdCUDJWaCyqA    `: # vault? `\
 --clone 6eadH6vSsEk5bPiXWPYRq5KRSjohfYd6Ug5HjGbxaWce \
 --clone DESVgJVGajEgKGXhb6XmqDHGz3VjdgP7rEVESBgxmroY   `: # serumProgramId`\
 --clone AvKzLiE8ezzp6kLVw49nq8drycE89NS1RQaBWjBNeUoF \
 --clone 8k7F9Xb36oFJsjpCKpsXvg4cgBRoZtwNTc3EzG5Ttd2o   `: # switchboard oracle` \
 --clone HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J   `: # oracle` \
 --clone EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw   `: # oracle` \
 --clone J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix   `: # oracle` \
 --clone 992moaMQKs32GKZ9dxi8keyM2bUmbrwBZpK4p2K6X5Vs   `: # oracle` \
 --clone 8PugCXTAHLM9kfLSQWe2njE5pzAgUdpPk3Nx5zSm7BD3   `: # oracle` \
 --clone 4GqTjGm686yihQ1m1YdTsSvfm4mNfadv6xskzgCYWNC5   `: # oracle` \
 --clone 4L6YhY8VvUgmqG5MvJkUJATtzB2rFqdrJwQCmFLv4Jzy   `: # oracle` \
 --clone BLArYBCUYhdWiY8PCUTpvFE21iaJq85dvxLk9bYMobcU   `: # oracle` \
 --clone 6vivTRs5ZPeeXbjo7dfburfaYDWoXjBtdtuYgQRuGfu    `: # oracle` \
 --clone 38xoQ4oeJCBrcVvca2cGk7iV1dAfrmTR1kmhSCJQ8Jto   `: # oracle` \
 --clone FHQtNjRHA9U5ahrH7mWky3gamouhesyQ5QvpeGKrTh2z   `: # BTC Perp market `\
 --clone F1Dcnq6F8NXR3gXADdsYqrXYBUUwoT7pfCtRuQWSyQFd   `: ## bidsKey `\
 --clone BFEBZsLYmEhj4quWDRKbyMKhW1Q9c7gu3LqsnipNGTVn   `: ## asksKey `\
 --clone Bu17U2YdBM9gRrqQ1zD6MpngQBb71RRAAn8dbxoFDSkU   `: ## eventsKey `\
 --clone 58vac8i9QXStG1hpaa4ouwE1X7ngeDjY9oY7R15hcbKJ   `: # SOL Perp market `\
 --clone 7HRgm8iXEDx2TmSETo3Lq9SXkF954HMVKNiq8t5sKvQS   `: ## bidsKey `\
 --clone 4oNxXQv1Rx3h7aNWjhTs3PWBoXdoPZjCaikSThV4yGb8   `: ## asksKey `\
 --clone CZ5MCRvkN38d5pnZDDEEyMiED3drgDUVpEUjkuJq31Kf   `: ## eventsKey `
#  > /dev/null 2>&1 &

# sleep 10

# echo "[Starting anchor tests]"
# anchor test --skip-local-validator

# do it manually for now
# Kill it once it's ran,configuration will persist in the local `./test-ledger/` folder
# echo "[Killing the solana test-validator]"
# pgrep solana-test-validator | xargs kill
 
# we could use these instead of having the Pyth/TestWritter, but at the same time the other one allow for more control and we can test with any price
# --clone HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J \ # BTC/USD price account pyth
# --clone J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix  # SOL/USD price account pyth


