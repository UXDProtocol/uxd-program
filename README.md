# UXD-Program

## Installation

```Zsh
$> yarn
```

Recommended to use <https://github.com/mozilla/sccache> to build faster, follow install instruction there.

The project uses a few line of optimisation for building taken from the Discord, but that need to be investigated further. (See the workspace cargo.toml)

## Running tests

Running rust unit tests :

```Zsh
$> cargo test --tests 
```

Running integration test in JS from the tests folder :

```Zsh
$> anchor test 
```

But usually you want a full clean test env, with new Program state, and depositories, new mango accounts etc.

To do so the easiest is to redeploy the whole thing and work with new Accounts (we do test on Devnet cause we need the mango stack, and doing so on localnet, although possible is tedious).

```Zsh
# This will override the current deployment key in the target/deploy folder, it's fine this is not used anywhere never except for testing
$> solana-keygen new -o ./target/deploy/uxd-keypair.json --force --no-bip39-passphrase
```

Press enter for no password and you'r good.

Once you ran this, you need to take the pubkey from the output `pubkey: G8QatVyH14hwT6h8Q6Ld5q9D1CbivErcf6syzukREFs3`
Replace the program adress in anchor and the program doing so :

```Rust
// In lib.rs this line at the beggining :
solana_program::declare_id!("G8QatVyH14hwT6h8Q6Ld5q9D1CbivErcf6syzukREFs3");
```

```Yaml
# In Anchor.toml this line :
[programs.localnet]
uxd = "G8QatVyH14hwT6h8Q6Ld5q9D1CbivErcf6syzukREFs3"
```

You'r then good to go to run `anchor test --skip-local-validator` (As the validator is devnet in our case)

You can then rerun this as many time as you want, but if you want a clean slate, just repro the steps above.

## Deployment

### Between dev and prod

- change de mango program ID in anchor_mango.rs
- change the program id in lib.rs and anchor.toml

```Zsh
$> anchor build
$> solana program deploy ...
```

_____

## Codebase org

The program is contained in `programs/uxd/`.
Its instructions are in `programs/uxd/src/instructions/`.

The project follows the code org as done in [Jet protocol](https://github.com/jet-lab/jet-v1) codebase.

The project uses `Anchor` IDL for safety and readability (over the finest performance tunning).

The project relies on `Mango Markets` [program](https://github.com/blockworks-foundation/mango-v3), a decentralised derivative exchange platform build on Solana, and controlled by a DAO.

This program contains 2 set of instructions, one permissionned and one permissionless

## Program Architecture

![uxd schema](uxd.jpg)

The initial state is initialized through calling `initialize`, from there a mint is created for UXD, the signer is kept as the administrative authority, and that's it.

It owns the UXD Mint currentely (TBD and though about).

```Rust
pub struct Controller {
    pub bump: u8,
    pub redeemable_mint_bump: u8,
    // Version used - for migrations later if needed
    pub version: u8,
    // The account that initialize this struct. Only this account can call permissionned instructions.
    pub authority: Pubkey,
    pub redeemable_mint: Pubkey,
    pub redeemable_mint_decimals: u8,
    //
    // The Mango Depositories registered with this Controller
    pub registered_mango_depositories: [Pubkey; 8], // MAX_REGISTERED_MANGO_DEPOSITORIES - IDL bug with constant...
    pub registered_mango_depositories_count: u8,
    //
    // Progressive roll out and safety ----------
    //
    // The total amount of UXD that can be in circulation, variable
    //  in redeemable Redeemable Native Amount (careful, usually Mint express this in full token, UI amount, u64)
    pub redeemable_global_supply_cap: u128,
    //
    // The max ammount of Redeemable affected by Mint and Redeem operations on `MangoDepository` instances, variable
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
    // Should add padding? or migrate?
}
```

The `authority` (admin) must then register some `Depository`/ies by calling `register_depository`.
One State is tied to many `Depository` accounts, each of them being a vault for a given Collateral Mint.

```Rust
pub struct MangoDepository {
    pub bump: u8,
    pub collateral_passthrough_bump: u8,
    pub insurance_passthrough_bump: u8,
    pub mango_account_bump: u8,
    // Version used - for migrations later if needed
    pub version: u8,
    pub collateral_mint: Pubkey,
    pub collateral_passthrough: Pubkey,
    pub insurance_mint: Pubkey,
    pub insurance_passthrough: Pubkey,
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
    // The amount of delta neutral position that is backing circulating redeemables.
    // Updated after each mint/redeem
    // In Redeemable native units
    pub redeemable_amount_under_management: u128,
    //
    // The amount of taker fee paid in quote while placing perp orders
    pub total_amount_paid_taker_fee: u128,
}
```

Each `Depository` is used to `mint()` and `redeem()` UXD with a specific collateral mint, and to do so each instantiate a Mango PDA that is used to deposit/withdraw collateral to mango and open/close short perp.

## Admin instructions

They setup the UXD account stack and provide management option related to insurance fund and capping.
Only the `authority` can interact with these calls.

### `Initialize`

This initialize the State of the program by instantiating a `Controller`. Called once, the signer becomes the authority, will be done through the DAO.
Only one controller can exist at anytime.

### `RegisterMangoDepository`

Instantiate a new `MangoDepository` PDA for a given collateral mint.
A depository is a vault in charge a Collateral type, the associated mango account and insurance fund.

Eject the mint auth from the program, ending the program. Maybe should be "deinitialize", need to think.

### `RebalanceDepository` (TODO - Planned post release as the first capped release will be protected from liquidation by the insurance in the event of overweighted positions)

Rebalance the health of one repository.
Short Perp PNL will change over time. When it does, other users can settle match us (forcing the update of our balance, as this unsettle PnL is virtual, i.e. we don't pay interests on it)

When settled on a negative PnL, account's USDC balance will become negative, effectively borrowing fund at the current rate.
We don't want this because that cost us.
At the same time, if we settle a positive PnL, the account USDC ba;ance become positive, effectively lending fund at the current rate and accruing interests (We want that obviously)

The strategy here of this rebalance call would be to resize the long and short positions to account for the unsettled negative PnL if any. (resize is a reduce in that case)
By doing so, we close a given short perp, hence we are overcollateralized in the collateral mint.
We then sell this amount of collateral at market price, and that put us a positive balance of USDC, that accrue interest.

THIS IS CURRENTLY PENDING due to the fact that it need to be done in a single atomic IX and that's impossible within 200k computing units. In Q1 2022 Solana will implement Adress map (accepted proposal).
At that point this will be possible and we will be able to raise the hard cap or redeemables.

### `RebalanceAllDepositories` (TODO - In the future to balance collateral + optimize yield)

Would rebalance the amount of collateral available inside each depository so that the pools don't become one sided (everyone deposit sol, then redeem BTC).
Would also allow for yield optimisation depending of the current values.

### `DepositInsuranceToMangoDepository` / `WidthdrawInsuranceFromMangoDepository`

Withdraw need to be specific cause it's PDA own accounts.

This would be used to add USDC to a depository mango account to fund it's insurance fund in UXD case.

### `setRedeemableGlobalSupplyCap`

Change the value of the global supply cap (virtual, not on the mint) for the Redeemable by the Controller.

### `setMangoDepositoriesRedeemableSoftCap`

Change the value of the Mango Depositories operations (Mint/Redeem) Redeemable cap, prevent minting/redeeming over this limit.

## User instructions

They allow end users to mint and redeem UXD, they are permissionless.
Keep in mind all described steps are done during an atomic instruction, one fails and it's all aborted.

### `mint_uxd`

Send collateral to the `Depository` taking that given mint
Estimate how much fill we can get to know how much collateral need to be actually deposited to mango to improve efficiency
Open equivalent perp position (FoK with the provided slippage)
Check that the position was fully openned else abort
Deduct perp opnening fees from the quote amount
Mint equivalent amount of UXD to user (as the value of the short perp - taker fees)

### `redeem_uxd`

User send an amount of UXD to a given `Depository`
We calculate how much collateral that's worth, provided the user slippage and the perp price from Mango
We create a Perp Short Close FoK order in that range
We check that it got filled 100%
We calculate how much USDC value that was, deduct fees
We burn the same value of UXD from what the user sent
We withdraw the collateral amount equivalent to the perpshort that has been closed previously (post taxes calculation)
We send that back to the user (and his remaining UXD are back too, if any)

Interface

- Web app allows user to deposit, withdraw collateral
- Options to mint, redeem UXD and view user account dashboard

There are two different tokens involved in the direct operation of the system (not counting the governance token UXP)
Each Depository issues its own redeemable token as an accounting system that can be ingested by the controller as proof
of collateral without actually directly owning and managing the collateral itself. This allows the attack surface of the
system as a whole to be reduced and creates a segmented risk profile such that all collateral tokens are not put in jeopardy
due to any vulnerability that arises in relation to any specific collateral type. These tokens can referred to as r-tokens
although the everyday user shouldn't have any need to refer to them at all since their primiary use is in the system back end
and the user facing mint function encompasses both the depository facing deposit instruction as well as the controller
facing mint function.

The Controller issues the UXD token itself and has sole priviledge and authority over the UXD mint, which it exercises to
create new uxd obligations proportional to underlying basis trade positions. The same UXD token mint and controller combo
can apply to arbitrarily many Depositories irrespective of the underlying perpetual swap markets, venues, mango groups, etc.
The UXD token is fully fungible and any holder can redeem it at any time for a proportional share of the underlying collateral
value. On a high level, the redemption process consists of buying back swaps equal to the intended redemption value (plus fees)
and releasing the collateral r-token to the user which can be exchanged for the initial collateral back.
