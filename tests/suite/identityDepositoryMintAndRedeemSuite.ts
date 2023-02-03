import { PublicKey, Signer } from '@solana/web3.js';
import {
  Controller,
  ControllerAccount,
  findATAAddrSync,
  IdentityDepository,
  nativeToUi,
  USDC_DECIMALS,
  USDC_DEVNET,
} from '@uxd-protocol/uxd-client';
import { expect } from 'chai';
import {
  createIdentityDepositoryDevnet,
  getBalance,
  transferTokens,
} from '../utils';
import { getConnection, TXN_OPTS } from '../connection';
import { BN } from '@project-serum/anchor';
import { IdentityDepositoryAccount } from '@uxd-protocol/uxd-client/dist/types/interfaces';
import { redeemFromIdentityDepositoryTest } from '../cases/redeemFromIdentityDepositoryTest';
import { mintWithIdentityDepositoryTest } from '../cases/mintWithIdentityDepositoryTest';
import { editControllerTest } from '../cases/editControllerTest';
import { editIdentityDepositoryTest } from '../cases/editIdentityDepositoryTest';
import { uxdProgramId } from '../constants';

export const identityDepositoryMintRedeemSuite = async function ({
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
  let initialRedeemableAccountBalance: number;
  let initialControllerGlobalRedeemableSupplyCap: BN;
  let initialRedeemableDepositorySupplyCap: BN;
  let userRedeemableATA: PublicKey;
  let onchainController: ControllerAccount;
  let onChainDepository: IdentityDepositoryAccount;
  let depository: IdentityDepository;

  before('Setup: fund user', async function () {
    depository = createIdentityDepositoryDevnet();

    console.log(
      'depository.collateralMint',
      depository.collateralMint.toBase58()
    );
    console.log(
      'depository.collateralMint decimals',
      depository.collateralMintDecimals
    );
    console.log('user.publicKey', user.publicKey.toBase58());

    await transferTokens(
      0.002,
      depository.collateralMint,
      depository.collateralMintDecimals,
      payer,
      user.publicKey
    );

    userRedeemableATA = findATAAddrSync(
      user.publicKey,
      controller.redeemableMintPda
    )[0];

    [initialRedeemableAccountBalance, onchainController, onChainDepository] =
      await Promise.all([
        getBalance(userRedeemableATA),
        controller.getOnchainAccount(getConnection(), TXN_OPTS),
        depository.getOnchainAccount(getConnection(), TXN_OPTS),
      ]);

    initialControllerGlobalRedeemableSupplyCap =
      onchainController.redeemableGlobalSupplyCap;
    initialRedeemableDepositorySupplyCap =
      onChainDepository.redeemableAmountUnderManagementCap;
  });

  describe('Enable minting', () => {
    it(`Set mintingDisabled to false`, async function () {
      await editIdentityDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          mintingDisabled: false,
        },
      });
    });
  });

  describe('Regular mint/redeem', () => {
    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralMintSymbol}`, async function () {
      const collateralAmount = 0.001;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralMintSymbol,
        ']'
      );

      await mintWithIdentityDepositoryTest({
        collateralAmount,
        user,
        controller,
        depository,
        payer,
      });
    });

    it(`Redeem all ${controller.redeemableMintSymbol} minted previously for ${depository.collateralMintSymbol}`, async function () {
      const redeemableAccountBalance = await getBalance(userRedeemableATA);

      const previouslyMintedRedeemableAmount =
        redeemableAccountBalance - initialRedeemableAccountBalance;

      console.log(
        '[🧾 redeemableAmount',
        previouslyMintedRedeemableAmount,
        depository.collateralMintSymbol,
        ']'
      );

      await redeemFromIdentityDepositoryTest({
        redeemableAmount: previouslyMintedRedeemableAmount,
        user,
        controller,
        depository,
        payer,
      });
    });
  });

  describe('Over limits', () => {
    it(`Mint for more ${depository.collateralMintSymbol} than owned (should fail)`, async function () {
      const collateralAmount = 1_000_000;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralMintSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithIdentityDepositoryTest({
          collateralAmount,
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
        `Should have failed - Do not own enough ${depository.collateralMintSymbol}`
      );
    });

    it(`Redeem for more ${controller.redeemableMintSymbol} than owned (should fail)`, async function () {
      const redeemableAmount = initialRedeemableAccountBalance + 1;

      console.log(
        '[🧾 redeemableAmount',
        redeemableAmount,
        controller.redeemableMintSymbol,
        ']'
      );

      let failure = false;
      try {
        await redeemFromIdentityDepositoryTest({
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
        `Should have failed - Only owned ${initialRedeemableAccountBalance} ${controller.redeemableMintSymbol}`
      );
    });

    it(`Mint for 0 ${depository.collateralMintSymbol} (should fail)`, async function () {
      const collateralAmount = 0;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralMintSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithIdentityDepositoryTest({
          collateralAmount,
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
        `Should have failed - Cannot mint for 0 ${depository.collateralMintSymbol}`
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
        await redeemFromIdentityDepositoryTest({
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
        `Should have failed - Cannot redeem for 0 ${controller.redeemableMintSymbol}`
      );
    });
  });

  describe('1 native unit mint/redeem', async () => {
    before(
      `Setup: Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralMintSymbol}`,
      async function () {
        const collateralAmount = 0.001;

        console.log(
          '[🧾 collateralAmount',
          collateralAmount,
          depository.collateralMintSymbol,
          ']'
        );

        await mintWithIdentityDepositoryTest({
          collateralAmount,
          user,
          controller,
          depository,
          payer,
        });
      }
    );

    it(`Mint for 1 native unit ${depository.collateralMintSymbol} (should succeed, because no precision loss on identity)`, async function () {
      const collateralAmount = Math.pow(10, -depository.collateralMintDecimals);
      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralMintSymbol,
        ']'
      );

      try {
        await mintWithIdentityDepositoryTest({
          collateralAmount,
          user,
          controller,
          depository,
          payer,
        });
      } catch {
        expect(true, 'Failing as planned');
      }

      expect(
        false,
        `Should have failed - User cannot mint for 0 ${controller.redeemableMintSymbol} (happens due to precision loss and fees)`
      );
    });

    after(
      `Cleanup: Redeem all ${controller.redeemableMintSymbol} minted previously for ${depository.collateralMintSymbol}`,
      async function () {
        const redeemableAccountBalance = await getBalance(userRedeemableATA);

        const previouslyMintedRedeemableAmount =
          redeemableAccountBalance - initialRedeemableAccountBalance;

        console.log(
          '[🧾 redeemableAmount',
          previouslyMintedRedeemableAmount,
          depository.collateralMintSymbol,
          ']'
        );

        await redeemFromIdentityDepositoryTest({
          redeemableAmount: previouslyMintedRedeemableAmount,
          user,
          controller,
          depository,
          payer,
        });
      }
    );
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

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralMintSymbol} (should fail)`, async function () {
      const collateralAmount = 0.001;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralMintSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithIdentityDepositoryTest({
          collateralAmount,
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

    it('Reset Global Redeemable supply cap back to its original value', async function () {
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

      await editIdentityDepositoryTest({
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

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralMintSymbol} (should fail)`, async function () {
      const collateralAmount = 0.001;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralMintSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithIdentityDepositoryTest({
          collateralAmount,
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

    it('Reset redeemable depository supply cap back to its original value', async function () {
      const redeemableAmountUnderManagementCap = nativeToUi(
        initialRedeemableDepositorySupplyCap,
        controller.redeemableMintDecimals
      );

      await editIdentityDepositoryTest({
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
    it('Disable minting on identity depository', async function () {
      await editIdentityDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          mintingDisabled: true,
        },
      });
    });

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralMintSymbol} (should fail)`, async function () {
      const collateralAmount = 0.001;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        depository.collateralMintSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithIdentityDepositoryTest({
          collateralAmount,
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

    it('Re-enable minting for identity depository', async function () {
      await editIdentityDepositoryTest({
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
