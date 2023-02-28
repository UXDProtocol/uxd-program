import { Keypair, Signer } from '@solana/web3.js';
import {
  Controller,
  UXD_DECIMALS,
  USDC_DEVNET,
  USDC_DECIMALS,
} from '@uxd-protocol/uxd-client';
import {
  authority,
  bank,
  MERCURIAL_USDC_DEVNET,
  MERCURIAL_USDC_DEVNET_DECIMALS,
  uxdProgramId,
} from './constants';
import {
  transferAllSol,
  transferAllTokens,
  transferSol,
  transferTokens,
} from './utils';
import { freezeProgramSuite } from './suite/freezeProgramSuite';
import { editControllerTest } from './cases/editControllerTest';

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

  let user: Signer = new Keypair();

  describe('Integration tests', function () {
    this.beforeAll(
      'Init and fund user (10 SOL and 100 usdc)',
      async function () {
        console.log('USER =>', user.publicKey.toString());

        await transferSol(1, bank, user.publicKey);
        await transferTokens(
          10,
          USDC_DEVNET,
          USDC_DECIMALS,
          bank,
          user.publicKey
        );
        await transferTokens(
          10,
          MERCURIAL_USDC_DEVNET,
          MERCURIAL_USDC_DEVNET_DECIMALS,
          bank,
          user.publicKey
        );
      }
    );

    describe('freezeProgramSuite', async function () {
      await freezeProgramSuite({
        authority,
        user,
        payer: bank,
        controller,
      });
    });

    this.afterAll('Transfer funds back to bank', async function () {
      await transferAllTokens(
        USDC_DEVNET,
        USDC_DECIMALS,
        authority,
        bank.publicKey
      );
      await transferAllTokens(
        MERCURIAL_USDC_DEVNET,
        MERCURIAL_USDC_DEVNET_DECIMALS,
        authority,
        bank.publicKey
      );
      await transferAllSol(user, bank.publicKey);
    });
  });
})();
