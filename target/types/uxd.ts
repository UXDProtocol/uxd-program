export type Uxd = {
  "version": "5.0.0",
  "name": "uxd",
  "instructions": [
    {
      "name": "initializeController",
      "docs": [
        "Initialize a Controller on chain account.",
        "",
        "Parameters:",
        "- redeemable_mint_decimals: the decimals of the redeemable mint.",
        "",
        "Note:",
        "Only one Controller on chain account will ever exist due to the",
        "PDA derivation seed having no variations.",
        "",
        "Note:",
        "In the case of UXDProtocol this is the one in charge of the UXD mint,",
        "and it has been locked to a single Controller to ever exist by only",
        "having one possible derivation. (but it's been made generic, and we",
        "could have added the authority to the seed generation for instance).",
        ""
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 The redeemable mint managed by the `controller` instance"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#5 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 Token Program"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#7 Rent Sysvar"
          ]
        }
      ],
      "args": [
        {
          "name": "redeemableMintDecimals",
          "type": "u8"
        }
      ]
    },
    {
      "name": "editController",
      "docs": [
        "Sets some fields of the provided `Controller` account.",
        "",
        "Parameters:",
        "- fields.quote_mint_and_redeem_soft_cap: Option<u64> // ignored if None",
        "- fields.redeemable_soft_cap: Option<u64> // ignored if None",
        "- fields.redeemable_global_supply_cap: Option<128> // ignored if None",
        "",
        "About: \"fields.redeemable_soft_cap\"",
        "Sets the `mango_depositories_redeemable_soft_cap` of the provided `Controller` account.",
        "Explanation:",
        "The `mango_depositories_redeemable_soft_cap` determines the",
        "max amount of redeemable tokens that can be minted during a",
        "single operation.",
        "The redeemable global supply cap determines the max total supply",
        "for the redeemable token. Program will abort when an instruction",
        "that mints new redeemable would bring the circulating supply",
        "beyond this value.",
        "Notes:",
        "- The `mango_depositories_redeemable_soft_cap` determines the",
        "max amount of redeemable tokens that can be minted during a",
        "single operation.",
        "- This only apply to Minting. Redeeming is always possible.",
        "- Purpose of this is to control the max amount minted at once on",
        "MangoMarkets Depositories.",
        "- If this is set to 0, it would effectively pause minting on",
        "MangoMarkets Depositories.",
        "",
        "About: \"fields.redeemable_global_supply_cap\"",
        "Sets the `redeemable_global_supply_cap` of the provided `Controller` account.",
        "Explanation:",
        "The redeemable global supply cap determines the max total supply",
        "for the redeemable token. Program will abort when an instruction",
        "that mints new redeemable would bring the circulating supply",
        "beyond this value.",
        "Notes:",
        "- Purpose of this is to roll out progressively for OI, and limit risks.",
        "- If this is set below the current circulating supply of UXD, it would effectively pause Minting.",
        ""
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#2 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        }
      ],
      "args": [
        {
          "name": "fields",
          "type": {
            "defined": "EditControllerFields"
          }
        }
      ]
    },
    {
      "name": "registerMangoDepository",
      "docs": [
        "Create a new `MangoDepository` and registers it to the provided",
        "`Controller` account.",
        "",
        "Note:",
        "Each `MangoDepository` account manages a specific collateral mint.",
        "They will only transact for this specific mint to segregate funding",
        "rates/deposit yield and risks.",
        "",
        "Note:",
        "Each `MangoDepository` owns a MangoAccount for trading spot/perp,",
        "leveraged.",
        "",
        "Update:",
        "In the new version of the MangoMarket Accounts",
        "this become mandatory too. (we are still using the old init)",
        ""
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 UXDProgram on chain account bound to a Controller instance",
            "The `MangoDepository` manages a MangoAccount for a single Collateral"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#5 The collateral mint used by the `depository` instance"
          ]
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 The insurance mint used by the `depository` instance"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#8 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#10 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#11 MangoMarketv3 Program"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12 Rent Sysvar"
          ]
        }
      ],
      "args": [
        {
          "name": "redeemableAmountUnderManagementCap",
          "type": "u128"
        }
      ]
    },
    {
      "name": "depositInsuranceToMangoDepository",
      "docs": [
        "Deposit `MangoDepository.quote_mint` tokens in the `MangoDepository`",
        "underlying `MangoAccount`",
        "",
        "Parameters:",
        "- amount: the amount of quote token to deposit in native unit.",
        "",
        "Note:",
        "Each `MangoDepository` underlying `MangoAccount` uses leverage to open",
        "and maintain short positions.",
        "",
        "Note:",
        "The LTV (Loan to value) ratio is different depending of the mint of",
        "the `MangoDepository.collateral_mint`.",
        "",
        "Note:",
        "LTV for BTC/ETH/SOL is at 0.9:1 (0.9$ lent for 1$ of value deposited).",
        "MangoMarkets Assets specs : https://docs.mango.markets/mango/token-specs",
        "",
        "Note:",
        "Beyond 80% the `MangoAccount` cannot borrow further, disabling the",
        "redemption of redeemable tokens or the withdrawal of deposited insurance.",
        "(Although the insurance should be gone at that point due to funding,",
        "except in the case of sharp collateral price increase without rebalancing)",
        "",
        "Note:",
        "Beyond 90% the `MangoAccount` can be liquidated by other mango accounts.",
        "(And borrows/withdraws are still disabled)",
        "",
        "Note:",
        "As the funding rate care be either negative or positive, the insurance",
        "is there as a buffer to ensure that redeemables can be swapped back",
        "at all time (by unwinding the backing amount of delta neutral",
        "position).",
        ""
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#2 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 UXDProgram on chain account bound to a Controller instance",
            "The `MangoDepository` manages a MangoAccount for a single Collateral"
          ]
        },
        {
          "name": "authorityQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 The `authority`'s ATA for the `quote_mint`",
            "Will be debited during this call"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoRootBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 [MangoMarkets CPI] Root Bank for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9 [MangoMarkets CPI] Node Bank for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Vault for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#11 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "withdrawInsuranceFromMangoDepository",
      "docs": [
        "Withdraw `MangoDepository.quote_mint` tokens from the `MangoDepository`",
        "underlying `MangoAccount`, if any available, in the limit of the account",
        "borrow health.",
        "",
        "Parameters:",
        "- amount: the amount of quote token to withdraw in native unit.",
        "",
        "Note:",
        "Withdrawal cannot borrow, nor bring the health of the account in",
        "liquidation territory.",
        "",
        "Notes:",
        "The `MangoDepository.insurance_amount_deposited` tracks the amount of",
        "`MangoDepository.quote_mint` tokens deposited, but does not represent",
        "the available amount as it moves depending of funding rates and",
        "perp positions PnL settlement (temporarily).",
        ""
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#2 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 UXDProgram on chain account bound to a Controller instance",
            "The `MangoDepository` manages a MangoAccount for a single Collateral"
          ]
        },
        {
          "name": "authorityQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 The `user`'s ATA for the `controller`'s `redeemable_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#7 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#8 [MangoMarkets CPI] Signer PDA"
          ]
        },
        {
          "name": "mangoRootBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9 [MangoMarkets CPI] Root Bank for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Node Bank for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11 [MangoMarkets CPI] Vault for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#13 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#14 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "rebalanceMangoDepositoryLite",
      "docs": [
        "Rebalance the delta neutral position of the underlying `MangoDepository`.",
        "",
        "Parameters:",
        "- max_rebalancing_amount: the maximum amount of quote this rebalance",
        "instruction will attempt to rebalance, in native unit.",
        "- polarity: the direction of the rebalancing. This is known on chain",
        "but required as an argument for clarity.",
        "- limit_price: the worst price the user is willing to trade at.",
        "",
        "Note:",
        "Acts as a swap, reducing the oustanding PnL (paper profit or losses) on",
        "the underlying `MangoAccount`.",
        "",
        "Note:",
        "This is the \"lite\" version as it force the caller to input some quote or",
        "collateral. This is done to skip the spot order on mango, saving computing",
        "and also bypassing the issue with teh 34 accounts limits.",
        "A new version is designed and waiting for the TransactionV2 proposal to hit",
        "along with the 1M computing units.",
        "",
        "Note:",
        "Paper profits are represented in Quote, it's currently USDC on",
        "MangoMarkets, as of 02/17/2022.",
        "",
        "Note:",
        "This call should goes with a call to `@uxdprotocol/uxd-client`'s",
        "`MangoDepository.settleMangoDepositoryMangoAccountPnl()`, which convert paper",
        "profits or losses into realized gain/losses. Once rebalancing is out,",
        "since it's permissionless, the PnL settlement should be called once in a while",
        "to make sure that unsettled Positive PNL accumulates and that the MangoAccount",
        "has to pay borrow rates for it. Some day when computing is plentiful and input",
        "accounts are increased through TransactionsV2 proposal, we can",
        "also call the onchain version.",
        "",
        "Note:",
        "TEMPORARY Although this create the associated token account for WSOL",
        "when the PnL is Negative, it's too short on computing. Please create beforehand."
      ],
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Public call accessible to any user",
            "Note - Mut required for WSOL unwrapping"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 UXDProgram on chain account bound to a Controller instance",
            "The `MangoDepository` manages a MangoAccount for a single Collateral"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#5 The collateral mint used by the `depository` instance",
            "Required to create the user_collateral ATA if needed"
          ]
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 The quote mint used by the `depository` instance",
            "Required to create the user_quote ATA if needed"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The `user`'s TA for the `depository`'s `collateral_mint`",
            "Will be debited during this instruction when `Polarity` is positive",
            "Will be credited during this instruction when `Polarity` is negative"
          ]
        },
        {
          "name": "userQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The `user`'s TA for the `depository`'s `quote_mint`",
            "Will be credited during this instruction when `Polarity` is positive",
            "Will be debited during this instruction when `Polarity` is negative"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Signer PDA"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#12 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoRootBankQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#13 [MangoMarkets CPI] Root Bank for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoNodeBankQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#14 [MangoMarkets CPI] Node Bank for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoVaultQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#15 [MangoMarkets CPI] Vault `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoRootBankCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#16 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoNodeBankCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#17 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoVaultCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#18 [MangoMarkets CPI] Vault for `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#19 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market"
          ]
        },
        {
          "name": "mangoBids",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#20 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids"
          ]
        },
        {
          "name": "mangoAsks",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#21 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks"
          ]
        },
        {
          "name": "mangoEventQueue",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#22 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#23 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#24 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#25 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "maxRebalancingAmount",
          "type": "u64"
        },
        {
          "name": "polarity",
          "type": {
            "defined": "PnlPolarity"
          }
        },
        {
          "name": "limitPrice",
          "type": "f32"
        }
      ]
    },
    {
      "name": "mintWithMangoDepository",
      "docs": [
        "Mint redeemable tokens in exchange of `MangoDepository.collateral_mint`",
        "tokens, increasing the size of the delta neutral position.",
        "",
        "Parameters:",
        "- collateral_amount: the amount of collateral to use, in",
        "collateral_mint native unit.",
        "- limit_price: the worse price the user is willing to trade at.",
        "",
        "Flow:",
        "- Starts by scanning the order book for the amount that we can fill.",
        "- Deposit to Mango account",
        "- Using the spot collateral deposited, the short perp position of equivalent",
        "size if opened (FoK emulated by using mango IoC + 100% fill verification).",
        "- Deducts the taker_fees (ceiled) form the value of the opened short, and",
        "mints the redeemable, then transfer to the user.",
        "- Internal accounting update + anchor event emission.",
        "",
        "Note:",
        "The caller pays for the incurred slippage and taker_fee (4bps at the time",
        "of writing). This ensures that the system stay \"closed\".",
        "",
        "Note:",
        "The value of the collateral is derived from the COLLATERAL-PERP price,",
        "expressed in USD value.",
        ""
      ],
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Public call accessible to any user"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 UXDProgram on chain account bound to a Controller instance.",
            "The `MangoDepository` manages a MangoAccount for a single Collateral."
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5 The redeemable mint managed by the `controller` instance",
            "Tokens will be minted during this instruction"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6 The `user`'s TA for the `depository` `collateral_mint`",
            "Will be debited during this instruction"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The `user`'s TA for the `controller`'s `redeemable_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoRootBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#12 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#13 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#14 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market"
          ]
        },
        {
          "name": "mangoBids",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#15 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids"
          ]
        },
        {
          "name": "mangoAsks",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#16 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks"
          ]
        },
        {
          "name": "mangoEventQueue",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#17 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#18 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#19 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#20 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "collateralAmount",
          "type": "u64"
        },
        {
          "name": "limitPrice",
          "type": "f32"
        }
      ]
    },
    {
      "name": "redeemFromMangoDepository",
      "docs": [
        "Redeem `MangoDepository.collateral_mint` by burning redeemable",
        "tokens, and unwind a part of the delta neutral position.",
        "",
        "Parameters:",
        "- redeemable_amount: the amount of collateral to use, in",
        "redeemable_mint native unit.",
        "- limit_price: the worse price the user is willing to trade at.",
        "",
        "Flow:",
        "- Starts by scanning the order book to find the best order for",
        "the redeemable_amount fillable (the requested amount minus max",
        "fees, as we repay them by keeping a piece of the DN position).",
        "- Closes the equivalent part of the delta neutral position (FoK",
        "emulated by using mango IoC + 100% fill verification).",
        "- Deducts the taker_fees (ceiled) form the value of the opened short, and",
        "transfer user redeemable token for that amount.",
        "- Burns the redeemable equivalent to fees + closed position,",
        "then withdraw resulting equivalent collateral to the user",
        "- Internal accounting update + anchor event emission.",
        "",
        "Note:",
        "The caller pays for the incurred slippage and taker_fee (4bps at the time",
        "of writing). This ensures that the system stay \"closed\".",
        "",
        "Note:",
        "The value of the collateral is derived from the COLLATERAL-PERP price,",
        "expressed in USD value.",
        ""
      ],
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#1 Public call accessible to any user",
            "Note - Mut required for WSOL unwrapping"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 UXDProgram on chain account bound to a Controller instance.",
            "The `MangoDepository` manages a MangoAccount for a single Collateral."
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#5 The collateral mint used by the `depository` instance",
            "Required to create the user_collateral ATA if needed"
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6 The redeemable mint managed by the `controller` instance",
            "Tokens will be burnt during this instruction"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The `user`'s ATA for the `depository`'s `collateral_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The `user`'s ATA for the `controller`'s `redeemable_mint`",
            "Will be debited during this instruction"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12 [MangoMarkets CPI] Signer PDA"
          ]
        },
        {
          "name": "mangoRootBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#13 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#14 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#15 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#16 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market"
          ]
        },
        {
          "name": "mangoBids",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#17 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids"
          ]
        },
        {
          "name": "mangoAsks",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#18 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks"
          ]
        },
        {
          "name": "mangoEventQueue",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#19 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#20 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#21 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#22 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "redeemableAmount",
          "type": "u64"
        },
        {
          "name": "limitPrice",
          "type": "f32"
        }
      ]
    },
    {
      "name": "quoteMintWithMangoDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Public call accessible to any user"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 UXDProgram on chain account bound to a Controller instance.",
            "The `MangoDepository` manages a MangoAccount for a single Collateral."
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5 The redeemable mint managed by the `controller` instance",
            "Tokens will be minted during this instruction"
          ]
        },
        {
          "name": "userQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6 The `user`'s ATA for one the `mango depository`s `quote_mint`",
            "Will be debited during this instruction"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The `user`'s ATA for the `controller`'s `redeemable_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoRootBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#12 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#13 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#14 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#15 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#16 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#17 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "quoteAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "quoteRedeemFromMangoDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Public call accessible to any user"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 UXDProgram on chain account bound to a Controller instance.",
            "The `MangoDepository` manages a MangoAccount for a single Collateral."
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5 The redeemable mint managed by the `controller` instance",
            "Tokens will be minted during this instruction"
          ]
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 The quote mint of the depository"
          ]
        },
        {
          "name": "userQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The `user`'s ATA for one the `mango depository`s `quote_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The `user`'s ATA for the `controller`'s `redeemable_mint`",
            "Will be debited during this instruction"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12 [MangoMarkets CPI] Signer PDA"
          ]
        },
        {
          "name": "mangoRootBank",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#13 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#14 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#15 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#16 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#17 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#18 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "redeemableAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "editMangoDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#2 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 UXDProgram on chain account bound to a Controller instance.",
            "The `MangoDepository` manages a MangoAccount for a single Collateral."
          ]
        }
      ],
      "args": [
        {
          "name": "fields",
          "type": {
            "defined": "EditMangoDepositoryFields"
          }
        }
      ]
    },
    {
      "name": "editMercurialVaultDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#2 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 UXDProgram on chain account bound to a Controller instance.",
            "The `MercurialVaultDepository` manages a MercurialVaultAccount for a single Collateral."
          ]
        }
      ],
      "args": [
        {
          "name": "fields",
          "type": {
            "defined": "EditMercurialVaultDepositoryFields"
          }
        }
      ]
    },
    {
      "name": "disableDepositoryRegularMinting",
      "docs": [
        "Disable or enable regular minting for given Mango Depository.",
        "",
        "Parameters:",
        "- disable: true to disable, false to enable.",
        "",
        "Note:",
        "The disabled flag is false by default that a freshly registered mango depository has enabled regular minting.",
        "This ix is for toggling that flag.",
        ""
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#2 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 UXDProgram on chain account bound to a Controller instance",
            "The `MangoDepository` manages a MangoAccount for a single Collateral"
          ]
        }
      ],
      "args": [
        {
          "name": "disable",
          "type": "bool"
        }
      ]
    },
    {
      "name": "mintWithMercurialVaultDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4"
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#7"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8"
          ]
        },
        {
          "name": "depositoryLpTokenVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9",
            "Token account holding the LP tokens minted by depositing collateral on mercurial vault"
          ]
        },
        {
          "name": "mercurialVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10"
          ]
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11"
          ]
        },
        {
          "name": "mercurialVaultCollateralTokenSafe",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#12",
            "Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault."
          ]
        },
        {
          "name": "mercurialVaultProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#13"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#14"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#15"
          ]
        }
      ],
      "args": [
        {
          "name": "collateralAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "registerMercurialVaultDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#5"
          ]
        },
        {
          "name": "mercurialVault",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6"
          ]
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#7"
          ]
        },
        {
          "name": "depositoryLpTokenVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8",
            "Token account holding the LP tokens minted by depositing collateral on mercurial vault"
          ]
        },
        {
          "name": "interestsAndFeesRedeemAuthority",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9",
            "Only wallet able to claim interests and fees"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#10"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#11"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12"
          ]
        }
      ],
      "args": [
        {
          "name": "mintingFeeInBps",
          "type": "u8"
        },
        {
          "name": "redeemingFeeInBps",
          "type": "u8"
        },
        {
          "name": "redeemableAmountUnderManagementCap",
          "type": "u128"
        }
      ]
    },
    {
      "name": "redeemFromMercurialVaultDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4"
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#7"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8"
          ]
        },
        {
          "name": "depositoryLpTokenVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9",
            "Token account holding the LP tokens minted by depositing collateral on mercurial vault"
          ]
        },
        {
          "name": "mercurialVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10"
          ]
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11"
          ]
        },
        {
          "name": "mercurialVaultCollateralTokenSafe",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#12",
            "Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault."
          ]
        },
        {
          "name": "mercurialVaultProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#13"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#14"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#15"
          ]
        }
      ],
      "args": [
        {
          "name": "redeemableAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "collectInterestsAndFeesFromMercurialVaultDepository",
      "accounts": [
        {
          "name": "interestsAndFeesRedeemAuthority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#5"
          ]
        },
        {
          "name": "interestsAndFeesRedeemAuthorityCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6"
          ]
        },
        {
          "name": "depositoryLpTokenVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7",
            "Token account holding the LP tokens minted by depositing collateral on mercurial vault"
          ]
        },
        {
          "name": "mercurialVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8"
          ]
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9"
          ]
        },
        {
          "name": "mercurialVaultCollateralTokenSafe",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10",
            "Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault."
          ]
        },
        {
          "name": "mercurialVaultProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#11"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#13"
          ]
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "controller",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "redeemableMintBump",
            "type": "u8"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "redeemableMint",
            "type": "publicKey"
          },
          {
            "name": "redeemableMintDecimals",
            "type": "u8"
          },
          {
            "name": "registeredMangoDepositories",
            "type": {
              "array": [
                "publicKey",
                8
              ]
            }
          },
          {
            "name": "registeredMangoDepositoriesCount",
            "type": "u8"
          },
          {
            "name": "redeemableGlobalSupplyCap",
            "type": "u128"
          },
          {
            "name": "mangoDepositoriesRedeemableSoftCap",
            "type": "u64"
          },
          {
            "name": "redeemableCirculatingSupply",
            "type": "u128"
          },
          {
            "name": "mangoDepositoriesQuoteRedeemableSoftCap",
            "type": "u64"
          },
          {
            "name": "registeredMercurialVaultDepositories",
            "type": {
              "array": [
                "publicKey",
                4
              ]
            }
          },
          {
            "name": "registeredMercurialVaultDepositoriesCount",
            "type": "u8"
          },
          {
            "name": "interestsAndFeesTotalCollected",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "mangoDepository",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "unused",
            "type": {
              "array": [
                "u8",
                2
              ]
            }
          },
          {
            "name": "mangoAccountBump",
            "type": "u8"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "collateralMint",
            "type": "publicKey"
          },
          {
            "name": "collateralMintDecimals",
            "type": "u8"
          },
          {
            "name": "unused2",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "quoteMint",
            "type": "publicKey"
          },
          {
            "name": "unused3",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "quoteMintDecimals",
            "type": "u8"
          },
          {
            "name": "mangoAccount",
            "type": "publicKey"
          },
          {
            "name": "controller",
            "type": "publicKey"
          },
          {
            "name": "insuranceAmountDeposited",
            "type": "u128"
          },
          {
            "name": "collateralAmountDeposited",
            "type": "u128"
          },
          {
            "name": "redeemableAmountUnderManagement",
            "type": "u128"
          },
          {
            "name": "totalAmountPaidTakerFee",
            "type": "u128"
          },
          {
            "name": "totalAmountRebalanced",
            "type": "u128"
          },
          {
            "name": "netQuoteMinted",
            "type": "i128"
          },
          {
            "name": "quoteMintAndRedeemFee",
            "type": "u8"
          },
          {
            "name": "totalQuoteMintAndRedeemFees",
            "type": "u128"
          },
          {
            "name": "regularMintingDisabled",
            "type": "bool"
          },
          {
            "name": "redeemableAmountUnderManagementCap",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "mercurialVaultDepository",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "collateralMint",
            "type": "publicKey"
          },
          {
            "name": "collateralMintDecimals",
            "type": "u8"
          },
          {
            "name": "controller",
            "type": "publicKey"
          },
          {
            "name": "collateralAmountDeposited",
            "type": "u128"
          },
          {
            "name": "redeemableAmountUnderManagement",
            "type": "u128"
          },
          {
            "name": "mercurialVault",
            "type": "publicKey"
          },
          {
            "name": "mercurialVaultLpMint",
            "type": "publicKey"
          },
          {
            "name": "mercurialVaultLpMintDecimals",
            "type": "u8"
          },
          {
            "name": "lpTokenVault",
            "type": "publicKey"
          },
          {
            "name": "lpTokenVaultBump",
            "type": "u8"
          },
          {
            "name": "mintingFeeInBps",
            "type": "u8"
          },
          {
            "name": "redeemingFeeInBps",
            "type": "u8"
          },
          {
            "name": "mintingFeeTotalAccrued",
            "type": "u128"
          },
          {
            "name": "redeemingFeeTotalAccrued",
            "type": "u128"
          },
          {
            "name": "redeemableAmountUnderManagementCap",
            "type": "u128"
          },
          {
            "name": "mintingDisabled",
            "type": "bool"
          },
          {
            "name": "interestsAndFeesRedeemAuthority",
            "type": "publicKey"
          },
          {
            "name": "interestsAndFeesTotalCollected",
            "type": "u128"
          },
          {
            "name": "lastInterestsAndFeesCollectionUnixTimestamp",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "EditControllerFields",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "quoteMintAndRedeemSoftCap",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "redeemableSoftCap",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "redeemableGlobalSupplyCap",
            "type": {
              "option": "u128"
            }
          }
        ]
      }
    },
    {
      "name": "EditMangoDepositoryFields",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "quoteMintAndRedeemFee",
            "type": {
              "option": "u8"
            }
          },
          {
            "name": "redeemableAmountUnderManagementCap",
            "type": {
              "option": "u128"
            }
          }
        ]
      }
    },
    {
      "name": "EditMercurialVaultDepositoryFields",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "redeemableAmountUnderManagementCap",
            "type": {
              "option": "u128"
            }
          },
          {
            "name": "mintingFeeInBps",
            "type": {
              "option": "u8"
            }
          },
          {
            "name": "redeemingFeeInBps",
            "type": {
              "option": "u8"
            }
          },
          {
            "name": "mintingDisabled",
            "type": {
              "option": "bool"
            }
          },
          {
            "name": "interestsAndFeesRedeemAuthority",
            "type": {
              "option": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "PnlPolarity",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Positive"
          },
          {
            "name": "Negative"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "InitializeControllerEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "authority",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "SetRedeemableGlobalSupplyCapEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "redeemableGlobalSupplyCap",
          "type": "u128",
          "index": false
        }
      ]
    },
    {
      "name": "RegisterMangoDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "collateralMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "insuranceMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "mangoAccount",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "RegisterMangoDepositoryEventV2",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "depositoryVersion",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "collateralMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "quoteMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "mangoAccount",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "RegisterMercurialVaultDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "depositoryVersion",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "mercurialVault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depositoryLpTokenVault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "collateralMint",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "SetMangoDepositoryRedeemableSoftCapEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "redeemableMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "redeemableMintDecimals",
          "type": "u8",
          "index": false
        },
        {
          "name": "redeemableSoftCap",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "DepositInsuranceToDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "quoteMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "quoteMintDecimals",
          "type": "u8",
          "index": false
        },
        {
          "name": "depositedAmount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "WithdrawInsuranceFromMangoDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "insuranceMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "insuranceMintDecimals",
          "type": "u8",
          "index": false
        },
        {
          "name": "withdrawnAmount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "WithdrawInsuranceFromDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "quoteMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "quoteMintDecimals",
          "type": "u8",
          "index": false
        },
        {
          "name": "withdrawnAmount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "MintWithMangoDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "collateralAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "limitPrice",
          "type": "f32",
          "index": false
        },
        {
          "name": "baseDelta",
          "type": "i64",
          "index": false
        },
        {
          "name": "quoteDelta",
          "type": "i64",
          "index": false
        },
        {
          "name": "feeDelta",
          "type": "i64",
          "index": false
        }
      ]
    },
    {
      "name": "RedeemFromMangoDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "redeemableAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "limitPrice",
          "type": "f32",
          "index": false
        },
        {
          "name": "baseDelta",
          "type": "i64",
          "index": false
        },
        {
          "name": "quoteDelta",
          "type": "i64",
          "index": false
        },
        {
          "name": "feeDelta",
          "type": "i64",
          "index": false
        }
      ]
    },
    {
      "name": "RebalanceMangoDepositoryLiteEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "depositoryVersion",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "polarity",
          "type": {
            "defined": "PnlPolarity"
          },
          "index": false
        },
        {
          "name": "rebalancingAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "rebalancedAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "limitPrice",
          "type": "f32",
          "index": false
        },
        {
          "name": "baseDelta",
          "type": "i64",
          "index": false
        },
        {
          "name": "quoteDelta",
          "type": "i64",
          "index": false
        },
        {
          "name": "feeDelta",
          "type": "i64",
          "index": false
        }
      ]
    },
    {
      "name": "SetMangoDepositoryQuoteMintAndRedeemSoftCapEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "quoteMintAndRedeemSoftCap",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "SetMangoDepositoryQuoteMintAndRedeemFeeEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "quoteMintAndRedeemFee",
          "type": "u8",
          "index": true
        }
      ]
    },
    {
      "name": "SetMangoDepositoryRedeemableAmountUnderManagementCapEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "redeemableAmountUnderManagementCap",
          "type": "u128",
          "index": true
        }
      ]
    },
    {
      "name": "SetMercurialVaultDepositoryRedeemableAmountUnderManagementCapEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "redeemableAmountUnderManagementCap",
          "type": "u128",
          "index": true
        }
      ]
    },
    {
      "name": "SetMercurialVaultDepositoryMintingFeeInBpsEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "mintingFeeInBps",
          "type": "u8",
          "index": true
        }
      ]
    },
    {
      "name": "SetMercurialVaultDepositoryRedeemingFeeInBpsEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "redeemingFeeInBps",
          "type": "u8",
          "index": true
        }
      ]
    },
    {
      "name": "SetMercurialVaultDepositoryMintingDisabledEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "mintingDisabled",
          "type": "bool",
          "index": true
        }
      ]
    },
    {
      "name": "SetMercurialVaultDepositoryInterestsAndFeesRedeemAuthorityEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "interestsAndFeesRedeemAuthority",
          "type": "publicKey",
          "index": true
        }
      ]
    },
    {
      "name": "QuoteRedeemFromMangoDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "quoteRedeemableAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "quoteRedeemFee",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "QuoteMintWithMangoDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "quoteMintAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "quoteMintFee",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "DisableDepositoryRegularMintingEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "regularMintingDisabled",
          "type": "bool",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidRedeemableMintDecimals",
      "msg": "The redeemable mint decimals must be between 0 and 9 (inclusive)."
    },
    {
      "code": 6001,
      "name": "InvalidRedeemableGlobalSupplyCap",
      "msg": "Redeemable global supply above."
    },
    {
      "code": 6002,
      "name": "RootBankIndexNotFound",
      "msg": "The associated mango root bank index cannot be found for the deposited coin.."
    },
    {
      "code": 6003,
      "name": "InvalidLimitPrice",
      "msg": "The provided limit_price value is invalid, must be > 0"
    },
    {
      "code": 6004,
      "name": "EffectiveOrderPriceBeyondLimitPrice",
      "msg": "Could not fill the order given order book state and provided slippage."
    },
    {
      "code": 6005,
      "name": "InvalidCollateralAmount",
      "msg": "Collateral amount cannot be 0"
    },
    {
      "code": 6006,
      "name": "InvalidQuoteAmount",
      "msg": "Quote amount must be > 0 in order to mint."
    },
    {
      "code": 6007,
      "name": "InvalidRedeemableAmount",
      "msg": "Redeemable amount must be > 0 in order to redeem."
    },
    {
      "code": 6008,
      "name": "InsufficientCollateralAmount",
      "msg": "The balance of the collateral ATA is not enough to fulfill the mint operation."
    },
    {
      "code": 6009,
      "name": "InsufficientQuoteAmountMint",
      "msg": "The balance of the quote ATA is not enough to fulfil the mint operation."
    },
    {
      "code": 6010,
      "name": "InsufficientRedeemableAmountMint",
      "msg": "The balance of the redeemable ATA is not enough to fulfil the redeem operation."
    },
    {
      "code": 6011,
      "name": "InsufficientRedeemableAmount",
      "msg": "The balance of the redeemable ATA is not enough to fulfill the redeem operation."
    },
    {
      "code": 6012,
      "name": "PerpOrderPartiallyFilled",
      "msg": "The perp position could not be fully filled with the provided slippage."
    },
    {
      "code": 6013,
      "name": "RedeemableGlobalSupplyCapReached",
      "msg": "Minting amount would go past the Redeemable Global Supply Cap."
    },
    {
      "code": 6014,
      "name": "RedeemableMangoAmountUnderManagementCap",
      "msg": "Minting amount would go past the mango depository Redeemable Amount Under Management Cap."
    },
    {
      "code": 6015,
      "name": "RedeemableMercurialVaultAmountUnderManagementCap",
      "msg": "Minting amount would go past the mercurial vault depository Redeemable Amount Under Management Cap."
    },
    {
      "code": 6016,
      "name": "MangoDepositoriesSoftCapOverflow",
      "msg": "Operation not allowed due to being over the Mango Redeemable soft Cap."
    },
    {
      "code": 6017,
      "name": "MaxNumberOfMangoDepositoriesRegisteredReached",
      "msg": "Cannot register more mango depositories, the limit has been reached."
    },
    {
      "code": 6018,
      "name": "InvalidInsuranceAmount",
      "msg": "The amount to withdraw from the Insurance Fund must be superior to zero.."
    },
    {
      "code": 6019,
      "name": "InsufficientAuthorityQuoteAmount",
      "msg": "The Quote ATA from authority doesn't have enough balance."
    },
    {
      "code": 6020,
      "name": "InvalidRebalancedAmount",
      "msg": "The rebalanced amount must be superior to zero.."
    },
    {
      "code": 6021,
      "name": "InsufficientOrderBookDepth",
      "msg": "Insufficient order book depth for order."
    },
    {
      "code": 6022,
      "name": "InvalidExecutedOrderSize",
      "msg": "The executed order size does not match the expected one."
    },
    {
      "code": 6023,
      "name": "InvalidMangoDepositoriesRedeemableSoftCap",
      "msg": "Mango depositories redeemable soft cap above."
    },
    {
      "code": 6024,
      "name": "InvalidQuoteDelta",
      "msg": "Quote_lot_delta can't be 0."
    },
    {
      "code": 6025,
      "name": "InvalidOrderDirection",
      "msg": "The perp order wasn't executed in the right direction."
    },
    {
      "code": 6026,
      "name": "MathError",
      "msg": "Math error."
    },
    {
      "code": 6027,
      "name": "SlippageReached",
      "msg": "The order couldn't be executed with the provided slippage."
    },
    {
      "code": 6028,
      "name": "InvalidRebalancingAmount",
      "msg": "The rebalancing amount must be above 0."
    },
    {
      "code": 6029,
      "name": "InsufficientQuoteAmount",
      "msg": "The Quote amount in the provided user_quote ATA must be >= max_amount_rebalancing."
    },
    {
      "code": 6030,
      "name": "InvalidPnlPolarity",
      "msg": "The PnL polarity provided is not the same as the perp position's one."
    },
    {
      "code": 6031,
      "name": "RebalancingError",
      "msg": "The rebalanced amount doesn't match the expected rebalance amount."
    },
    {
      "code": 6032,
      "name": "BumpError",
      "msg": "A bump was expected but is missing."
    },
    {
      "code": 6033,
      "name": "OrderSizeBelowMinLotSize",
      "msg": "The order is below size is below the min lot size."
    },
    {
      "code": 6034,
      "name": "InvalidCollateralDelta",
      "msg": "The collateral delta post perp order doesn't match the planned one."
    },
    {
      "code": 6035,
      "name": "MangoPerpMarketIndexNotFound",
      "msg": "The perp market index could not be found for this MangoMarkets Pair."
    },
    {
      "code": 6036,
      "name": "CannotLoadMangoGroup",
      "msg": "Could not load the provided MangoGroup account."
    },
    {
      "code": 6037,
      "name": "QuantityBelowContractSize",
      "msg": "The order quantity is below contract_size of the perp market."
    },
    {
      "code": 6038,
      "name": "QuoteAmountTooHigh",
      "msg": "The amount trying to be quote minted is larger than quote mintable."
    },
    {
      "code": 6039,
      "name": "RedeemableAmountTooHigh",
      "msg": "The amount trying to be quote redeemed is larger than quote redeemable."
    },
    {
      "code": 6040,
      "name": "MintingDisabled",
      "msg": "Minting is disabled for the current depository."
    },
    {
      "code": 6041,
      "name": "MintingAlreadyDisabledOrEnabled",
      "msg": "Minting is already disabled/enabled."
    },
    {
      "code": 6042,
      "name": "QuoteAmountExceedsSoftCap",
      "msg": "The quote amount requested is beyond the soft cap limitation."
    },
    {
      "code": 6043,
      "name": "InvalidQuoteCurrency",
      "msg": "The quote currency is not the expected one."
    },
    {
      "code": 6044,
      "name": "InvalidMercurialVaultLpMint",
      "msg": "The mercurial vault lp mint does not match the Depository's one."
    },
    {
      "code": 6045,
      "name": "MaxNumberOfMercurialVaultDepositoriesRegisteredReached",
      "msg": "Cannot register more mercurial vault depositories, the limit has been reached."
    },
    {
      "code": 6046,
      "name": "MercurialVaultDoNotMatchCollateral",
      "msg": "The provided collateral do not match the provided mercurial vault token."
    },
    {
      "code": 6047,
      "name": "CollateralMintEqualToRedeemableMint",
      "msg": "Collateral mint should be different than redeemable mint."
    },
    {
      "code": 6048,
      "name": "CollateralMintNotAllowed",
      "msg": "Provided collateral mint is not allowed."
    },
    {
      "code": 6049,
      "name": "MinimumMintedRedeemableAmountError",
      "msg": "Mint resulted to 0 redeemable token being minted."
    },
    {
      "code": 6050,
      "name": "MinimumRedeemedCollateralAmountError",
      "msg": "Redeem resulted to 0 collateral token being redeemed."
    },
    {
      "code": 6051,
      "name": "InvalidDepositoryLpTokenVault",
      "msg": "The depository lp token vault does not match the Depository's one."
    },
    {
      "code": 6052,
      "name": "UnAllowedMangoGroup",
      "msg": "The mango group is not accepted."
    },
    {
      "code": 6053,
      "name": "InvalidMercurialVaultInterestsAndFeesAuthority",
      "msg": "Only the mercurial vault interests and fees authority can access this instructions."
    },
    {
      "code": 6054,
      "name": "InvalidAuthority",
      "msg": "Only the Program initializer authority can access this instructions."
    },
    {
      "code": 6055,
      "name": "InvalidController",
      "msg": "The Depository's controller doesn't match the provided Controller."
    },
    {
      "code": 6056,
      "name": "InvalidDepository",
      "msg": "The Depository provided is not registered with the Controller."
    },
    {
      "code": 6057,
      "name": "InvalidCollateralMint",
      "msg": "The provided collateral mint does not match the depository's collateral mint."
    },
    {
      "code": 6058,
      "name": "InvalidQuoteMint",
      "msg": "The provided quote mint does not match the depository's quote mint."
    },
    {
      "code": 6059,
      "name": "InvalidMangoAccount",
      "msg": "The Mango Account isn't the Depository one."
    },
    {
      "code": 6060,
      "name": "InvalidRedeemableMint",
      "msg": "The Redeemable Mint provided does not match the Controller's one."
    },
    {
      "code": 6061,
      "name": "InvalidDexMarket",
      "msg": "The provided perp_market is not the one tied to this Depository."
    },
    {
      "code": 6062,
      "name": "InvalidOwner",
      "msg": "The provided token account is not owner by the expected party."
    },
    {
      "code": 6063,
      "name": "InvalidMaxBaseQuantity",
      "msg": "The max base quantity must be above 0."
    },
    {
      "code": 6064,
      "name": "InvalidMaxQuoteQuantity",
      "msg": "The max quote quantity must be above 0."
    },
    {
      "code": 6065,
      "name": "InvalidMercurialVault",
      "msg": "The provided mercurial vault does not match the Depository's one."
    },
    {
      "code": 6066,
      "name": "InvalidMercurialVaultCollateralTokenSafe",
      "msg": "The provided mercurial vault collateral token safe does not match the mercurial vault one."
    },
    {
      "code": 6067,
      "name": "Default",
      "msg": "Default - Check the source code for more info."
    }
  ]
};

export const IDL: Uxd = {
  "version": "5.0.0",
  "name": "uxd",
  "instructions": [
    {
      "name": "initializeController",
      "docs": [
        "Initialize a Controller on chain account.",
        "",
        "Parameters:",
        "- redeemable_mint_decimals: the decimals of the redeemable mint.",
        "",
        "Note:",
        "Only one Controller on chain account will ever exist due to the",
        "PDA derivation seed having no variations.",
        "",
        "Note:",
        "In the case of UXDProtocol this is the one in charge of the UXD mint,",
        "and it has been locked to a single Controller to ever exist by only",
        "having one possible derivation. (but it's been made generic, and we",
        "could have added the authority to the seed generation for instance).",
        ""
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 The redeemable mint managed by the `controller` instance"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#5 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 Token Program"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#7 Rent Sysvar"
          ]
        }
      ],
      "args": [
        {
          "name": "redeemableMintDecimals",
          "type": "u8"
        }
      ]
    },
    {
      "name": "editController",
      "docs": [
        "Sets some fields of the provided `Controller` account.",
        "",
        "Parameters:",
        "- fields.quote_mint_and_redeem_soft_cap: Option<u64> // ignored if None",
        "- fields.redeemable_soft_cap: Option<u64> // ignored if None",
        "- fields.redeemable_global_supply_cap: Option<128> // ignored if None",
        "",
        "About: \"fields.redeemable_soft_cap\"",
        "Sets the `mango_depositories_redeemable_soft_cap` of the provided `Controller` account.",
        "Explanation:",
        "The `mango_depositories_redeemable_soft_cap` determines the",
        "max amount of redeemable tokens that can be minted during a",
        "single operation.",
        "The redeemable global supply cap determines the max total supply",
        "for the redeemable token. Program will abort when an instruction",
        "that mints new redeemable would bring the circulating supply",
        "beyond this value.",
        "Notes:",
        "- The `mango_depositories_redeemable_soft_cap` determines the",
        "max amount of redeemable tokens that can be minted during a",
        "single operation.",
        "- This only apply to Minting. Redeeming is always possible.",
        "- Purpose of this is to control the max amount minted at once on",
        "MangoMarkets Depositories.",
        "- If this is set to 0, it would effectively pause minting on",
        "MangoMarkets Depositories.",
        "",
        "About: \"fields.redeemable_global_supply_cap\"",
        "Sets the `redeemable_global_supply_cap` of the provided `Controller` account.",
        "Explanation:",
        "The redeemable global supply cap determines the max total supply",
        "for the redeemable token. Program will abort when an instruction",
        "that mints new redeemable would bring the circulating supply",
        "beyond this value.",
        "Notes:",
        "- Purpose of this is to roll out progressively for OI, and limit risks.",
        "- If this is set below the current circulating supply of UXD, it would effectively pause Minting.",
        ""
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#2 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        }
      ],
      "args": [
        {
          "name": "fields",
          "type": {
            "defined": "EditControllerFields"
          }
        }
      ]
    },
    {
      "name": "registerMangoDepository",
      "docs": [
        "Create a new `MangoDepository` and registers it to the provided",
        "`Controller` account.",
        "",
        "Note:",
        "Each `MangoDepository` account manages a specific collateral mint.",
        "They will only transact for this specific mint to segregate funding",
        "rates/deposit yield and risks.",
        "",
        "Note:",
        "Each `MangoDepository` owns a MangoAccount for trading spot/perp,",
        "leveraged.",
        "",
        "Update:",
        "In the new version of the MangoMarket Accounts",
        "this become mandatory too. (we are still using the old init)",
        ""
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 UXDProgram on chain account bound to a Controller instance",
            "The `MangoDepository` manages a MangoAccount for a single Collateral"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#5 The collateral mint used by the `depository` instance"
          ]
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 The insurance mint used by the `depository` instance"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#8 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#10 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#11 MangoMarketv3 Program"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12 Rent Sysvar"
          ]
        }
      ],
      "args": [
        {
          "name": "redeemableAmountUnderManagementCap",
          "type": "u128"
        }
      ]
    },
    {
      "name": "depositInsuranceToMangoDepository",
      "docs": [
        "Deposit `MangoDepository.quote_mint` tokens in the `MangoDepository`",
        "underlying `MangoAccount`",
        "",
        "Parameters:",
        "- amount: the amount of quote token to deposit in native unit.",
        "",
        "Note:",
        "Each `MangoDepository` underlying `MangoAccount` uses leverage to open",
        "and maintain short positions.",
        "",
        "Note:",
        "The LTV (Loan to value) ratio is different depending of the mint of",
        "the `MangoDepository.collateral_mint`.",
        "",
        "Note:",
        "LTV for BTC/ETH/SOL is at 0.9:1 (0.9$ lent for 1$ of value deposited).",
        "MangoMarkets Assets specs : https://docs.mango.markets/mango/token-specs",
        "",
        "Note:",
        "Beyond 80% the `MangoAccount` cannot borrow further, disabling the",
        "redemption of redeemable tokens or the withdrawal of deposited insurance.",
        "(Although the insurance should be gone at that point due to funding,",
        "except in the case of sharp collateral price increase without rebalancing)",
        "",
        "Note:",
        "Beyond 90% the `MangoAccount` can be liquidated by other mango accounts.",
        "(And borrows/withdraws are still disabled)",
        "",
        "Note:",
        "As the funding rate care be either negative or positive, the insurance",
        "is there as a buffer to ensure that redeemables can be swapped back",
        "at all time (by unwinding the backing amount of delta neutral",
        "position).",
        ""
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#2 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 UXDProgram on chain account bound to a Controller instance",
            "The `MangoDepository` manages a MangoAccount for a single Collateral"
          ]
        },
        {
          "name": "authorityQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 The `authority`'s ATA for the `quote_mint`",
            "Will be debited during this call"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoRootBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 [MangoMarkets CPI] Root Bank for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9 [MangoMarkets CPI] Node Bank for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Vault for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#11 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "withdrawInsuranceFromMangoDepository",
      "docs": [
        "Withdraw `MangoDepository.quote_mint` tokens from the `MangoDepository`",
        "underlying `MangoAccount`, if any available, in the limit of the account",
        "borrow health.",
        "",
        "Parameters:",
        "- amount: the amount of quote token to withdraw in native unit.",
        "",
        "Note:",
        "Withdrawal cannot borrow, nor bring the health of the account in",
        "liquidation territory.",
        "",
        "Notes:",
        "The `MangoDepository.insurance_amount_deposited` tracks the amount of",
        "`MangoDepository.quote_mint` tokens deposited, but does not represent",
        "the available amount as it moves depending of funding rates and",
        "perp positions PnL settlement (temporarily).",
        ""
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#2 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 UXDProgram on chain account bound to a Controller instance",
            "The `MangoDepository` manages a MangoAccount for a single Collateral"
          ]
        },
        {
          "name": "authorityQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 The `user`'s ATA for the `controller`'s `redeemable_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#7 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#8 [MangoMarkets CPI] Signer PDA"
          ]
        },
        {
          "name": "mangoRootBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9 [MangoMarkets CPI] Root Bank for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Node Bank for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11 [MangoMarkets CPI] Vault for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#13 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#14 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "rebalanceMangoDepositoryLite",
      "docs": [
        "Rebalance the delta neutral position of the underlying `MangoDepository`.",
        "",
        "Parameters:",
        "- max_rebalancing_amount: the maximum amount of quote this rebalance",
        "instruction will attempt to rebalance, in native unit.",
        "- polarity: the direction of the rebalancing. This is known on chain",
        "but required as an argument for clarity.",
        "- limit_price: the worst price the user is willing to trade at.",
        "",
        "Note:",
        "Acts as a swap, reducing the oustanding PnL (paper profit or losses) on",
        "the underlying `MangoAccount`.",
        "",
        "Note:",
        "This is the \"lite\" version as it force the caller to input some quote or",
        "collateral. This is done to skip the spot order on mango, saving computing",
        "and also bypassing the issue with teh 34 accounts limits.",
        "A new version is designed and waiting for the TransactionV2 proposal to hit",
        "along with the 1M computing units.",
        "",
        "Note:",
        "Paper profits are represented in Quote, it's currently USDC on",
        "MangoMarkets, as of 02/17/2022.",
        "",
        "Note:",
        "This call should goes with a call to `@uxdprotocol/uxd-client`'s",
        "`MangoDepository.settleMangoDepositoryMangoAccountPnl()`, which convert paper",
        "profits or losses into realized gain/losses. Once rebalancing is out,",
        "since it's permissionless, the PnL settlement should be called once in a while",
        "to make sure that unsettled Positive PNL accumulates and that the MangoAccount",
        "has to pay borrow rates for it. Some day when computing is plentiful and input",
        "accounts are increased through TransactionsV2 proposal, we can",
        "also call the onchain version.",
        "",
        "Note:",
        "TEMPORARY Although this create the associated token account for WSOL",
        "when the PnL is Negative, it's too short on computing. Please create beforehand."
      ],
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Public call accessible to any user",
            "Note - Mut required for WSOL unwrapping"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 UXDProgram on chain account bound to a Controller instance",
            "The `MangoDepository` manages a MangoAccount for a single Collateral"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#5 The collateral mint used by the `depository` instance",
            "Required to create the user_collateral ATA if needed"
          ]
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 The quote mint used by the `depository` instance",
            "Required to create the user_quote ATA if needed"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The `user`'s TA for the `depository`'s `collateral_mint`",
            "Will be debited during this instruction when `Polarity` is positive",
            "Will be credited during this instruction when `Polarity` is negative"
          ]
        },
        {
          "name": "userQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The `user`'s TA for the `depository`'s `quote_mint`",
            "Will be credited during this instruction when `Polarity` is positive",
            "Will be debited during this instruction when `Polarity` is negative"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Signer PDA"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#12 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoRootBankQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#13 [MangoMarkets CPI] Root Bank for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoNodeBankQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#14 [MangoMarkets CPI] Node Bank for the `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoVaultQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#15 [MangoMarkets CPI] Vault `depository`'s `quote_mint`"
          ]
        },
        {
          "name": "mangoRootBankCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#16 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoNodeBankCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#17 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoVaultCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#18 [MangoMarkets CPI] Vault for `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#19 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market"
          ]
        },
        {
          "name": "mangoBids",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#20 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids"
          ]
        },
        {
          "name": "mangoAsks",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#21 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks"
          ]
        },
        {
          "name": "mangoEventQueue",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#22 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#23 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#24 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#25 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "maxRebalancingAmount",
          "type": "u64"
        },
        {
          "name": "polarity",
          "type": {
            "defined": "PnlPolarity"
          }
        },
        {
          "name": "limitPrice",
          "type": "f32"
        }
      ]
    },
    {
      "name": "mintWithMangoDepository",
      "docs": [
        "Mint redeemable tokens in exchange of `MangoDepository.collateral_mint`",
        "tokens, increasing the size of the delta neutral position.",
        "",
        "Parameters:",
        "- collateral_amount: the amount of collateral to use, in",
        "collateral_mint native unit.",
        "- limit_price: the worse price the user is willing to trade at.",
        "",
        "Flow:",
        "- Starts by scanning the order book for the amount that we can fill.",
        "- Deposit to Mango account",
        "- Using the spot collateral deposited, the short perp position of equivalent",
        "size if opened (FoK emulated by using mango IoC + 100% fill verification).",
        "- Deducts the taker_fees (ceiled) form the value of the opened short, and",
        "mints the redeemable, then transfer to the user.",
        "- Internal accounting update + anchor event emission.",
        "",
        "Note:",
        "The caller pays for the incurred slippage and taker_fee (4bps at the time",
        "of writing). This ensures that the system stay \"closed\".",
        "",
        "Note:",
        "The value of the collateral is derived from the COLLATERAL-PERP price,",
        "expressed in USD value.",
        ""
      ],
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Public call accessible to any user"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 UXDProgram on chain account bound to a Controller instance.",
            "The `MangoDepository` manages a MangoAccount for a single Collateral."
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5 The redeemable mint managed by the `controller` instance",
            "Tokens will be minted during this instruction"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6 The `user`'s TA for the `depository` `collateral_mint`",
            "Will be debited during this instruction"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The `user`'s TA for the `controller`'s `redeemable_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoRootBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#12 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#13 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#14 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market"
          ]
        },
        {
          "name": "mangoBids",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#15 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids"
          ]
        },
        {
          "name": "mangoAsks",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#16 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks"
          ]
        },
        {
          "name": "mangoEventQueue",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#17 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#18 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#19 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#20 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "collateralAmount",
          "type": "u64"
        },
        {
          "name": "limitPrice",
          "type": "f32"
        }
      ]
    },
    {
      "name": "redeemFromMangoDepository",
      "docs": [
        "Redeem `MangoDepository.collateral_mint` by burning redeemable",
        "tokens, and unwind a part of the delta neutral position.",
        "",
        "Parameters:",
        "- redeemable_amount: the amount of collateral to use, in",
        "redeemable_mint native unit.",
        "- limit_price: the worse price the user is willing to trade at.",
        "",
        "Flow:",
        "- Starts by scanning the order book to find the best order for",
        "the redeemable_amount fillable (the requested amount minus max",
        "fees, as we repay them by keeping a piece of the DN position).",
        "- Closes the equivalent part of the delta neutral position (FoK",
        "emulated by using mango IoC + 100% fill verification).",
        "- Deducts the taker_fees (ceiled) form the value of the opened short, and",
        "transfer user redeemable token for that amount.",
        "- Burns the redeemable equivalent to fees + closed position,",
        "then withdraw resulting equivalent collateral to the user",
        "- Internal accounting update + anchor event emission.",
        "",
        "Note:",
        "The caller pays for the incurred slippage and taker_fee (4bps at the time",
        "of writing). This ensures that the system stay \"closed\".",
        "",
        "Note:",
        "The value of the collateral is derived from the COLLATERAL-PERP price,",
        "expressed in USD value.",
        ""
      ],
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#1 Public call accessible to any user",
            "Note - Mut required for WSOL unwrapping"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 UXDProgram on chain account bound to a Controller instance.",
            "The `MangoDepository` manages a MangoAccount for a single Collateral."
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#5 The collateral mint used by the `depository` instance",
            "Required to create the user_collateral ATA if needed"
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6 The redeemable mint managed by the `controller` instance",
            "Tokens will be burnt during this instruction"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The `user`'s ATA for the `depository`'s `collateral_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The `user`'s ATA for the `controller`'s `redeemable_mint`",
            "Will be debited during this instruction"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12 [MangoMarkets CPI] Signer PDA"
          ]
        },
        {
          "name": "mangoRootBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#13 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#14 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#15 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#16 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market"
          ]
        },
        {
          "name": "mangoBids",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#17 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids"
          ]
        },
        {
          "name": "mangoAsks",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#18 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks"
          ]
        },
        {
          "name": "mangoEventQueue",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#19 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#20 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#21 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#22 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "redeemableAmount",
          "type": "u64"
        },
        {
          "name": "limitPrice",
          "type": "f32"
        }
      ]
    },
    {
      "name": "quoteMintWithMangoDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Public call accessible to any user"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 UXDProgram on chain account bound to a Controller instance.",
            "The `MangoDepository` manages a MangoAccount for a single Collateral."
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5 The redeemable mint managed by the `controller` instance",
            "Tokens will be minted during this instruction"
          ]
        },
        {
          "name": "userQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6 The `user`'s ATA for one the `mango depository`s `quote_mint`",
            "Will be debited during this instruction"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The `user`'s ATA for the `controller`'s `redeemable_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoRootBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#12 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#13 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#14 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#15 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#16 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#17 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "quoteAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "quoteRedeemFromMangoDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Public call accessible to any user"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4 UXDProgram on chain account bound to a Controller instance.",
            "The `MangoDepository` manages a MangoAccount for a single Collateral."
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5 The redeemable mint managed by the `controller` instance",
            "Tokens will be minted during this instruction"
          ]
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 The quote mint of the depository"
          ]
        },
        {
          "name": "userQuote",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The `user`'s ATA for one the `mango depository`s `quote_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The `user`'s ATA for the `controller`'s `redeemable_mint`",
            "Will be debited during this instruction"
          ]
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9 The MangoMarkets Account (MangoAccount) managed by the `depository`",
            "CHECK : Seeds checked. Depository registered"
          ]
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#10 [MangoMarkets CPI] Index grouping perp and spot markets"
          ]
        },
        {
          "name": "mangoCache",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11 [MangoMarkets CPI] Cache"
          ]
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12 [MangoMarkets CPI] Signer PDA"
          ]
        },
        {
          "name": "mangoRootBank",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#13 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#14 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`"
          ]
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#15 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#16 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#17 Token Program"
          ]
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#18 MangoMarketv3 Program"
          ]
        }
      ],
      "args": [
        {
          "name": "redeemableAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "editMangoDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#2 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 UXDProgram on chain account bound to a Controller instance.",
            "The `MangoDepository` manages a MangoAccount for a single Collateral."
          ]
        }
      ],
      "args": [
        {
          "name": "fields",
          "type": {
            "defined": "EditMangoDepositoryFields"
          }
        }
      ]
    },
    {
      "name": "editMercurialVaultDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#2 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 UXDProgram on chain account bound to a Controller instance.",
            "The `MercurialVaultDepository` manages a MercurialVaultAccount for a single Collateral."
          ]
        }
      ],
      "args": [
        {
          "name": "fields",
          "type": {
            "defined": "EditMercurialVaultDepositoryFields"
          }
        }
      ]
    },
    {
      "name": "disableDepositoryRegularMinting",
      "docs": [
        "Disable or enable regular minting for given Mango Depository.",
        "",
        "Parameters:",
        "- disable: true to disable, false to enable.",
        "",
        "Note:",
        "The disabled flag is false by default that a freshly registered mango depository has enabled regular minting.",
        "This ix is for toggling that flag.",
        ""
      ],
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1 Authored call accessible only to the signer matching Controller.authority"
          ]
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#2 The top level UXDProgram on chain account managing the redeemable mint"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3 UXDProgram on chain account bound to a Controller instance",
            "The `MangoDepository` manages a MangoAccount for a single Collateral"
          ]
        }
      ],
      "args": [
        {
          "name": "disable",
          "type": "bool"
        }
      ]
    },
    {
      "name": "mintWithMercurialVaultDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4"
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#7"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8"
          ]
        },
        {
          "name": "depositoryLpTokenVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9",
            "Token account holding the LP tokens minted by depositing collateral on mercurial vault"
          ]
        },
        {
          "name": "mercurialVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10"
          ]
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11"
          ]
        },
        {
          "name": "mercurialVaultCollateralTokenSafe",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#12",
            "Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault."
          ]
        },
        {
          "name": "mercurialVaultProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#13"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#14"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#15"
          ]
        }
      ],
      "args": [
        {
          "name": "collateralAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "registerMercurialVaultDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#5"
          ]
        },
        {
          "name": "mercurialVault",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6"
          ]
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#7"
          ]
        },
        {
          "name": "depositoryLpTokenVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8",
            "Token account holding the LP tokens minted by depositing collateral on mercurial vault"
          ]
        },
        {
          "name": "interestsAndFeesRedeemAuthority",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9",
            "Only wallet able to claim interests and fees"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#10"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#11"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12"
          ]
        }
      ],
      "args": [
        {
          "name": "mintingFeeInBps",
          "type": "u8"
        },
        {
          "name": "redeemingFeeInBps",
          "type": "u8"
        },
        {
          "name": "redeemableAmountUnderManagementCap",
          "type": "u128"
        }
      ]
    },
    {
      "name": "redeemFromMercurialVaultDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4"
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#7"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8"
          ]
        },
        {
          "name": "depositoryLpTokenVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9",
            "Token account holding the LP tokens minted by depositing collateral on mercurial vault"
          ]
        },
        {
          "name": "mercurialVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10"
          ]
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#11"
          ]
        },
        {
          "name": "mercurialVaultCollateralTokenSafe",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#12",
            "Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault."
          ]
        },
        {
          "name": "mercurialVaultProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#13"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#14"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#15"
          ]
        }
      ],
      "args": [
        {
          "name": "redeemableAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "collectInterestsAndFeesFromMercurialVaultDepository",
      "accounts": [
        {
          "name": "interestsAndFeesRedeemAuthority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "#1"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "#2"
          ]
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#3"
          ]
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#4"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#5"
          ]
        },
        {
          "name": "interestsAndFeesRedeemAuthorityCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6"
          ]
        },
        {
          "name": "depositoryLpTokenVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7",
            "Token account holding the LP tokens minted by depositing collateral on mercurial vault"
          ]
        },
        {
          "name": "mercurialVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8"
          ]
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9"
          ]
        },
        {
          "name": "mercurialVaultCollateralTokenSafe",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#10",
            "Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault."
          ]
        },
        {
          "name": "mercurialVaultProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#11"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#12"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#13"
          ]
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "controller",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "redeemableMintBump",
            "type": "u8"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "redeemableMint",
            "type": "publicKey"
          },
          {
            "name": "redeemableMintDecimals",
            "type": "u8"
          },
          {
            "name": "registeredMangoDepositories",
            "type": {
              "array": [
                "publicKey",
                8
              ]
            }
          },
          {
            "name": "registeredMangoDepositoriesCount",
            "type": "u8"
          },
          {
            "name": "redeemableGlobalSupplyCap",
            "type": "u128"
          },
          {
            "name": "mangoDepositoriesRedeemableSoftCap",
            "type": "u64"
          },
          {
            "name": "redeemableCirculatingSupply",
            "type": "u128"
          },
          {
            "name": "mangoDepositoriesQuoteRedeemableSoftCap",
            "type": "u64"
          },
          {
            "name": "registeredMercurialVaultDepositories",
            "type": {
              "array": [
                "publicKey",
                4
              ]
            }
          },
          {
            "name": "registeredMercurialVaultDepositoriesCount",
            "type": "u8"
          },
          {
            "name": "interestsAndFeesTotalCollected",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "mangoDepository",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "unused",
            "type": {
              "array": [
                "u8",
                2
              ]
            }
          },
          {
            "name": "mangoAccountBump",
            "type": "u8"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "collateralMint",
            "type": "publicKey"
          },
          {
            "name": "collateralMintDecimals",
            "type": "u8"
          },
          {
            "name": "unused2",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "quoteMint",
            "type": "publicKey"
          },
          {
            "name": "unused3",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "quoteMintDecimals",
            "type": "u8"
          },
          {
            "name": "mangoAccount",
            "type": "publicKey"
          },
          {
            "name": "controller",
            "type": "publicKey"
          },
          {
            "name": "insuranceAmountDeposited",
            "type": "u128"
          },
          {
            "name": "collateralAmountDeposited",
            "type": "u128"
          },
          {
            "name": "redeemableAmountUnderManagement",
            "type": "u128"
          },
          {
            "name": "totalAmountPaidTakerFee",
            "type": "u128"
          },
          {
            "name": "totalAmountRebalanced",
            "type": "u128"
          },
          {
            "name": "netQuoteMinted",
            "type": "i128"
          },
          {
            "name": "quoteMintAndRedeemFee",
            "type": "u8"
          },
          {
            "name": "totalQuoteMintAndRedeemFees",
            "type": "u128"
          },
          {
            "name": "regularMintingDisabled",
            "type": "bool"
          },
          {
            "name": "redeemableAmountUnderManagementCap",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "mercurialVaultDepository",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "collateralMint",
            "type": "publicKey"
          },
          {
            "name": "collateralMintDecimals",
            "type": "u8"
          },
          {
            "name": "controller",
            "type": "publicKey"
          },
          {
            "name": "collateralAmountDeposited",
            "type": "u128"
          },
          {
            "name": "redeemableAmountUnderManagement",
            "type": "u128"
          },
          {
            "name": "mercurialVault",
            "type": "publicKey"
          },
          {
            "name": "mercurialVaultLpMint",
            "type": "publicKey"
          },
          {
            "name": "mercurialVaultLpMintDecimals",
            "type": "u8"
          },
          {
            "name": "lpTokenVault",
            "type": "publicKey"
          },
          {
            "name": "lpTokenVaultBump",
            "type": "u8"
          },
          {
            "name": "mintingFeeInBps",
            "type": "u8"
          },
          {
            "name": "redeemingFeeInBps",
            "type": "u8"
          },
          {
            "name": "mintingFeeTotalAccrued",
            "type": "u128"
          },
          {
            "name": "redeemingFeeTotalAccrued",
            "type": "u128"
          },
          {
            "name": "redeemableAmountUnderManagementCap",
            "type": "u128"
          },
          {
            "name": "mintingDisabled",
            "type": "bool"
          },
          {
            "name": "interestsAndFeesRedeemAuthority",
            "type": "publicKey"
          },
          {
            "name": "interestsAndFeesTotalCollected",
            "type": "u128"
          },
          {
            "name": "lastInterestsAndFeesCollectionUnixTimestamp",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "EditControllerFields",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "quoteMintAndRedeemSoftCap",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "redeemableSoftCap",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "redeemableGlobalSupplyCap",
            "type": {
              "option": "u128"
            }
          }
        ]
      }
    },
    {
      "name": "EditMangoDepositoryFields",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "quoteMintAndRedeemFee",
            "type": {
              "option": "u8"
            }
          },
          {
            "name": "redeemableAmountUnderManagementCap",
            "type": {
              "option": "u128"
            }
          }
        ]
      }
    },
    {
      "name": "EditMercurialVaultDepositoryFields",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "redeemableAmountUnderManagementCap",
            "type": {
              "option": "u128"
            }
          },
          {
            "name": "mintingFeeInBps",
            "type": {
              "option": "u8"
            }
          },
          {
            "name": "redeemingFeeInBps",
            "type": {
              "option": "u8"
            }
          },
          {
            "name": "mintingDisabled",
            "type": {
              "option": "bool"
            }
          },
          {
            "name": "interestsAndFeesRedeemAuthority",
            "type": {
              "option": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "PnlPolarity",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Positive"
          },
          {
            "name": "Negative"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "InitializeControllerEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "authority",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "SetRedeemableGlobalSupplyCapEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "redeemableGlobalSupplyCap",
          "type": "u128",
          "index": false
        }
      ]
    },
    {
      "name": "RegisterMangoDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "collateralMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "insuranceMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "mangoAccount",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "RegisterMangoDepositoryEventV2",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "depositoryVersion",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "collateralMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "quoteMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "mangoAccount",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "RegisterMercurialVaultDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "depositoryVersion",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "mercurialVault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depositoryLpTokenVault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "collateralMint",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "SetMangoDepositoryRedeemableSoftCapEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "redeemableMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "redeemableMintDecimals",
          "type": "u8",
          "index": false
        },
        {
          "name": "redeemableSoftCap",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "DepositInsuranceToDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "quoteMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "quoteMintDecimals",
          "type": "u8",
          "index": false
        },
        {
          "name": "depositedAmount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "WithdrawInsuranceFromMangoDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "insuranceMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "insuranceMintDecimals",
          "type": "u8",
          "index": false
        },
        {
          "name": "withdrawnAmount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "WithdrawInsuranceFromDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "quoteMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "quoteMintDecimals",
          "type": "u8",
          "index": false
        },
        {
          "name": "withdrawnAmount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "MintWithMangoDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "collateralAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "limitPrice",
          "type": "f32",
          "index": false
        },
        {
          "name": "baseDelta",
          "type": "i64",
          "index": false
        },
        {
          "name": "quoteDelta",
          "type": "i64",
          "index": false
        },
        {
          "name": "feeDelta",
          "type": "i64",
          "index": false
        }
      ]
    },
    {
      "name": "RedeemFromMangoDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "redeemableAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "limitPrice",
          "type": "f32",
          "index": false
        },
        {
          "name": "baseDelta",
          "type": "i64",
          "index": false
        },
        {
          "name": "quoteDelta",
          "type": "i64",
          "index": false
        },
        {
          "name": "feeDelta",
          "type": "i64",
          "index": false
        }
      ]
    },
    {
      "name": "RebalanceMangoDepositoryLiteEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
        },
        {
          "name": "depositoryVersion",
          "type": "u8",
          "index": false
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "polarity",
          "type": {
            "defined": "PnlPolarity"
          },
          "index": false
        },
        {
          "name": "rebalancingAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "rebalancedAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "limitPrice",
          "type": "f32",
          "index": false
        },
        {
          "name": "baseDelta",
          "type": "i64",
          "index": false
        },
        {
          "name": "quoteDelta",
          "type": "i64",
          "index": false
        },
        {
          "name": "feeDelta",
          "type": "i64",
          "index": false
        }
      ]
    },
    {
      "name": "SetMangoDepositoryQuoteMintAndRedeemSoftCapEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "quoteMintAndRedeemSoftCap",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "SetMangoDepositoryQuoteMintAndRedeemFeeEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "quoteMintAndRedeemFee",
          "type": "u8",
          "index": true
        }
      ]
    },
    {
      "name": "SetMangoDepositoryRedeemableAmountUnderManagementCapEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "redeemableAmountUnderManagementCap",
          "type": "u128",
          "index": true
        }
      ]
    },
    {
      "name": "SetMercurialVaultDepositoryRedeemableAmountUnderManagementCapEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "redeemableAmountUnderManagementCap",
          "type": "u128",
          "index": true
        }
      ]
    },
    {
      "name": "SetMercurialVaultDepositoryMintingFeeInBpsEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "mintingFeeInBps",
          "type": "u8",
          "index": true
        }
      ]
    },
    {
      "name": "SetMercurialVaultDepositoryRedeemingFeeInBpsEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "redeemingFeeInBps",
          "type": "u8",
          "index": true
        }
      ]
    },
    {
      "name": "SetMercurialVaultDepositoryMintingDisabledEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "mintingDisabled",
          "type": "bool",
          "index": true
        }
      ]
    },
    {
      "name": "SetMercurialVaultDepositoryInterestsAndFeesRedeemAuthorityEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "interestsAndFeesRedeemAuthority",
          "type": "publicKey",
          "index": true
        }
      ]
    },
    {
      "name": "QuoteRedeemFromMangoDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "quoteRedeemableAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "quoteRedeemFee",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "QuoteMintWithMangoDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "user",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "quoteMintAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "quoteMintFee",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "DisableDepositoryRegularMintingEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": true
        },
        {
          "name": "controller",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "depository",
          "type": "publicKey",
          "index": true
        },
        {
          "name": "regularMintingDisabled",
          "type": "bool",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidRedeemableMintDecimals",
      "msg": "The redeemable mint decimals must be between 0 and 9 (inclusive)."
    },
    {
      "code": 6001,
      "name": "InvalidRedeemableGlobalSupplyCap",
      "msg": "Redeemable global supply above."
    },
    {
      "code": 6002,
      "name": "RootBankIndexNotFound",
      "msg": "The associated mango root bank index cannot be found for the deposited coin.."
    },
    {
      "code": 6003,
      "name": "InvalidLimitPrice",
      "msg": "The provided limit_price value is invalid, must be > 0"
    },
    {
      "code": 6004,
      "name": "EffectiveOrderPriceBeyondLimitPrice",
      "msg": "Could not fill the order given order book state and provided slippage."
    },
    {
      "code": 6005,
      "name": "InvalidCollateralAmount",
      "msg": "Collateral amount cannot be 0"
    },
    {
      "code": 6006,
      "name": "InvalidQuoteAmount",
      "msg": "Quote amount must be > 0 in order to mint."
    },
    {
      "code": 6007,
      "name": "InvalidRedeemableAmount",
      "msg": "Redeemable amount must be > 0 in order to redeem."
    },
    {
      "code": 6008,
      "name": "InsufficientCollateralAmount",
      "msg": "The balance of the collateral ATA is not enough to fulfill the mint operation."
    },
    {
      "code": 6009,
      "name": "InsufficientQuoteAmountMint",
      "msg": "The balance of the quote ATA is not enough to fulfil the mint operation."
    },
    {
      "code": 6010,
      "name": "InsufficientRedeemableAmountMint",
      "msg": "The balance of the redeemable ATA is not enough to fulfil the redeem operation."
    },
    {
      "code": 6011,
      "name": "InsufficientRedeemableAmount",
      "msg": "The balance of the redeemable ATA is not enough to fulfill the redeem operation."
    },
    {
      "code": 6012,
      "name": "PerpOrderPartiallyFilled",
      "msg": "The perp position could not be fully filled with the provided slippage."
    },
    {
      "code": 6013,
      "name": "RedeemableGlobalSupplyCapReached",
      "msg": "Minting amount would go past the Redeemable Global Supply Cap."
    },
    {
      "code": 6014,
      "name": "RedeemableMangoAmountUnderManagementCap",
      "msg": "Minting amount would go past the mango depository Redeemable Amount Under Management Cap."
    },
    {
      "code": 6015,
      "name": "RedeemableMercurialVaultAmountUnderManagementCap",
      "msg": "Minting amount would go past the mercurial vault depository Redeemable Amount Under Management Cap."
    },
    {
      "code": 6016,
      "name": "MangoDepositoriesSoftCapOverflow",
      "msg": "Operation not allowed due to being over the Mango Redeemable soft Cap."
    },
    {
      "code": 6017,
      "name": "MaxNumberOfMangoDepositoriesRegisteredReached",
      "msg": "Cannot register more mango depositories, the limit has been reached."
    },
    {
      "code": 6018,
      "name": "InvalidInsuranceAmount",
      "msg": "The amount to withdraw from the Insurance Fund must be superior to zero.."
    },
    {
      "code": 6019,
      "name": "InsufficientAuthorityQuoteAmount",
      "msg": "The Quote ATA from authority doesn't have enough balance."
    },
    {
      "code": 6020,
      "name": "InvalidRebalancedAmount",
      "msg": "The rebalanced amount must be superior to zero.."
    },
    {
      "code": 6021,
      "name": "InsufficientOrderBookDepth",
      "msg": "Insufficient order book depth for order."
    },
    {
      "code": 6022,
      "name": "InvalidExecutedOrderSize",
      "msg": "The executed order size does not match the expected one."
    },
    {
      "code": 6023,
      "name": "InvalidMangoDepositoriesRedeemableSoftCap",
      "msg": "Mango depositories redeemable soft cap above."
    },
    {
      "code": 6024,
      "name": "InvalidQuoteDelta",
      "msg": "Quote_lot_delta can't be 0."
    },
    {
      "code": 6025,
      "name": "InvalidOrderDirection",
      "msg": "The perp order wasn't executed in the right direction."
    },
    {
      "code": 6026,
      "name": "MathError",
      "msg": "Math error."
    },
    {
      "code": 6027,
      "name": "SlippageReached",
      "msg": "The order couldn't be executed with the provided slippage."
    },
    {
      "code": 6028,
      "name": "InvalidRebalancingAmount",
      "msg": "The rebalancing amount must be above 0."
    },
    {
      "code": 6029,
      "name": "InsufficientQuoteAmount",
      "msg": "The Quote amount in the provided user_quote ATA must be >= max_amount_rebalancing."
    },
    {
      "code": 6030,
      "name": "InvalidPnlPolarity",
      "msg": "The PnL polarity provided is not the same as the perp position's one."
    },
    {
      "code": 6031,
      "name": "RebalancingError",
      "msg": "The rebalanced amount doesn't match the expected rebalance amount."
    },
    {
      "code": 6032,
      "name": "BumpError",
      "msg": "A bump was expected but is missing."
    },
    {
      "code": 6033,
      "name": "OrderSizeBelowMinLotSize",
      "msg": "The order is below size is below the min lot size."
    },
    {
      "code": 6034,
      "name": "InvalidCollateralDelta",
      "msg": "The collateral delta post perp order doesn't match the planned one."
    },
    {
      "code": 6035,
      "name": "MangoPerpMarketIndexNotFound",
      "msg": "The perp market index could not be found for this MangoMarkets Pair."
    },
    {
      "code": 6036,
      "name": "CannotLoadMangoGroup",
      "msg": "Could not load the provided MangoGroup account."
    },
    {
      "code": 6037,
      "name": "QuantityBelowContractSize",
      "msg": "The order quantity is below contract_size of the perp market."
    },
    {
      "code": 6038,
      "name": "QuoteAmountTooHigh",
      "msg": "The amount trying to be quote minted is larger than quote mintable."
    },
    {
      "code": 6039,
      "name": "RedeemableAmountTooHigh",
      "msg": "The amount trying to be quote redeemed is larger than quote redeemable."
    },
    {
      "code": 6040,
      "name": "MintingDisabled",
      "msg": "Minting is disabled for the current depository."
    },
    {
      "code": 6041,
      "name": "MintingAlreadyDisabledOrEnabled",
      "msg": "Minting is already disabled/enabled."
    },
    {
      "code": 6042,
      "name": "QuoteAmountExceedsSoftCap",
      "msg": "The quote amount requested is beyond the soft cap limitation."
    },
    {
      "code": 6043,
      "name": "InvalidQuoteCurrency",
      "msg": "The quote currency is not the expected one."
    },
    {
      "code": 6044,
      "name": "InvalidMercurialVaultLpMint",
      "msg": "The mercurial vault lp mint does not match the Depository's one."
    },
    {
      "code": 6045,
      "name": "MaxNumberOfMercurialVaultDepositoriesRegisteredReached",
      "msg": "Cannot register more mercurial vault depositories, the limit has been reached."
    },
    {
      "code": 6046,
      "name": "MercurialVaultDoNotMatchCollateral",
      "msg": "The provided collateral do not match the provided mercurial vault token."
    },
    {
      "code": 6047,
      "name": "CollateralMintEqualToRedeemableMint",
      "msg": "Collateral mint should be different than redeemable mint."
    },
    {
      "code": 6048,
      "name": "CollateralMintNotAllowed",
      "msg": "Provided collateral mint is not allowed."
    },
    {
      "code": 6049,
      "name": "MinimumMintedRedeemableAmountError",
      "msg": "Mint resulted to 0 redeemable token being minted."
    },
    {
      "code": 6050,
      "name": "MinimumRedeemedCollateralAmountError",
      "msg": "Redeem resulted to 0 collateral token being redeemed."
    },
    {
      "code": 6051,
      "name": "InvalidDepositoryLpTokenVault",
      "msg": "The depository lp token vault does not match the Depository's one."
    },
    {
      "code": 6052,
      "name": "UnAllowedMangoGroup",
      "msg": "The mango group is not accepted."
    },
    {
      "code": 6053,
      "name": "InvalidMercurialVaultInterestsAndFeesAuthority",
      "msg": "Only the mercurial vault interests and fees authority can access this instructions."
    },
    {
      "code": 6054,
      "name": "InvalidAuthority",
      "msg": "Only the Program initializer authority can access this instructions."
    },
    {
      "code": 6055,
      "name": "InvalidController",
      "msg": "The Depository's controller doesn't match the provided Controller."
    },
    {
      "code": 6056,
      "name": "InvalidDepository",
      "msg": "The Depository provided is not registered with the Controller."
    },
    {
      "code": 6057,
      "name": "InvalidCollateralMint",
      "msg": "The provided collateral mint does not match the depository's collateral mint."
    },
    {
      "code": 6058,
      "name": "InvalidQuoteMint",
      "msg": "The provided quote mint does not match the depository's quote mint."
    },
    {
      "code": 6059,
      "name": "InvalidMangoAccount",
      "msg": "The Mango Account isn't the Depository one."
    },
    {
      "code": 6060,
      "name": "InvalidRedeemableMint",
      "msg": "The Redeemable Mint provided does not match the Controller's one."
    },
    {
      "code": 6061,
      "name": "InvalidDexMarket",
      "msg": "The provided perp_market is not the one tied to this Depository."
    },
    {
      "code": 6062,
      "name": "InvalidOwner",
      "msg": "The provided token account is not owner by the expected party."
    },
    {
      "code": 6063,
      "name": "InvalidMaxBaseQuantity",
      "msg": "The max base quantity must be above 0."
    },
    {
      "code": 6064,
      "name": "InvalidMaxQuoteQuantity",
      "msg": "The max quote quantity must be above 0."
    },
    {
      "code": 6065,
      "name": "InvalidMercurialVault",
      "msg": "The provided mercurial vault does not match the Depository's one."
    },
    {
      "code": 6066,
      "name": "InvalidMercurialVaultCollateralTokenSafe",
      "msg": "The provided mercurial vault collateral token safe does not match the mercurial vault one."
    },
    {
      "code": 6067,
      "name": "Default",
      "msg": "Default - Check the source code for more info."
    }
  ]
};
