export type Uxd = {
  version: '8.3.0';
  name: 'uxd';
  instructions: [
    {
      name: 'initializeController';
      docs: [
        'Initialize a Controller on chain account.',
        '',
        'Parameters:',
        '- redeemable_mint_decimals: the decimals of the redeemable mint.',
        '',
        'Note:',
        'Only one Controller on chain account will ever exist due to the',
        'PDA derivation seed having no variations.',
        '',
        'Note:',
        'In the case of UXDProtocol this is the one in charge of the UXD mint,',
        'and it has been locked to a single Controller to ever exist by only',
        "having one possible derivation. (but it's been made generic, and we",
        'could have added the authority to the seed generation for instance).',
        ''
      ];
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority'
          ];
        },
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#2'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: [
            '#3 The top level UXDProgram on chain account managing the redeemable mint'
          ];
        },
        {
          name: 'redeemableMint';
          isMut: true;
          isSigner: false;
          docs: ['#4 The redeemable mint managed by the `controller` instance'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#5 System Program'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#6 Token Program'];
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
          docs: ['#7 Rent Sysvar'];
        }
      ];
      args: [
        {
          name: 'redeemableMintDecimals';
          type: 'u8';
        }
      ];
    },
    {
      name: 'editController';
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority'
          ];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: [
            '#2 The top level UXDProgram on chain account managing the redeemable mint'
          ];
        }
      ];
      args: [
        {
          name: 'fields';
          type: {
            defined: 'EditControllerFields';
          };
        }
      ];
    },
    {
      name: 'editMercurialVaultDepository';
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority'
          ];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: [
            '#2 The top level UXDProgram on chain account managing the redeemable mint'
          ];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: [
            '#3 UXDProgram on chain account bound to a Controller instance.',
            'The `MercurialVaultDepository` manages a MercurialVaultAccount for a single Collateral.'
          ];
        }
      ];
      args: [
        {
          name: 'fields';
          type: {
            defined: 'EditMercurialVaultDepositoryFields';
          };
        }
      ];
    },
    {
      name: 'editIdentityDepository';
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority'
          ];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: [
            '#2 The top level UXDProgram on chain account managing the redeemable mint'
          ];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: [
            '#3 UXDProgram on chain account bound to a Controller instance.'
          ];
        }
      ];
      args: [
        {
          name: 'fields';
          type: {
            defined: 'EditIdentityDepositoryFields';
          };
        }
      ];
    },
    {
      name: 'editCredixLpDepository';
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority'
          ];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: [
            '#2 The top level UXDProgram on chain account managing the redeemable mint'
          ];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: ['#3'];
        }
      ];
      args: [
        {
          name: 'fields';
          type: {
            defined: 'EditCredixLpDepositoryFields';
          };
        }
      ];
    },
    {
      name: 'mint';
      accounts: [
        {
          name: 'user';
          isMut: false;
          isSigner: true;
          docs: ['#1'];
        },
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#2'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: ['#3'];
        },
        {
          name: 'redeemableMint';
          isMut: true;
          isSigner: false;
          docs: ['#4'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#5'];
        },
        {
          name: 'userRedeemable';
          isMut: true;
          isSigner: false;
          docs: ['#6'];
        },
        {
          name: 'userCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#7'];
        },
        {
          name: 'identityDepository';
          isMut: true;
          isSigner: false;
          docs: [
            '#8 - UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming'
          ];
        },
        {
          name: 'identityDepositoryCollateralVault';
          isMut: true;
          isSigner: false;
          docs: ['#9 - Token account holding the collateral from minting'];
        },
        {
          name: 'mercurialVaultDepository';
          isMut: true;
          isSigner: false;
          docs: ['#10'];
        },
        {
          name: 'mercurialVaultDepositoryLpTokenVault';
          isMut: true;
          isSigner: false;
          docs: [
            '#11 - Token account holding the LP tokens minted by depositing collateral on mercurial vault'
          ];
        },
        {
          name: 'mercurialVaultDepositoryVault';
          isMut: true;
          isSigner: false;
          docs: ['#12'];
        },
        {
          name: 'mercurialVaultDepositoryVaultLpMint';
          isMut: true;
          isSigner: false;
          docs: ['#13'];
        },
        {
          name: 'mercurialVaultDepositoryCollateralTokenSafe';
          isMut: true;
          isSigner: false;
          docs: [
            '#14 - Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.'
          ];
        },
        {
          name: 'credixLpDepository';
          isMut: true;
          isSigner: false;
          docs: ['#15'];
        },
        {
          name: 'credixLpDepositoryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#16'];
        },
        {
          name: 'credixLpDepositoryShares';
          isMut: true;
          isSigner: false;
          docs: ['#17'];
        },
        {
          name: 'credixLpDepositoryPass';
          isMut: false;
          isSigner: false;
          docs: ['#18'];
        },
        {
          name: 'credixLpDepositoryGlobalMarketState';
          isMut: false;
          isSigner: false;
          docs: ['#19'];
        },
        {
          name: 'credixLpDepositorySigningAuthority';
          isMut: false;
          isSigner: false;
          docs: ['#20 - CHECK: unused by us, checked by credix'];
        },
        {
          name: 'credixLpDepositoryLiquidityCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#21'];
        },
        {
          name: 'credixLpDepositorySharesMint';
          isMut: true;
          isSigner: false;
          docs: ['#22'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#23'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#24'];
        },
        {
          name: 'associatedTokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#25'];
        },
        {
          name: 'mercurialVaultProgram';
          isMut: false;
          isSigner: false;
          docs: ['#26'];
        },
        {
          name: 'credixProgram';
          isMut: false;
          isSigner: false;
          docs: ['#27'];
        },
        {
          name: 'uxdProgram';
          isMut: false;
          isSigner: false;
          docs: ['#28'];
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
          docs: ['#29'];
        }
      ];
      args: [
        {
          name: 'collateralAmount';
          type: 'u64';
        }
      ];
    },
    {
      name: 'redeem';
      accounts: [
        {
          name: 'user';
          isMut: false;
          isSigner: true;
          docs: ['#1'];
        },
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#2'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: ['#3'];
        },
        {
          name: 'redeemableMint';
          isMut: true;
          isSigner: false;
          docs: ['#4'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#5'];
        },
        {
          name: 'userRedeemable';
          isMut: true;
          isSigner: false;
          docs: ['#6'];
        },
        {
          name: 'userCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#7'];
        },
        {
          name: 'identityDepository';
          isMut: true;
          isSigner: false;
          docs: [
            '#8 - UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming'
          ];
        },
        {
          name: 'identityDepositoryCollateralVault';
          isMut: true;
          isSigner: false;
          docs: ['#9 - Token account holding the collateral from minting'];
        },
        {
          name: 'mercurialVaultDepository';
          isMut: true;
          isSigner: false;
          docs: ['#10'];
        },
        {
          name: 'mercurialVaultDepositoryLpTokenVault';
          isMut: true;
          isSigner: false;
          docs: [
            '#11 - Token account holding the LP tokens minted by depositing collateral on mercurial vault'
          ];
        },
        {
          name: 'mercurialVaultDepositoryVault';
          isMut: true;
          isSigner: false;
          docs: ['#12'];
        },
        {
          name: 'mercurialVaultDepositoryVaultLpMint';
          isMut: true;
          isSigner: false;
          docs: ['#13'];
        },
        {
          name: 'mercurialVaultDepositoryCollateralTokenSafe';
          isMut: true;
          isSigner: false;
          docs: [
            '#14 - Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.'
          ];
        },
        {
          name: 'credixLpDepository';
          isMut: true;
          isSigner: false;
          docs: ['#15'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#16'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#17'];
        },
        {
          name: 'associatedTokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#18'];
        },
        {
          name: 'mercurialVaultProgram';
          isMut: false;
          isSigner: false;
          docs: ['#19'];
        },
        {
          name: 'uxdProgram';
          isMut: false;
          isSigner: false;
          docs: ['#20'];
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
          docs: ['#21'];
        }
      ];
      args: [
        {
          name: 'redeemableAmount';
          type: 'u64';
        }
      ];
    },
    {
      name: 'mintWithMercurialVaultDepository';
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: [
            '#1 This IX should only be accessible by the router or the DAO'
          ];
        },
        {
          name: 'user';
          isMut: false;
          isSigner: true;
          docs: ['#2'];
        },
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#3'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: ['#4'];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: ['#5'];
        },
        {
          name: 'redeemableMint';
          isMut: true;
          isSigner: false;
          docs: ['#6'];
        },
        {
          name: 'userRedeemable';
          isMut: true;
          isSigner: false;
          docs: ['#7'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#8'];
        },
        {
          name: 'userCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#9'];
        },
        {
          name: 'depositoryLpTokenVault';
          isMut: true;
          isSigner: false;
          docs: [
            '#10',
            'Token account holding the LP tokens minted by depositing collateral on mercurial vault'
          ];
        },
        {
          name: 'mercurialVault';
          isMut: true;
          isSigner: false;
          docs: ['#11'];
        },
        {
          name: 'mercurialVaultLpMint';
          isMut: true;
          isSigner: false;
          docs: ['#12'];
        },
        {
          name: 'mercurialVaultCollateralTokenSafe';
          isMut: true;
          isSigner: false;
          docs: [
            '#13',
            'Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.'
          ];
        },
        {
          name: 'mercurialVaultProgram';
          isMut: false;
          isSigner: false;
          docs: ['#14'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#15'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#16'];
        }
      ];
      args: [
        {
          name: 'collateralAmount';
          type: 'u64';
        }
      ];
    },
    {
      name: 'registerMercurialVaultDepository';
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: ['#1'];
        },
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#2'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: ['#3'];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: ['#4'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#5'];
        },
        {
          name: 'mercurialVault';
          isMut: false;
          isSigner: false;
          docs: ['#6'];
        },
        {
          name: 'mercurialVaultLpMint';
          isMut: false;
          isSigner: false;
          docs: ['#7'];
        },
        {
          name: 'depositoryLpTokenVault';
          isMut: true;
          isSigner: false;
          docs: [
            '#8',
            'Token account holding the LP tokens minted by depositing collateral on mercurial vault'
          ];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#9'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#10'];
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
          docs: ['#11'];
        }
      ];
      args: [
        {
          name: 'mintingFeeInBps';
          type: 'u8';
        },
        {
          name: 'redeemingFeeInBps';
          type: 'u8';
        },
        {
          name: 'redeemableAmountUnderManagementCap';
          type: 'u128';
        }
      ];
    },
    {
      name: 'redeemFromMercurialVaultDepository';
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: [
            '#1 This IX should only be accessible by the router or the DAO'
          ];
        },
        {
          name: 'user';
          isMut: false;
          isSigner: true;
          docs: ['#2'];
        },
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#3'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: ['#4'];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: ['#5'];
        },
        {
          name: 'redeemableMint';
          isMut: true;
          isSigner: false;
          docs: ['#6'];
        },
        {
          name: 'userRedeemable';
          isMut: true;
          isSigner: false;
          docs: ['#7'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#8'];
        },
        {
          name: 'userCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#9'];
        },
        {
          name: 'depositoryLpTokenVault';
          isMut: true;
          isSigner: false;
          docs: [
            '#10',
            'Token account holding the LP tokens minted by depositing collateral on mercurial vault'
          ];
        },
        {
          name: 'mercurialVault';
          isMut: true;
          isSigner: false;
          docs: ['#11'];
        },
        {
          name: 'mercurialVaultLpMint';
          isMut: true;
          isSigner: false;
          docs: ['#12'];
        },
        {
          name: 'mercurialVaultCollateralTokenSafe';
          isMut: true;
          isSigner: false;
          docs: [
            '#13',
            'Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.'
          ];
        },
        {
          name: 'mercurialVaultProgram';
          isMut: false;
          isSigner: false;
          docs: ['#14'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#15'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#16'];
        }
      ];
      args: [
        {
          name: 'redeemableAmount';
          type: 'u64';
        }
      ];
    },
    {
      name: 'collectProfitsOfMercurialVaultDepository';
      accounts: [
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#1'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: ['#2'];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: ['#3'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#4'];
        },
        {
          name: 'profitsBeneficiaryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#5'];
        },
        {
          name: 'depositoryLpTokenVault';
          isMut: true;
          isSigner: false;
          docs: [
            '#6',
            'Token account holding the LP tokens minted by depositing collateral on mercurial vault'
          ];
        },
        {
          name: 'mercurialVault';
          isMut: true;
          isSigner: false;
          docs: ['#7'];
        },
        {
          name: 'mercurialVaultLpMint';
          isMut: true;
          isSigner: false;
          docs: ['#8'];
        },
        {
          name: 'mercurialVaultCollateralTokenSafe';
          isMut: true;
          isSigner: false;
          docs: [
            '#9',
            'Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.'
          ];
        },
        {
          name: 'mercurialVaultProgram';
          isMut: false;
          isSigner: false;
          docs: ['#10'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#11'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#12'];
        }
      ];
      args: [];
    },
    {
      name: 'initializeIdentityDepository';
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority'
          ];
        },
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#2'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: [
            '#3 The top level UXDProgram on chain account managing the redeemable mint'
          ];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: [
            '#4 UXDProgram on chain account bound to a Controller instance'
          ];
        },
        {
          name: 'collateralVault';
          isMut: true;
          isSigner: false;
          docs: ['#5', 'Token account holding the collateral from minting'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#6 The collateral mint used by the `depository` instance'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#7 System Program'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#8 Token Program'];
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
          docs: ['#9 Rent Sysvar'];
        }
      ];
      args: [];
    },
    {
      name: 'mintWithIdentityDepository';
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: [
            '#1 This IX should only be accessible by the router or the DAO'
          ];
        },
        {
          name: 'user';
          isMut: false;
          isSigner: true;
          docs: ['#2'];
        },
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#3'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: [
            '#4 The top level UXDProgram on chain account managing the redeemable mint'
          ];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: [
            '#5 UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming'
          ];
        },
        {
          name: 'collateralVault';
          isMut: true;
          isSigner: false;
          docs: ['#6', 'Token account holding the collateral from minting'];
        },
        {
          name: 'redeemableMint';
          isMut: true;
          isSigner: false;
          docs: [
            '#7 The redeemable mint managed by the `controller` instance',
            'Tokens will be minted during this instruction'
          ];
        },
        {
          name: 'userCollateral';
          isMut: true;
          isSigner: false;
          docs: [
            "#8 The `user`'s TA for the `depository` `collateral_mint`",
            'Will be debited during this instruction'
          ];
        },
        {
          name: 'userRedeemable';
          isMut: true;
          isSigner: false;
          docs: [
            "#9 The `user`'s TA for the `controller`'s `redeemable_mint`",
            'Will be credited during this instruction'
          ];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#10'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#11 Token Program'];
        }
      ];
      args: [
        {
          name: 'collateralAmount';
          type: 'u64';
        }
      ];
    },
    {
      name: 'redeemFromIdentityDepository';
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: [
            '#1 This IX should only be accessible by the router or the DAO'
          ];
        },
        {
          name: 'user';
          isMut: false;
          isSigner: true;
          docs: ['#2'];
        },
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#3'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: [
            '#4 The top level UXDProgram on chain account managing the redeemable mint'
          ];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: [
            '#5 UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming'
          ];
        },
        {
          name: 'collateralVault';
          isMut: true;
          isSigner: false;
          docs: ['#6', 'Token account holding the collateral from minting'];
        },
        {
          name: 'redeemableMint';
          isMut: true;
          isSigner: false;
          docs: [
            '#7 The redeemable mint managed by the `controller` instance',
            'Tokens will be burnt during this instruction'
          ];
        },
        {
          name: 'userCollateral';
          isMut: true;
          isSigner: false;
          docs: [
            "#8 The `user`'s ATA for the `depository`'s `collateral_mint`",
            'Will be credited during this instruction'
          ];
        },
        {
          name: 'userRedeemable';
          isMut: true;
          isSigner: false;
          docs: [
            "#9 The `user`'s ATA for the `controller`'s `redeemable_mint`",
            'Will be debited during this instruction'
          ];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#10'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#11'];
        }
      ];
      args: [
        {
          name: 'redeemableAmount';
          type: 'u64';
        }
      ];
    },
    {
      name: 'registerCredixLpDepository';
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: ['#1'];
        },
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#2'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: ['#3'];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: ['#4'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#5'];
        },
        {
          name: 'depositoryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#6'];
        },
        {
          name: 'depositoryShares';
          isMut: true;
          isSigner: false;
          docs: ['#7'];
        },
        {
          name: 'credixProgramState';
          isMut: false;
          isSigner: false;
          docs: ['#8'];
        },
        {
          name: 'credixGlobalMarketState';
          isMut: false;
          isSigner: false;
          docs: ['#9'];
        },
        {
          name: 'credixSigningAuthority';
          isMut: false;
          isSigner: false;
          docs: ['#10'];
        },
        {
          name: 'credixLiquidityCollateral';
          isMut: false;
          isSigner: false;
          docs: ['#11'];
        },
        {
          name: 'credixSharesMint';
          isMut: false;
          isSigner: false;
          docs: ['#12'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#13'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#14'];
        },
        {
          name: 'associatedTokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#15'];
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
          docs: ['#16'];
        }
      ];
      args: [
        {
          name: 'mintingFeeInBps';
          type: 'u8';
        },
        {
          name: 'redeemingFeeInBps';
          type: 'u8';
        },
        {
          name: 'redeemableAmountUnderManagementCap';
          type: 'u128';
        }
      ];
    },
    {
      name: 'mintWithCredixLpDepository';
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: [
            '#1 This IX should only be accessible by the router or the DAO'
          ];
        },
        {
          name: 'user';
          isMut: false;
          isSigner: true;
          docs: ['#2'];
        },
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#3'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: ['#4'];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: ['#5'];
        },
        {
          name: 'redeemableMint';
          isMut: true;
          isSigner: false;
          docs: ['#6'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#7'];
        },
        {
          name: 'userRedeemable';
          isMut: true;
          isSigner: false;
          docs: ['#8'];
        },
        {
          name: 'userCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#9'];
        },
        {
          name: 'depositoryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#10'];
        },
        {
          name: 'depositoryShares';
          isMut: true;
          isSigner: false;
          docs: ['#11'];
        },
        {
          name: 'credixGlobalMarketState';
          isMut: false;
          isSigner: false;
          docs: ['#12'];
        },
        {
          name: 'credixSigningAuthority';
          isMut: false;
          isSigner: false;
          docs: ['#13'];
        },
        {
          name: 'credixLiquidityCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#14'];
        },
        {
          name: 'credixSharesMint';
          isMut: true;
          isSigner: false;
          docs: ['#15'];
        },
        {
          name: 'credixPass';
          isMut: false;
          isSigner: false;
          docs: ['#16'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#17'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#18'];
        },
        {
          name: 'associatedTokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#19'];
        },
        {
          name: 'credixProgram';
          isMut: false;
          isSigner: false;
          docs: ['#20'];
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
          docs: ['#21'];
        }
      ];
      args: [
        {
          name: 'collateralAmount';
          type: 'u64';
        }
      ];
    },
    {
      name: 'redeemFromCredixLpDepository';
      accounts: [
        {
          name: 'user';
          isMut: false;
          isSigner: true;
          docs: ['#1'];
        },
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#2'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: ['#3'];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: ['#4'];
        },
        {
          name: 'redeemableMint';
          isMut: true;
          isSigner: false;
          docs: ['#5'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#6'];
        },
        {
          name: 'userRedeemable';
          isMut: true;
          isSigner: false;
          docs: ['#7'];
        },
        {
          name: 'userCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#8'];
        },
        {
          name: 'depositoryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#9'];
        },
        {
          name: 'depositoryShares';
          isMut: true;
          isSigner: false;
          docs: ['#10'];
        },
        {
          name: 'credixProgramState';
          isMut: false;
          isSigner: false;
          docs: ['#11'];
        },
        {
          name: 'credixGlobalMarketState';
          isMut: true;
          isSigner: false;
          docs: ['#12'];
        },
        {
          name: 'credixSigningAuthority';
          isMut: false;
          isSigner: false;
          docs: ['#13'];
        },
        {
          name: 'credixLiquidityCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#14'];
        },
        {
          name: 'credixSharesMint';
          isMut: true;
          isSigner: false;
          docs: ['#15'];
        },
        {
          name: 'credixPass';
          isMut: false;
          isSigner: false;
          docs: ['#16'];
        },
        {
          name: 'credixTreasuryPoolCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#17'];
        },
        {
          name: 'credixTreasury';
          isMut: false;
          isSigner: false;
          docs: ['#18'];
        },
        {
          name: 'credixTreasuryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#19'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#20'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#21'];
        },
        {
          name: 'associatedTokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#22'];
        },
        {
          name: 'credixProgram';
          isMut: false;
          isSigner: false;
          docs: ['#23'];
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
          docs: ['#24'];
        }
      ];
      args: [
        {
          name: 'redeemableAmount';
          type: 'u64';
        }
      ];
    },
    {
      name: 'collectProfitsOfCredixLpDepository';
      accounts: [
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#1'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: ['#2'];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: ['#3'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#4'];
        },
        {
          name: 'depositoryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#5'];
        },
        {
          name: 'depositoryShares';
          isMut: true;
          isSigner: false;
          docs: ['#6'];
        },
        {
          name: 'credixProgramState';
          isMut: false;
          isSigner: false;
          docs: ['#7'];
        },
        {
          name: 'credixGlobalMarketState';
          isMut: true;
          isSigner: false;
          docs: ['#8'];
        },
        {
          name: 'credixSigningAuthority';
          isMut: false;
          isSigner: false;
          docs: ['#9'];
        },
        {
          name: 'credixLiquidityCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#10'];
        },
        {
          name: 'credixSharesMint';
          isMut: true;
          isSigner: false;
          docs: ['#11'];
        },
        {
          name: 'credixPass';
          isMut: true;
          isSigner: false;
          docs: ['#12'];
        },
        {
          name: 'credixTreasuryPoolCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#13'];
        },
        {
          name: 'credixTreasury';
          isMut: false;
          isSigner: false;
          docs: ['#14'];
        },
        {
          name: 'credixTreasuryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#15'];
        },
        {
          name: 'profitsBeneficiaryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#16'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#17'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#18'];
        },
        {
          name: 'associatedTokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#19'];
        },
        {
          name: 'credixProgram';
          isMut: false;
          isSigner: false;
          docs: ['#20'];
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
          docs: ['#21'];
        }
      ];
      args: [];
    },
    {
      name: 'exchangeLiquidityWithCredixLpDepository';
      accounts: [
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: ['#1'];
        },
        {
          name: 'user';
          isMut: false;
          isSigner: true;
          docs: ['#2'];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: ['#3'];
        },
        {
          name: 'identityDepository';
          isMut: true;
          isSigner: false;
          docs: ['#4'];
        },
        {
          name: 'identityDepositoryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#5'];
        },
        {
          name: 'credixLpDepository';
          isMut: true;
          isSigner: false;
          docs: ['#6'];
        },
        {
          name: 'credixLpDepositoryShares';
          isMut: true;
          isSigner: false;
          docs: ['#7'];
        },
        {
          name: 'userCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#8'];
        },
        {
          name: 'receiverCredixShares';
          isMut: true;
          isSigner: false;
          docs: ['#9'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#10'];
        },
        {
          name: 'credixSharesMint';
          isMut: false;
          isSigner: false;
          docs: ['#11'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#12'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#13'];
        }
      ];
      args: [
        {
          name: 'collateralAmount';
          type: 'u64';
        }
      ];
    },
    {
      name: 'rebalanceCreateWithdrawRequestFromCredixLpDepository';
      accounts: [
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: [
            '#1',
            'Permissionless IX that can be called by anyone at any time'
          ];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: ['#2'];
        },
        {
          name: 'identityDepository';
          isMut: true;
          isSigner: false;
          docs: ['#3'];
        },
        {
          name: 'mercurialVaultDepository';
          isMut: true;
          isSigner: false;
          docs: ['#4'];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: ['#5'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#6'];
        },
        {
          name: 'depositoryShares';
          isMut: true;
          isSigner: false;
          docs: ['#7'];
        },
        {
          name: 'credixGlobalMarketState';
          isMut: false;
          isSigner: false;
          docs: ['#8'];
        },
        {
          name: 'credixSigningAuthority';
          isMut: false;
          isSigner: false;
          docs: ['#9'];
        },
        {
          name: 'credixLiquidityCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#10'];
        },
        {
          name: 'credixSharesMint';
          isMut: true;
          isSigner: false;
          docs: ['#11'];
        },
        {
          name: 'credixPass';
          isMut: false;
          isSigner: false;
          docs: ['#12'];
        },
        {
          name: 'credixWithdrawEpoch';
          isMut: true;
          isSigner: false;
          docs: ['#13'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#14'];
        },
        {
          name: 'credixProgram';
          isMut: false;
          isSigner: false;
          docs: ['#15'];
        }
      ];
      args: [];
    },
    {
      name: 'rebalanceRedeemWithdrawRequestFromCredixLpDepository';
      accounts: [
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
          docs: [
            '#1 // Permissionless IX that can be called by anyone at any time'
          ];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: ['#2'];
        },
        {
          name: 'identityDepository';
          isMut: true;
          isSigner: false;
          docs: ['#3'];
        },
        {
          name: 'identityDepositoryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#4'];
        },
        {
          name: 'mercurialVaultDepository';
          isMut: true;
          isSigner: false;
          docs: ['#5'];
        },
        {
          name: 'depository';
          isMut: true;
          isSigner: false;
          docs: ['#6'];
        },
        {
          name: 'collateralMint';
          isMut: false;
          isSigner: false;
          docs: ['#7'];
        },
        {
          name: 'depositoryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#8'];
        },
        {
          name: 'depositoryShares';
          isMut: true;
          isSigner: false;
          docs: ['#9'];
        },
        {
          name: 'credixProgramState';
          isMut: false;
          isSigner: false;
          docs: ['#10'];
        },
        {
          name: 'credixGlobalMarketState';
          isMut: true;
          isSigner: false;
          docs: ['#11'];
        },
        {
          name: 'credixSigningAuthority';
          isMut: false;
          isSigner: false;
          docs: ['#12'];
        },
        {
          name: 'credixLiquidityCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#13'];
        },
        {
          name: 'credixSharesMint';
          isMut: true;
          isSigner: false;
          docs: ['#14'];
        },
        {
          name: 'credixPass';
          isMut: true;
          isSigner: false;
          docs: ['#15'];
        },
        {
          name: 'credixTreasuryPoolCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#16'];
        },
        {
          name: 'credixTreasury';
          isMut: false;
          isSigner: false;
          docs: ['#17'];
        },
        {
          name: 'credixTreasuryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#18'];
        },
        {
          name: 'credixWithdrawEpoch';
          isMut: true;
          isSigner: false;
          docs: ['#19'];
        },
        {
          name: 'profitsBeneficiaryCollateral';
          isMut: true;
          isSigner: false;
          docs: ['#20'];
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['#21'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#22'];
        },
        {
          name: 'associatedTokenProgram';
          isMut: false;
          isSigner: false;
          docs: ['#23'];
        },
        {
          name: 'credixProgram';
          isMut: false;
          isSigner: false;
          docs: ['#24'];
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
          docs: ['#25'];
        }
      ];
      args: [];
    },
    {
      name: 'freezeProgram';
      docs: [
        'Freeze or resume all ixs associated with the controller (except this one).',
        '',
        'Parameters:',
        '- freeze: bool param to flip the `is_frozen` property in the controller',
        '',
        'Note:',
        'This is a wildcard to stop the program temporarily when a vulnerability has been detected to allow the team to do servicing work.',
        ''
      ];
      accounts: [
        {
          name: 'authority';
          isMut: false;
          isSigner: true;
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority'
          ];
        },
        {
          name: 'controller';
          isMut: true;
          isSigner: false;
          docs: [
            '#2 The top level UXDProgram on chain account managing the redeemable mint'
          ];
        }
      ];
      args: [
        {
          name: 'freeze';
          type: 'bool';
        }
      ];
    }
  ];
  accounts: [
    {
      name: 'controller';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'bump';
            type: 'u8';
          },
          {
            name: 'redeemableMintBump';
            type: 'u8';
          },
          {
            name: 'version';
            type: 'u8';
          },
          {
            name: 'authority';
            type: 'publicKey';
          },
          {
            name: 'redeemableMint';
            type: 'publicKey';
          },
          {
            name: 'redeemableMintDecimals';
            type: 'u8';
          },
          {
            name: 'unused';
            type: {
              array: ['u8', 255];
            };
          },
          {
            name: 'isFrozen';
            type: 'bool';
          },
          {
            name: 'unused2';
            type: 'u8';
          },
          {
            name: 'redeemableGlobalSupplyCap';
            type: 'u128';
          },
          {
            name: 'unused3';
            type: {
              array: ['u8', 8];
            };
          },
          {
            name: 'redeemableCirculatingSupply';
            type: 'u128';
          },
          {
            name: 'unused4';
            type: {
              array: ['u8', 266];
            };
          },
          {
            name: 'profitsTotalCollected';
            type: 'u128';
          },
          {
            name: 'identityDepositoryWeightBps';
            type: 'u16';
          },
          {
            name: 'mercurialVaultDepositoryWeightBps';
            type: 'u16';
          },
          {
            name: 'credixLpDepositoryWeightBps';
            type: 'u16';
          },
          {
            name: 'identityDepository';
            type: 'publicKey';
          },
          {
            name: 'mercurialVaultDepository';
            type: 'publicKey';
          },
          {
            name: 'credixLpDepository';
            type: 'publicKey';
          },
          {
            name: 'outflowLimitPerEpochAmount';
            type: 'u64';
          },
          {
            name: 'outflowLimitPerEpochBps';
            type: 'u16';
          },
          {
            name: 'slotsPerEpoch';
            type: 'u64';
          },
          {
            name: 'epochOutflowAmount';
            type: 'u64';
          },
          {
            name: 'lastOutflowSlot';
            type: 'u64';
          },
          {
            name: 'reserved';
            type: {
              array: ['u8', 94];
            };
          }
        ];
      };
    },
    {
      name: 'credixLpDepository';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'bump';
            type: 'u8';
          },
          {
            name: 'version';
            type: 'u8';
          },
          {
            name: 'controller';
            type: 'publicKey';
          },
          {
            name: 'collateralMint';
            type: 'publicKey';
          },
          {
            name: 'depositoryCollateral';
            type: 'publicKey';
          },
          {
            name: 'depositoryShares';
            type: 'publicKey';
          },
          {
            name: 'credixProgramState';
            type: 'publicKey';
          },
          {
            name: 'credixGlobalMarketState';
            type: 'publicKey';
          },
          {
            name: 'credixSigningAuthority';
            type: 'publicKey';
          },
          {
            name: 'credixLiquidityCollateral';
            type: 'publicKey';
          },
          {
            name: 'credixSharesMint';
            type: 'publicKey';
          },
          {
            name: 'redeemableAmountUnderManagementCap';
            type: 'u128';
          },
          {
            name: 'mintingFeeInBps';
            type: 'u8';
          },
          {
            name: 'redeemingFeeInBps';
            type: 'u8';
          },
          {
            name: 'mintingDisabled';
            type: 'bool';
          },
          {
            name: 'collateralAmountDeposited';
            type: 'u128';
          },
          {
            name: 'redeemableAmountUnderManagement';
            type: 'u128';
          },
          {
            name: 'mintingFeeTotalAccrued';
            type: 'u128';
          },
          {
            name: 'redeemingFeeTotalAccrued';
            type: 'u128';
          },
          {
            name: 'profitsTotalCollected';
            type: 'u128';
          },
          {
            name: 'profitsBeneficiaryCollateral';
            type: 'publicKey';
          },
          {
            name: 'reserved';
            type: {
              array: ['u8', 768];
            };
          }
        ];
      };
    },
    {
      name: 'identityDepository';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'bump';
            type: 'u8';
          },
          {
            name: 'version';
            type: 'u8';
          },
          {
            name: 'collateralMint';
            type: 'publicKey';
          },
          {
            name: 'collateralMintDecimals';
            type: 'u8';
          },
          {
            name: 'collateralVault';
            type: 'publicKey';
          },
          {
            name: 'collateralVaultBump';
            type: 'u8';
          },
          {
            name: 'collateralAmountDeposited';
            type: 'u128';
          },
          {
            name: 'redeemableAmountUnderManagement';
            type: 'u128';
          },
          {
            name: 'redeemableAmountUnderManagementCap';
            type: 'u128';
          },
          {
            name: 'mintingDisabled';
            type: 'bool';
          },
          {
            name: 'mangoCollateralReinjectedWsol';
            type: 'bool';
          },
          {
            name: 'mangoCollateralReinjectedBtc';
            type: 'bool';
          },
          {
            name: 'mangoCollateralReinjectedEth';
            type: 'bool';
          },
          {
            name: 'reserved';
            type: {
              array: ['u8', 512];
            };
          }
        ];
      };
    },
    {
      name: 'mercurialVaultDepository';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'bump';
            type: 'u8';
          },
          {
            name: 'version';
            type: 'u8';
          },
          {
            name: 'collateralMint';
            type: 'publicKey';
          },
          {
            name: 'collateralMintDecimals';
            type: 'u8';
          },
          {
            name: 'controller';
            type: 'publicKey';
          },
          {
            name: 'collateralAmountDeposited';
            type: 'u128';
          },
          {
            name: 'redeemableAmountUnderManagement';
            type: 'u128';
          },
          {
            name: 'mercurialVault';
            type: 'publicKey';
          },
          {
            name: 'mercurialVaultLpMint';
            type: 'publicKey';
          },
          {
            name: 'mercurialVaultLpMintDecimals';
            type: 'u8';
          },
          {
            name: 'lpTokenVault';
            type: 'publicKey';
          },
          {
            name: 'lpTokenVaultBump';
            type: 'u8';
          },
          {
            name: 'mintingFeeInBps';
            type: 'u8';
          },
          {
            name: 'redeemingFeeInBps';
            type: 'u8';
          },
          {
            name: 'mintingFeeTotalAccrued';
            type: 'u128';
          },
          {
            name: 'redeemingFeeTotalAccrued';
            type: 'u128';
          },
          {
            name: 'redeemableAmountUnderManagementCap';
            type: 'u128';
          },
          {
            name: 'mintingDisabled';
            type: 'bool';
          },
          {
            name: 'profitsTotalCollected';
            type: 'u128';
          },
          {
            name: 'lastProfitsCollectionUnixTimestamp';
            type: 'u64';
          },
          {
            name: 'profitsBeneficiaryCollateral';
            type: 'publicKey';
          },
          {
            name: 'reserved';
            type: {
              array: ['u8', 588];
            };
          }
        ];
      };
    }
  ];
  types: [
    {
      name: 'EditCredixLpDepositoryFields';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'redeemableAmountUnderManagementCap';
            type: {
              option: 'u128';
            };
          },
          {
            name: 'mintingFeeInBps';
            type: {
              option: 'u8';
            };
          },
          {
            name: 'redeemingFeeInBps';
            type: {
              option: 'u8';
            };
          },
          {
            name: 'mintingDisabled';
            type: {
              option: 'bool';
            };
          },
          {
            name: 'profitsBeneficiaryCollateral';
            type: {
              option: 'publicKey';
            };
          }
        ];
      };
    },
    {
      name: 'EditDepositoriesRoutingWeightBps';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'identityDepositoryWeightBps';
            type: 'u16';
          },
          {
            name: 'mercurialVaultDepositoryWeightBps';
            type: 'u16';
          },
          {
            name: 'credixLpDepositoryWeightBps';
            type: 'u16';
          }
        ];
      };
    },
    {
      name: 'EditRouterDepositories';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'identityDepository';
            type: 'publicKey';
          },
          {
            name: 'mercurialVaultDepository';
            type: 'publicKey';
          },
          {
            name: 'credixLpDepository';
            type: 'publicKey';
          }
        ];
      };
    },
    {
      name: 'EditControllerFields';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'redeemableGlobalSupplyCap';
            type: {
              option: 'u128';
            };
          },
          {
            name: 'depositoriesRoutingWeightBps';
            type: {
              option: {
                defined: 'EditDepositoriesRoutingWeightBps';
              };
            };
          },
          {
            name: 'routerDepositories';
            type: {
              option: {
                defined: 'EditRouterDepositories';
              };
            };
          },
          {
            name: 'outflowLimitPerEpochAmount';
            type: {
              option: 'u64';
            };
          },
          {
            name: 'outflowLimitPerEpochBps';
            type: {
              option: 'u16';
            };
          },
          {
            name: 'slotsPerEpoch';
            type: {
              option: 'u64';
            };
          }
        ];
      };
    },
    {
      name: 'EditIdentityDepositoryFields';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'redeemableAmountUnderManagementCap';
            type: {
              option: 'u128';
            };
          },
          {
            name: 'mintingDisabled';
            type: {
              option: 'bool';
            };
          }
        ];
      };
    },
    {
      name: 'EditMercurialVaultDepositoryFields';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'redeemableAmountUnderManagementCap';
            type: {
              option: 'u128';
            };
          },
          {
            name: 'mintingFeeInBps';
            type: {
              option: 'u8';
            };
          },
          {
            name: 'redeemingFeeInBps';
            type: {
              option: 'u8';
            };
          },
          {
            name: 'mintingDisabled';
            type: {
              option: 'bool';
            };
          },
          {
            name: 'profitsBeneficiaryCollateral';
            type: {
              option: 'publicKey';
            };
          }
        ];
      };
    }
  ];
  events: [
    {
      name: 'InitializeControllerEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: false;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'authority';
          type: 'publicKey';
          index: false;
        }
      ];
    },
    {
      name: 'SetRedeemableGlobalSupplyCapEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: false;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'redeemableGlobalSupplyCap';
          type: 'u128';
          index: false;
        }
      ];
    },
    {
      name: 'SetOutflowLimitPerEpochAmountEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: false;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'outflowLimitPerEpochAmount';
          type: 'u64';
          index: false;
        }
      ];
    },
    {
      name: 'SetOutflowLimitPerEpochBpsEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: false;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'outflowLimitPerEpochBps';
          type: 'u16';
          index: false;
        }
      ];
    },
    {
      name: 'SetSlotsPerEpochEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: false;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'slotsPerEpoch';
          type: 'u64';
          index: false;
        }
      ];
    },
    {
      name: 'SetRouterDepositoriesWeightBps';
      fields: [
        {
          name: 'controllerVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'identityDepositoryWeightBps';
          type: 'u16';
          index: false;
        },
        {
          name: 'mercurialVaultDepositoryWeightBps';
          type: 'u16';
          index: false;
        },
        {
          name: 'credixLpDepositoryWeightBps';
          type: 'u16';
          index: false;
        }
      ];
    },
    {
      name: 'SetRouterDepositories';
      fields: [
        {
          name: 'controllerVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'identityDepository';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'mercurialVaultDepository';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'credixLpDepository';
          type: 'publicKey';
          index: false;
        }
      ];
    },
    {
      name: 'RegisterMercurialVaultDepositoryEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: true;
        },
        {
          name: 'depositoryVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'mercurialVault';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'depositoryLpTokenVault';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'collateralMint';
          type: 'publicKey';
          index: false;
        }
      ];
    },
    {
      name: 'SetDepositoryRedeemableAmountUnderManagementCapEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'redeemableAmountUnderManagementCap';
          type: 'u128';
          index: true;
        }
      ];
    },
    {
      name: 'SetDepositoryMintingFeeInBpsEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'mintingFeeInBps';
          type: 'u8';
          index: true;
        }
      ];
    },
    {
      name: 'SetDepositoryRedeemingFeeInBpsEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'redeemingFeeInBps';
          type: 'u8';
          index: true;
        }
      ];
    },
    {
      name: 'SetDepositoryMintingDisabledEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'mintingDisabled';
          type: 'bool';
          index: true;
        }
      ];
    },
    {
      name: 'SetDepositoryProfitsBeneficiaryCollateralEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'profitsBeneficiaryCollateral';
          type: 'publicKey';
          index: true;
        }
      ];
    },
    {
      name: 'InitializeIdentityDepositoryEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: false;
        },
        {
          name: 'depositoryVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'collateralMint';
          type: 'publicKey';
          index: false;
        }
      ];
    },
    {
      name: 'MintWithIdentityDepositoryEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: false;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'user';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'collateralAmount';
          type: 'u64';
          index: false;
        }
      ];
    },
    {
      name: 'RedeemFromIdentityDepositoryEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: false;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'user';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'redeemableAmount';
          type: 'u64';
          index: false;
        }
      ];
    },
    {
      name: 'RegisterCredixLpDepositoryEvent';
      fields: [
        {
          name: 'controllerVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'depositoryVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'collateralMint';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'credixGlobalMarketState';
          type: 'publicKey';
          index: false;
        }
      ];
    },
    {
      name: 'MintWithCredixLpDepositoryEvent';
      fields: [
        {
          name: 'controllerVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'depositoryVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'user';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'collateralAmount';
          type: 'u64';
          index: false;
        },
        {
          name: 'redeemableAmount';
          type: 'u64';
          index: false;
        },
        {
          name: 'mintingFeePaid';
          type: 'u64';
          index: false;
        }
      ];
    },
    {
      name: 'RedeemFromCredixLpDepositoryEvent';
      fields: [
        {
          name: 'controllerVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'depositoryVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'user';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'collateralAmount';
          type: 'u64';
          index: false;
        },
        {
          name: 'redeemableAmount';
          type: 'u64';
          index: false;
        },
        {
          name: 'redeemingFeePaid';
          type: 'u64';
          index: false;
        }
      ];
    },
    {
      name: 'CollectProfitsOfCredixLpDepositoryEvent';
      fields: [
        {
          name: 'controllerVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'depositoryVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'collateralAmount';
          type: 'u64';
          index: false;
        }
      ];
    },
    {
      name: 'ExchangeLiquidityWithCredixLpDepositoryEvent';
      fields: [
        {
          name: 'controllerVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'depositoryVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'collateralAmount';
          type: 'u64';
          index: false;
        },
        {
          name: 'sharesAmount';
          type: 'u64';
          index: false;
        }
      ];
    },
    {
      name: 'RebalanceCreateWithdrawRequestFromCredixLpDepositoryEvent';
      fields: [
        {
          name: 'controllerVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'depositoryVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'overflowValue';
          type: 'u64';
          index: false;
        },
        {
          name: 'profitsCollateralAmount';
          type: 'u64';
          index: false;
        },
        {
          name: 'requestedCollateralAmount';
          type: 'u64';
          index: false;
        }
      ];
    },
    {
      name: 'RebalanceRedeemWithdrawRequestFromCredixLpDepositoryEvent';
      fields: [
        {
          name: 'controllerVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'depositoryVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'overflowValue';
          type: 'u64';
          index: false;
        },
        {
          name: 'profitsCollateralAmount';
          type: 'u64';
          index: false;
        },
        {
          name: 'requestedCollateralAmount';
          type: 'u64';
          index: false;
        }
      ];
    },
    {
      name: 'CollectProfitsOfMercurialVaultDepositoryEvent';
      fields: [
        {
          name: 'controllerVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'depositoryVersion';
          type: 'u8';
          index: true;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'depository';
          type: 'publicKey';
          index: true;
        },
        {
          name: 'collateralAmount';
          type: 'u64';
          index: false;
        }
      ];
    },
    {
      name: 'FreezeProgramEvent';
      fields: [
        {
          name: 'version';
          type: 'u8';
          index: false;
        },
        {
          name: 'controller';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'isFrozen';
          type: 'bool';
          index: false;
        }
      ];
    }
  ];
  errors: [
    {
      code: 6000;
      name: 'InvalidRedeemableMintDecimals';
      msg: 'The redeemable mint decimals must be between 0 and 9 (inclusive).';
    },
    {
      code: 6001;
      name: 'InvalidRedeemableGlobalSupplyCap';
      msg: 'Redeemable global supply above.';
    },
    {
      code: 6002;
      name: 'InvalidDepositoriesWeightBps';
      msg: 'Depositories weights do not add up to 100%.';
    },
    {
      code: 6003;
      name: 'InvalidDepositoriesVector';
      msg: 'Depositories vector passed as parameter is not of the expected length';
    },
    {
      code: 6004;
      name: 'InvalidCollateralAmount';
      msg: 'Collateral amount cannot be 0';
    },
    {
      code: 6005;
      name: 'InvalidRedeemableAmount';
      msg: 'Redeemable amount must be > 0 in order to redeem.';
    },
    {
      code: 6006;
      name: 'InsufficientCollateralAmount';
      msg: 'The balance of the collateral ATA is not enough to fulfill the mint operation.';
    },
    {
      code: 6007;
      name: 'InsufficientRedeemableAmount';
      msg: 'The balance of the redeemable ATA is not enough to fulfill the redeem operation.';
    },
    {
      code: 6008;
      name: 'DepositoriesTargerRedeemableAmountReached';
      msg: 'Minting amount would go past the depositories target redeemable amount.';
    },
    {
      code: 6009;
      name: 'RedeemableGlobalSupplyCapReached';
      msg: 'Minting amount would go past the Redeemable Global Supply Cap.';
    },
    {
      code: 6010;
      name: 'RedeemableMercurialVaultAmountUnderManagementCap';
      msg: 'Minting amount would go past the mercurial vault depository Redeemable Amount Under Management Cap.';
    },
    {
      code: 6011;
      name: 'RedeemableCredixLpAmountUnderManagementCap';
      msg: 'Minting amount would go past the credix lp depository Redeemable Amount Under Management Cap.';
    },
    {
      code: 6012;
      name: 'MathOverflow';
      msg: 'Math overflow.';
    },
    {
      code: 6013;
      name: 'SlippageReached';
      msg: "The order couldn't be executed with the provided slippage.";
    },
    {
      code: 6014;
      name: 'BumpError';
      msg: 'A bump was expected but is missing.';
    },
    {
      code: 6015;
      name: 'MintingDisabled';
      msg: 'Minting is disabled for the current depository.';
    },
    {
      code: 6016;
      name: 'CollateralDepositHasRemainingDust';
      msg: 'Collateral deposit left some value unaccounted for.';
    },
    {
      code: 6017;
      name: 'CollateralDepositAmountsDoesntMatch';
      msg: "Collateral deposit didn't result in the correct amounts being moved.";
    },
    {
      code: 6018;
      name: 'CollateralDepositDoesntMatchTokenValue';
      msg: "Received token of which the value doesn't match the deposited collateral.";
    },
    {
      code: 6019;
      name: 'InvalidMercurialVaultLpMint';
      msg: "The mercurial vault lp mint does not match the Depository's one.";
    },
    {
      code: 6020;
      name: 'MaxNumberOfMercurialVaultDepositoriesRegisteredReached';
      msg: 'Cannot register more mercurial vault depositories, the limit has been reached.';
    },
    {
      code: 6021;
      name: 'MaxNumberOfCredixLpDepositoriesRegisteredReached';
      msg: 'Cannot register more credix lp depositories, the limit has been reached.';
    },
    {
      code: 6022;
      name: 'MercurialVaultDoNotMatchCollateral';
      msg: 'The provided collateral do not match the provided mercurial vault token.';
    },
    {
      code: 6023;
      name: 'CredixLpDoNotMatchCollateral';
      msg: 'The provided collateral do not match the provided credix lp token.';
    },
    {
      code: 6024;
      name: 'CollateralMintEqualToRedeemableMint';
      msg: 'Collateral mint should be different than redeemable mint.';
    },
    {
      code: 6025;
      name: 'CollateralMintNotAllowed';
      msg: 'Provided collateral mint is not allowed.';
    },
    {
      code: 6026;
      name: 'MinimumMintedRedeemableAmountError';
      msg: 'Mint resulted to 0 redeemable token being minted.';
    },
    {
      code: 6027;
      name: 'MaximumMintedRedeemableAmountError';
      msg: 'Mint resulted into too much redeemable token being minted.';
    },
    {
      code: 6028;
      name: 'MinimumRedeemedCollateralAmountError';
      msg: 'Redeem resulted to 0 collateral token being redeemed.';
    },
    {
      code: 6029;
      name: 'MaximumRedeemedCollateralAmountError';
      msg: 'Redeem resulted into too much collateral token being redeemed.';
    },
    {
      code: 6030;
      name: 'InvalidDepositoryLpTokenVault';
      msg: "The depository lp token vault does not match the Depository's one.";
    },
    {
      code: 6031;
      name: 'InvalidAuthority';
      msg: 'Only the Program initializer authority can access this instructions.';
    },
    {
      code: 6032;
      name: 'InvalidController';
      msg: "The Depository's controller doesn't match the provided Controller.";
    },
    {
      code: 6033;
      name: 'InvalidDepository';
      msg: 'The Depository provided is not matching the one stored in the Controller.';
    },
    {
      code: 6034;
      name: 'InvalidCollateralMint';
      msg: "The provided collateral mint does not match the depository's collateral mint.";
    },
    {
      code: 6035;
      name: 'InvalidRedeemableMint';
      msg: "The Redeemable Mint provided does not match the Controller's one.";
    },
    {
      code: 6036;
      name: 'InvalidOwner';
      msg: 'The provided token account is not owner by the expected party.';
    },
    {
      code: 6037;
      name: 'InvalidDepositoryCollateral';
      msg: "The provided depository collateral does not match the depository's one.";
    },
    {
      code: 6038;
      name: 'InvalidDepositoryShares';
      msg: "The provided depository shares does not match the depository's one.";
    },
    {
      code: 6039;
      name: 'InvalidProfitsBeneficiaryCollateral';
      msg: "The Profits beneficiary collateral provided does not match the depository's one.";
    },
    {
      code: 6040;
      name: 'InvalidMercurialVault';
      msg: "The provided mercurial vault does not match the Depository's one.";
    },
    {
      code: 6041;
      name: 'InvalidMercurialVaultCollateralTokenSafe';
      msg: 'The provided mercurial vault collateral token safe does not match the mercurial vault one.';
    },
    {
      code: 6042;
      name: 'RedeemableIdentityDepositoryAmountUnderManagementCap';
      msg: 'Minting amount would go past the identity depository Redeemable Amount Under Management Cap.';
    },
    {
      code: 6043;
      name: 'ProgramAlreadyFrozenOrResumed';
      msg: 'Program is already frozen/resumed.';
    },
    {
      code: 6044;
      name: 'ProgramFrozen';
      msg: 'The program is currently in Frozen state.';
    },
    {
      code: 6045;
      name: 'InvalidCredixProgramState';
      msg: "The Credix ProgramState isn't the Depository one.";
    },
    {
      code: 6046;
      name: 'InvalidCredixGlobalMarketState';
      msg: "The Credix GlobalMarketState isn't the Depository one.";
    },
    {
      code: 6047;
      name: 'InvalidCredixSigningAuthority';
      msg: "The Credix SigningAuthority isn't the Depository one.";
    },
    {
      code: 6048;
      name: 'InvalidCredixLiquidityCollateral';
      msg: "The Credix LiquidityCollateral isn't the Depository one.";
    },
    {
      code: 6049;
      name: 'InvalidCredixSharesMint';
      msg: "The Credix SharesMint isn't the Depository one.";
    },
    {
      code: 6050;
      name: 'InvalidCredixPass';
      msg: "The Credix Pass isn't the one owned by the correct depository.";
    },
    {
      code: 6051;
      name: 'InvalidCredixPassNoFees';
      msg: "The Credix Pass doesn't have the fees exemption.";
    },
    {
      code: 6052;
      name: 'InvalidCredixTreasury';
      msg: "The Credix Treasury isn't the ProgramState one.";
    },
    {
      code: 6053;
      name: 'InvalidCredixTreasuryPoolCollateral';
      msg: "The Credix TreasuryPool isn't the GlobalMarketState one.";
    },
    {
      code: 6054;
      name: 'InvalidCredixWithdrawEpochRequestPhase';
      msg: "The Credix WithdrawEpoch isn't in its request phase.";
    },
    {
      code: 6055;
      name: 'InvalidCredixWithdrawEpochRedeemPhase';
      msg: "The Credix WithdrawEpoch isn't in its redeem phase.";
    },
    {
      code: 6056;
      name: 'Default';
      msg: 'Default - Check the source code for more info.';
    },
    {
      code: 6057;
      name: 'MaximumOutflowAmountError';
      msg: 'Redeem resulted into too much outflow in this epoch, please wait or try again with a smaller amount.';
    },
    {
      code: 6058;
      name: 'InvalidOutflowLimitPerEpochBps';
      msg: 'The outflow_limit_per_epoch_bps is invalid: over 100%.';
    }
  ];
};

export const IDL: Uxd = {
  version: '8.3.0',
  name: 'uxd',
  instructions: [
    {
      name: 'initializeController',
      docs: [
        'Initialize a Controller on chain account.',
        '',
        'Parameters:',
        '- redeemable_mint_decimals: the decimals of the redeemable mint.',
        '',
        'Note:',
        'Only one Controller on chain account will ever exist due to the',
        'PDA derivation seed having no variations.',
        '',
        'Note:',
        'In the case of UXDProtocol this is the one in charge of the UXD mint,',
        'and it has been locked to a single Controller to ever exist by only',
        "having one possible derivation. (but it's been made generic, and we",
        'could have added the authority to the seed generation for instance).',
        '',
      ],
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority',
          ],
        },
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#2'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: [
            '#3 The top level UXDProgram on chain account managing the redeemable mint',
          ],
        },
        {
          name: 'redeemableMint',
          isMut: true,
          isSigner: false,
          docs: ['#4 The redeemable mint managed by the `controller` instance'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#5 System Program'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#6 Token Program'],
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
          docs: ['#7 Rent Sysvar'],
        },
      ],
      args: [
        {
          name: 'redeemableMintDecimals',
          type: 'u8',
        },
      ],
    },
    {
      name: 'editController',
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority',
          ],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: [
            '#2 The top level UXDProgram on chain account managing the redeemable mint',
          ],
        },
      ],
      args: [
        {
          name: 'fields',
          type: {
            defined: 'EditControllerFields',
          },
        },
      ],
    },
    {
      name: 'editMercurialVaultDepository',
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority',
          ],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: [
            '#2 The top level UXDProgram on chain account managing the redeemable mint',
          ],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: [
            '#3 UXDProgram on chain account bound to a Controller instance.',
            'The `MercurialVaultDepository` manages a MercurialVaultAccount for a single Collateral.',
          ],
        },
      ],
      args: [
        {
          name: 'fields',
          type: {
            defined: 'EditMercurialVaultDepositoryFields',
          },
        },
      ],
    },
    {
      name: 'editIdentityDepository',
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority',
          ],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: [
            '#2 The top level UXDProgram on chain account managing the redeemable mint',
          ],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: [
            '#3 UXDProgram on chain account bound to a Controller instance.',
          ],
        },
      ],
      args: [
        {
          name: 'fields',
          type: {
            defined: 'EditIdentityDepositoryFields',
          },
        },
      ],
    },
    {
      name: 'editCredixLpDepository',
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority',
          ],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: [
            '#2 The top level UXDProgram on chain account managing the redeemable mint',
          ],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: ['#3'],
        },
      ],
      args: [
        {
          name: 'fields',
          type: {
            defined: 'EditCredixLpDepositoryFields',
          },
        },
      ],
    },
    {
      name: 'mint',
      accounts: [
        {
          name: 'user',
          isMut: false,
          isSigner: true,
          docs: ['#1'],
        },
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#2'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: ['#3'],
        },
        {
          name: 'redeemableMint',
          isMut: true,
          isSigner: false,
          docs: ['#4'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#5'],
        },
        {
          name: 'userRedeemable',
          isMut: true,
          isSigner: false,
          docs: ['#6'],
        },
        {
          name: 'userCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#7'],
        },
        {
          name: 'identityDepository',
          isMut: true,
          isSigner: false,
          docs: [
            '#8 - UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming',
          ],
        },
        {
          name: 'identityDepositoryCollateralVault',
          isMut: true,
          isSigner: false,
          docs: ['#9 - Token account holding the collateral from minting'],
        },
        {
          name: 'mercurialVaultDepository',
          isMut: true,
          isSigner: false,
          docs: ['#10'],
        },
        {
          name: 'mercurialVaultDepositoryLpTokenVault',
          isMut: true,
          isSigner: false,
          docs: [
            '#11 - Token account holding the LP tokens minted by depositing collateral on mercurial vault',
          ],
        },
        {
          name: 'mercurialVaultDepositoryVault',
          isMut: true,
          isSigner: false,
          docs: ['#12'],
        },
        {
          name: 'mercurialVaultDepositoryVaultLpMint',
          isMut: true,
          isSigner: false,
          docs: ['#13'],
        },
        {
          name: 'mercurialVaultDepositoryCollateralTokenSafe',
          isMut: true,
          isSigner: false,
          docs: [
            '#14 - Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.',
          ],
        },
        {
          name: 'credixLpDepository',
          isMut: true,
          isSigner: false,
          docs: ['#15'],
        },
        {
          name: 'credixLpDepositoryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#16'],
        },
        {
          name: 'credixLpDepositoryShares',
          isMut: true,
          isSigner: false,
          docs: ['#17'],
        },
        {
          name: 'credixLpDepositoryPass',
          isMut: false,
          isSigner: false,
          docs: ['#18'],
        },
        {
          name: 'credixLpDepositoryGlobalMarketState',
          isMut: false,
          isSigner: false,
          docs: ['#19'],
        },
        {
          name: 'credixLpDepositorySigningAuthority',
          isMut: false,
          isSigner: false,
          docs: ['#20 - CHECK: unused by us, checked by credix'],
        },
        {
          name: 'credixLpDepositoryLiquidityCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#21'],
        },
        {
          name: 'credixLpDepositorySharesMint',
          isMut: true,
          isSigner: false,
          docs: ['#22'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#23'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#24'],
        },
        {
          name: 'associatedTokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#25'],
        },
        {
          name: 'mercurialVaultProgram',
          isMut: false,
          isSigner: false,
          docs: ['#26'],
        },
        {
          name: 'credixProgram',
          isMut: false,
          isSigner: false,
          docs: ['#27'],
        },
        {
          name: 'uxdProgram',
          isMut: false,
          isSigner: false,
          docs: ['#28'],
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
          docs: ['#29'],
        },
      ],
      args: [
        {
          name: 'collateralAmount',
          type: 'u64',
        },
      ],
    },
    {
      name: 'redeem',
      accounts: [
        {
          name: 'user',
          isMut: false,
          isSigner: true,
          docs: ['#1'],
        },
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#2'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: ['#3'],
        },
        {
          name: 'redeemableMint',
          isMut: true,
          isSigner: false,
          docs: ['#4'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#5'],
        },
        {
          name: 'userRedeemable',
          isMut: true,
          isSigner: false,
          docs: ['#6'],
        },
        {
          name: 'userCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#7'],
        },
        {
          name: 'identityDepository',
          isMut: true,
          isSigner: false,
          docs: [
            '#8 - UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming',
          ],
        },
        {
          name: 'identityDepositoryCollateralVault',
          isMut: true,
          isSigner: false,
          docs: ['#9 - Token account holding the collateral from minting'],
        },
        {
          name: 'mercurialVaultDepository',
          isMut: true,
          isSigner: false,
          docs: ['#10'],
        },
        {
          name: 'mercurialVaultDepositoryLpTokenVault',
          isMut: true,
          isSigner: false,
          docs: [
            '#11 - Token account holding the LP tokens minted by depositing collateral on mercurial vault',
          ],
        },
        {
          name: 'mercurialVaultDepositoryVault',
          isMut: true,
          isSigner: false,
          docs: ['#12'],
        },
        {
          name: 'mercurialVaultDepositoryVaultLpMint',
          isMut: true,
          isSigner: false,
          docs: ['#13'],
        },
        {
          name: 'mercurialVaultDepositoryCollateralTokenSafe',
          isMut: true,
          isSigner: false,
          docs: [
            '#14 - Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.',
          ],
        },
        {
          name: 'credixLpDepository',
          isMut: true,
          isSigner: false,
          docs: ['#15'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#16'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#17'],
        },
        {
          name: 'associatedTokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#18'],
        },
        {
          name: 'mercurialVaultProgram',
          isMut: false,
          isSigner: false,
          docs: ['#19'],
        },
        {
          name: 'uxdProgram',
          isMut: false,
          isSigner: false,
          docs: ['#20'],
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
          docs: ['#21'],
        },
      ],
      args: [
        {
          name: 'redeemableAmount',
          type: 'u64',
        },
      ],
    },
    {
      name: 'mintWithMercurialVaultDepository',
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: [
            '#1 This IX should only be accessible by the router or the DAO',
          ],
        },
        {
          name: 'user',
          isMut: false,
          isSigner: true,
          docs: ['#2'],
        },
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#3'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: ['#4'],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: ['#5'],
        },
        {
          name: 'redeemableMint',
          isMut: true,
          isSigner: false,
          docs: ['#6'],
        },
        {
          name: 'userRedeemable',
          isMut: true,
          isSigner: false,
          docs: ['#7'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#8'],
        },
        {
          name: 'userCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#9'],
        },
        {
          name: 'depositoryLpTokenVault',
          isMut: true,
          isSigner: false,
          docs: [
            '#10',
            'Token account holding the LP tokens minted by depositing collateral on mercurial vault',
          ],
        },
        {
          name: 'mercurialVault',
          isMut: true,
          isSigner: false,
          docs: ['#11'],
        },
        {
          name: 'mercurialVaultLpMint',
          isMut: true,
          isSigner: false,
          docs: ['#12'],
        },
        {
          name: 'mercurialVaultCollateralTokenSafe',
          isMut: true,
          isSigner: false,
          docs: [
            '#13',
            'Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.',
          ],
        },
        {
          name: 'mercurialVaultProgram',
          isMut: false,
          isSigner: false,
          docs: ['#14'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#15'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#16'],
        },
      ],
      args: [
        {
          name: 'collateralAmount',
          type: 'u64',
        },
      ],
    },
    {
      name: 'registerMercurialVaultDepository',
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: ['#1'],
        },
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#2'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: ['#3'],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: ['#4'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#5'],
        },
        {
          name: 'mercurialVault',
          isMut: false,
          isSigner: false,
          docs: ['#6'],
        },
        {
          name: 'mercurialVaultLpMint',
          isMut: false,
          isSigner: false,
          docs: ['#7'],
        },
        {
          name: 'depositoryLpTokenVault',
          isMut: true,
          isSigner: false,
          docs: [
            '#8',
            'Token account holding the LP tokens minted by depositing collateral on mercurial vault',
          ],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#9'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#10'],
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
          docs: ['#11'],
        },
      ],
      args: [
        {
          name: 'mintingFeeInBps',
          type: 'u8',
        },
        {
          name: 'redeemingFeeInBps',
          type: 'u8',
        },
        {
          name: 'redeemableAmountUnderManagementCap',
          type: 'u128',
        },
      ],
    },
    {
      name: 'redeemFromMercurialVaultDepository',
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: [
            '#1 This IX should only be accessible by the router or the DAO',
          ],
        },
        {
          name: 'user',
          isMut: false,
          isSigner: true,
          docs: ['#2'],
        },
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#3'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: ['#4'],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: ['#5'],
        },
        {
          name: 'redeemableMint',
          isMut: true,
          isSigner: false,
          docs: ['#6'],
        },
        {
          name: 'userRedeemable',
          isMut: true,
          isSigner: false,
          docs: ['#7'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#8'],
        },
        {
          name: 'userCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#9'],
        },
        {
          name: 'depositoryLpTokenVault',
          isMut: true,
          isSigner: false,
          docs: [
            '#10',
            'Token account holding the LP tokens minted by depositing collateral on mercurial vault',
          ],
        },
        {
          name: 'mercurialVault',
          isMut: true,
          isSigner: false,
          docs: ['#11'],
        },
        {
          name: 'mercurialVaultLpMint',
          isMut: true,
          isSigner: false,
          docs: ['#12'],
        },
        {
          name: 'mercurialVaultCollateralTokenSafe',
          isMut: true,
          isSigner: false,
          docs: [
            '#13',
            'Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.',
          ],
        },
        {
          name: 'mercurialVaultProgram',
          isMut: false,
          isSigner: false,
          docs: ['#14'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#15'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#16'],
        },
      ],
      args: [
        {
          name: 'redeemableAmount',
          type: 'u64',
        },
      ],
    },
    {
      name: 'collectProfitsOfMercurialVaultDepository',
      accounts: [
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#1'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: ['#2'],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: ['#3'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#4'],
        },
        {
          name: 'profitsBeneficiaryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#5'],
        },
        {
          name: 'depositoryLpTokenVault',
          isMut: true,
          isSigner: false,
          docs: [
            '#6',
            'Token account holding the LP tokens minted by depositing collateral on mercurial vault',
          ],
        },
        {
          name: 'mercurialVault',
          isMut: true,
          isSigner: false,
          docs: ['#7'],
        },
        {
          name: 'mercurialVaultLpMint',
          isMut: true,
          isSigner: false,
          docs: ['#8'],
        },
        {
          name: 'mercurialVaultCollateralTokenSafe',
          isMut: true,
          isSigner: false,
          docs: [
            '#9',
            'Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.',
          ],
        },
        {
          name: 'mercurialVaultProgram',
          isMut: false,
          isSigner: false,
          docs: ['#10'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#11'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#12'],
        },
      ],
      args: [],
    },
    {
      name: 'initializeIdentityDepository',
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority',
          ],
        },
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#2'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: [
            '#3 The top level UXDProgram on chain account managing the redeemable mint',
          ],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: [
            '#4 UXDProgram on chain account bound to a Controller instance',
          ],
        },
        {
          name: 'collateralVault',
          isMut: true,
          isSigner: false,
          docs: ['#5', 'Token account holding the collateral from minting'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#6 The collateral mint used by the `depository` instance'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#7 System Program'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#8 Token Program'],
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
          docs: ['#9 Rent Sysvar'],
        },
      ],
      args: [],
    },
    {
      name: 'mintWithIdentityDepository',
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: [
            '#1 This IX should only be accessible by the router or the DAO',
          ],
        },
        {
          name: 'user',
          isMut: false,
          isSigner: true,
          docs: ['#2'],
        },
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#3'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: [
            '#4 The top level UXDProgram on chain account managing the redeemable mint',
          ],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: [
            '#5 UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming',
          ],
        },
        {
          name: 'collateralVault',
          isMut: true,
          isSigner: false,
          docs: ['#6', 'Token account holding the collateral from minting'],
        },
        {
          name: 'redeemableMint',
          isMut: true,
          isSigner: false,
          docs: [
            '#7 The redeemable mint managed by the `controller` instance',
            'Tokens will be minted during this instruction',
          ],
        },
        {
          name: 'userCollateral',
          isMut: true,
          isSigner: false,
          docs: [
            "#8 The `user`'s TA for the `depository` `collateral_mint`",
            'Will be debited during this instruction',
          ],
        },
        {
          name: 'userRedeemable',
          isMut: true,
          isSigner: false,
          docs: [
            "#9 The `user`'s TA for the `controller`'s `redeemable_mint`",
            'Will be credited during this instruction',
          ],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#10'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#11 Token Program'],
        },
      ],
      args: [
        {
          name: 'collateralAmount',
          type: 'u64',
        },
      ],
    },
    {
      name: 'redeemFromIdentityDepository',
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: [
            '#1 This IX should only be accessible by the router or the DAO',
          ],
        },
        {
          name: 'user',
          isMut: false,
          isSigner: true,
          docs: ['#2'],
        },
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#3'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: [
            '#4 The top level UXDProgram on chain account managing the redeemable mint',
          ],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: [
            '#5 UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming',
          ],
        },
        {
          name: 'collateralVault',
          isMut: true,
          isSigner: false,
          docs: ['#6', 'Token account holding the collateral from minting'],
        },
        {
          name: 'redeemableMint',
          isMut: true,
          isSigner: false,
          docs: [
            '#7 The redeemable mint managed by the `controller` instance',
            'Tokens will be burnt during this instruction',
          ],
        },
        {
          name: 'userCollateral',
          isMut: true,
          isSigner: false,
          docs: [
            "#8 The `user`'s ATA for the `depository`'s `collateral_mint`",
            'Will be credited during this instruction',
          ],
        },
        {
          name: 'userRedeemable',
          isMut: true,
          isSigner: false,
          docs: [
            "#9 The `user`'s ATA for the `controller`'s `redeemable_mint`",
            'Will be debited during this instruction',
          ],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#10'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#11'],
        },
      ],
      args: [
        {
          name: 'redeemableAmount',
          type: 'u64',
        },
      ],
    },
    {
      name: 'registerCredixLpDepository',
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: ['#1'],
        },
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#2'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: ['#3'],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: ['#4'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#5'],
        },
        {
          name: 'depositoryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#6'],
        },
        {
          name: 'depositoryShares',
          isMut: true,
          isSigner: false,
          docs: ['#7'],
        },
        {
          name: 'credixProgramState',
          isMut: false,
          isSigner: false,
          docs: ['#8'],
        },
        {
          name: 'credixGlobalMarketState',
          isMut: false,
          isSigner: false,
          docs: ['#9'],
        },
        {
          name: 'credixSigningAuthority',
          isMut: false,
          isSigner: false,
          docs: ['#10'],
        },
        {
          name: 'credixLiquidityCollateral',
          isMut: false,
          isSigner: false,
          docs: ['#11'],
        },
        {
          name: 'credixSharesMint',
          isMut: false,
          isSigner: false,
          docs: ['#12'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#13'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#14'],
        },
        {
          name: 'associatedTokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#15'],
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
          docs: ['#16'],
        },
      ],
      args: [
        {
          name: 'mintingFeeInBps',
          type: 'u8',
        },
        {
          name: 'redeemingFeeInBps',
          type: 'u8',
        },
        {
          name: 'redeemableAmountUnderManagementCap',
          type: 'u128',
        },
      ],
    },
    {
      name: 'mintWithCredixLpDepository',
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: [
            '#1 This IX should only be accessible by the router or the DAO',
          ],
        },
        {
          name: 'user',
          isMut: false,
          isSigner: true,
          docs: ['#2'],
        },
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#3'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: ['#4'],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: ['#5'],
        },
        {
          name: 'redeemableMint',
          isMut: true,
          isSigner: false,
          docs: ['#6'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#7'],
        },
        {
          name: 'userRedeemable',
          isMut: true,
          isSigner: false,
          docs: ['#8'],
        },
        {
          name: 'userCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#9'],
        },
        {
          name: 'depositoryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#10'],
        },
        {
          name: 'depositoryShares',
          isMut: true,
          isSigner: false,
          docs: ['#11'],
        },
        {
          name: 'credixGlobalMarketState',
          isMut: false,
          isSigner: false,
          docs: ['#12'],
        },
        {
          name: 'credixSigningAuthority',
          isMut: false,
          isSigner: false,
          docs: ['#13'],
        },
        {
          name: 'credixLiquidityCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#14'],
        },
        {
          name: 'credixSharesMint',
          isMut: true,
          isSigner: false,
          docs: ['#15'],
        },
        {
          name: 'credixPass',
          isMut: false,
          isSigner: false,
          docs: ['#16'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#17'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#18'],
        },
        {
          name: 'associatedTokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#19'],
        },
        {
          name: 'credixProgram',
          isMut: false,
          isSigner: false,
          docs: ['#20'],
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
          docs: ['#21'],
        },
      ],
      args: [
        {
          name: 'collateralAmount',
          type: 'u64',
        },
      ],
    },
    {
      name: 'redeemFromCredixLpDepository',
      accounts: [
        {
          name: 'user',
          isMut: false,
          isSigner: true,
          docs: ['#1'],
        },
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#2'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: ['#3'],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: ['#4'],
        },
        {
          name: 'redeemableMint',
          isMut: true,
          isSigner: false,
          docs: ['#5'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#6'],
        },
        {
          name: 'userRedeemable',
          isMut: true,
          isSigner: false,
          docs: ['#7'],
        },
        {
          name: 'userCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#8'],
        },
        {
          name: 'depositoryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#9'],
        },
        {
          name: 'depositoryShares',
          isMut: true,
          isSigner: false,
          docs: ['#10'],
        },
        {
          name: 'credixProgramState',
          isMut: false,
          isSigner: false,
          docs: ['#11'],
        },
        {
          name: 'credixGlobalMarketState',
          isMut: true,
          isSigner: false,
          docs: ['#12'],
        },
        {
          name: 'credixSigningAuthority',
          isMut: false,
          isSigner: false,
          docs: ['#13'],
        },
        {
          name: 'credixLiquidityCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#14'],
        },
        {
          name: 'credixSharesMint',
          isMut: true,
          isSigner: false,
          docs: ['#15'],
        },
        {
          name: 'credixPass',
          isMut: false,
          isSigner: false,
          docs: ['#16'],
        },
        {
          name: 'credixTreasuryPoolCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#17'],
        },
        {
          name: 'credixTreasury',
          isMut: false,
          isSigner: false,
          docs: ['#18'],
        },
        {
          name: 'credixTreasuryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#19'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#20'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#21'],
        },
        {
          name: 'associatedTokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#22'],
        },
        {
          name: 'credixProgram',
          isMut: false,
          isSigner: false,
          docs: ['#23'],
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
          docs: ['#24'],
        },
      ],
      args: [
        {
          name: 'redeemableAmount',
          type: 'u64',
        },
      ],
    },
    {
      name: 'collectProfitsOfCredixLpDepository',
      accounts: [
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#1'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: ['#2'],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: ['#3'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#4'],
        },
        {
          name: 'depositoryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#5'],
        },
        {
          name: 'depositoryShares',
          isMut: true,
          isSigner: false,
          docs: ['#6'],
        },
        {
          name: 'credixProgramState',
          isMut: false,
          isSigner: false,
          docs: ['#7'],
        },
        {
          name: 'credixGlobalMarketState',
          isMut: true,
          isSigner: false,
          docs: ['#8'],
        },
        {
          name: 'credixSigningAuthority',
          isMut: false,
          isSigner: false,
          docs: ['#9'],
        },
        {
          name: 'credixLiquidityCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#10'],
        },
        {
          name: 'credixSharesMint',
          isMut: true,
          isSigner: false,
          docs: ['#11'],
        },
        {
          name: 'credixPass',
          isMut: true,
          isSigner: false,
          docs: ['#12'],
        },
        {
          name: 'credixTreasuryPoolCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#13'],
        },
        {
          name: 'credixTreasury',
          isMut: false,
          isSigner: false,
          docs: ['#14'],
        },
        {
          name: 'credixTreasuryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#15'],
        },
        {
          name: 'profitsBeneficiaryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#16'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#17'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#18'],
        },
        {
          name: 'associatedTokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#19'],
        },
        {
          name: 'credixProgram',
          isMut: false,
          isSigner: false,
          docs: ['#20'],
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
          docs: ['#21'],
        },
      ],
      args: [],
    },
    {
      name: 'exchangeLiquidityWithCredixLpDepository',
      accounts: [
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: ['#1'],
        },
        {
          name: 'user',
          isMut: false,
          isSigner: true,
          docs: ['#2'],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: ['#3'],
        },
        {
          name: 'identityDepository',
          isMut: true,
          isSigner: false,
          docs: ['#4'],
        },
        {
          name: 'identityDepositoryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#5'],
        },
        {
          name: 'credixLpDepository',
          isMut: true,
          isSigner: false,
          docs: ['#6'],
        },
        {
          name: 'credixLpDepositoryShares',
          isMut: true,
          isSigner: false,
          docs: ['#7'],
        },
        {
          name: 'userCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#8'],
        },
        {
          name: 'receiverCredixShares',
          isMut: true,
          isSigner: false,
          docs: ['#9'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#10'],
        },
        {
          name: 'credixSharesMint',
          isMut: false,
          isSigner: false,
          docs: ['#11'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#12'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#13'],
        },
      ],
      args: [
        {
          name: 'collateralAmount',
          type: 'u64',
        },
      ],
    },
    {
      name: 'rebalanceCreateWithdrawRequestFromCredixLpDepository',
      accounts: [
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: [
            '#1',
            'Permissionless IX that can be called by anyone at any time',
          ],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: ['#2'],
        },
        {
          name: 'identityDepository',
          isMut: true,
          isSigner: false,
          docs: ['#3'],
        },
        {
          name: 'mercurialVaultDepository',
          isMut: true,
          isSigner: false,
          docs: ['#4'],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: ['#5'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#6'],
        },
        {
          name: 'depositoryShares',
          isMut: true,
          isSigner: false,
          docs: ['#7'],
        },
        {
          name: 'credixGlobalMarketState',
          isMut: false,
          isSigner: false,
          docs: ['#8'],
        },
        {
          name: 'credixSigningAuthority',
          isMut: false,
          isSigner: false,
          docs: ['#9'],
        },
        {
          name: 'credixLiquidityCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#10'],
        },
        {
          name: 'credixSharesMint',
          isMut: true,
          isSigner: false,
          docs: ['#11'],
        },
        {
          name: 'credixPass',
          isMut: false,
          isSigner: false,
          docs: ['#12'],
        },
        {
          name: 'credixWithdrawEpoch',
          isMut: true,
          isSigner: false,
          docs: ['#13'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#14'],
        },
        {
          name: 'credixProgram',
          isMut: false,
          isSigner: false,
          docs: ['#15'],
        },
      ],
      args: [],
    },
    {
      name: 'rebalanceRedeemWithdrawRequestFromCredixLpDepository',
      accounts: [
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
          docs: [
            '#1 // Permissionless IX that can be called by anyone at any time',
          ],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: ['#2'],
        },
        {
          name: 'identityDepository',
          isMut: true,
          isSigner: false,
          docs: ['#3'],
        },
        {
          name: 'identityDepositoryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#4'],
        },
        {
          name: 'mercurialVaultDepository',
          isMut: true,
          isSigner: false,
          docs: ['#5'],
        },
        {
          name: 'depository',
          isMut: true,
          isSigner: false,
          docs: ['#6'],
        },
        {
          name: 'collateralMint',
          isMut: false,
          isSigner: false,
          docs: ['#7'],
        },
        {
          name: 'depositoryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#8'],
        },
        {
          name: 'depositoryShares',
          isMut: true,
          isSigner: false,
          docs: ['#9'],
        },
        {
          name: 'credixProgramState',
          isMut: false,
          isSigner: false,
          docs: ['#10'],
        },
        {
          name: 'credixGlobalMarketState',
          isMut: true,
          isSigner: false,
          docs: ['#11'],
        },
        {
          name: 'credixSigningAuthority',
          isMut: false,
          isSigner: false,
          docs: ['#12'],
        },
        {
          name: 'credixLiquidityCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#13'],
        },
        {
          name: 'credixSharesMint',
          isMut: true,
          isSigner: false,
          docs: ['#14'],
        },
        {
          name: 'credixPass',
          isMut: true,
          isSigner: false,
          docs: ['#15'],
        },
        {
          name: 'credixTreasuryPoolCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#16'],
        },
        {
          name: 'credixTreasury',
          isMut: false,
          isSigner: false,
          docs: ['#17'],
        },
        {
          name: 'credixTreasuryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#18'],
        },
        {
          name: 'credixWithdrawEpoch',
          isMut: true,
          isSigner: false,
          docs: ['#19'],
        },
        {
          name: 'profitsBeneficiaryCollateral',
          isMut: true,
          isSigner: false,
          docs: ['#20'],
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['#21'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#22'],
        },
        {
          name: 'associatedTokenProgram',
          isMut: false,
          isSigner: false,
          docs: ['#23'],
        },
        {
          name: 'credixProgram',
          isMut: false,
          isSigner: false,
          docs: ['#24'],
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
          docs: ['#25'],
        },
      ],
      args: [],
    },
    {
      name: 'freezeProgram',
      docs: [
        'Freeze or resume all ixs associated with the controller (except this one).',
        '',
        'Parameters:',
        '- freeze: bool param to flip the `is_frozen` property in the controller',
        '',
        'Note:',
        'This is a wildcard to stop the program temporarily when a vulnerability has been detected to allow the team to do servicing work.',
        '',
      ],
      accounts: [
        {
          name: 'authority',
          isMut: false,
          isSigner: true,
          docs: [
            '#1 Authored call accessible only to the signer matching Controller.authority',
          ],
        },
        {
          name: 'controller',
          isMut: true,
          isSigner: false,
          docs: [
            '#2 The top level UXDProgram on chain account managing the redeemable mint',
          ],
        },
      ],
      args: [
        {
          name: 'freeze',
          type: 'bool',
        },
      ],
    },
  ],
  accounts: [
    {
      name: 'controller',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'bump',
            type: 'u8',
          },
          {
            name: 'redeemableMintBump',
            type: 'u8',
          },
          {
            name: 'version',
            type: 'u8',
          },
          {
            name: 'authority',
            type: 'publicKey',
          },
          {
            name: 'redeemableMint',
            type: 'publicKey',
          },
          {
            name: 'redeemableMintDecimals',
            type: 'u8',
          },
          {
            name: 'unused',
            type: {
              array: ['u8', 255],
            },
          },
          {
            name: 'isFrozen',
            type: 'bool',
          },
          {
            name: 'unused2',
            type: 'u8',
          },
          {
            name: 'redeemableGlobalSupplyCap',
            type: 'u128',
          },
          {
            name: 'unused3',
            type: {
              array: ['u8', 8],
            },
          },
          {
            name: 'redeemableCirculatingSupply',
            type: 'u128',
          },
          {
            name: 'unused4',
            type: {
              array: ['u8', 266],
            },
          },
          {
            name: 'profitsTotalCollected',
            type: 'u128',
          },
          {
            name: 'identityDepositoryWeightBps',
            type: 'u16',
          },
          {
            name: 'mercurialVaultDepositoryWeightBps',
            type: 'u16',
          },
          {
            name: 'credixLpDepositoryWeightBps',
            type: 'u16',
          },
          {
            name: 'identityDepository',
            type: 'publicKey',
          },
          {
            name: 'mercurialVaultDepository',
            type: 'publicKey',
          },
          {
            name: 'credixLpDepository',
            type: 'publicKey',
          },
          {
            name: 'outflowLimitPerEpochAmount',
            type: 'u64',
          },
          {
            name: 'outflowLimitPerEpochBps',
            type: 'u16',
          },
          {
            name: 'slotsPerEpoch',
            type: 'u64',
          },
          {
            name: 'epochOutflowAmount',
            type: 'u64',
          },
          {
            name: 'lastOutflowSlot',
            type: 'u64',
          },
          {
            name: 'reserved',
            type: {
              array: ['u8', 94],
            },
          },
        ],
      },
    },
    {
      name: 'credixLpDepository',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'bump',
            type: 'u8',
          },
          {
            name: 'version',
            type: 'u8',
          },
          {
            name: 'controller',
            type: 'publicKey',
          },
          {
            name: 'collateralMint',
            type: 'publicKey',
          },
          {
            name: 'depositoryCollateral',
            type: 'publicKey',
          },
          {
            name: 'depositoryShares',
            type: 'publicKey',
          },
          {
            name: 'credixProgramState',
            type: 'publicKey',
          },
          {
            name: 'credixGlobalMarketState',
            type: 'publicKey',
          },
          {
            name: 'credixSigningAuthority',
            type: 'publicKey',
          },
          {
            name: 'credixLiquidityCollateral',
            type: 'publicKey',
          },
          {
            name: 'credixSharesMint',
            type: 'publicKey',
          },
          {
            name: 'redeemableAmountUnderManagementCap',
            type: 'u128',
          },
          {
            name: 'mintingFeeInBps',
            type: 'u8',
          },
          {
            name: 'redeemingFeeInBps',
            type: 'u8',
          },
          {
            name: 'mintingDisabled',
            type: 'bool',
          },
          {
            name: 'collateralAmountDeposited',
            type: 'u128',
          },
          {
            name: 'redeemableAmountUnderManagement',
            type: 'u128',
          },
          {
            name: 'mintingFeeTotalAccrued',
            type: 'u128',
          },
          {
            name: 'redeemingFeeTotalAccrued',
            type: 'u128',
          },
          {
            name: 'profitsTotalCollected',
            type: 'u128',
          },
          {
            name: 'profitsBeneficiaryCollateral',
            type: 'publicKey',
          },
          {
            name: 'reserved',
            type: {
              array: ['u8', 768],
            },
          },
        ],
      },
    },
    {
      name: 'identityDepository',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'bump',
            type: 'u8',
          },
          {
            name: 'version',
            type: 'u8',
          },
          {
            name: 'collateralMint',
            type: 'publicKey',
          },
          {
            name: 'collateralMintDecimals',
            type: 'u8',
          },
          {
            name: 'collateralVault',
            type: 'publicKey',
          },
          {
            name: 'collateralVaultBump',
            type: 'u8',
          },
          {
            name: 'collateralAmountDeposited',
            type: 'u128',
          },
          {
            name: 'redeemableAmountUnderManagement',
            type: 'u128',
          },
          {
            name: 'redeemableAmountUnderManagementCap',
            type: 'u128',
          },
          {
            name: 'mintingDisabled',
            type: 'bool',
          },
          {
            name: 'mangoCollateralReinjectedWsol',
            type: 'bool',
          },
          {
            name: 'mangoCollateralReinjectedBtc',
            type: 'bool',
          },
          {
            name: 'mangoCollateralReinjectedEth',
            type: 'bool',
          },
          {
            name: 'reserved',
            type: {
              array: ['u8', 512],
            },
          },
        ],
      },
    },
    {
      name: 'mercurialVaultDepository',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'bump',
            type: 'u8',
          },
          {
            name: 'version',
            type: 'u8',
          },
          {
            name: 'collateralMint',
            type: 'publicKey',
          },
          {
            name: 'collateralMintDecimals',
            type: 'u8',
          },
          {
            name: 'controller',
            type: 'publicKey',
          },
          {
            name: 'collateralAmountDeposited',
            type: 'u128',
          },
          {
            name: 'redeemableAmountUnderManagement',
            type: 'u128',
          },
          {
            name: 'mercurialVault',
            type: 'publicKey',
          },
          {
            name: 'mercurialVaultLpMint',
            type: 'publicKey',
          },
          {
            name: 'mercurialVaultLpMintDecimals',
            type: 'u8',
          },
          {
            name: 'lpTokenVault',
            type: 'publicKey',
          },
          {
            name: 'lpTokenVaultBump',
            type: 'u8',
          },
          {
            name: 'mintingFeeInBps',
            type: 'u8',
          },
          {
            name: 'redeemingFeeInBps',
            type: 'u8',
          },
          {
            name: 'mintingFeeTotalAccrued',
            type: 'u128',
          },
          {
            name: 'redeemingFeeTotalAccrued',
            type: 'u128',
          },
          {
            name: 'redeemableAmountUnderManagementCap',
            type: 'u128',
          },
          {
            name: 'mintingDisabled',
            type: 'bool',
          },
          {
            name: 'profitsTotalCollected',
            type: 'u128',
          },
          {
            name: 'lastProfitsCollectionUnixTimestamp',
            type: 'u64',
          },
          {
            name: 'profitsBeneficiaryCollateral',
            type: 'publicKey',
          },
          {
            name: 'reserved',
            type: {
              array: ['u8', 588],
            },
          },
        ],
      },
    },
  ],
  types: [
    {
      name: 'EditCredixLpDepositoryFields',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'redeemableAmountUnderManagementCap',
            type: {
              option: 'u128',
            },
          },
          {
            name: 'mintingFeeInBps',
            type: {
              option: 'u8',
            },
          },
          {
            name: 'redeemingFeeInBps',
            type: {
              option: 'u8',
            },
          },
          {
            name: 'mintingDisabled',
            type: {
              option: 'bool',
            },
          },
          {
            name: 'profitsBeneficiaryCollateral',
            type: {
              option: 'publicKey',
            },
          },
        ],
      },
    },
    {
      name: 'EditDepositoriesRoutingWeightBps',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'identityDepositoryWeightBps',
            type: 'u16',
          },
          {
            name: 'mercurialVaultDepositoryWeightBps',
            type: 'u16',
          },
          {
            name: 'credixLpDepositoryWeightBps',
            type: 'u16',
          },
        ],
      },
    },
    {
      name: 'EditRouterDepositories',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'identityDepository',
            type: 'publicKey',
          },
          {
            name: 'mercurialVaultDepository',
            type: 'publicKey',
          },
          {
            name: 'credixLpDepository',
            type: 'publicKey',
          },
        ],
      },
    },
    {
      name: 'EditControllerFields',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'redeemableGlobalSupplyCap',
            type: {
              option: 'u128',
            },
          },
          {
            name: 'depositoriesRoutingWeightBps',
            type: {
              option: {
                defined: 'EditDepositoriesRoutingWeightBps',
              },
            },
          },
          {
            name: 'routerDepositories',
            type: {
              option: {
                defined: 'EditRouterDepositories',
              },
            },
          },
          {
            name: 'outflowLimitPerEpochAmount',
            type: {
              option: 'u64',
            },
          },
          {
            name: 'outflowLimitPerEpochBps',
            type: {
              option: 'u16',
            },
          },
          {
            name: 'slotsPerEpoch',
            type: {
              option: 'u64',
            },
          },
        ],
      },
    },
    {
      name: 'EditIdentityDepositoryFields',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'redeemableAmountUnderManagementCap',
            type: {
              option: 'u128',
            },
          },
          {
            name: 'mintingDisabled',
            type: {
              option: 'bool',
            },
          },
        ],
      },
    },
    {
      name: 'EditMercurialVaultDepositoryFields',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'redeemableAmountUnderManagementCap',
            type: {
              option: 'u128',
            },
          },
          {
            name: 'mintingFeeInBps',
            type: {
              option: 'u8',
            },
          },
          {
            name: 'redeemingFeeInBps',
            type: {
              option: 'u8',
            },
          },
          {
            name: 'mintingDisabled',
            type: {
              option: 'bool',
            },
          },
          {
            name: 'profitsBeneficiaryCollateral',
            type: {
              option: 'publicKey',
            },
          },
        ],
      },
    },
  ],
  events: [
    {
      name: 'InitializeControllerEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: false,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'authority',
          type: 'publicKey',
          index: false,
        },
      ],
    },
    {
      name: 'SetRedeemableGlobalSupplyCapEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: false,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'redeemableGlobalSupplyCap',
          type: 'u128',
          index: false,
        },
      ],
    },
    {
      name: 'SetOutflowLimitPerEpochAmountEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: false,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'outflowLimitPerEpochAmount',
          type: 'u64',
          index: false,
        },
      ],
    },
    {
      name: 'SetOutflowLimitPerEpochBpsEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: false,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'outflowLimitPerEpochBps',
          type: 'u16',
          index: false,
        },
      ],
    },
    {
      name: 'SetSlotsPerEpochEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: false,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'slotsPerEpoch',
          type: 'u64',
          index: false,
        },
      ],
    },
    {
      name: 'SetRouterDepositoriesWeightBps',
      fields: [
        {
          name: 'controllerVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'identityDepositoryWeightBps',
          type: 'u16',
          index: false,
        },
        {
          name: 'mercurialVaultDepositoryWeightBps',
          type: 'u16',
          index: false,
        },
        {
          name: 'credixLpDepositoryWeightBps',
          type: 'u16',
          index: false,
        },
      ],
    },
    {
      name: 'SetRouterDepositories',
      fields: [
        {
          name: 'controllerVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'identityDepository',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'mercurialVaultDepository',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'credixLpDepository',
          type: 'publicKey',
          index: false,
        },
      ],
    },
    {
      name: 'RegisterMercurialVaultDepositoryEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: true,
        },
        {
          name: 'depositoryVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'mercurialVault',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'depositoryLpTokenVault',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'collateralMint',
          type: 'publicKey',
          index: false,
        },
      ],
    },
    {
      name: 'SetDepositoryRedeemableAmountUnderManagementCapEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'redeemableAmountUnderManagementCap',
          type: 'u128',
          index: true,
        },
      ],
    },
    {
      name: 'SetDepositoryMintingFeeInBpsEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'mintingFeeInBps',
          type: 'u8',
          index: true,
        },
      ],
    },
    {
      name: 'SetDepositoryRedeemingFeeInBpsEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'redeemingFeeInBps',
          type: 'u8',
          index: true,
        },
      ],
    },
    {
      name: 'SetDepositoryMintingDisabledEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'mintingDisabled',
          type: 'bool',
          index: true,
        },
      ],
    },
    {
      name: 'SetDepositoryProfitsBeneficiaryCollateralEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'profitsBeneficiaryCollateral',
          type: 'publicKey',
          index: true,
        },
      ],
    },
    {
      name: 'InitializeIdentityDepositoryEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: false,
        },
        {
          name: 'depositoryVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'collateralMint',
          type: 'publicKey',
          index: false,
        },
      ],
    },
    {
      name: 'MintWithIdentityDepositoryEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: false,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'user',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'collateralAmount',
          type: 'u64',
          index: false,
        },
      ],
    },
    {
      name: 'RedeemFromIdentityDepositoryEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: false,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'user',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'redeemableAmount',
          type: 'u64',
          index: false,
        },
      ],
    },
    {
      name: 'RegisterCredixLpDepositoryEvent',
      fields: [
        {
          name: 'controllerVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'depositoryVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'collateralMint',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'credixGlobalMarketState',
          type: 'publicKey',
          index: false,
        },
      ],
    },
    {
      name: 'MintWithCredixLpDepositoryEvent',
      fields: [
        {
          name: 'controllerVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'depositoryVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'user',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'collateralAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'redeemableAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'mintingFeePaid',
          type: 'u64',
          index: false,
        },
      ],
    },
    {
      name: 'RedeemFromCredixLpDepositoryEvent',
      fields: [
        {
          name: 'controllerVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'depositoryVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'user',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'collateralAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'redeemableAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'redeemingFeePaid',
          type: 'u64',
          index: false,
        },
      ],
    },
    {
      name: 'CollectProfitsOfCredixLpDepositoryEvent',
      fields: [
        {
          name: 'controllerVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'depositoryVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'collateralAmount',
          type: 'u64',
          index: false,
        },
      ],
    },
    {
      name: 'ExchangeLiquidityWithCredixLpDepositoryEvent',
      fields: [
        {
          name: 'controllerVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'depositoryVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'collateralAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'sharesAmount',
          type: 'u64',
          index: false,
        },
      ],
    },
    {
      name: 'RebalanceCreateWithdrawRequestFromCredixLpDepositoryEvent',
      fields: [
        {
          name: 'controllerVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'depositoryVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'overflowValue',
          type: 'u64',
          index: false,
        },
        {
          name: 'profitsCollateralAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'requestedCollateralAmount',
          type: 'u64',
          index: false,
        },
      ],
    },
    {
      name: 'RebalanceRedeemWithdrawRequestFromCredixLpDepositoryEvent',
      fields: [
        {
          name: 'controllerVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'depositoryVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'overflowValue',
          type: 'u64',
          index: false,
        },
        {
          name: 'profitsCollateralAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'requestedCollateralAmount',
          type: 'u64',
          index: false,
        },
      ],
    },
    {
      name: 'CollectProfitsOfMercurialVaultDepositoryEvent',
      fields: [
        {
          name: 'controllerVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'depositoryVersion',
          type: 'u8',
          index: true,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'depository',
          type: 'publicKey',
          index: true,
        },
        {
          name: 'collateralAmount',
          type: 'u64',
          index: false,
        },
      ],
    },
    {
      name: 'FreezeProgramEvent',
      fields: [
        {
          name: 'version',
          type: 'u8',
          index: false,
        },
        {
          name: 'controller',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'isFrozen',
          type: 'bool',
          index: false,
        },
      ],
    },
  ],
  errors: [
    {
      code: 6000,
      name: 'InvalidRedeemableMintDecimals',
      msg: 'The redeemable mint decimals must be between 0 and 9 (inclusive).',
    },
    {
      code: 6001,
      name: 'InvalidRedeemableGlobalSupplyCap',
      msg: 'Redeemable global supply above.',
    },
    {
      code: 6002,
      name: 'InvalidDepositoriesWeightBps',
      msg: 'Depositories weights do not add up to 100%.',
    },
    {
      code: 6003,
      name: 'InvalidDepositoriesVector',
      msg: 'Depositories vector passed as parameter is not of the expected length',
    },
    {
      code: 6004,
      name: 'InvalidCollateralAmount',
      msg: 'Collateral amount cannot be 0',
    },
    {
      code: 6005,
      name: 'InvalidRedeemableAmount',
      msg: 'Redeemable amount must be > 0 in order to redeem.',
    },
    {
      code: 6006,
      name: 'InsufficientCollateralAmount',
      msg: 'The balance of the collateral ATA is not enough to fulfill the mint operation.',
    },
    {
      code: 6007,
      name: 'InsufficientRedeemableAmount',
      msg: 'The balance of the redeemable ATA is not enough to fulfill the redeem operation.',
    },
    {
      code: 6008,
      name: 'DepositoriesTargerRedeemableAmountReached',
      msg: 'Minting amount would go past the depositories target redeemable amount.',
    },
    {
      code: 6009,
      name: 'RedeemableGlobalSupplyCapReached',
      msg: 'Minting amount would go past the Redeemable Global Supply Cap.',
    },
    {
      code: 6010,
      name: 'RedeemableMercurialVaultAmountUnderManagementCap',
      msg: 'Minting amount would go past the mercurial vault depository Redeemable Amount Under Management Cap.',
    },
    {
      code: 6011,
      name: 'RedeemableCredixLpAmountUnderManagementCap',
      msg: 'Minting amount would go past the credix lp depository Redeemable Amount Under Management Cap.',
    },
    {
      code: 6012,
      name: 'MathOverflow',
      msg: 'Math overflow.',
    },
    {
      code: 6013,
      name: 'SlippageReached',
      msg: "The order couldn't be executed with the provided slippage.",
    },
    {
      code: 6014,
      name: 'BumpError',
      msg: 'A bump was expected but is missing.',
    },
    {
      code: 6015,
      name: 'MintingDisabled',
      msg: 'Minting is disabled for the current depository.',
    },
    {
      code: 6016,
      name: 'CollateralDepositHasRemainingDust',
      msg: 'Collateral deposit left some value unaccounted for.',
    },
    {
      code: 6017,
      name: 'CollateralDepositAmountsDoesntMatch',
      msg: "Collateral deposit didn't result in the correct amounts being moved.",
    },
    {
      code: 6018,
      name: 'CollateralDepositDoesntMatchTokenValue',
      msg: "Received token of which the value doesn't match the deposited collateral.",
    },
    {
      code: 6019,
      name: 'InvalidMercurialVaultLpMint',
      msg: "The mercurial vault lp mint does not match the Depository's one.",
    },
    {
      code: 6020,
      name: 'MaxNumberOfMercurialVaultDepositoriesRegisteredReached',
      msg: 'Cannot register more mercurial vault depositories, the limit has been reached.',
    },
    {
      code: 6021,
      name: 'MaxNumberOfCredixLpDepositoriesRegisteredReached',
      msg: 'Cannot register more credix lp depositories, the limit has been reached.',
    },
    {
      code: 6022,
      name: 'MercurialVaultDoNotMatchCollateral',
      msg: 'The provided collateral do not match the provided mercurial vault token.',
    },
    {
      code: 6023,
      name: 'CredixLpDoNotMatchCollateral',
      msg: 'The provided collateral do not match the provided credix lp token.',
    },
    {
      code: 6024,
      name: 'CollateralMintEqualToRedeemableMint',
      msg: 'Collateral mint should be different than redeemable mint.',
    },
    {
      code: 6025,
      name: 'CollateralMintNotAllowed',
      msg: 'Provided collateral mint is not allowed.',
    },
    {
      code: 6026,
      name: 'MinimumMintedRedeemableAmountError',
      msg: 'Mint resulted to 0 redeemable token being minted.',
    },
    {
      code: 6027,
      name: 'MaximumMintedRedeemableAmountError',
      msg: 'Mint resulted into too much redeemable token being minted.',
    },
    {
      code: 6028,
      name: 'MinimumRedeemedCollateralAmountError',
      msg: 'Redeem resulted to 0 collateral token being redeemed.',
    },
    {
      code: 6029,
      name: 'MaximumRedeemedCollateralAmountError',
      msg: 'Redeem resulted into too much collateral token being redeemed.',
    },
    {
      code: 6030,
      name: 'InvalidDepositoryLpTokenVault',
      msg: "The depository lp token vault does not match the Depository's one.",
    },
    {
      code: 6031,
      name: 'InvalidAuthority',
      msg: 'Only the Program initializer authority can access this instructions.',
    },
    {
      code: 6032,
      name: 'InvalidController',
      msg: "The Depository's controller doesn't match the provided Controller.",
    },
    {
      code: 6033,
      name: 'InvalidDepository',
      msg: 'The Depository provided is not matching the one stored in the Controller.',
    },
    {
      code: 6034,
      name: 'InvalidCollateralMint',
      msg: "The provided collateral mint does not match the depository's collateral mint.",
    },
    {
      code: 6035,
      name: 'InvalidRedeemableMint',
      msg: "The Redeemable Mint provided does not match the Controller's one.",
    },
    {
      code: 6036,
      name: 'InvalidOwner',
      msg: 'The provided token account is not owner by the expected party.',
    },
    {
      code: 6037,
      name: 'InvalidDepositoryCollateral',
      msg: "The provided depository collateral does not match the depository's one.",
    },
    {
      code: 6038,
      name: 'InvalidDepositoryShares',
      msg: "The provided depository shares does not match the depository's one.",
    },
    {
      code: 6039,
      name: 'InvalidProfitsBeneficiaryCollateral',
      msg: "The Profits beneficiary collateral provided does not match the depository's one.",
    },
    {
      code: 6040,
      name: 'InvalidMercurialVault',
      msg: "The provided mercurial vault does not match the Depository's one.",
    },
    {
      code: 6041,
      name: 'InvalidMercurialVaultCollateralTokenSafe',
      msg: 'The provided mercurial vault collateral token safe does not match the mercurial vault one.',
    },
    {
      code: 6042,
      name: 'RedeemableIdentityDepositoryAmountUnderManagementCap',
      msg: 'Minting amount would go past the identity depository Redeemable Amount Under Management Cap.',
    },
    {
      code: 6043,
      name: 'ProgramAlreadyFrozenOrResumed',
      msg: 'Program is already frozen/resumed.',
    },
    {
      code: 6044,
      name: 'ProgramFrozen',
      msg: 'The program is currently in Frozen state.',
    },
    {
      code: 6045,
      name: 'InvalidCredixProgramState',
      msg: "The Credix ProgramState isn't the Depository one.",
    },
    {
      code: 6046,
      name: 'InvalidCredixGlobalMarketState',
      msg: "The Credix GlobalMarketState isn't the Depository one.",
    },
    {
      code: 6047,
      name: 'InvalidCredixSigningAuthority',
      msg: "The Credix SigningAuthority isn't the Depository one.",
    },
    {
      code: 6048,
      name: 'InvalidCredixLiquidityCollateral',
      msg: "The Credix LiquidityCollateral isn't the Depository one.",
    },
    {
      code: 6049,
      name: 'InvalidCredixSharesMint',
      msg: "The Credix SharesMint isn't the Depository one.",
    },
    {
      code: 6050,
      name: 'InvalidCredixPass',
      msg: "The Credix Pass isn't the one owned by the correct depository.",
    },
    {
      code: 6051,
      name: 'InvalidCredixPassNoFees',
      msg: "The Credix Pass doesn't have the fees exemption.",
    },
    {
      code: 6052,
      name: 'InvalidCredixTreasury',
      msg: "The Credix Treasury isn't the ProgramState one.",
    },
    {
      code: 6053,
      name: 'InvalidCredixTreasuryPoolCollateral',
      msg: "The Credix TreasuryPool isn't the GlobalMarketState one.",
    },
    {
      code: 6054,
      name: 'InvalidCredixWithdrawEpochRequestPhase',
      msg: "The Credix WithdrawEpoch isn't in its request phase.",
    },
    {
      code: 6055,
      name: 'InvalidCredixWithdrawEpochRedeemPhase',
      msg: "The Credix WithdrawEpoch isn't in its redeem phase.",
    },
    {
      code: 6056,
      name: 'Default',
      msg: 'Default - Check the source code for more info.',
    },
    {
      code: 6057,
      name: 'MaximumOutflowAmountError',
      msg: 'Redeem resulted into too much outflow in this epoch, please wait or try again with a smaller amount.',
    },
    {
      code: 6058,
      name: 'InvalidOutflowLimitPerEpochBps',
      msg: 'The outflow_limit_per_epoch_bps is invalid: over 100%.',
    },
  ],
};
