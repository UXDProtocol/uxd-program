import { Signer, Keypair } from '@solana/web3.js';
import { USDC_DECIMALS, USDC_DEVNET } from '@uxd-protocol/uxd-client';
import { Controller, UXD_DECIMALS } from '@uxd-protocol/uxd-client';
import { authority, bank, uxdProgramId } from './constants';
import { editIdentityDepositorySuite } from './suite/editIdentityDepositorySuite';
import { identityDepositoryMintRedeemSuite } from './suite/identityDepositoryMintAndRedeemSuite';
import { transferSol, transferAllSol, transferAllTokens } from './utils';

(async () => {
  const controller = new Controller('UXD', UXD_DECIMALS, uxdProgramId);

  beforeEach('\n', function () {
    console.log('=============================================\n\n');
  });

  const user: Signer = new Keypair();

  describe('Identity depository integration tests: USDC', async function () {
    this.beforeAll('Setup: fund user', async function () {
      console.log('USER =>', user.publicKey.toString());
      await transferSol(1, bank, user.publicKey);
    });

    describe('editIdentityDepositorySuite', function () {
      editIdentityDepositorySuite({
        authority,
        controller,
      });
    });

    describe('identityDepositoryMintRedeemSuite', function () {
      identityDepositoryMintRedeemSuite({
        authority,
        user,
        controller,
        payer: bank,
      });
    });

    this.afterAll('Transfer funds back to bank', async function () {
      await transferAllTokens(USDC_DEVNET, USDC_DECIMALS, user, bank.publicKey);
      await transferAllSol(user, bank.publicKey);
    });
  });
})();
