import { Signer, Keypair } from '@solana/web3.js';
import { Controller, UXD_DECIMALS } from '@uxd-protocol/uxd-client';
import { editControllerTest } from './cases/editControllerTest';
import {
  authority,
  bank,
  MERCURIAL_USDC_DEVNET,
  MERCURIAL_USDC_DEVNET_DECIMALS,
  uxdProgramId,
} from './constants';
import { editMercurialVaultDepositorySuite } from './suite/editMercurialVaultDepositorySuite';
import { mercurialVaultDepositoryMintRedeemSuite } from './suite/mercurialVaultMintAndRedeemSuite';
import { transferSol, transferAllSol, transferAllTokens } from './utils';

(async () => {
  const controller = new Controller('UXD', UXD_DECIMALS, uxdProgramId);

  beforeEach('\n', function () {
    console.log('=============================================\n\n');
  });

  it('Set controller global supply cap to 25mm', async function () {
    await editControllerTest({
      authority,
      controller,
      uiFields: {
        redeemableGlobalSupplyCap: 25_000_000,
      },
    });
  });

  const user: Signer = new Keypair();

  describe('Mercurial vault integration tests: USDC', async function () {
    this.beforeAll('Setup: fund user', async function () {
      console.log('USER =>', user.publicKey.toString());
      await transferSol(1, bank, user.publicKey);
    });

    describe('mercurialVaultDepositoryMintRedeemSuite', function () {
      mercurialVaultDepositoryMintRedeemSuite({
        authority,
        user,
        controller,
        payer: bank,
      });
    });

    describe('editMercurialVaultDepositorySuite', function () {
      editMercurialVaultDepositorySuite({
        authority,
        controller,
      });
    });

    this.afterAll('Transfer funds back to bank', async function () {
      await transferAllTokens(
        MERCURIAL_USDC_DEVNET,
        MERCURIAL_USDC_DEVNET_DECIMALS,
        user,
        bank.publicKey
      );
      await transferAllSol(user, bank.publicKey);
    });
  });
})();
