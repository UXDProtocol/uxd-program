export type Uxd = {
  "version": "5.1.0",
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
      "name": "editController",
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
          "name": "fields",
          "type": {
            "defined": "EditControllerFields"
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
      "name": "editIdentityDepository",
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
        }
      ],
      "args": [
        {
          "name": "fields",
          "type": {
            "defined": "EditIdentityDepositoryFields"
          }
        }
      ]
    },
    {
      "name": "mintWithMercurialVaultDepository",
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
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryLpTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVaultCollateralTokenSafe",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVaultProgram",
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
          "name": "mercurialVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "depositoryLpTokenVault",
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
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryLpTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVaultCollateralTokenSafe",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVaultProgram",
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
      "name": "initializeIdentityDepository",
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
          "name": "collateralVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collateralMint",
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
      "name": "mintWithIdentityDepository",
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
          "name": "collateralVault",
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
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
      "name": "redeemFromIdentityDepository",
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
          "name": "collateralVault",
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "redeemableAmount",
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
            "name": "unused",
            "type": {
              "array": [
                "u8",
                257
              ]
            }
          },
          {
            "name": "redeemableGlobalSupplyCap",
            "type": "u128"
          },
          {
            "name": "unused2",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "redeemableCirculatingSupply",
            "type": "u128"
          },
          {
            "name": "unused3",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
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
          }
        ]
      }
    },
    {
      "name": "identityDepository",
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
            "name": "collateralVault",
            "type": "publicKey"
          },
          {
            "name": "collateralVaultBump",
            "type": "u8"
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
            "name": "redeemableAmountUnderManagementCap",
            "type": "u128"
          },
          {
            "name": "mintingDisabled",
            "type": "bool"
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
            "name": "redeemableGlobalSupplyCap",
            "type": {
              "option": "u128"
            }
          }
        ]
      }
    },
    {
      "name": "EditIdentityDepositoryFields",
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
            "name": "mintingDisabled",
            "type": {
              "option": "bool"
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
      "name": "SetDepositoryRedeemableAmountUnderManagementCapEvent",
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
      "name": "SetDepositoryMintingFeeInBpsEvent",
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
      "name": "SetDepositoryRedeemingFeeInBpsEvent",
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
      "name": "SetDepositoryMintingDisabledEvent",
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
      "name": "InitializeIdentityDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
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
          "name": "collateralMint",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "MintWithIdentityDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
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
          "index": false
        },
        {
          "name": "collateralAmount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "RedeemFromIdentityDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
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
          "index": false
        },
        {
          "name": "redeemableAmount",
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
      "name": "InvalidAuthority",
      "msg": "Only the Program initializer authority can access this instructions."
    },
    {
      "code": 6054,
      "name": "InvalidController",
      "msg": "The Depository's controller doesn't match the provided Controller."
    },
    {
      "code": 6055,
      "name": "InvalidDepository",
      "msg": "The Depository provided is not registered with the Controller."
    },
    {
      "code": 6056,
      "name": "InvalidCollateralMint",
      "msg": "The provided collateral mint does not match the depository's collateral mint."
    },
    {
      "code": 6057,
      "name": "InvalidQuoteMint",
      "msg": "The provided quote mint does not match the depository's quote mint."
    },
    {
      "code": 6058,
      "name": "InvalidMangoAccount",
      "msg": "The Mango Account isn't the Depository one."
    },
    {
      "code": 6059,
      "name": "InvalidRedeemableMint",
      "msg": "The Redeemable Mint provided does not match the Controller's one."
    },
    {
      "code": 6060,
      "name": "InvalidDexMarket",
      "msg": "The provided perp_market is not the one tied to this Depository."
    },
    {
      "code": 6061,
      "name": "InvalidOwner",
      "msg": "The provided token account is not owner by the expected party."
    },
    {
      "code": 6062,
      "name": "InvalidMaxBaseQuantity",
      "msg": "The max base quantity must be above 0."
    },
    {
      "code": 6063,
      "name": "InvalidMaxQuoteQuantity",
      "msg": "The max quote quantity must be above 0."
    },
    {
      "code": 6064,
      "name": "InvalidMercurialVault",
      "msg": "The provided mercurial vault does not match the Depository's one."
    },
    {
      "code": 6065,
      "name": "InvalidMercurialVaultCollateralTokenSafe",
      "msg": "The provided mercurial vault collateral token safe does not match the mercurial vault one."
    },
    {
      "code": 6066,
      "name": "Default",
      "msg": "Default - Check the source code for more info."
    }
  ]
};

export const IDL: Uxd = {
  "version": "5.1.0",
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
      "name": "editController",
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
          "name": "fields",
          "type": {
            "defined": "EditControllerFields"
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
      "name": "editIdentityDepository",
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
        }
      ],
      "args": [
        {
          "name": "fields",
          "type": {
            "defined": "EditIdentityDepositoryFields"
          }
        }
      ]
    },
    {
      "name": "mintWithMercurialVaultDepository",
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
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryLpTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVaultCollateralTokenSafe",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVaultProgram",
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
          "name": "mercurialVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "depositoryLpTokenVault",
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
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "depositoryLpTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVaultCollateralTokenSafe",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mercurialVaultProgram",
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
      "name": "initializeIdentityDepository",
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
          "name": "collateralVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collateralMint",
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
      "name": "mintWithIdentityDepository",
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
          "name": "collateralVault",
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
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
      "name": "redeemFromIdentityDepository",
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
          "name": "collateralVault",
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "redeemableAmount",
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
            "name": "unused",
            "type": {
              "array": [
                "u8",
                257
              ]
            }
          },
          {
            "name": "redeemableGlobalSupplyCap",
            "type": "u128"
          },
          {
            "name": "unused2",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "redeemableCirculatingSupply",
            "type": "u128"
          },
          {
            "name": "unused3",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
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
          }
        ]
      }
    },
    {
      "name": "identityDepository",
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
            "name": "collateralVault",
            "type": "publicKey"
          },
          {
            "name": "collateralVaultBump",
            "type": "u8"
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
            "name": "redeemableAmountUnderManagementCap",
            "type": "u128"
          },
          {
            "name": "mintingDisabled",
            "type": "bool"
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
            "name": "redeemableGlobalSupplyCap",
            "type": {
              "option": "u128"
            }
          }
        ]
      }
    },
    {
      "name": "EditIdentityDepositoryFields",
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
            "name": "mintingDisabled",
            "type": {
              "option": "bool"
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
      "name": "SetDepositoryRedeemableAmountUnderManagementCapEvent",
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
      "name": "SetDepositoryMintingFeeInBpsEvent",
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
      "name": "SetDepositoryRedeemingFeeInBpsEvent",
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
      "name": "SetDepositoryMintingDisabledEvent",
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
      "name": "InitializeIdentityDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
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
          "name": "collateralMint",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "MintWithIdentityDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
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
          "index": false
        },
        {
          "name": "collateralAmount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "RedeemFromIdentityDepositoryEvent",
      "fields": [
        {
          "name": "version",
          "type": "u8",
          "index": false
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
          "index": false
        },
        {
          "name": "redeemableAmount",
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
      "name": "InvalidAuthority",
      "msg": "Only the Program initializer authority can access this instructions."
    },
    {
      "code": 6054,
      "name": "InvalidController",
      "msg": "The Depository's controller doesn't match the provided Controller."
    },
    {
      "code": 6055,
      "name": "InvalidDepository",
      "msg": "The Depository provided is not registered with the Controller."
    },
    {
      "code": 6056,
      "name": "InvalidCollateralMint",
      "msg": "The provided collateral mint does not match the depository's collateral mint."
    },
    {
      "code": 6057,
      "name": "InvalidQuoteMint",
      "msg": "The provided quote mint does not match the depository's quote mint."
    },
    {
      "code": 6058,
      "name": "InvalidMangoAccount",
      "msg": "The Mango Account isn't the Depository one."
    },
    {
      "code": 6059,
      "name": "InvalidRedeemableMint",
      "msg": "The Redeemable Mint provided does not match the Controller's one."
    },
    {
      "code": 6060,
      "name": "InvalidDexMarket",
      "msg": "The provided perp_market is not the one tied to this Depository."
    },
    {
      "code": 6061,
      "name": "InvalidOwner",
      "msg": "The provided token account is not owner by the expected party."
    },
    {
      "code": 6062,
      "name": "InvalidMaxBaseQuantity",
      "msg": "The max base quantity must be above 0."
    },
    {
      "code": 6063,
      "name": "InvalidMaxQuoteQuantity",
      "msg": "The max quote quantity must be above 0."
    },
    {
      "code": 6064,
      "name": "InvalidMercurialVault",
      "msg": "The provided mercurial vault does not match the Depository's one."
    },
    {
      "code": 6065,
      "name": "InvalidMercurialVaultCollateralTokenSafe",
      "msg": "The provided mercurial vault collateral token safe does not match the mercurial vault one."
    },
    {
      "code": 6066,
      "name": "Default",
      "msg": "Default - Check the source code for more info."
    }
  ]
};
