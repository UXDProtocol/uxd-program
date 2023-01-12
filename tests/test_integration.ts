import { Keypair, Signer } from '@solana/web3.js';
import { Controller, UXD_DECIMALS } from '@uxd-protocol/uxd-client';
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
import { editMercurialVaultDepositorySuite } from './suite/editMercurialVaultDepositorySuite';
import { mercurialVaultDepositoryCollectProfitSuite } from './suite/mercurialVaultCollectProfitSuite';

const controller = new Controller('UXD', UXD_DECIMALS, uxdProgramId);

beforeEach('\n', function () {
  console.log('=============================================\n\n');
});

describe('UXD Controller Suite', function () {
  controllerIntegrationSuite({
    authority,
    controller,
    payer: bank,
  });
});

let user: Signer = new Keypair();
let profitsBeneficiaryKey: Signer = new Keypair();

describe('Mercurial vault integration tests: USDC', async function () {
  this.beforeAll('Setup: fund user', async function () {
    console.log('USER =>', user.publicKey.toString());
    await transferSol(1, bank, user.publicKey);
  });

  mercurialVaultDepositorySetupSuite({
    authority,
    controller,
    mintingFeeInBps: 0,
    redeemingFeeInBps: 5,
    redeemableAmountUnderManagementCap: 1_000,
    payer: bank,
  });

  describe('mercurialVaultDepositoryMintRedeemSuite', () => {
    mercurialVaultDepositoryMintRedeemSuite({
      authority,
      user,
      controller,
      payer: bank,
    });
  });

  describe('editMercurialVaultDepositorySuite', () => {
    editMercurialVaultDepositorySuite({
      authority,
      controller,
    });
  });

  describe('mercurialVaultDepositoryCollectProfitSuite', () => {
    mercurialVaultDepositoryCollectProfitSuite({
      authority,
      controller,
      profitsBeneficiaryKey,
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
      profitsBeneficiaryKey,
      bank.publicKey
    );
    await transferAllSol(user, bank.publicKey);
  });
});
