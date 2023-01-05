# CHANGELOG

## [v6.0.0]

Deployed: | Slot:

- Credix Lp Depository ALM integration (mint/redeem/collect)

## [v5.1.0]

Deployed: | Slot: TBD

- Identity Depository (mint/redeem)
- Removal of MangoMarketsV3 related code after their exploit
- Adding a one time MangoMarketsV3 accounting fix instruction

## [v5.0.0]

Deployed: | Slot:

- Mercurial Vault Depository ALM integration (mint/redeem)

## [v4.0.0]

Deployed: | Slot:

- Use place_perp_order_v2 to save computing and reduce composability issues
- Pass a limit_price in place of the slippage to ensure execution price match user expectations
- Add `01` support (mint/redeem/depositories creation)
- Remove init_if_needed (and remove Rent and Associated Token Program from inputs)
- Updates Anchor to version 0.24.2

## [v3.0.2] (audit report fixes Soteria)

Deployed: Mar 31, 2022 at 08:50:52 UTC | Slot: 127,530,926

1. Remove un needed passthrough accounts through the app to save computing

## [v3.0.1] (audited Soteria)

Deployed: | Slot:

1. Added an additional payer to Mint and Redeem instructions (although they are too big to fit current governance program proposals)
2. Added MigrateMangoDepositoryToV2 instruction to migrate V1 MangoDepository accounts to their V2 counterpart
3. Added RebalanceMangoDepositoryLite instruction to rebalance the delta neutral position by providing either quote or collateral depending of the PnL polarity

## [v2.4.0] (security hotfix)

Deployed: Mar 18, 2022 at 09:19:49 UTC | Slot: 125,497,411

1. Add verification to the `mango_perp_market` account passed in mint/redeem/rebalance to enforce its mint matches the depository's one

## [v2.3.1] (detached branch patch)

Deployed: Mar 8, 2022 at 09:51:13 UTC | Slot: 124,008,429

1. Removes 2 accounts from the withdraw instruction be able to execute it through the governance (tx byte size limit)

## [v2.3.0] (composability patch)

Deployed: Feb 24, 2022 at 09:08:28 UTC | Slot: 122,253,178

1. Compatibility update for MangoMarketsV3 v3.4.0

## [v2.2.0] (hotfix)

Deployed: Feb 14, 2022 at 14:49:53 UTC | Slot: 120,810,169

1. Fix incorrect slippage calculations.

## [v2.1.0] (hotfix)

Deployed: Feb 14, 2022 at 11:46:41 UTC | Slot: 120,791,670

1. Remove unnecessary account passed as input to mango instructions (mango 3.3.5)
2. Owner of the MangoAccount passed as parameter to MangoMarketV3 CPI is now always a signer

## [v2.0.0] (audited Brahma)

Deployed: January 14, 2022 14:19:32 PM +UTC | Slot: 116,223,281

1. Initial deployment
