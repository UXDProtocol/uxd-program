# UXD-Program

The actual deployed state of each contract should live in a protected master branch. The latest master should always reflect the code deployed to all relevant chains

It currently sits at:

<!-- ### Solana -->
- mainnet-beta `UXD8m9cvwk4RcSxnX2HZ9VudQCEeDH6fRnB4CAP57Dr`
- devnet `CXzEE9YjFgw3Ggz2r1oLHqJTd4mpzFWRKm9fioTjpk45` (Used by CI, this address should be update accordingly in ci files)

## Running tests

Running rust unit tests :

```Zsh
$> cargo test && cargo test-bpf 
```

Running E2E test in TS from the tests folder :

```Zsh
$> GROUP=devnet.2 CLUSTER=devnet KEYPAIR=$(cat /Users/acamill/.config/solana/id.json) yarn keeper # in a https://github.com/blockworks-foundation/mango-client-v3 repo to run the Keeper (cranking)
$> GROUP=devnet.2 CLUSTER=devnet KEYPAIR=$(cat /Users/acamill/.config/solana/id.json) MANGO_ACCOUNT_PUBKEY=8fbL4156uoVYYyY9cvA6hVBBTdui9356tdKmFbkC6t6w MARKET=SOL yarn mm # in a https://github.com/blockworks-foundation/mango-client-v3 repo to run the Market Making bot
$> GROUP=devnet.2 CLUSTER=devnet KEYPAIR=$(cat /Users/acamill/.config/solana/id.json) MANGO_ACCOUNT_PUBKEY=8fbL4156uoVYYyY9cvA6hVBBTdui9356tdKmFbkC6t6w MARKET=BTC yarn mm
$> GROUP=devnet.2 CLUSTER=devnet KEYPAIR=$(cat /Users/acamill/.config/solana/id.json) MANGO_ACCOUNT_PUBKEY=8fbL4156uoVYYyY9cvA6hVBBTdui9356tdKmFbkC6t6w MARKET=ETH yarn mm
```

```Zsh
$> anchor test # Then this will build, deploy and run the tests
```

The keeper is mandatory to run on Devnet, as there might not be another one running. This is the process settling events and cranking mango state in between order, without it tests won't work.
The Market Marking bots are also mandatory to keep a semi consistent order book.

Usually you want with a clean test env, with new Program state, and depositories, new mango accounts etc.

To reset the program ID in the project, you can run :

```Zsh
$> ./scripts/reset_program_id.sh # Will generate a new Pubkey for deploying a fresh program, and replace the ref in lib.rs, anchor.toml and the IDL 
```

You'r then good to go to run `anchor test --skip-local-validator` (As the validator is devnet in our case)

You can then rerun this as many time as you want, but if you want a clean slate, just reset the program_id with the script.

## Testing strategy with CI

There are a few script in the test folder with following the `test_ci_*.ts`, these are related to the github workflow.
It's quite unstable to test on devnet with typescript, and expect MangoMarkets order book to be consistent, but it somehow works.

The CI strategy is to upgrade the devnet program, run the market making bots (these are the two pre condition done in parallel), then it start 4 testing suites in parallel for each Collateral (for now on mango and later on more with new Dexes).

The CI also runs cargo fmt, clippy, test and test-bpf.

Cargo audit and Soteria (automated auditing tool) are run on main branch merges.

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

_____

## Codebase org

The program is contained in `programs/uxd/`.
Its instructions are in `programs/uxd/src/instructions/`.

The project follows the code org as done in [Jet protocol](https://github.com/jet-lab/jet-v1) codebase.

The project uses `Anchor` IDL for safety and readability (over the finest performance tunning).

The project relies on `Mango Markets` [program](https://github.com/blockworks-foundation/mango-v3), a decentralized derivative exchange platform build on Solana, and controlled by a DAO.

This program contains 2 set of instructions, one permissionned and one permissionless

## Program Architecture

![uxd schema](uxd.jpg)

The initial state is initialized through calling `initializeController`, from there a mint is created for Redeemable, the signer is kept as the administrative authority, and that's it.

It owns the Redeemable Mint currently. In the future there could be added instruction to transfer Authority/Mint to another program due to migration, if needs be.

```Rust
#[account]
#[derive(Default)]
pub struct Controller {
    pub bump: u8,
    pub redeemable_mint_bump: u8,
    // Version used
    pub version: u8,
    // The account with authority over the UXD stack
    pub authority: Pubkey,
    pub redeemable_mint: Pubkey,
    pub redeemable_mint_decimals: u8,
    //
    // The Mango Depositories registered with this Controller
    pub registered_mango_depositories: [Pubkey; 8], //  - IDL bug with constant, so hard 8 literal. -- Still not working in 0.20.0 although it should
    pub registered_mango_depositories_count: u8,
    //
    // Progressive roll out and safety ----------
    //
    // The total amount of UXD that can be in circulation, variable
    //  in redeemable Redeemable Native Amount (careful, usually Mint express this in full token, UI amount, u64)
    pub redeemable_global_supply_cap: u128,
    //
    // The max amount of Redeemable affected by Mint and Redeem operations on `MangoDepository` instances, variable
    //  in redeemable Redeemable Native Amount
    pub mango_depositories_redeemable_soft_cap: u64,
    //
    // Accounting -------------------------------
    //
    // The actual circulating supply of Redeemable
    // This should always be equal to the sum of all Depositories' `redeemable_amount_under_management`
    //  in redeemable Redeemable Native Amount
    pub redeemable_circulating_supply: u128,
    //
    // Note : This is the last thing I'm working on and I would love some guidance from the audit. Anchor doesn't seems to play nice with padding
    pub _reserved: ControllerPadding,
}
```

The `authority` (admin) must then register some `Depository`/ies by calling `register_depository`.
One State is tied to many `Depository` accounts, each of them being a vault for a given Collateral Mint.

```Rust
#[account]
#[derive(Default)]
pub struct MangoDepository {
    pub bump: u8,
    pub collateral_passthrough_bump: u8,
    pub insurance_passthrough_bump: u8,
    pub mango_account_bump: u8,
    // Version used
    pub version: u8,
    pub collateral_mint: Pubkey,
    pub collateral_mint_decimals: u8,
    pub collateral_passthrough: Pubkey,
    pub insurance_mint: Pubkey,
    pub insurance_passthrough: Pubkey,
    pub insurance_mint_decimals: u8,
    pub mango_account: Pubkey,
    //
    // The Controller instance for which this Depository works for
    pub controller: Pubkey,
    //
    // Accounting -------------------------------
    // Note : To keep track of the in and out of a depository
    //
    // The amount of USDC InsuranceFund deposited/withdrawn by Authority on the underlying Mango Account - The actual amount might be lower/higher depending of funding rate changes
    // In Collateral native units
    pub insurance_amount_deposited: u128,
    //
    // The amount of collateral deposited by users to mint UXD
    // Updated after each mint/redeem
    // In Collateral native units
    pub collateral_amount_deposited: u128,
    //
    // The amount of delta neutral position that is backing circulating redeemable.
    // Updated after each mint/redeem
    // In Redeemable native units
    pub redeemable_amount_under_management: u128,
    //
    // The amount of taker fee paid in quote while placing perp orders
    pub total_amount_paid_taker_fee: u128,
    //
    pub _reserved: u8,
    //
    // This information is shared by all the Depositories, and as such would have been a good
    // candidate for the Controller, but we will lack space in the controller sooner than here.
    //
    // v2 -83 bytes
    pub quote_passthrough_bump: u8,
    pub quote_mint: Pubkey,
    pub quote_passthrough: Pubkey,
    pub quote_mint_decimals: u8,
    //
    // The amount of DN position that has been rebalanced (in quote native units)
    pub total_amount_rebalanced: u128,
    //
    pub _reserved1: MangoDepositoryPadding,
}
```

Each `Depository` is used to `mint()` and `redeem()` Redeemable tokens with a specific collateral mint, and to do so each instantiate a Mango PDA that is used to deposit/withdraw collateral to mango and open/close short perp.

## Admin instructions

They setup the UXD account stack and provide management option related to insurance fund and capping.
Only the `authority` can interact with these calls.

### `Initialize`

This initialize the State of the program by instantiating a `Controller`. Called once, the signer becomes the authority, will be done through the DAO.
Only one controller can exist at anytime.

### `RegisterMangoDepository`

Instantiate a new `MangoDepository` PDA for a given collateral mint.
A depository is a vault in charge a Collateral type, the associated mango account and insurance fund.

### `RebalanceDepository` (TODO - Planned post release as the first capped release will be protected from liquidation by the insurance in the event of overweighed positions)

Rebalance the health of one repository.
Short Perp PNL will change over time. When it does, other users can settle match us (forcing the update of our balance, as this unsettle PnL is virtual, i.e. we don't pay interests on it)

When settled on a negative PnL, account's USDC balance will become negative, effectively borrowing fund at the current rate.
We don't want this because that cost us.
At the same time, if we settle a positive PnL, the account USDC balance become positive, effectively lending fund at the current rate and accruing interests (We want that obviously)

The strategy here of this rebalance call would be to resize the long and short positions to account for the unsettled negative PnL if any. (resize is a reduce in that case)
By doing so, we close a given short perp, hence we are over-collateralized in the collateral mint.
We then sell this amount of collateral at market price, and that put us a positive balance of USDC, that accrue interest.

THIS IS CURRENTLY PENDING due to the fact that it need to be done in a single atomic IX and that's impossible within 200k computing units. In Q1 2022 Solana will implement Address map (accepted proposal).
At that point this will be possible and we will be able to raise the hard cap or redeemable tokens.

### `RebalanceAllDepositories` (TODO - In the future to balance collateral + optimize yield)

Would rebalance the amount of collateral available inside each depository so that the pools don't become one sided (everyone deposit sol, then redeem BTC).
Would also allow for yield optimization depending of the current values.

### `DepositInsuranceToMangoDepository` / `WithdrawInsuranceFromMangoDepository`

Withdraw need to be specific cause it's PDA own accounts.

This would be used to add USDC to a depository mango account to fund it's insurance fund in UXD case.

### `setRedeemableGlobalSupplyCap`

Change the value of the global supply cap (virtual, not on the mint) for the Redeemable by the Controller.

### `setMangoDepositoriesRedeemableSoftCap`

Change the value of the Mango Depositories operations (Mint/Redeem) Redeemable cap, prevent minting/redeeming over this limit.

## User instructions

They allow end users to mint and redeem redeemable tokens, they are permissionless.
Keep in mind all described steps are done during an atomic instruction, one fails and it's all aborted.

### `mint_uxd`

Send collateral to the `Depository` taking that given mint
Estimate how much fill we can get to know how much collateral need to be actually deposited to mango to improve efficiency
Open equivalent perp position (FoK with the provided slippage)
Check that the position was fully opened else abort
Deduct perp opening fees from the quote amount
Mint equivalent amount of UXD to user (as the value of the short perp - taker fees)

### `redeem_uxd`

User send an amount of UXD to a given `Depository`
We calculate how much collateral that's worth, provided the user slippage and the perp price from Mango
We create a Perp Short Close FoK order in that range
We check that it got filled 100%
We calculate how much USDC value that was, deduct fees
We burn the same value of UXD from what the user sent
We withdraw the collateral amount equivalent to the perp short that has been closed previously (post taxes calculation)
We send that back to the user (and his remaining UXD are back too, if any)

Interface

- Web app allows user to deposit, withdraw collateral
- Options to mint, redeem UXD and view user account dashboard

There are two different tokens involved in the direct operation of the system (not counting the governance token UXP)
Each Depository issues its own redeemable token as an accounting system that can be ingested by the controller as proof
of collateral without actually directly owning and managing the collateral itself. This allows the attack surface of the
system as a whole to be reduced and creates a segmented risk profile such that all collateral tokens are not put in jeopardy
due to any vulnerability that arises in relation to any specific collateral type. These tokens can referred to as r-tokens
although the everyday user shouldn't have any need to refer to them at all since their primary use is in the system back end
and the user facing mint function encompasses both the depository facing deposit instruction as well as the controller
facing mint function.

The Controller issues the UXD token itself and has sole privilege and authority over the UXD mint, which it exercises to
create new uxd obligations proportional to underlying basis trade positions. The same UXD token mint and controller combo
can apply to arbitrarily many Depositories irrespective of the underlying perpetual swap markets, venues, mango groups, etc.
The UXD token is fully fungible and any holder can redeem it at any time for a proportional share of the underlying collateral
value. On a high level, the redemption process consists of buying back swaps equal to the intended redemption value (plus fees)
and releasing the collateral r-token to the user which can be exchanged for the initial collateral back.
