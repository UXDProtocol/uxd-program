## Program Architecture

The initial state is initialized through calling `initializeController`, from there a mint is created for Redeemable, the signer is kept as the administrative authority, and that's it.

The `Controller` owns the Redeemable Mint. There is only a single `Controller` that can ever exists due to the chosen seed derivation.

Each `Depository` is used to `mint()` and `redeem()` Redeemable (UXD) tokens with a specific collateral mint, and to do so each instantiate a MangoAccount PDA that is used to deposit/withdraw collateral to mango and open/close short perp.

## Admin instructions

They setup the UXD account stack and provide access to the settings.
Only the `authority` set in the `Controller` can interact with these instructions.

### `Initialize`

This initialize the State of the program by instantiating a `Controller`. Called once, the signer becomes the authority, will be done through the DAO.
Only one controller can exist at anytime.

### `RegisterMangoDepository`

Instantiate a new `MangoDepository` PDA for a given collateral mint.
A depository is a vault in charge a Collateral type, the associated mango account and insurance fund.

### `DepositInsuranceToMangoDepository` / `WithdrawInsuranceFromMangoDepository`

Withdraw need to be specific cause it's PDA own accounts.

This would be used to add USDC to a depository mango account to fund it's insurance fund in UXD case.

### `setRedeemableGlobalSupplyCap`

Change the value of the global supply cap (virtual, not on the mint) for the Redeemable by the Controller.

### `setMangoDepositoriesRedeemableSoftCap`

Change the value of the Mango Depositories operations (Mint/Redeem) Redeemable cap, prevent minting/redeeming over this limit.

### `SetMangoDepositoryQuoteMintAndRedeemFee`

Change the value of the Mango Depositories quote redeem/mint fee.

## User instructions

They allow end users to mint and redeem redeemable tokens, they are permissionless.

### `MintWithMangoDepository`

Send collateral to the `Depository` taking that given mint
Estimate how much fill we can get to know how much collateral need to be actually deposited to mango to improve efficiency
Open equivalent perp position (FoK with the provided slippage)
Check that the position was fully opened else abort
Deduct perp opening fees from the quote amount
Mint equivalent amount of UXD to user (as the value of the short perp - taker fees)

### `QuoteRedeemFromMangoDepository`

Similar to classic mint but only available when the DN position PnL is negative. Takes USDC (quote) as input, erase some negative PnL balance and mint equivalent UXD amount.

### `RedeemFromMangoDepository`

User send an amount of UXD to a given `Depository`
We calculate how much collateral that's worth, provided the user slippage and the perp price from Mango
We create a Perp Short Close FoK order in that range
We check that it got filled 100%
We calculate how much USDC value that was, deduct fees
We burn the same value of UXD from what the user sent
We withdraw the collateral amount equivalent to the perp short that has been closed previously (post taxes calculation)
We send that back to the user (and his remaining UXD are back too, if any)

### `QuoteMintWithMangoDepository`

Similar to classic redeem but only available when the DN position PnL is positive. Takes UXD as input, returns some USDC (quote) from the positive PnL balance.

### `RebalanceMangoDepositoryLite`

Convert any paper profits from the short perp part of the delta neutral position back into the delta neutral position, either increasing or decreasing it's size.

If the PnL is positive, profits are used to buy more spot collateral and an equivalent amount of short perp is opened.
If the PnL is negative, some collateral is sold spot, and the equivalent amount of short perp is closed.

Currently it's the lite version, because we cannot do all this atomically in 200k computing nor with 34~ accounts on mango markets. (~34 is the best when we implement place_perp_order_v2).
In order to circumvent this limitation, we skip the spot part by send QUOTE or COLLATERAL (and returning the resulting COLLATERAL or QUOTE). It acts as a swap for taker fees + slippage.

It is open as it won't fit a the nested instruction space of DAO proposal (we might also incentivize rebalancing with UXP rewards at some point or find a decentralized way to keep the PnL in check).

_____
