# CHANGELOG

## v3.0.0-beta

Deployed: | Slot:

1. Added an additional payer to Mint and Redeem instructions
2. Added two instructions (MigrateMangoDepositoryToV2, RebalanceMangoDepositoryLite) to help with rebalancing the delta neutral position
3. Refactor and increase coverage or E2E tests (+ Mochawesome test report generation)
4. Updates UXD-Client to 5.0.0-beta

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
