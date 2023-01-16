import { Keypair, Signer } from '@solana/web3.js';
import {
  Controller,
  UXD_DECIMALS,
  MercurialVaultDepository,
} from '@uxd-protocol/uxd-client';
import {
  authority,
  bank,
  uxdProgramId,
  MERCURIAL_USDC_DEVNET,
  MERCURIAL_USDC_DEVNET_DECIMALS,
} from './constants';
import { transferAllSol, transferAllTokens, transferSol } from './utils';
import {
  controllerIntegrationSuite,
  controllerIntegrationSuiteParameters,
} from './suite/controllerIntegrationSuite';
import { mercurialVaultDepositorySetupSuite } from './suite/mercurialVaultDepositorySetup';
import { mercurialVaultDepositoryMintRedeemSuite } from './suite/mercurialVaultMintAndRedeemSuite';
import { getConnection } from './connection';
import { editMercurialVaultDepositorySuite } from './suite/editMercurialVaultDepositorySuite';
import { mercurialVaultDepositoryCollectProfitSuite } from './suite/mercurialVaultCollectProfitSuite';

(async () => {
  const controllerUXD = new Controller('UXD', UXD_DECIMALS, uxdProgramId);

  beforeEach('\n', function () {
    console.log('=============================================\n\n');
  });

  describe('UXD Controller Suite', function () {
    const params = new controllerIntegrationSuiteParameters(25_000_000);
    controllerIntegrationSuite(authority, bank, controllerUXD, params);
  });

  let user: Signer = new Keypair();
  let profitsBeneficiary: Signer = new Keypair();

  describe('Mercurial vault integration tests: USDC', async function () {
    this.beforeAll('Setup: fund user', async function () {
      console.log('USER =>', user.publicKey.toString());
      await transferSol(1, bank, user.publicKey);
    });

    const mintingFeeInBps = 0;
    const redeemingFeeInBps = 5;
    const uiRedeemableDepositorySupplyCap = 1_000;

    describe('mercurialVaultDepositorySetupSuite', async function () {
      const mercurialVaultDepository =
        await MercurialVaultDepository.initialize({
          connection: getConnection(),
          collateralMint: {
            mint: MERCURIAL_USDC_DEVNET,
            name: 'USDC',
            symbol: 'USDC',
            decimals: MERCURIAL_USDC_DEVNET_DECIMALS,
          },
          uxdProgramId,
        });
      mercurialVaultDepositorySetupSuite(
        authority,
        bank,
        controllerUXD,
        mercurialVaultDepository,
        mintingFeeInBps,
        redeemingFeeInBps,
        uiRedeemableDepositorySupplyCap
      );
    });

    describe('mercurialVaultDepositoryMintRedeemSuite', async function () {
      const mercurialVaultDepository =
        await MercurialVaultDepository.initialize({
          connection: getConnection(),
          collateralMint: {
            mint: MERCURIAL_USDC_DEVNET,
            name: 'USDC',
            symbol: 'USDC',
            decimals: MERCURIAL_USDC_DEVNET_DECIMALS,
          },
          uxdProgramId,
        });
      mercurialVaultDepositoryMintRedeemSuite(
        authority,
        user,
        bank,
        controllerUXD,
        mercurialVaultDepository
      );
    });

    describe('mercurialVaultDepositoryCollectProfitSuite', () => {
      mercurialVaultDepositoryCollectProfitSuite({
        authority,
        controller: controllerUXD,
        profitsBeneficiary,
        payer: bank,
      });
    });

    this.afterAll('Transfer funds back to bank', async function () {
      await transferAllTokens(
        MERCURIAL_USDC_DEVNET,
        MERCURIAL_USDC_DEVNET_DECIMALS,
        user,
        bank.publicKey
      );
      await transferAllTokens(
        MERCURIAL_USDC_DEVNET,
        MERCURIAL_USDC_DEVNET_DECIMALS,
        profitsBeneficiary,
        bank.publicKey
      );
      await transferAllSol(user, bank.publicKey);
    });
  });
})();
