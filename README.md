# UXD-Program

[![UXD Composability testing](https://github.com/blockworks-foundation/mango-v3/actions/workflows/ci-uxd.yml/badge.svg?branch=main&event=push)](https://github.com/blockworks-foundation/mango-v3/actions/workflows/ci-uxd.yml)
[![Anchor Test](https://github.com/UXDProtocol/uxd-program/actions/workflows/ci-anchor-test.yml/badge.svg?branch=main)](https://github.com/UXDProtocol/uxd-program/actions/workflows/ci-anchor-test.yml)
[![Lint and Test](https://github.com/UXDProtocol/uxd-program/actions/workflows/ci-cargo-lint-test.yml/badge.svg?branch=main)](https://github.com/UXDProtocol/uxd-program/actions/workflows/ci-cargo-lint-test.yml)
[![Soteria Audit](https://github.com/UXDProtocol/uxd-program/actions/workflows/ci-soteria-audit.yml/badge.svg)](https://github.com/UXDProtocol/uxd-program/actions/workflows/ci-soteria-audit.yml)
[![Cargo Audit](https://github.com/UXDProtocol/uxd-program/actions/workflows/ci-cargo-audit.yml/badge.svg?branch=main)](https://github.com/UXDProtocol/uxd-program/actions/workflows/ci-cargo-audit.yml)

The actual deployed state of each contract should live in a protected master branch. The latest master should always reflect the code deployed to all relevant chains

It currently sits at:

<!-- ### Solana -->
- mainnet-beta `UXD8m9cvwk4RcSxnX2HZ9VudQCEeDH6fRnB4CAP57Dr`
- devnet `882VXWftqQ9wsVq99SJqBVsz6tVeBt63jKE9XiwEHDeN` (Public version for front end)
- devnet `55NneSZjuFv6cVDQxYKZ1UF99JoximnzP9aY65fJ4JT9` (Used by CI, this address should be update accordingly in ci files)

_____

## UXDProtocol business knowledge

If you want to learn more about the high level concept of UXDProtocol, the [UXDProtocol Git book](https://docs.uxd.fi/uxdprotocol/) is available.

## Running tests

### Rust unit tests

```Zsh
$> cargo test && cargo build-bpf && cargo test-bpf 
```

### E2E Tests

In order to have the environment ready to host test, the mango market devnet must be running as expected. To do so we must run market making bots and the Keeper, from [MangoClientV3](https://github.com/blockworks-foundation/mango-client-v3) repo or [mango-explorer](https://github.com/blockworks-foundation/mango-explorer/blob/main/docs/MarketmakingOrderChain.md).
Keeper does the cranking (settle orders), and MM bots ensure that the order book is coherent.

When test are failing due to odd reasons, first thing to do is to check the [MangoMarkets V3](https://devnet.mango.markets/?name=SOL-PERP) market state. Verify that the order book is being updated, and run a market making bot.

(deprecated, see <https://github.com/blockworks-foundation/mango-explorer/blob/main/docs/MarketmakingOrderChain.md> for updated way)

A example script to run a maker maker for SOL/BTC/ETH markets on MangoMarketsv3 using mango-explorer is

```Zsh
#!/bin/sh

marketmaker --name "UXD-SOL-MM" --market SOL-PERP --oracle-provider pyth --chain ratios --ratios-spread 0.005 --chain ratios --ratios-position-size 0.02 --chain fixedspread --fixedspread-value 0.1 --order-type LIMIT --pulse-interval 30 --log-level INFO --cluster-name devnet --account 2s2hNn44RTWQsTEkBbpy8ieA8NtLBFQif3Q41BmfPu3a  &

marketmaker --name "UXD-BTC-MM" --market BTC-PERP --oracle-provider pyth --chain ratios --ratios-spread 0.005 --chain ratios --ratios-position-size 0.02 --chain fixedspread --fixedspread-value 0.1 --order-type LIMIT --pulse-interval 30 --log-level INFO --cluster-name devnet --account 2s2hNn44RTWQsTEkBbpy8ieA8NtLBFQif3Q41BmfPu3a  &

marketmaker --name "UXD-ETH-MM" --market ETH-PERP --oracle-provider pyth --chain ratios --ratios-spread 0.005 --chain ratios --ratios-position-size 0.02 --chain fixedspread --fixedspread-value 0.1 --order-type LIMIT --pulse-interval 30 --log-level INFO --cluster-name devnet --account 2s2hNn44RTWQsTEkBbpy8ieA8NtLBFQif3Q41BmfPu3a
```

```Zsh
$> GROUP=devnet.2 CLUSTER=devnet KEYPAIR=$(cat /Users/acamill/.config/solana/id.json) yarn keeper 
$> GROUP=devnet.2 CLUSTER=devnet KEYPAIR=$(cat /Users/acamill/.config/solana/id.json) MANGO_ACCOUNT_PUBKEY=8fbL4156uoVYYyY9cvA6hVBBTdui9356tdKmFbkC6t6w MARKET=SOL yarn mm # in a https://github.com/blockworks-foundation/mango-client-v3 repo to run the Market Making bot
$> GROUP=devnet.2 CLUSTER=devnet KEYPAIR=$(cat /Users/acamill/.config/solana/id.json) MANGO_ACCOUNT_PUBKEY=8fbL4156uoVYYyY9cvA6hVBBTdui9356tdKmFbkC6t6w MARKET=BTC yarn mm
$> GROUP=devnet.2 CLUSTER=devnet KEYPAIR=$(cat /Users/acamill/.config/solana/id.json) MANGO_ACCOUNT_PUBKEY=8fbL4156uoVYYyY9cvA6hVBBTdui9356tdKmFbkC6t6w MARKET=ETH yarn mm
```

Once this is setup, in another terminal you can build, deploy and run the test :

```Zsh
$> ./scripts/reset_program_id.sh # Optional, will reset the program ID in all files where it's needed to start with a clean slate
$> anchor test # Will build, deploy and run the tests
```

If you want to re-run the tests with the already deployed program (without registering changes to the rust code), you can run :

```Zsh
$> anchor test --skip-build --skip-deploy
```

If you made changes to the Rust code, you have to re-run the lengthy :

```Zsh
$> anchor test
```

Loop theses as many time as you want, and if you want a clean slate, just reset the program_id with the script (`./script/reset_program_id.sh`).

_____

## Codebase org

The program (smart contract) is contained in `programs/uxd/`.
Its instructions are in `programs/uxd/src/instructions/`.

The project follows the code org as done in [Jet protocol](https://github.com/jet-lab/jet-v1) codebase.

The project uses `Anchor` for safety, maintainability and readability.

The project relies on `Mango Markets` [program](https://github.com/blockworks-foundation/mango-v3), a decentralized derivative exchange platform build on Solana, and controlled by a DAO.

This program contains 2 set of instructions, one permissionned and one permissionless. Permissionned instruction are called by [our DAO](https://governance.uxd.fi/dao/UXP).

_____

## Testing strategy with CI

There are a few script in the test folder with following the `test_ci_*.ts`, these are related to the github workflow.
It's quite unstable to test on devnet with typescript, and expect MangoMarkets order book to be consistent, but it somehow works.

The CI strategy for E2E :

- use the ci-resident-program (call ./scripts/swap_ci_resident_program.sh)
- use it's upgrade authority stored in `target/deploy/ci-resident-upgrade-authority.json` for deployment
- upgrade program
- run the market making bots
- then starts X testing suites in parallel for each Collateral/Dex whatever case (for now on mango and later on more with new Dexes).

Note that it don't do concurrent run of this workflow, as they test some internal state of the program and would collide.

The CI also runs cargo fmt, clippy, test and test-bpf.

Cargo audit and Soteria (automated auditing tool) are run on main branch merges.

_____

## Deployment and Program Upgrades

By default the program builds with the `development` feature, and the ProgramID for devnet.

Building for mainnet uses `anchor build -- --no-default-features --features production`

The program upgrade are done through our [DAO](https://governance.uxd.fi/dao/UXP).

It required to build for release then to prepare a buffer with :

```Zsh
$> solana program write-buffer  ./target/deploy/uxd.so 
# anchor verify -p uxd <Buffer ID from previous command>  //TODO
$> solana program set-buffer-authority <BufferID> --new-buffer-authority CzZySsi1dRHMitTtNe2P12w3ja2XmfcGgqJBS8ytBhhY
```

![Governance upgrade](dao_program_upgrade.png)