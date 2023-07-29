import { Signer } from '@solana/web3.js';
import {
  Controller,
  CredixLpDepository,
  nativeToUi,
} from '@uxd-protocol/uxd-client';
import { expect } from 'chai';
import { mintWithCredixLpDepositoryTest } from '../cases/mintWithCredixLpDepositoryTest';
import { createCredixLpDepositoryDevnetUSDC, transferTokens } from '../utils';
import { getConnection, TXN_OPTS } from '../connection';
import { BN } from '@project-serum/anchor';
import { editCredixLpDepositoryTest } from '../cases/editCredixLpDepositoryTest';
import { editControllerTest } from '../cases/editControllerTest';

export const credixLpDepositoryMintSuite = async function ({
  authority,
  user,
  payer,
  controller,
}: {
  authority: Signer;
  user: Signer;
  payer: Signer;
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

    await editCredixLpDepositoryTest({
      authority,
      controller,
      depository,
      uiFields: {
        redeemableAmountUnderManagementCap: 25_000_000,
      },
    });
  });

  describe('Regular mint', () => {
    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${collateralSymbol}`, async function () {
      const uiAmountCollateralDeposited = 0.001;

      console.log(
        '[🧾 uiAmountCollateralDeposited',
        uiAmountCollateralDeposited,
        depository.collateralSymbol,
        ']'
      );
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
          authority,
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
          authority,
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
  });

  describe('1 native unit mint', async () => {
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
          authority,
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
          authority,
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
          authority,
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
          authority,
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
          authority,
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
};
