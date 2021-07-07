# solana-usds
implementation of usds token on solana

look in "depository" and "hana" branches

Consists of 3 main components as currently designed

  Depository
  - Accepts user funds and issues redeemable tokens
  - Has an authority field for the "owner" which can transfer funds from the
    depository under the right conditions

  Controller
  - Interacts with user funds via multiple depositories
  - owns collateral token accounts for perpetual swaps venue
  - Rebalances positions and mints usdx

  Interface
  - Web app allows user to deposit, withdraw collateral
  - Options to mint, redeem usdx and view user account dashboard
