# CHANGELOG

## v3.1.0

1. Use place_perp_order_v2 to save computing and reduce composability issues

## v3.0.0 (audited Soteria)

Deployed: | Slot:

1. Added an additional payer to Mint and Redeem instructions (although they are too big to fit current governance program proposals)
2. Added MigrateMangoDepositoryToV2 instruction to migrate V1 MangoDepository accounts to their V2 counterpart
3. Added RebalanceMangoDepositoryLite instruction to rebalance the delta neutral position by providing either quote or collateral depending of the PnL polarity

## v2.3.0 (composability patch)

Deployed: Feb 24,2022 at  UTC | Slot:

1. Compatibility update for MangoMarketsV3 v3.4.0

## v2.2.0 (hotfix)

Deployed: Feb 14, 2022 at 14:49:53 UTC | Slot: 120,810,169

1. Fix incorrect slippage calculations.

## v2.1.0 (hotfix)

Deployed: Feb 14, 2022 at 11:46:41 UTC | Slot: 120,791,670

1. Remove unnecessary account passed as input to mango instructions (mango 3.3.5)
2. Owner of the MangoAccount passed as parameter to MangoMarketV3 CPI is now always a signer

## v2.0.0 (audited Brahma)

Deployed: January 14, 2022 14:19:32 PM +UTC | Slot: 116,223,281

1. Initial deployment
