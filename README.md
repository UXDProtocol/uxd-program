# solana-usds

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

## Testing

```Bash
$> yarn
$> anchor test ## with optional provider --provider.cluster <devnet|localnet|testnet>
$>
```
