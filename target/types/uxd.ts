export type Uxd = {
  "version": "3.1.0",
  "name": "uxd",
  "instructions": [
    {
      "name": "initializeController",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "name": "setRedeemableGlobalSupplyCap",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "redeemableGlobalSupplyCap",
          "type": "u128"
        }
      ]
    },
    {
      "name": "setMangoDepositoriesRedeemableSoftCap",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "redeemableSoftCap",
          "type": "u64"
        }
      ]
    },
    {
      "name": "registerMangoDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "registerZoDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initializeZoDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoOpenOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoState",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoStateSigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoDexMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoControl",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoDexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "depositInsuranceToMangoDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authorityQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoCache",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoRootBank",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false
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
      "name": "depositInsuranceToZoDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authorityQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoState",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoStateSigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoCache",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoProgram",
          "isMut": false,
          "isSigner": false
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
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authorityQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoGroup",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoCache",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoRootBank",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false
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
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoGroup",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoCache",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoRootBankQuote",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoNodeBankQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoVaultQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoRootBankCollateral",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoNodeBankCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoVaultCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoBids",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAsks",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoEventQueue",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoCache",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoRootBank",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoBids",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAsks",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoEventQueue",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "name": "mintWithZoDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoState",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoStateSigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoCache",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoControl",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoOpenOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoDexMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoReqQ",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoEventQ",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoMarketBids",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoMarketAsks",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoDexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "maxBaseQuantity",
          "type": "u64"
        },
        {
          "name": "maxQuoteQuantity",
          "type": "u64"
        },
        {
          "name": "limitPrice",
          "type": "u64"
        }
      ]
    },
    {
      "name": "redeemFromMangoDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoGroup",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoCache",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoRootBank",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoBids",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAsks",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoEventQueue",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "name": "redeemFromZoDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoState",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoStateSigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoCache",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoControl",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoOpenOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoDexMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoReqQ",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoEventQ",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoMarketBids",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoMarketAsks",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoDexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "maxBaseQuantity",
          "type": "u64"
        },
        {
          "name": "maxQuoteQuantity",
          "type": "u64"
        },
        {
          "name": "limitPrice",
          "type": "u64"
        }
      ]
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
            "name": "reserved",
            "type": "u8"
          },
          {
            "name": "registeredZoDepositories",
            "type": {
              "array": [
                "publicKey",
                8
              ]
            }
          },
          {
            "name": "registeredZoDepositoriesCount",
            "type": "u8"
          },
          {
            "name": "reserved1",
            "type": {
              "defined": "ControllerPadding"
            }
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
            "name": "reserved",
            "type": {
              "defined": "MangoDepositoryPadding"
            }
          }
        ]
      }
    },
    {
      "name": "zoDepository",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "zoAccountBump",
            "type": "u8"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "isInitialized",
            "type": "bool"
          },
          {
            "name": "zoDexMarket",
            "type": "publicKey"
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
            "name": "quoteMint",
            "type": "publicKey"
          },
          {
            "name": "quoteMintDecimals",
            "type": "u8"
          },
          {
            "name": "zoAccount",
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
            "name": "totalAmountRebalanced",
            "type": "u128"
          }
        ]
      }
    }
  ],
  "types": [
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
      "name": "RegisterZoDepositoryEvent",
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
        }
      ]
    },
    {
      "name": "InitializeZoDepositoryEvent",
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
          "name": "zoAccount",
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
      "name": "WithdrawInsuranceFromMangoDepositoryEventV2",
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
      "name": "RedeemFromDepositoryEvent",
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
      "name": "ZoMintEvent",
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
          "name": "collateralDepositedAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "limitPrice",
          "type": "u64",
          "index": false
        },
        {
          "name": "mintedAmount",
          "type": "u64",
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
      "name": "InsufficientCollateralAmount",
      "msg": "The balance of the collateral ATA is not enough to fulfill the mint operation."
    },
    {
      "code": 6007,
      "name": "InvalidRedeemableAmount",
      "msg": "The redeemable amount for redeem must be superior to 0."
    },
    {
      "code": 6008,
      "name": "InsufficientRedeemableAmount",
      "msg": "The balance of the redeemable ATA is not enough to fulfill the redeem operation."
    },
    {
      "code": 6009,
      "name": "PerpOrderPartiallyFilled",
      "msg": "The perp position could not be fully filled with the provided slippage."
    },
    {
      "code": 6010,
      "name": "RedeemableGlobalSupplyCapReached",
      "msg": "Minting amount would go past the Redeemable Global Supply Cap."
    },
    {
      "code": 6011,
      "name": "MangoDepositoriesSoftCapOverflow",
      "msg": "Operation not allowed due to being over the Mango Redeemable soft Cap."
    },
    {
      "code": 6012,
      "name": "MaxNumberOfMangoDepositoriesRegisteredReached",
      "msg": "Cannot register more mango depositories, the limit has been reached."
    },
    {
      "code": 6013,
      "name": "InvalidInsuranceAmount",
      "msg": "The amount to withdraw from the Insurance Fund must be superior to zero.."
    },
    {
      "code": 6014,
      "name": "InsufficientAuthorityQuoteAmount",
      "msg": "The Quote ATA from authority doesn't have enough balance."
    },
    {
      "code": 6015,
      "name": "InvalidRebalancedAmount",
      "msg": "The rebalanced amount must be superior to zero.."
    },
    {
      "code": 6016,
      "name": "InsufficientOrderBookDepth",
      "msg": "Insufficient order book depth for order."
    },
    {
      "code": 6017,
      "name": "InvalidExecutedOrderSize",
      "msg": "The executed order size does not match the expected one."
    },
    {
      "code": 6018,
      "name": "InvalidMangoDepositoriesRedeemableSoftCap",
      "msg": "Mango depositories redeemable soft cap above."
    },
    {
      "code": 6019,
      "name": "InvalidQuoteDelta",
      "msg": "Quote_lot_delta can't be 0."
    },
    {
      "code": 6020,
      "name": "InvalidOrderDirection",
      "msg": "The perp order wasn't executed in the right direction."
    },
    {
      "code": 6021,
      "name": "MathError",
      "msg": "Math error."
    },
    {
      "code": 6022,
      "name": "SlippageReached",
      "msg": "The order couldn't be executed with the provided slippage."
    },
    {
      "code": 6023,
      "name": "InvalidRebalancingAmount",
      "msg": "The rebalancing amount must be above 0."
    },
    {
      "code": 6024,
      "name": "InsufficientQuoteAmount",
      "msg": "The Quote amount in the provided user_quote ATA must be >= max_amount_rebalancing."
    },
    {
      "code": 6025,
      "name": "InvalidPnlPolarity",
      "msg": "The PnL polarity provided is not the same as the perp position's one."
    },
    {
      "code": 6026,
      "name": "RebalancingError",
      "msg": "The rebalanced amount doesn't match the expected rebalance amount."
    },
    {
      "code": 6027,
      "name": "BumpError",
      "msg": "A bump was expected but is missing."
    },
    {
      "code": 6028,
      "name": "OrderSizeBelowMinLotSize",
      "msg": "The order is below size is below the min lot size."
    },
    {
      "code": 6029,
      "name": "InvalidCollateralDelta",
      "msg": "The collateral delta post perp order doesn't match the planned one."
    },
    {
      "code": 6030,
      "name": "MangoPerpMarketIndexNotFound",
      "msg": "The perp market index could not be found for this MangoMarkets Pair."
    },
    {
      "code": 6031,
      "name": "MaxNumberOfZoDepositoriesRegisteredReached",
      "msg": "Cannot register more ZO depositories, the limit has been reached."
    },
    {
      "code": 6032,
      "name": "ZoDepositoriesSoftCapOverflow",
      "msg": "Operation not allowed due to being over the ZO Redeemable soft Cap."
    },
    {
      "code": 6033,
      "name": "InvalidMangoGroup",
      "msg": "Could not load the provided MangoGroup account."
    },
    {
      "code": 6034,
      "name": "QuantityBelowContractSize",
      "msg": "The order quantity is below contract_size of the perp market."
    },
    {
      "code": 6035,
      "name": "InvalidAuthority",
      "msg": "Only the Program initializer authority can access this instructions."
    },
    {
      "code": 6036,
      "name": "InvalidController",
      "msg": "The Depository's controller doesn't match the provided Controller."
    },
    {
      "code": 6037,
      "name": "InvalidDepository",
      "msg": "The Depository provided is not registered with the Controller."
    },
    {
      "code": 6038,
      "name": "InvalidCollateralMint",
      "msg": "The provided collateral mint does not match the depository's collateral mint."
    },
    {
      "code": 6039,
      "name": "InvalidQuoteMint",
      "msg": "The provided quote mint does not match the depository's quote mint."
    },
    {
      "code": 6040,
      "name": "InvalidAuthorityQuoteATAMint",
      "msg": "The authority's Quote ATA's mint does not match the Depository's one."
    },
    {
      "code": 6041,
      "name": "InvalidMangoAccount",
      "msg": "The Mango Account isn't the Depository one."
    },
    {
      "code": 6042,
      "name": "InvalidRedeemableMint",
      "msg": "The Redeemable Mint provided does not match the Controller's one."
    },
    {
      "code": 6043,
      "name": "InvalidZoAccount",
      "msg": "The Zo Account isn't the Depository's one."
    },
    {
      "code": 6044,
      "name": "ZOPerpMarketNotFound",
      "msg": "The Zo PerpMarket index could not be found."
    },
    {
      "code": 6045,
      "name": "ZOPerpMarketInfoNotFound",
      "msg": "The Zo PerpMarketInfo could not be found."
    },
    {
      "code": 6046,
      "name": "ZOOpenOrdersInfoNotFound",
      "msg": "The Zo OpenOrdersInfo could not be found."
    },
    {
      "code": 6047,
      "name": "ZOInvalidControlState",
      "msg": "The Zo Control is in an invalid state."
    },
    {
      "code": 6048,
      "name": "ZoDepositoryAlreadyInitialized",
      "msg": "The Zo depository PDAs are already initialized."
    },
    {
      "code": 6049,
      "name": "ZoDepositoryNotInitialized",
      "msg": "The Zo depository PDAs haven't been initialized yet."
    },
    {
      "code": 6050,
      "name": "InvalidDexMarket",
      "msg": "The provided perp_market is not the one tied to this Depository."
    },
    {
      "code": 6051,
      "name": "Default",
      "msg": "Default - Check the source code for more info"
    }
  ]
};

export const IDL: Uxd = {
  "version": "3.1.0",
  "name": "uxd",
  "instructions": [
    {
      "name": "initializeController",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "name": "setRedeemableGlobalSupplyCap",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "redeemableGlobalSupplyCap",
          "type": "u128"
        }
      ]
    },
    {
      "name": "setMangoDepositoriesRedeemableSoftCap",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "redeemableSoftCap",
          "type": "u64"
        }
      ]
    },
    {
      "name": "registerMangoDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "registerZoDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initializeZoDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoOpenOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoState",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoStateSigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoDexMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoControl",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoDexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "depositInsuranceToMangoDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authorityQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoCache",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoRootBank",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false
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
      "name": "depositInsuranceToZoDepository",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authorityQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoState",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoStateSigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoCache",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoProgram",
          "isMut": false,
          "isSigner": false
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
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authorityQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoGroup",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoCache",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoRootBank",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false
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
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoGroup",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoCache",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoRootBankQuote",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoNodeBankQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoVaultQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoRootBankCollateral",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoNodeBankCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoVaultCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoBids",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAsks",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoEventQueue",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoGroup",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoCache",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoRootBank",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoBids",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAsks",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoEventQueue",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "name": "mintWithZoDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoState",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoStateSigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoCache",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoControl",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoOpenOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoDexMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoReqQ",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoEventQ",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoMarketBids",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoMarketAsks",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoDexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "maxBaseQuantity",
          "type": "u64"
        },
        {
          "name": "maxQuoteQuantity",
          "type": "u64"
        },
        {
          "name": "limitPrice",
          "type": "u64"
        }
      ]
    },
    {
      "name": "redeemFromMangoDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoGroup",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoCache",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoRootBank",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoNodeBank",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoPerpMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoBids",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAsks",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoEventQueue",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mangoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
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
      "name": "redeemFromZoDepository",
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "controller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depository",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoState",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoStateSigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoCache",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoControl",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoOpenOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoDexMarket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoReqQ",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoEventQ",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoMarketBids",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoMarketAsks",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zoDexProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zoProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "maxBaseQuantity",
          "type": "u64"
        },
        {
          "name": "maxQuoteQuantity",
          "type": "u64"
        },
        {
          "name": "limitPrice",
          "type": "u64"
        }
      ]
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
            "name": "reserved",
            "type": "u8"
          },
          {
            "name": "registeredZoDepositories",
            "type": {
              "array": [
                "publicKey",
                8
              ]
            }
          },
          {
            "name": "registeredZoDepositoriesCount",
            "type": "u8"
          },
          {
            "name": "reserved1",
            "type": {
              "defined": "ControllerPadding"
            }
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
            "name": "reserved",
            "type": {
              "defined": "MangoDepositoryPadding"
            }
          }
        ]
      }
    },
    {
      "name": "zoDepository",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "zoAccountBump",
            "type": "u8"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "isInitialized",
            "type": "bool"
          },
          {
            "name": "zoDexMarket",
            "type": "publicKey"
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
            "name": "quoteMint",
            "type": "publicKey"
          },
          {
            "name": "quoteMintDecimals",
            "type": "u8"
          },
          {
            "name": "zoAccount",
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
            "name": "totalAmountRebalanced",
            "type": "u128"
          }
        ]
      }
    }
  ],
  "types": [
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
      "name": "RegisterZoDepositoryEvent",
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
        }
      ]
    },
    {
      "name": "InitializeZoDepositoryEvent",
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
          "name": "zoAccount",
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
      "name": "WithdrawInsuranceFromMangoDepositoryEventV2",
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
      "name": "RedeemFromDepositoryEvent",
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
      "name": "ZoMintEvent",
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
          "name": "collateralDepositedAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "limitPrice",
          "type": "u64",
          "index": false
        },
        {
          "name": "mintedAmount",
          "type": "u64",
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
      "name": "InsufficientCollateralAmount",
      "msg": "The balance of the collateral ATA is not enough to fulfill the mint operation."
    },
    {
      "code": 6007,
      "name": "InvalidRedeemableAmount",
      "msg": "The redeemable amount for redeem must be superior to 0."
    },
    {
      "code": 6008,
      "name": "InsufficientRedeemableAmount",
      "msg": "The balance of the redeemable ATA is not enough to fulfill the redeem operation."
    },
    {
      "code": 6009,
      "name": "PerpOrderPartiallyFilled",
      "msg": "The perp position could not be fully filled with the provided slippage."
    },
    {
      "code": 6010,
      "name": "RedeemableGlobalSupplyCapReached",
      "msg": "Minting amount would go past the Redeemable Global Supply Cap."
    },
    {
      "code": 6011,
      "name": "MangoDepositoriesSoftCapOverflow",
      "msg": "Operation not allowed due to being over the Mango Redeemable soft Cap."
    },
    {
      "code": 6012,
      "name": "MaxNumberOfMangoDepositoriesRegisteredReached",
      "msg": "Cannot register more mango depositories, the limit has been reached."
    },
    {
      "code": 6013,
      "name": "InvalidInsuranceAmount",
      "msg": "The amount to withdraw from the Insurance Fund must be superior to zero.."
    },
    {
      "code": 6014,
      "name": "InsufficientAuthorityQuoteAmount",
      "msg": "The Quote ATA from authority doesn't have enough balance."
    },
    {
      "code": 6015,
      "name": "InvalidRebalancedAmount",
      "msg": "The rebalanced amount must be superior to zero.."
    },
    {
      "code": 6016,
      "name": "InsufficientOrderBookDepth",
      "msg": "Insufficient order book depth for order."
    },
    {
      "code": 6017,
      "name": "InvalidExecutedOrderSize",
      "msg": "The executed order size does not match the expected one."
    },
    {
      "code": 6018,
      "name": "InvalidMangoDepositoriesRedeemableSoftCap",
      "msg": "Mango depositories redeemable soft cap above."
    },
    {
      "code": 6019,
      "name": "InvalidQuoteDelta",
      "msg": "Quote_lot_delta can't be 0."
    },
    {
      "code": 6020,
      "name": "InvalidOrderDirection",
      "msg": "The perp order wasn't executed in the right direction."
    },
    {
      "code": 6021,
      "name": "MathError",
      "msg": "Math error."
    },
    {
      "code": 6022,
      "name": "SlippageReached",
      "msg": "The order couldn't be executed with the provided slippage."
    },
    {
      "code": 6023,
      "name": "InvalidRebalancingAmount",
      "msg": "The rebalancing amount must be above 0."
    },
    {
      "code": 6024,
      "name": "InsufficientQuoteAmount",
      "msg": "The Quote amount in the provided user_quote ATA must be >= max_amount_rebalancing."
    },
    {
      "code": 6025,
      "name": "InvalidPnlPolarity",
      "msg": "The PnL polarity provided is not the same as the perp position's one."
    },
    {
      "code": 6026,
      "name": "RebalancingError",
      "msg": "The rebalanced amount doesn't match the expected rebalance amount."
    },
    {
      "code": 6027,
      "name": "BumpError",
      "msg": "A bump was expected but is missing."
    },
    {
      "code": 6028,
      "name": "OrderSizeBelowMinLotSize",
      "msg": "The order is below size is below the min lot size."
    },
    {
      "code": 6029,
      "name": "InvalidCollateralDelta",
      "msg": "The collateral delta post perp order doesn't match the planned one."
    },
    {
      "code": 6030,
      "name": "MangoPerpMarketIndexNotFound",
      "msg": "The perp market index could not be found for this MangoMarkets Pair."
    },
    {
      "code": 6031,
      "name": "MaxNumberOfZoDepositoriesRegisteredReached",
      "msg": "Cannot register more ZO depositories, the limit has been reached."
    },
    {
      "code": 6032,
      "name": "ZoDepositoriesSoftCapOverflow",
      "msg": "Operation not allowed due to being over the ZO Redeemable soft Cap."
    },
    {
      "code": 6033,
      "name": "InvalidMangoGroup",
      "msg": "Could not load the provided MangoGroup account."
    },
    {
      "code": 6034,
      "name": "QuantityBelowContractSize",
      "msg": "The order quantity is below contract_size of the perp market."
    },
    {
      "code": 6035,
      "name": "InvalidAuthority",
      "msg": "Only the Program initializer authority can access this instructions."
    },
    {
      "code": 6036,
      "name": "InvalidController",
      "msg": "The Depository's controller doesn't match the provided Controller."
    },
    {
      "code": 6037,
      "name": "InvalidDepository",
      "msg": "The Depository provided is not registered with the Controller."
    },
    {
      "code": 6038,
      "name": "InvalidCollateralMint",
      "msg": "The provided collateral mint does not match the depository's collateral mint."
    },
    {
      "code": 6039,
      "name": "InvalidQuoteMint",
      "msg": "The provided quote mint does not match the depository's quote mint."
    },
    {
      "code": 6040,
      "name": "InvalidAuthorityQuoteATAMint",
      "msg": "The authority's Quote ATA's mint does not match the Depository's one."
    },
    {
      "code": 6041,
      "name": "InvalidMangoAccount",
      "msg": "The Mango Account isn't the Depository one."
    },
    {
      "code": 6042,
      "name": "InvalidRedeemableMint",
      "msg": "The Redeemable Mint provided does not match the Controller's one."
    },
    {
      "code": 6043,
      "name": "InvalidZoAccount",
      "msg": "The Zo Account isn't the Depository's one."
    },
    {
      "code": 6044,
      "name": "ZOPerpMarketNotFound",
      "msg": "The Zo PerpMarket index could not be found."
    },
    {
      "code": 6045,
      "name": "ZOPerpMarketInfoNotFound",
      "msg": "The Zo PerpMarketInfo could not be found."
    },
    {
      "code": 6046,
      "name": "ZOOpenOrdersInfoNotFound",
      "msg": "The Zo OpenOrdersInfo could not be found."
    },
    {
      "code": 6047,
      "name": "ZOInvalidControlState",
      "msg": "The Zo Control is in an invalid state."
    },
    {
      "code": 6048,
      "name": "ZoDepositoryAlreadyInitialized",
      "msg": "The Zo depository PDAs are already initialized."
    },
    {
      "code": 6049,
      "name": "ZoDepositoryNotInitialized",
      "msg": "The Zo depository PDAs haven't been initialized yet."
    },
    {
      "code": 6050,
      "name": "InvalidDexMarket",
      "msg": "The provided perp_market is not the one tied to this Depository."
    },
    {
      "code": 6051,
      "name": "Default",
      "msg": "Default - Check the source code for more info"
    }
  ]
};
