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
$> solana-keygen new -o ./target/deploy/uxd-keypair.json --force
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

```Zsh
$> anchor deploy ... TODO
```

_____

## Architecture

The program is contained in `programs/uxd/`.
Its instructions are in `programs/uxd/src/instructions/`.

The project follows the code org as done in [Jet protocol](https://github.com/jet-lab/jet-v1) codebase.

The project uses `Anchor` IDL for safety and readability (over the finest performance tunning).

The project relies on `Mango Markets` [program](https://github.com/blockworks-foundation/mango-v3), a decentralised derivative exchange platform build on Solana, and controlled by a DAO.

This program contains 2 set of instructions, one for administration and one for front facing users.

## Program flow

The initial state is initialized through calling `initialize`, from there a mint is created for UXD, the signer is kept as the administrative authority, and that's it.

It owns the UXD Mint currentely (TBD and though about).

```Rust
pub struct State {
    pub bump: u8,
    pub authority: Pubkey,
    pub uxd_mint: Pubkey,
    pub uxd_mint_bump: u8,
}
```

The `authority` (admin) must then register some `Depository`/ies by calling `register_depository`.
One State is tied to many `Depository` accounts, each of them being a vault for a given Collateral Mint.

```Rust
pub struct Depository {
    pub bump: u8,
    pub collateral_mint: Pubkey,
    pub collateral_passthrough: Pubkey,
    pub collateral_passthrough_bump: u8,
    pub mango_account: Pubkey,
}
```

Each `Depository` is used to `mint()` and `redeem()` UXD with a specific collateral mint, and to do so each instantiate a Mango PDA that is used to deposit/withdraw collateral to mango and open/close short perp.

## Admin instructions

They setup the UXD account stack and provide management option related to insurance fund and capping.
Only the `authority_key` can interact with these calls.

### `Initialize`

This initialize the State of the program. Called once, the signer becomes the authority, should be done by a multisig/DAO.

### `RegisterDepository`

Instantiate a new `Depisitory` PDA for a given collateral mint.
A depository is a vault in charge a Collateral type, the associated mango account and insurance fund.

### `TransferAuthority`

Update the Controller authority

### `TransferMintAuthority`

Eject the mint auth from the program, ending the program. Maybe should be "deinitialize", need to think.


### `SettlePositiveFundingMangoDepository`

part of the IX below

### `RebalanceMangoDepository`

Rebalance the health of a Mango Depository.
Short Perp PNL will change over time. When it does, other users can settle match us (forcing the update of our balance, as this unsettle PnL is virtual, i.e. we don't pay interests on it)

When settled on a negative PnL, account's USDC balance will become negative, effectively borrowing fund at the current rate.
We don't want this because that cost us.
At the same time, if we settle a positive PnL, the account USDC ba;ance become positive, effectively lending fund at the current rate and accruing interests (We want that obviously)

The strategy here of this rebalance call would be to resize the long and short positions to account for the unsettled negative PnL if any. (resize is a reduce in that case)
By doing so, we close some short perp, hence we are overcollateralized in the collateral mint.
We then sell this amount of collateral at market price, and that put us a positive balance of USDC, that accrue interest.

> This balance is technically not ours, cause at any time we could be settled by the algorythm, but we don't want to initiate this action because we gain interests from letting it pending (0% APR borrow and 5-30% APY lending).
> At the same time we are safe and ready to be settled, cause our USDC balance reflect te outstanding negative PnL. (Also it contains the insurance fund, so we should do good accounting of these)

### `RebalanceAllDepositories` (Todo)

Would rebalance the amount of collateral available inside each depository so that the pools don't become one sided (everyone deposit sol, then redeem BTC).
Would also allow for yield optimisation depending of the current values.

### `DepositInsurance` / `WidthdrawInsurance` (Todo)

Deposit can just use the default Mango deposit. But that's better to use a interface doing it all through the program for coherence.

Withdraw need to be specific cause it's PDA own accounts.

This would be used to add USDC to a depository mango account to fund it's insurance fund.

### `setUXDHardCap` (Todo)

This would be to edit a hard cap of total UXP minted, past this minting would be unavailable.

### `Freeze` (Todo? No like)

This would prevent all Minting and Redeem in case of issue (Centralized AF, I don't want that but I think we mentionned it once?)

## User instructions

They allow end users to mint and redeem UXD, they are permissionless.
Keep in mind all described steps are done during an atomic instruction, one fails and it's all abort.

### `mint_uxd`

Send collateral to the `Depository` taking that given mint
Open equivalent perp position (FoK with the provided slippage)
Check that the position was fully openned else abort
Deduct perp opnening fees
Mint equivalent amount of UXD to user (as the value of the short perp)

### `redeem_uxd`

User send an amount of UXD to a given `Depository`
We calculate how much collateral that's worth, provided the user slippage and the perp price from Mango
We create a Perp Short Close FoK order in that range
We check that it got filled 100%
We calculate how much USDC value that was, deduct fees
We burn the same value of UXD from what the user sent
We withdraw the collateral amount equivalent to the perpshort that has been closed previously (post taxes calculation)
We send that back to the user (and his remaining UXD are back too, if any)

_____

## how it work (Deprecated from Hana/Patrick - The R tokens are gone, thoughs? we removed them with Hana already)

Implementation of UXD token on solana

The UXD contract system consists of 2 different classes, the depository which is the input/output point for outside funds
and the Controller, which manages the deposited funds by calculating positions, distributing funds to external derivatives
platforms, establishing and rebalancing derivatives positions, and closing out positions in the event of withdrawals.

Both components are permissionless to the end user but require an authority account to initialize them and setup the relational
hierarchy of depositories and controller. However the authority account acts only to establish the initial trust relationship
between components and does not at any point have access to user funds.

Depository

- Accepts user funds and issues redeemable tokens
- One Depository per collateral type
- Must be matched to a perpetual swap market for that same collateral token
- Multiple Depositories can operate in concert with one controller

Controller

- Interacts with user funds via depositories and swap venues
- Exposes Mint and Redeem instructions to users
- Does Not directly hold user funds or control withdrawals, but does handle the operations that underlie them.
- Has sole permissions to control UXD token mint

Interface

- Web app allows user to deposit, withdraw collateral
- Options to mint, redeem usdx and view user account dashboard

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
