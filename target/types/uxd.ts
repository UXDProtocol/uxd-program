export type Uxd = {
  "version": "5.1.0",
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
      "name": "editIdentityDepository",
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
            "#3 UXDProgram on chain account bound to a Controller instance."
          ]
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#10"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#11"
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
      "name": "initializeIdentityDepository",
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
            "#4 UXDProgram on chain account bound to a Controller instance"
          ]
        },
        {
          "name": "collateralVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5",
            "Token account holding the collateral from minting"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 The collateral mint used by the `depository` instance"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#7 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#8 Token Program"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9 Rent Sysvar"
          ]
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
            "#4 UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming"
          ]
        },
        {
          "name": "collateralVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5",
            "Token account holding the collateral from minting"
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6 The redeemable mint managed by the `controller` instance",
            "Tokens will be minted during this instruction"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The `user`'s TA for the `depository` `collateral_mint`",
            "Will be debited during this instruction"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The `user`'s TA for the `controller`'s `redeemable_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#10 Token Program"
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
      "name": "redeemFromIdentityDepository",
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
            "#4 UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming"
          ]
        },
        {
          "name": "collateralVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5",
            "Token account holding the collateral from minting"
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The redeemable mint managed by the `controller` instance",
            "Tokens will be burnt during this instruction"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The `user`'s ATA for the `depository`'s `collateral_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9 The `user`'s ATA for the `controller`'s `redeemable_mint`",
            "Will be debited during this instruction"
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
      "name": "collectProfitOfMercurialVaultDepository",
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
          "name": "authorityCollateral",
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
          },
          {
            "name": "profitsTotalCollected",
            "type": "u128"
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
          },
          {
            "name": "mangoCollateralReinjectedWsol",
            "type": "bool"
          },
          {
            "name": "mangoCollateralReinjectedBtc",
            "type": "bool"
          },
          {
            "name": "mangoCollateralReinjectedEth",
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
          },
          {
            "name": "profitsTotalCollected",
            "type": "u128"
          },
          {
            "name": "lastProfitsCollectionUnixTimestamp",
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
      "name": "InvalidCollateralAmount",
      "msg": "Collateral amount cannot be 0"
    },
    {
      "code": 6003,
      "name": "InvalidRedeemableAmount",
      "msg": "Redeemable amount must be > 0 in order to redeem."
    },
    {
      "code": 6004,
      "name": "InsufficientCollateralAmount",
      "msg": "The balance of the collateral ATA is not enough to fulfill the mint operation."
    },
    {
      "code": 6005,
      "name": "InsufficientRedeemableAmount",
      "msg": "The balance of the redeemable ATA is not enough to fulfill the redeem operation."
    },
    {
      "code": 6006,
      "name": "RedeemableGlobalSupplyCapReached",
      "msg": "Minting amount would go past the Redeemable Global Supply Cap."
    },
    {
      "code": 6007,
      "name": "RedeemableMercurialVaultAmountUnderManagementCap",
      "msg": "Minting amount would go past the mercurial vault depository Redeemable Amount Under Management Cap."
    },
    {
      "code": 6008,
      "name": "MathError",
      "msg": "Math error."
    },
    {
      "code": 6009,
      "name": "SlippageReached",
      "msg": "The order couldn't be executed with the provided slippage."
    },
    {
      "code": 6010,
      "name": "BumpError",
      "msg": "A bump was expected but is missing."
    },
    {
      "code": 6011,
      "name": "MintingDisabled",
      "msg": "Minting is disabled for the current depository."
    },
    {
      "code": 6012,
      "name": "InvalidMercurialVaultLpMint",
      "msg": "The mercurial vault lp mint does not match the Depository's one."
    },
    {
      "code": 6013,
      "name": "MaxNumberOfMercurialVaultDepositoriesRegisteredReached",
      "msg": "Cannot register more mercurial vault depositories, the limit has been reached."
    },
    {
      "code": 6014,
      "name": "MercurialVaultDoNotMatchCollateral",
      "msg": "The provided collateral do not match the provided mercurial vault token."
    },
    {
      "code": 6015,
      "name": "CollateralMintEqualToRedeemableMint",
      "msg": "Collateral mint should be different than redeemable mint."
    },
    {
      "code": 6016,
      "name": "CollateralMintNotAllowed",
      "msg": "Provided collateral mint is not allowed."
    },
    {
      "code": 6017,
      "name": "MinimumMintedRedeemableAmountError",
      "msg": "Mint resulted to 0 redeemable token being minted."
    },
    {
      "code": 6018,
      "name": "MinimumRedeemedCollateralAmountError",
      "msg": "Redeem resulted to 0 collateral token being redeemed."
    },
    {
      "code": 6019,
      "name": "InvalidDepositoryLpTokenVault",
      "msg": "The depository lp token vault does not match the Depository's one."
    },
    {
      "code": 6020,
      "name": "InvalidAuthority",
      "msg": "Only the Program initializer authority can access this instructions."
    },
    {
      "code": 6021,
      "name": "InvalidController",
      "msg": "The Depository's controller doesn't match the provided Controller."
    },
    {
      "code": 6022,
      "name": "InvalidDepository",
      "msg": "The Depository provided is not registered with the Controller."
    },
    {
      "code": 6023,
      "name": "InvalidCollateralMint",
      "msg": "The provided collateral mint does not match the depository's collateral mint."
    },
    {
      "code": 6024,
      "name": "InvalidRedeemableMint",
      "msg": "The Redeemable Mint provided does not match the Controller's one."
    },
    {
      "code": 6025,
      "name": "InvalidOwner",
      "msg": "The provided token account is not owner by the expected party."
    },
    {
      "code": 6026,
      "name": "InvalidMercurialVault",
      "msg": "The provided mercurial vault does not match the Depository's one."
    },
    {
      "code": 6027,
      "name": "InvalidMercurialVaultCollateralTokenSafe",
      "msg": "The provided mercurial vault collateral token safe does not match the mercurial vault one."
    },
    {
      "code": 6028,
      "name": "RedeemableIdentityDepositoryAmountUnderManagementCap",
      "msg": "Minting amount would go past the identity depository Redeemable Amount Under Management Cap."
    },
    {
      "code": 6029,
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
      "name": "editIdentityDepository",
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
            "#3 UXDProgram on chain account bound to a Controller instance."
          ]
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
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#10"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#11"
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
      "name": "initializeIdentityDepository",
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
            "#4 UXDProgram on chain account bound to a Controller instance"
          ]
        },
        {
          "name": "collateralVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5",
            "Token account holding the collateral from minting"
          ]
        },
        {
          "name": "collateralMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#6 The collateral mint used by the `depository` instance"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#7 System Program"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#8 Token Program"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9 Rent Sysvar"
          ]
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
            "#4 UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming"
          ]
        },
        {
          "name": "collateralVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5",
            "Token account holding the collateral from minting"
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#6 The redeemable mint managed by the `controller` instance",
            "Tokens will be minted during this instruction"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The `user`'s TA for the `depository` `collateral_mint`",
            "Will be debited during this instruction"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The `user`'s TA for the `controller`'s `redeemable_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#9"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "#10 Token Program"
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
      "name": "redeemFromIdentityDepository",
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
            "#4 UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming"
          ]
        },
        {
          "name": "collateralVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#5",
            "Token account holding the collateral from minting"
          ]
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#7 The redeemable mint managed by the `controller` instance",
            "Tokens will be burnt during this instruction"
          ]
        },
        {
          "name": "userCollateral",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#8 The `user`'s ATA for the `depository`'s `collateral_mint`",
            "Will be credited during this instruction"
          ]
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "#9 The `user`'s ATA for the `controller`'s `redeemable_mint`",
            "Will be debited during this instruction"
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
      "name": "collectProfitOfMercurialVaultDepository",
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
          "name": "authorityCollateral",
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
          },
          {
            "name": "profitsTotalCollected",
            "type": "u128"
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
          },
          {
            "name": "mangoCollateralReinjectedWsol",
            "type": "bool"
          },
          {
            "name": "mangoCollateralReinjectedBtc",
            "type": "bool"
          },
          {
            "name": "mangoCollateralReinjectedEth",
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
          },
          {
            "name": "profitsTotalCollected",
            "type": "u128"
          },
          {
            "name": "lastProfitsCollectionUnixTimestamp",
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
      "name": "InvalidCollateralAmount",
      "msg": "Collateral amount cannot be 0"
    },
    {
      "code": 6003,
      "name": "InvalidRedeemableAmount",
      "msg": "Redeemable amount must be > 0 in order to redeem."
    },
    {
      "code": 6004,
      "name": "InsufficientCollateralAmount",
      "msg": "The balance of the collateral ATA is not enough to fulfill the mint operation."
    },
    {
      "code": 6005,
      "name": "InsufficientRedeemableAmount",
      "msg": "The balance of the redeemable ATA is not enough to fulfill the redeem operation."
    },
    {
      "code": 6006,
      "name": "RedeemableGlobalSupplyCapReached",
      "msg": "Minting amount would go past the Redeemable Global Supply Cap."
    },
    {
      "code": 6007,
      "name": "RedeemableMercurialVaultAmountUnderManagementCap",
      "msg": "Minting amount would go past the mercurial vault depository Redeemable Amount Under Management Cap."
    },
    {
      "code": 6008,
      "name": "MathError",
      "msg": "Math error."
    },
    {
      "code": 6009,
      "name": "SlippageReached",
      "msg": "The order couldn't be executed with the provided slippage."
    },
    {
      "code": 6010,
      "name": "BumpError",
      "msg": "A bump was expected but is missing."
    },
    {
      "code": 6011,
      "name": "MintingDisabled",
      "msg": "Minting is disabled for the current depository."
    },
    {
      "code": 6012,
      "name": "InvalidMercurialVaultLpMint",
      "msg": "The mercurial vault lp mint does not match the Depository's one."
    },
    {
      "code": 6013,
      "name": "MaxNumberOfMercurialVaultDepositoriesRegisteredReached",
      "msg": "Cannot register more mercurial vault depositories, the limit has been reached."
    },
    {
      "code": 6014,
      "name": "MercurialVaultDoNotMatchCollateral",
      "msg": "The provided collateral do not match the provided mercurial vault token."
    },
    {
      "code": 6015,
      "name": "CollateralMintEqualToRedeemableMint",
      "msg": "Collateral mint should be different than redeemable mint."
    },
    {
      "code": 6016,
      "name": "CollateralMintNotAllowed",
      "msg": "Provided collateral mint is not allowed."
    },
    {
      "code": 6017,
      "name": "MinimumMintedRedeemableAmountError",
      "msg": "Mint resulted to 0 redeemable token being minted."
    },
    {
      "code": 6018,
      "name": "MinimumRedeemedCollateralAmountError",
      "msg": "Redeem resulted to 0 collateral token being redeemed."
    },
    {
      "code": 6019,
      "name": "InvalidDepositoryLpTokenVault",
      "msg": "The depository lp token vault does not match the Depository's one."
    },
    {
      "code": 6020,
      "name": "InvalidAuthority",
      "msg": "Only the Program initializer authority can access this instructions."
    },
    {
      "code": 6021,
      "name": "InvalidController",
      "msg": "The Depository's controller doesn't match the provided Controller."
    },
    {
      "code": 6022,
      "name": "InvalidDepository",
      "msg": "The Depository provided is not registered with the Controller."
    },
    {
      "code": 6023,
      "name": "InvalidCollateralMint",
      "msg": "The provided collateral mint does not match the depository's collateral mint."
    },
    {
      "code": 6024,
      "name": "InvalidRedeemableMint",
      "msg": "The Redeemable Mint provided does not match the Controller's one."
    },
    {
      "code": 6025,
      "name": "InvalidOwner",
      "msg": "The provided token account is not owner by the expected party."
    },
    {
      "code": 6026,
      "name": "InvalidMercurialVault",
      "msg": "The provided mercurial vault does not match the Depository's one."
    },
    {
      "code": 6027,
      "name": "InvalidMercurialVaultCollateralTokenSafe",
      "msg": "The provided mercurial vault collateral token safe does not match the mercurial vault one."
    },
    {
      "code": 6028,
      "name": "RedeemableIdentityDepositoryAmountUnderManagementCap",
      "msg": "Minting amount would go past the identity depository Redeemable Amount Under Management Cap."
    },
    {
      "code": 6029,
      "name": "Default",
      "msg": "Default - Check the source code for more info."
    }
  ]
};
