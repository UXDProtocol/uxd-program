import { PublicKey, Signer } from '@solana/web3.js';
import {
  Controller,
  CredixLpDepository,
  findATAAddrSync,
  nativeToUi,
} from '@uxd-protocol/uxd-client';
import { expect } from 'chai';
import { mintWithCredixLpDepositoryTest } from '../cases/mintWithCredixLpDepositoryTest';
import { transferTokens } from '../utils';
import { getConnection, TXN_OPTS } from '../connection';
import { BN } from '@project-serum/anchor';
import { editCredixLpDepositoryTest } from '../cases/editCredixLpDepositoryTest';
import { editControllerTest } from '../cases/editControllerTest';
import { redeemFromCredixLpDepositoryTest } from '../cases/redeemFromCredixLpDepositoryTest';
import { collectProfitOfCredixLpDepositoryTest } from '../cases/collectProfitOfCredixLpDepositoryTest';

export const credixLpDepositoryMintAndRedeemSuite = async function (
  authority: Signer,
  user: Signer,
  payer: Signer,
  profitsBeneficiary: Signer,
  controller: Controller,
  depository: CredixLpDepository
) {
  let initialControllerGlobalRedeemableSupplyCap: BN;
  let initialRedeemableDepositorySupplyCap: BN;

  before('Setup: fund user', async function () {
    await transferTokens(
      0.002,
      depository.collateralMint,
      depository.collateralDecimals,
      payer,
      user.publicKey
    );

    let [onChainController, onChainDepository] = await Promise.all([
      controller.getOnchainAccount(getConnection(), TXN_OPTS),
      depository.getOnchainAccount(getConnection(), TXN_OPTS),
    ]);

    initialControllerGlobalRedeemableSupplyCap =
      onChainController.redeemableGlobalSupplyCap;
    initialRedeemableDepositorySupplyCap =
      onChainDepository.redeemableAmountUnderManagementCap;
  });

  describe('Regular mint/redeem', () => {
    it(`Mint then redeem ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol}`, async function () {
      const collateralAmount = 0.001;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralSymbol,
        ']'
      );

      const redeemableAmount = await mintWithCredixLpDepositoryTest(
        collateralAmount,
        user,
        controller,
        depository,
        payer
      );

      console.log(
        '[🧾 redeemableAmount',
        redeemableAmount,
        controller.redeemableMintSymbol,
        ']'
      );

      await redeemFromCredixLpDepositoryTest(
        redeemableAmount,
        user,
        controller,
        depository,
        payer
      );
    });
  });

  describe('Over limits', () => {
    it(`Mint for more ${depository.collateralSymbol} than owned (should fail)`, async function () {
      const collateralAmount = 1_000_000;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithCredixLpDepositoryTest(
          collateralAmount,
          user,
          controller,
          depository,
          payer
        );
      } catch {
        failure = true;
      }
      expect(failure).eq(
        true,
        `Should have failed - Do not own enough ${depository.collateralSymbol}`
      );
    });

    it(`Redeem for more ${controller.redeemableMintSymbol} than owned (should fail)`, async function () {
      const redeemableAmount = 1_000_000;

      console.log(
        '[🧾 redeemableAmount',
        redeemableAmount,
        controller.redeemableMintSymbol,
        ']'
      );

      let failure = false;
      try {
        await redeemFromCredixLpDepositoryTest(
          redeemableAmount,
          user,
          controller,
          depository,
          payer
        );
      } catch {
        failure = true;
      }
      expect(failure).eq(
        true,
        `Should have failed - Do not own enough ${controller.redeemableMintSymbol}`
      );
    });

    it(`Mint for 0 ${depository.collateralSymbol} (should fail)`, async function () {
      const collateralAmount = 0;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithCredixLpDepositoryTest(
          collateralAmount,
          user,
          controller,
          depository,
          payer
        );
      } catch {
        failure = true;
      }

      expect(failure).eq(
        true,
        `Should have failed - Cannot mint for 0 ${depository.collateralSymbol}`
      );
    });

    it(`Redeem for 0 ${controller.redeemableMintSymbol} (should fail)`, async function () {
      const redeemableAmount = 0;

      console.log(
        '[🧾 redeemableAmount',
        redeemableAmount,
        controller.redeemableMintSymbol,
        ']'
      );

      let failure = false;
      try {
        await redeemFromCredixLpDepositoryTest(
          redeemableAmount,
          user,
          controller,
          depository,
          payer
        );
      } catch {
        failure = true;
      }
      expect(failure).eq(
        true,
        `Should have failed - Do not own enough ${controller.redeemableMintSymbol}`
      );
    });
  });

  describe('1 native unit mint/redeem', async () => {
    before(
      `Setup: Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol}`,
      async function () {
        const collateralAmount = 0.001;

        console.log(
          '[🧾 collateralAmount',
          collateralAmount,
          depository.collateralSymbol,
          ']'
        );

        await mintWithCredixLpDepositoryTest(
          collateralAmount,
          user,
          controller,
          depository,
          payer
        );
      }
    );

    it(`Mint for 1 native unit ${depository.collateralSymbol} (should fail)`, async function () {
      const collateralAmount = Math.pow(10, -depository.collateralDecimals);

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithCredixLpDepositoryTest(
          collateralAmount,
          user,
          controller,
          depository,
          payer
        );
      } catch {
        failure = true;
      }

      expect(failure).eq(
        true,
        `Should have failed - User cannot mint for 0 ${controller.redeemableMintSymbol} (happens due to precision loss and fees)`
      );
    });

    it(`Redeem for 1 native unit ${controller.redeemableMintSymbol} (should fail)`, async function () {
      const redeemableAmount = Math.pow(10, -controller.redeemableMintDecimals);

      console.log(
        '[🧾 redeemableAmount',
        redeemableAmount,
        controller.redeemableMintSymbol,
        ']'
      );

      let failure = false;
      try {
        await redeemFromCredixLpDepositoryTest(
          redeemableAmount,
          user,
          controller,
          depository,
          payer
        );
      } catch {
        failure = true;
      }

      expect(failure).eq(
        true,
        `Should have failed - User cannot redeem for 0 ${controller.redeemableMintSymbol} (happens due to precision loss and fees)`
      );
    });
  });

  describe('Global redeemable supply cap overflow', () => {
    it('Set global redeemable supply cap to 0', () =>
      editControllerTest(authority, controller, {
        redeemableGlobalSupplyCap: 0,
      }));

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol} (should fail)`, async function () {
      const collateralAmount = 0.001;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithCredixLpDepositoryTest(
          collateralAmount,
          user,
          controller,
          depository,
          payer
        );
      } catch {
        failure = true;
      }

      expect(failure).eq(
        true,
        `Should have failed - amount of redeemable overflow the global redeemable supply cap`
      );
    });

    it(`Reset Global Redeemable supply cap back to its original value`, async function () {
      const globalRedeemableSupplyCap = nativeToUi(
        initialControllerGlobalRedeemableSupplyCap,
        controller.redeemableMintDecimals
      );

      await editControllerTest(authority, controller, {
        redeemableGlobalSupplyCap: globalRedeemableSupplyCap,
      });
    });
  });

  describe('Redeemable depository supply cap overflow', () => {
    it('Set redeemable depository supply cap to 0,0005 more than actual minted amount', async function () {
      const onChainDepository = await depository.getOnchainAccount(
        getConnection(),
        TXN_OPTS
      );

      await editCredixLpDepositoryTest(authority, controller, depository, {
        redeemableAmountUnderManagementCap:
          nativeToUi(
            onChainDepository.redeemableAmountUnderManagement,
            controller.redeemableMintDecimals
          ) + 0.0005,
      });
    });

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol} (should fail)`, async function () {
      const collateralAmount = 0.001;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithCredixLpDepositoryTest(
          collateralAmount,
          user,
          controller,
          depository,
          payer
        );
      } catch {
        failure = true;
      }

      expect(failure).eq(
        true,
        `Should have failed - amount of redeemable overflow the redeemable depository supply cap`
      );
    });

    it(`Reset redeemable depository supply cap back to its original value`, async function () {
      const redeemableAmountUnderManagementCap = nativeToUi(
        initialRedeemableDepositorySupplyCap,
        controller.redeemableMintDecimals
      );

      await editCredixLpDepositoryTest(authority, controller, depository, {
        redeemableAmountUnderManagementCap,
      });
    });
  });

  describe('Disabled minting', () => {
    it('Disable minting on credix lp depository', async function () {
      await editCredixLpDepositoryTest(authority, controller, depository, {
        mintingDisabled: true,
      });
    });

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol} (should fail)`, async function () {
      const collateralAmount = 0.001;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithCredixLpDepositoryTest(
          collateralAmount,
          user,
          controller,
          depository,
          payer
        );
      } catch {
        failure = true;
      }

      expect(failure).eq(true, `Should have failed - minting is disabled`);
    });

    it(`Re-enable minting for credix lp depository`, async function () {
      await editCredixLpDepositoryTest(authority, controller, depository, {
        mintingDisabled: false,
      });
    });
  });

  describe('Collecting profits', () => {
    it(`Collecting profits of credixLpDepository should work`, async function () {
      console.log('[🧾 collectProfits]');
      const profitsBeneficiaryCollateral = findATAAddrSync(
        profitsBeneficiary.publicKey,
        depository.collateralMint
      )[0];
      await editCredixLpDepositoryTest(authority, controller, depository, {
        profitsBeneficiaryCollateral: profitsBeneficiaryCollateral,
      });
      await collectProfitOfCredixLpDepositoryTest(
        payer,
        profitsBeneficiaryCollateral,
        controller,
        depository
      );
    });

    it(`Collecting profits of credixLpDepository should not work for invalid collateral address`, async function () {
      console.log('[🧾 collectProfits]');
      await editCredixLpDepositoryTest(authority, controller, depository, {
        profitsBeneficiaryCollateral: PublicKey.default,
      });
      let failure = false;
      try {
        await collectProfitOfCredixLpDepositoryTest(
          payer,
          PublicKey.default,
          controller,
          depository
        );
      } catch {
        failure = true;
      }
      expect(failure).eq(
        true,
        `Should have failed - Invalid profit beneficiary`
      );
    });
  });
};
