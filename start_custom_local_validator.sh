#!/bin/bash



 # https://api.devnet.solana.com
# Reset then configure the local validator to mimi the mango accounts on devnet
# echo "[Setting up local validator with --cloned accounts...]"
solana-test-validator \
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
 --clone 8k7F9Xb36oFJsjpCKpsXvg4cgBRoZtwNTc3EzG5Ttd2o   `: # ??? these account were needed but idk wtf mango devnet use them for` \
 --clone 4GqTjGm686yihQ1m1YdTsSvfm4mNfadv6xskzgCYWNC5   `: # oracle xau pyth?` \
 --clone 8PugCXTAHLM9kfLSQWe2njE5pzAgUdpPk3Nx5zSm7BD3   `: # ? luna pyth?` \
 --clone 4L6YhY8VvUgmqG5MvJkUJATtzB2rFqdrJwQCmFLv4Jzy   `: # ? doge pyth?` \
 --clone BLArYBCUYhdWiY8PCUTpvFE21iaJq85dvxLk9bYMobcU   `: # ? btc pyth?` \
 --clone 6vivTRs5ZPeeXbjo7dfburfaYDWoXjBtdtuYgQRuGfu    `: # ? ftt pyth?` \
 --clone 3m1y5h2uv7EQL3KaJZehvAJa4yDNvgc5yAdL9KPMKwvk   `: #  BTC/USD Pyth Account`\
 --clone HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J   `: #: BTC/USD Pyth Price Account`\
 --clone 3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E   `: #: SOL/USD Pyth Account`\
 --clone J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix   `: #: SOL/USD Pyth Price Account`\
 --clone EssaQC37YW2LVXTsEVjijNte3FTUw21MJBkZYHDdyakc   `: #: mngo/USD Pyth Account`\
 --clone DCNw5mwZgjfTcoNsSZWUiXqU61ushNvr3JRQJRi1Nf95   `: #: mngo/USD Pyth Price Account`\
 --clone 2ciUuGZiee5macAMeQ7bHGTJtwcYTgnt6jdmQnnKZrfu   `: #: eth/USD Pyth Account`\
 --clone EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw   `: #: eth/USD Pyth Price Account`\
 --clone 6NpdXrQEpmDZ3jZKmM2rhdmkd3H6QAk23j2x8bkXcHKA   `: #: usdc/USD Pyth Account`\
 --clone 5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7   `: #: usdc/USD Pyth Price Account`\
 --clone 6MEwdxe4g1NeAF9u6KDG14anJpFsVEa2cvr5H6iriFZ8   `: #: srm/USD Pyth Account`\
 --clone 992moaMQKs32GKZ9dxi8keyM2bUmbrwBZpK4p2K6X5Vs   `: #: srm/USD Pyth Price Account`\
 --clone 3BtxtRxitVDcsd7pPUWUnFm9KvmNDy9usS4gE6pUFhpH   `: #: ray/USD Pyth Account`\
 --clone EhgAdTrgxi4ZoVZLQx1n93vULucPpiFi2BQtz9RJr1y6   `: #: ray/USD Pyth Price Account`\
 --clone C5wDxND9E61RZ1wZhaSTWkoA8udumaHnoQY6BBsiaVpn   `: #: usdt/USD Pyth Account`\
 --clone 38xoQ4oeJCBrcVvca2cGk7iV1dAfrmTR1kmhSCJQ8Jto   `: #: usdt/USD Pyth Price Account`\
 --clone FHQtNjRHA9U5ahrH7mWky3gamouhesyQ5QvpeGKrTh2z   `: # BTC Perp market `\
 --clone F1Dcnq6F8NXR3gXADdsYqrXYBUUwoT7pfCtRuQWSyQFd   `: ## bidsKey `\
 --clone BFEBZsLYmEhj4quWDRKbyMKhW1Q9c7gu3LqsnipNGTVn   `: ## asksKey `\
 --clone Bu17U2YdBM9gRrqQ1zD6MpngQBb71RRAAn8dbxoFDSkU   `: ## eventsKey `\
 --clone 58vac8i9QXStG1hpaa4ouwE1X7ngeDjY9oY7R15hcbKJ   `: # SOL Perp market `\
 --clone 7HRgm8iXEDx2TmSETo3Lq9SXkF954HMVKNiq8t5sKvQS   `: ## bidsKey `\
 --clone 4oNxXQv1Rx3h7aNWjhTs3PWBoXdoPZjCaikSThV4yGb8   `: ## asksKey `\
 --clone CZ5MCRvkN38d5pnZDDEEyMiED3drgDUVpEUjkuJq31Kf   `: ## eventsKey `\
 --clone 8W8Hrj16TZhM4RrFzHBuyWGbh396ig3hJtLPJRGmxPVG   `: # MNGO/USDC Spot market `\
 --clone Diynh714TQsx3qP2bsUuKZk8P31UtKWS9xU6jAZ7A97q   `: ## bidsKey `\
 --clone EhgGeUyv42vZkfWTi2Bxk53cUEW9WWDWrt3qURo8Jfzm   `: ## asksKey `\
 --clone 5vE1a72aw1Hi6JR8sK6ny9mRetWExkryYVxPFQ7zNGY2   `: ## eventsKey `\
 --clone DW83EpHFywBxCHmyARxwj3nzxJd7MUdSeznmrdzZKNZB   `: # BTC/USDC Spot market `\
 --clone PuDcnQDEpoR3WwVAi8PqnHJxHbVEwiusM4PnyHEykFT   `: ## bidsKey `\
 --clone 998DHpQmViDq67vMFKYYXgaHs3CJ5YHEBQSoiwxCjsCW   `: ## asksKey `\
 --clone CQxwLPMoqAwi5wcfkULzF6Fwh7cf4Aiz8tR6DY4NNCN1   `: ## eventsKey `\
 --clone BkAraCyL9TTLbeMY3L1VWrPcv32DvSi5QDDQjik1J6Ac   `: # ETH/USDC Spot market `\
 --clone ETf3PZi9VaBsfpMU5e3SAn4SMjkaM6tyrn2Td9N2kSRx   `: ## bidsKey `\
 --clone 3pfYeG2GKSh8SSZJEEwjYqgaHwYkq5vvSDET2M33nQAf   `: ## asksKey `\
 --clone F43gimmdvBPQoGA4eDxt2N2ooiYWHvQ8pEATrtsArKuC   `: ## eventsKey `\
 --clone 5xWpt56U1NCuHoAEtpLeUrQcxDkEpNfScjfLFaRzLPgR   `: # SOL/USDC Spot market `\
 --clone 8ezpneRznTJNZWFSLeQvtPCagpsUVWA7djLSzqp3Hx4p   `: ## bidsKey `\
 --clone 8gJhxSwbLJkDQbqgzbJ6mDvJYnEVWB6NHWEN9oZZkwz7   `: ## asksKey `\
 --clone 48be6VKEq86awgUjfvbKDmEzXr4WNR7hzDxfF6ZPptmd   `: ## eventsKey `\
 --clone 249LDNPLLL29nRq8kjBTg9hKdXMcZf4vK2UvxszZYcuZ   `: # SRM/USDC Spot market `\
 --clone 5p39nxjdx9RXDVjSwTaej9oUgiLHeV8tfUn7PJoJRLgu   `: ## bidsKey `\
 --clone 2sxkmiwvaMNtuh7eFQPMycJHrwrcJixDASXQFJx79y6C   `: ## asksKey `\
 --clone F66CSYP3TgxGjVongBJ7Cjbiq9j267Keos7cUUUbBZx7   `: ## eventsKey `\
 --clone 5xhm43GzigfEh8XAo5PwgoKK3gFkRr2PUgzWAmLzUTv2   `: # RAY/USDC Spot market `\
 --clone BVDy8YmnbVtfidu8N5YJBDoHXf7vn5B6xnsV6ZLFnFdD   `: ## bidsKey `\
 --clone 4QpxvtDNetYt4pbC8Ng66i6BZdkJEEtSux8HVdGKZbxh   `: ## asksKey `\
 --clone DgojAawYqQqp4Wn9RwahP6yXMNGXsAtBfnoLNqNaWeLy   `: ## eventsKey `\
 --clone E7ch7T7v4DTHcc2YF6ioQow4UPfubbSdpgYqyxoEhiMu   `: # USDT/USDC Spot market `\
 --clone ELwx9pggHdz9CKDpnyCg6L1b8U67WPGsQ4TTbNsLjZJc   `: ## bidsKey `\
 --clone Gr9rsX5uGCTDbhSPzCfrzufSt1mTCggSJAgPfwhBBX1r   `: ## asksKey `\
 --clone H1gJZngRXUtj7N91xnnydC39XqmbU8d2jQZxwqSf21jX   `: ## eventsKey ` \
 --reset                                                `: # Reset the test-ledger -- Put it at the end to ensure there is no silent error above`

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


