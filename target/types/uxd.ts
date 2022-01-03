export type Uxd = {
  "version": "1.3.31",
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
          "name": "bump",
          "type": "u8"
        },
        {
          "name": "redeemableMintBump",
          "type": "u8"
        },
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
          "name": "insuranceMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "depositoryCollateralPassthroughAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryInsurancePassthroughAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryMangoAccount",
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
      "args": [
        {
          "name": "bump",
          "type": "u8"
        },
        {
          "name": "collateralPassthroughBump",
          "type": "u8"
        },
        {
          "name": "insurancePassthroughBump",
          "type": "u8"
        },
        {
          "name": "mangoAccountBump",
          "type": "u8"
        }
      ]
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
          "name": "insuranceMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authorityInsurance",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryInsurancePassthroughAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryMangoAccount",
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
          "name": "insuranceAmount",
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
          "name": "insuranceMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authorityInsurance",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryInsurancePassthroughAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryMangoAccount",
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
          "name": "insuranceAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "mintWithMangoDepository",
      "accounts": [
        {
          "name": "user",
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
          "name": "depositoryCollateralPassthroughAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryMangoAccount",
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
          "name": "slippage",
          "type": "u32"
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
          "name": "depositoryCollateralPassthroughAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryMangoAccount",
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
          "name": "slippage",
          "type": "u32"
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
            "name": "collateralPassthroughBump",
            "type": "u8"
          },
          {
            "name": "insurancePassthroughBump",
            "type": "u8"
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
            "name": "collateralPassthrough",
            "type": "publicKey"
          },
          {
            "name": "insuranceMint",
            "type": "publicKey"
          },
          {
            "name": "insurancePassthrough",
            "type": "publicKey"
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
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "SourceFileId",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "InstructionInitializeController"
          },
          {
            "name": "InstructionSetRedeemableGlobalSupplyCap"
          },
          {
            "name": "InstructionMangoDexRegisterMangoDepository"
          },
          {
            "name": "InstructionMangoDexMintWithMangoDepository"
          },
          {
            "name": "InstructionMangoDexRedeemFromMangoDepository"
          },
          {
            "name": "InstructionMangoDexSetMangoDepositoriesRedeemableSoftCap"
          },
          {
            "name": "InstructionMangoDexDepositInsuranceToMangoDepository"
          },
          {
            "name": "InstructionMangoDexWithdrawInsuranceFromMangoDepository"
          },
          {
            "name": "MangoProgramAnchorMango"
          },
          {
            "name": "MangoProgramDeposit"
          },
          {
            "name": "MangoProgramInitMangoAccount"
          },
          {
            "name": "MangoProgramPlacePerpOrder"
          },
          {
            "name": "MangoProgramWithdraw"
          },
          {
            "name": "MangoUtilsLimitUtils"
          },
          {
            "name": "MangoUtilsOrderDelta"
          },
          {
            "name": "MangoUtilsOrder"
          },
          {
            "name": "MangoUtilsPerpAccountUtils"
          },
          {
            "name": "MangoUtilsPerpInfo"
          },
          {
            "name": "StateController"
          },
          {
            "name": "StateMangoDepository"
          },
          {
            "name": "Error"
          },
          {
            "name": "Lib"
          }
        ]
      }
    },
    {
      "name": "UxdError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "ProgramError",
            "fields": [
              {
                "defined": "ProgramError"
              }
            ]
          },
          {
            "name": "UxdErrorCode",
            "fields": [
              {
                "name": "uxd_error_code",
                "type": {
                  "defined": "UxdErrorCode"
                }
              },
              {
                "name": "line",
                "type": "u32"
              },
              {
                "name": "source_file_id",
                "type": {
                  "defined": "SourceFileId"
                }
              }
            ]
          }
        ]
      }
    },
    {
      "name": "UxdErrorCode",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "InvalidRedeemableMintDecimals"
          },
          {
            "name": "InvalidRedeemableGlobalSupplyCap"
          },
          {
            "name": "RootBankIndexNotFound"
          },
          {
            "name": "InvalidSlippage"
          },
          {
            "name": "InvalidCollateralAmount"
          },
          {
            "name": "InsufficientCollateralAmount"
          },
          {
            "name": "InvalidRedeemableAmount"
          },
          {
            "name": "InsufficientRedeemableAmount"
          },
          {
            "name": "PerpOrderPartiallyFilled"
          },
          {
            "name": "RedeemableGlobalSupplyCapReached"
          },
          {
            "name": "MangoDepositoriesSoftCapOverflow"
          },
          {
            "name": "MaxNumberOfMangoDepositoriesRegisteredReached"
          },
          {
            "name": "InvalidInsuranceAmount"
          },
          {
            "name": "InsufficientAuthorityInsuranceAmount"
          },
          {
            "name": "InvalidRebalancedAmount"
          },
          {
            "name": "InsufficientOrderBookDepth"
          },
          {
            "name": "InvalidExecutedOrderSize"
          },
          {
            "name": "MangoPerpMarketIndexNotFound"
          },
          {
            "name": "InvalidMangoDepositoriesRedeemableSoftCap"
          },
          {
            "name": "InvalidQuoteLotDelta"
          },
          {
            "name": "InvalidOrderDirection"
          },
          {
            "name": "MathError"
          },
          {
            "name": "Default"
          }
        ]
      }
    },
    {
      "name": "AccountingEvent",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Deposit"
          },
          {
            "name": "Withdraw"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidAuthority",
      "msg": "Only the Program initializer authority can access this instructions."
    },
    {
      "code": 6001,
      "name": "InvalidController",
      "msg": "The Depository's controller doesn't match the provided Controller."
    },
    {
      "code": 6002,
      "name": "InvalidDepository",
      "msg": "The Depository provided is not registered with the Controller."
    },
    {
      "code": 6003,
      "name": "InvalidCollateralMint",
      "msg": "The provided collateral mint does not match the depository's collateral mint."
    },
    {
      "code": 6004,
      "name": "InvalidInsuranceMint",
      "msg": "The provided insurance mint does not match the depository's insurance mint."
    },
    {
      "code": 6005,
      "name": "InvalidAuthorityInsuranceATAMint",
      "msg": "The authority's Insurance ATA's mint does not match the Depository's one."
    },
    {
      "code": 6006,
      "name": "InvalidCollateralPassthroughAccount",
      "msg": "The Collateral Passthrough Account isn't the Depository one."
    },
    {
      "code": 6007,
      "name": "InvalidInsurancePassthroughAccount",
      "msg": "The Insurance Passthrough Account isn't the Depository one."
    },
    {
      "code": 6008,
      "name": "InvalidMangoAccount",
      "msg": "The Mango Account isn't the Depository one."
    },
    {
      "code": 6009,
      "name": "InvalidInsurancePassthroughATAMint",
      "msg": "The Insurance Passthrough ATA's mint does not match the Depository's one."
    },
    {
      "code": 6010,
      "name": "InvalidRedeemableMint",
      "msg": "The Redeemable Mint provided does not match the Controller's one."
    },
    {
      "code": 6011,
      "name": "InvalidCollateralPassthroughATAMint",
      "msg": "The Collateral Passthrough ATA's mint does not match the Depository's one."
    }
  ]
};

export const IDL: Uxd = {
  "version": "1.3.3",
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
          "name": "bump",
          "type": "u8"
        },
        {
          "name": "redeemableMintBump",
          "type": "u8"
        },
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
          "name": "insuranceMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "depositoryCollateralPassthroughAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryInsurancePassthroughAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryMangoAccount",
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
      "args": [
        {
          "name": "bump",
          "type": "u8"
        },
        {
          "name": "collateralPassthroughBump",
          "type": "u8"
        },
        {
          "name": "insurancePassthroughBump",
          "type": "u8"
        },
        {
          "name": "mangoAccountBump",
          "type": "u8"
        }
      ]
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
          "name": "insuranceMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authorityInsurance",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryInsurancePassthroughAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryMangoAccount",
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
          "name": "insuranceAmount",
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
          "name": "insuranceMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authorityInsurance",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryInsurancePassthroughAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryMangoAccount",
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
          "name": "insuranceAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "mintWithMangoDepository",
      "accounts": [
        {
          "name": "user",
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
          "name": "depositoryCollateralPassthroughAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryMangoAccount",
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
          "name": "slippage",
          "type": "u32"
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
          "name": "depositoryCollateralPassthroughAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryMangoAccount",
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
          "name": "slippage",
          "type": "u32"
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
            "name": "collateralPassthroughBump",
            "type": "u8"
          },
          {
            "name": "insurancePassthroughBump",
            "type": "u8"
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
            "name": "collateralPassthrough",
            "type": "publicKey"
          },
          {
            "name": "insuranceMint",
            "type": "publicKey"
          },
          {
            "name": "insurancePassthrough",
            "type": "publicKey"
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
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "SourceFileId",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "InstructionInitializeController"
          },
          {
            "name": "InstructionSetRedeemableGlobalSupplyCap"
          },
          {
            "name": "InstructionMangoDexRegisterMangoDepository"
          },
          {
            "name": "InstructionMangoDexMintWithMangoDepository"
          },
          {
            "name": "InstructionMangoDexRedeemFromMangoDepository"
          },
          {
            "name": "InstructionMangoDexSetMangoDepositoriesRedeemableSoftCap"
          },
          {
            "name": "InstructionMangoDexDepositInsuranceToMangoDepository"
          },
          {
            "name": "InstructionMangoDexWithdrawInsuranceFromMangoDepository"
          },
          {
            "name": "MangoProgramAnchorMango"
          },
          {
            "name": "MangoProgramDeposit"
          },
          {
            "name": "MangoProgramInitMangoAccount"
          },
          {
            "name": "MangoProgramPlacePerpOrder"
          },
          {
            "name": "MangoProgramWithdraw"
          },
          {
            "name": "MangoUtilsLimitUtils"
          },
          {
            "name": "MangoUtilsOrderDelta"
          },
          {
            "name": "MangoUtilsOrder"
          },
          {
            "name": "MangoUtilsPerpAccountUtils"
          },
          {
            "name": "MangoUtilsPerpInfo"
          },
          {
            "name": "StateController"
          },
          {
            "name": "StateMangoDepository"
          },
          {
            "name": "Error"
          },
          {
            "name": "Lib"
          }
        ]
      }
    },
    {
      "name": "UxdError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "ProgramError",
            "fields": [
              {
                "defined": "ProgramError"
              }
            ]
          },
          {
            "name": "UxdErrorCode",
            "fields": [
              {
                "name": "uxd_error_code",
                "type": {
                  "defined": "UxdErrorCode"
                }
              },
              {
                "name": "line",
                "type": "u32"
              },
              {
                "name": "source_file_id",
                "type": {
                  "defined": "SourceFileId"
                }
              }
            ]
          }
        ]
      }
    },
    {
      "name": "UxdErrorCode",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "InvalidRedeemableMintDecimals"
          },
          {
            "name": "InvalidRedeemableGlobalSupplyCap"
          },
          {
            "name": "RootBankIndexNotFound"
          },
          {
            "name": "InvalidSlippage"
          },
          {
            "name": "InvalidCollateralAmount"
          },
          {
            "name": "InsufficientCollateralAmount"
          },
          {
            "name": "InvalidRedeemableAmount"
          },
          {
            "name": "InsufficientRedeemableAmount"
          },
          {
            "name": "PerpOrderPartiallyFilled"
          },
          {
            "name": "RedeemableGlobalSupplyCapReached"
          },
          {
            "name": "MangoDepositoriesSoftCapOverflow"
          },
          {
            "name": "MaxNumberOfMangoDepositoriesRegisteredReached"
          },
          {
            "name": "InvalidInsuranceAmount"
          },
          {
            "name": "InsufficientAuthorityInsuranceAmount"
          },
          {
            "name": "InvalidRebalancedAmount"
          },
          {
            "name": "InsufficientOrderBookDepth"
          },
          {
            "name": "InvalidExecutedOrderSize"
          },
          {
            "name": "MangoPerpMarketIndexNotFound"
          },
          {
            "name": "InvalidMangoDepositoriesRedeemableSoftCap"
          },
          {
            "name": "InvalidQuoteLotDelta"
          },
          {
            "name": "InvalidOrderDirection"
          },
          {
            "name": "MathError"
          },
          {
            "name": "Default"
          }
        ]
      }
    },
    {
      "name": "AccountingEvent",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Deposit"
          },
          {
            "name": "Withdraw"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidAuthority",
      "msg": "Only the Program initializer authority can access this instructions."
    },
    {
      "code": 6001,
      "name": "InvalidController",
      "msg": "The Depository's controller doesn't match the provided Controller."
    },
    {
      "code": 6002,
      "name": "InvalidDepository",
      "msg": "The Depository provided is not registered with the Controller."
    },
    {
      "code": 6003,
      "name": "InvalidCollateralMint",
      "msg": "The provided collateral mint does not match the depository's collateral mint."
    },
    {
      "code": 6004,
      "name": "InvalidInsuranceMint",
      "msg": "The provided insurance mint does not match the depository's insurance mint."
    },
    {
      "code": 6005,
      "name": "InvalidAuthorityInsuranceATAMint",
      "msg": "The authority's Insurance ATA's mint does not match the Depository's one."
    },
    {
      "code": 6006,
      "name": "InvalidCollateralPassthroughAccount",
      "msg": "The Collateral Passthrough Account isn't the Depository one."
    },
    {
      "code": 6007,
      "name": "InvalidInsurancePassthroughAccount",
      "msg": "The Insurance Passthrough Account isn't the Depository one."
    },
    {
      "code": 6008,
      "name": "InvalidMangoAccount",
      "msg": "The Mango Account isn't the Depository one."
    },
    {
      "code": 6009,
      "name": "InvalidInsurancePassthroughATAMint",
      "msg": "The Insurance Passthrough ATA's mint does not match the Depository's one."
    },
    {
      "code": 6010,
      "name": "InvalidRedeemableMint",
      "msg": "The Redeemable Mint provided does not match the Controller's one."
    },
    {
      "code": 6011,
      "name": "InvalidCollateralPassthroughATAMint",
      "msg": "The Collateral Passthrough ATA's mint does not match the Depository's one."
    }
  ]
};
