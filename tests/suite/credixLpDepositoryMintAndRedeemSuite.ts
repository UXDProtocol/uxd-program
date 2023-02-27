import { PublicKey, Signer } from '@solana/web3.js';
import {
  Controller,
  CredixLpDepository,
  findATAAddrSync,
  nativeToUi,
} from '@uxd-protocol/uxd-client';
import { expect } from 'chai';
import { mintWithCredixLpDepositoryTest } from '../cases/mintWithCredixLpDepositoryTest';
import { createCredixLpDepositoryDevnetUSDC, transferTokens } from '../utils';
import { getConnection, TXN_OPTS } from '../connection';
import { BN } from '@project-serum/anchor';
import { editCredixLpDepositoryTest } from '../cases/editCredixLpDepositoryTest';
import { editControllerTest } from '../cases/editControllerTest';
import { redeemFromCredixLpDepositoryTest } from '../cases/redeemFromCredixLpDepositoryTest';
import { collectProfitOfCredixLpDepositoryTest } from '../cases/collectProfitsOfCredixLpDepositoryTest';

export const credixLpDepositoryMintAndRedeemSuite = async function ({
  authority,
  user,
  payer,
  profitsBeneficiary,
  controller,
}: {
  authority: Signer;
  user: Signer;
  payer: Signer;
  profitsBeneficiary: Signer;
  controller: Controller;
}) {
  const collateralSymbol = 'USDC';

  let initialControllerGlobalRedeemableSupplyCap: BN;
  let initialRedeemableDepositorySupplyCap: BN;
  let depository: CredixLpDepository;

  before('Setup: fund user', async function () {
    depository = await createCredixLpDepositoryDevnetUSDC();
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

    console.log(
      'initialControllerGlobalRedeemableSupplyCap',
      initialControllerGlobalRedeemableSupplyCap
    );
    console.log(
      'initialRedeemableDepositorySupplyCap',
      initialRedeemableDepositorySupplyCap
    );
  });

  describe('Regular mint/redeem', () => {
    it(`Mint then redeem ${controller.redeemableMintSymbol} with 0.001 ${collateralSymbol}`, async function () {
      const uiAmountCollateralDeposited = 0.001;

      console.log(
        '[🧾 uiAmountCollateralDeposited',
        uiAmountCollateralDeposited,
        depository.collateralSymbol,
        ']'
      );

      const redeemableAmount = await mintWithCredixLpDepositoryTest({
        uiAmountCollateralDeposited,
        user,
        controller,
        depository,
        payer,
      });

      console.log(
        '[🧾 redeemableAmount',
        redeemableAmount,
        controller.redeemableMintSymbol,
        ']'
      );

      await redeemFromCredixLpDepositoryTest({
        redeemableAmount,
        user,
        controller,
        depository,
        payer,
      });
    });
  });

  describe('Over limits', () => {
    it(`Mint for more ${collateralSymbol} than owned (should fail)`, async function () {
      const uiAmountCollateralDeposited = 1_000_000;

      console.log(
        '[🧾 uiAmountCollateralDeposited',
        uiAmountCollateralDeposited,
        depository.collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithCredixLpDepositoryTest({
          uiAmountCollateralDeposited,
          user,
          controller,
          depository,
          payer,
        });
      } catch {
        failure = true;
      }
      expect(failure).eq(
        true,
        `Should have failed - Do not own enough ${collateralSymbol}`
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
        await redeemFromCredixLpDepositoryTest({
          redeemableAmount,
          user,
          controller,
          depository,
          payer,
        });
      } catch {
        failure = true;
      }
      expect(failure).eq(
        true,
        `Should have failed - Do not own enough ${controller.redeemableMintSymbol}`
      );
    });

    it(`Mint for 0 ${collateralSymbol} (should fail)`, async function () {
      const uiAmountCollateralDeposited = 0;

      console.log(
        '[🧾 uiAmountCollateralDeposited',
        uiAmountCollateralDeposited,
        depository.collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithCredixLpDepositoryTest({
          uiAmountCollateralDeposited,
          user,
          controller,
          depository,
          payer,
        });
      } catch {
        failure = true;
      }

      expect(failure).eq(
        true,
        `Should have failed - Cannot mint for 0 ${collateralSymbol}`
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
        await redeemFromCredixLpDepositoryTest({
          redeemableAmount,
          user,
          controller,
          depository,
          payer,
        });
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
      `Setup: Mint ${controller.redeemableMintSymbol} with 0.001 ${collateralSymbol}`,
      async function () {
        const uiAmountCollateralDeposited = 0.001;

        console.log(
          '[🧾 uiAmountCollateralDeposited',
          uiAmountCollateralDeposited,
          depository.collateralSymbol,
          ']'
        );

        await mintWithCredixLpDepositoryTest({
          uiAmountCollateralDeposited,
          user,
          controller,
          depository,
          payer,
        });
      }
    );

    it(`Mint for 1 native unit ${collateralSymbol} (should fail)`, async function () {
      const uiAmountCollateralDeposited = Math.pow(
        10,
        -depository.collateralDecimals
      );

      console.log(
        '[🧾 uiAmountCollateralDeposited',
        uiAmountCollateralDeposited,
        depository.collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithCredixLpDepositoryTest({
          uiAmountCollateralDeposited,
          user,
          controller,
          depository,
          payer,
        });
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
        await redeemFromCredixLpDepositoryTest({
          redeemableAmount,
          user,
          controller,
          depository,
          payer,
        });
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
      editControllerTest({
        authority,
        controller,
        uiFields: {
          redeemableGlobalSupplyCap: 0,
        },
      }));

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${collateralSymbol} (should fail)`, async function () {
      const uiAmountCollateralDeposited = 0.001;

      console.log(
        '[🧾 uiAmountCollateralDeposited',
        uiAmountCollateralDeposited,
        depository.collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithCredixLpDepositoryTest({
          uiAmountCollateralDeposited,
          user,
          controller,
          depository,
          payer,
        });
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

      await editControllerTest({
        authority,
        controller,
        uiFields: {
          redeemableGlobalSupplyCap: globalRedeemableSupplyCap,
        },
      });
    });
  });

  describe('Redeemable depository supply cap overflow', () => {
    it('Set redeemable depository supply cap to 0,0005 more than actual minted amount', async function () {
      const onChainDepository = await depository.getOnchainAccount(
        getConnection(),
        TXN_OPTS
      );

      await editCredixLpDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          redeemableAmountUnderManagementCap:
            nativeToUi(
              onChainDepository.redeemableAmountUnderManagement,
              controller.redeemableMintDecimals
            ) + 0.0005,
        },
      });
    });

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${collateralSymbol} (should fail)`, async function () {
      const uiAmountCollateralDeposited = 0.001;

      console.log(
        '[🧾 collateralAmount',
        uiAmountCollateralDeposited,
        depository.collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithCredixLpDepositoryTest({
          uiAmountCollateralDeposited,
          user,
          controller,
          depository,
          payer,
        });
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

      await editCredixLpDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          redeemableAmountUnderManagementCap,
        },
      });
    });
  });

  describe('Disabled minting', () => {
    it('Disable minting on credix lp depository', async function () {
      await editCredixLpDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          mintingDisabled: true,
        },
      });
    });

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${collateralSymbol} (should fail)`, async function () {
      const uiAmountCollateralDeposited = 0.001;

      console.log(
        '[🧾 uiAmountCollateralDeposited',
        uiAmountCollateralDeposited,
        depository.collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithCredixLpDepositoryTest({
          uiAmountCollateralDeposited,
          user,
          controller,
          depository,
          payer,
        });
      } catch {
        failure = true;
      }

      expect(failure).eq(true, `Should have failed - minting is disabled`);
    });

    it(`Re-enable minting for credix lp depository`, async function () {
      await editCredixLpDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          mintingDisabled: false,
        },
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
      await editCredixLpDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          profitsBeneficiaryCollateral,
        },
      });
      await collectProfitOfCredixLpDepositoryTest({
        payer,
        profitsBeneficiaryCollateral,
        controller,
        depository,
      });
    });

    it(`Collecting profits of credixLpDepository should not work for invalid collateral address`, async function () {
      console.log('[🧾 collectProfits]');
      await editCredixLpDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          profitsBeneficiaryCollateral: PublicKey.default,
        },
      });
      let failure = false;
      try {
        await collectProfitOfCredixLpDepositoryTest({
          payer,
          profitsBeneficiaryCollateral: PublicKey.default,
          controller,
          depository,
        });
      } catch {
        failure = true;
      }
      expect(failure).eq(
        true,
        `Should have failed - Invalid profits beneficiary`
      );
    });
  });
};
