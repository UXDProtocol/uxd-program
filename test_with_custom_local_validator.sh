#!/bin/bash

# clone_account:
#   "3m1y5h2uv7EQL3KaJZehvAJa4yDNvgc5yAdL9KPMKwvk" #: BTC/USD Pyth Account
#   "HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J" #: BTC/USD Pyth Price Account
#   "3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E" #: SOL/USD Pyth Account
#   "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix" #: SOL/USD Pyth Price Account

 # https://api.devnet.solana.com
# Reset then configure the local validator to mimi the mango accounts on devnet
echo "[Setting up local validator with --cloned accounts...]"
solana-test-validator \
 --reset                                                `: # Reset the test-ledger` \
 --url https://mango.devnet.rpcpool.com                 `: # Cluster from which the below program will be cloned` \
 --clone Ec2enZyoC4nGpEfu2sUNAa2nUGJHWxoUWYSEJ2hNTWTA   `: # MANGO starts - devnet.2 cluster `\
 --clone 4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA   `: # mangoProgramId `\
 --clone 6eadH6vSsEk5bPiXWPYRq5KRSjohfYd6Ug5HjGbxaWce \
 --clone DESVgJVGajEgKGXhb6XmqDHGz3VjdgP7rEVESBgxmroY   `: # serumProgramId`\
 --clone AvKzLiE8ezzp6kLVw49nq8drycE89NS1RQaBWjBNeUoF \
 --clone 3UNBZ6o52WTWwjac2kPUb4FyodhU1vFkRJheu1Sh2TvU   `: # BTC`\
 --clone HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J   `: # BTC oracle `\
 --clone J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix   `: # SOL oracle `\
 --clone FHQtNjRHA9U5ahrH7mWky3gamouhesyQ5QvpeGKrTh2z   `: # BTC Perp market `\
 --clone F1Dcnq6F8NXR3gXADdsYqrXYBUUwoT7pfCtRuQWSyQFd   `: ## bidsKey `\
 --clone BFEBZsLYmEhj4quWDRKbyMKhW1Q9c7gu3LqsnipNGTVn   `: ## asksKey `\
 --clone Bu17U2YdBM9gRrqQ1zD6MpngQBb71RRAAn8dbxoFDSkU   `: ## eventsKey `\
 --clone 58vac8i9QXStG1hpaa4ouwE1X7ngeDjY9oY7R15hcbKJ   `: # SOL Perp market `\
 --clone 7HRgm8iXEDx2TmSETo3Lq9SXkF954HMVKNiq8t5sKvQS   `: ## bidsKey `\
 --clone 4oNxXQv1Rx3h7aNWjhTs3PWBoXdoPZjCaikSThV4yGb8   `: ## asksKey `\
 --clone CZ5MCRvkN38d5pnZDDEEyMiED3drgDUVpEUjkuJq31Kf   `: ## eventsKey ` \
 > /dev/null 2>&1 &

sleep 10

echo "[Starting anchor tests]"
anchor test --skip-local-validator

# do it manually for now
# Kill it once it's ran,configuration will persist in the local `./test-ledger/` folder
echo "[Killing the solana test-validator]"
pgrep solana-test-validator | xargs kill
 
# we could use these instead of having the Pyth/TestWritter, but at the same time the other one allow for more control and we can test with any price
# --clone HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J \ # BTC/USD price account pyth
# --clone J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix  # SOL/USD price account pyth


