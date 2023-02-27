import { Signer, Keypair } from '@solana/web3.js';
import { Controller, UXD_DECIMALS } from '@uxd-protocol/uxd-client';
import { editControllerTest } from './cases/editControllerTest';
import { initializeControllerTest } from './cases/initializeControllerTest';
import { authority, bank, uxdProgramId } from './constants';
import { credixLpDepositoryEditSuite } from './suite/credixLpDepositoryEditSuite';
import { credixLpDepositoryMintAndRedeemSuite } from './suite/credixLpDepositoryMintAndRedeemSuite';
import { credixLpDepositorySetupSuite } from './suite/credixLpDepositorySetupSuite';
import {
  transferSol,
  transferAllSol,
  transferAllTokens,
  createCredixLpDepositoryDevnetUSDC,
  transferTokens,
} from './utils';

(async () => {
  const controller = new Controller('UXD', UXD_DECIMALS, uxdProgramId);

  beforeEach('\n', function () {
    console.log('=============================================\n\n');
  });

  it('Initialize Controller', async function () {
    await initializeControllerTest({ authority, controller, payer: bank });
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
  const profitsBeneficiary: Signer = new Keypair();

  const credixLpDepository = await createCredixLpDepositoryDevnetUSDC();
  const collateralMint = credixLpDepository.collateralMint;
  const collateralDecimals = credixLpDepository.collateralDecimals;

  describe('Credix Lp integration tests: USDC', async function () {
    this.beforeAll(
      'Setup: fund user and profitsBeneficiary',
      async function () {
        await transferSol(1, bank, user.publicKey);
        await transferSol(1, bank, profitsBeneficiary.publicKey);
        await transferTokens(
          0.1,
          collateralMint,
          collateralDecimals,
          bank,
          user.publicKey
        );
        await transferTokens(
          0.1,
          collateralMint,
          collateralDecimals,
          bank,
          profitsBeneficiary.publicKey
        );
      }
    );

    describe('credixLpDepositorySetupSuite', function () {
      credixLpDepositorySetupSuite({
        authority,
        payer: bank,
        controller,
        mintingFeeInBps: 0,
        redeemingFeeInBps: 0,
        uiRedeemableAmountUnderManagementCap: 1_000_000,
      });
    });

    describe('credixLpDepositoryMintAndRedeemSuite', function () {
      credixLpDepositoryMintAndRedeemSuite({
        authority,
        user,
        payer: bank,
        profitsBeneficiary,
        controller,
      });
    });

    describe('credixLpDepositoryEditSuite', function () {
      credixLpDepositoryEditSuite({ authority, controller });
    });

    this.afterAll('Transfer funds back to bank', async function () {
      await transferAllTokens(
        collateralMint,
        collateralDecimals,
        user,
        bank.publicKey
      );
      await transferAllTokens(
        collateralMint,
        collateralDecimals,
        profitsBeneficiary,
        bank.publicKey
      );
      await transferAllSol(user, bank.publicKey);
      await transferAllSol(profitsBeneficiary, bank.publicKey);
    });
  });
})();
