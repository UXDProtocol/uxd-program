import { PublicKey, Signer } from '@solana/web3.js';
import {
  Controller,
  ControllerAccount,
  findATAAddrSync,
  MercurialVaultDepository,
  nativeToUi,
} from '@uxd-protocol/uxd-client';
import { expect } from 'chai';
import { redeemFromMercurialVaultDepositoryTest } from '../cases/redeemFromMercurialVaultDepositoryTest';
import { mintWithMercurialVaultDepositoryTest } from '../cases/mintWithMercurialVaultDepositoryTest';
import {
  createMercurialVaultDepositoryDevnet,
  getBalance,
  transferTokens,
} from '../utils';
import { getConnection, TXN_OPTS } from '../connection';
import { BN } from '@project-serum/anchor';
import { editMercurialVaultDepositoryTest } from '../cases/editMercurialVaultDepositoryTest';
import { MercurialVaultDepositoryAccount } from '@uxd-protocol/uxd-client';
import { editControllerTest } from '../cases/editControllerTest';

export const mercurialVaultDepositoryMintRedeemSuite = async function ({
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

  let initialRedeemableAccountBalance: number;
  let initialControllerGlobalRedeemableSupplyCap: BN;
  let initialRedeemableDepositorySupplyCap: BN;
  let userRedeemableATA: PublicKey;
  let onchainController: ControllerAccount;
  let onChainDepository: MercurialVaultDepositoryAccount;
  let depository: MercurialVaultDepository;

  before('Setup: fund user', async function () {
    console.log(' user.publicKey', user.publicKey.toBase58());

    depository = await createMercurialVaultDepositoryDevnet();

    console.log(
      'depository.collateralMint.mint',
      depository.collateralMint.mint.toBase58()
    );
    console.log(
      'depository.collateralMint.decimals',
      depository.collateralMint.decimals
    );

    await transferTokens(
      0.002,
      depository.collateralMint.mint,
      depository.collateralMint.decimals,
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
    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${collateralSymbol}`, async function () {
      const collateralAmount = 0.001;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        collateralSymbol,
        ']'
      );

      await mintWithMercurialVaultDepositoryTest({
        collateralAmount,
        user,
        controller,
        depository,
        payer,
      });
    });

    it(`Redeem all ${controller.redeemableMintSymbol} minted previously for ${collateralSymbol}`, async function () {
      const redeemableAccountBalance = await getBalance(userRedeemableATA);

      const previouslyMintedRedeemableAmount =
        redeemableAccountBalance - initialRedeemableAccountBalance;

      console.log(
        '[🧾 redeemableAmount',
        previouslyMintedRedeemableAmount,
        collateralSymbol,
        ']'
      );

      await redeemFromMercurialVaultDepositoryTest({
        redeemableAmount: previouslyMintedRedeemableAmount,
        user,
        controller,
        depository,
        payer,
      });
    });
  });

  describe('Over limits', () => {
    it(`Mint for more ${collateralSymbol} than owned (should fail)`, async function () {
      const collateralAmount = 1_000_000;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithMercurialVaultDepositoryTest({
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
        `Should have failed - Do not own enough ${collateralSymbol}`
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
        await redeemFromMercurialVaultDepositoryTest({
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

    it(`Mint for 0 ${collateralSymbol} (should fail)`, async function () {
      const collateralAmount = 0;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithMercurialVaultDepositoryTest({
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
        await redeemFromMercurialVaultDepositoryTest({
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
      `Setup: Mint ${controller.redeemableMintSymbol} with 0.001 ${collateralSymbol}`,
      async function () {
        const collateralAmount = 0.001;

        console.log(
          '[🧾 collateralAmount',
          collateralAmount,
          collateralSymbol,
          ']'
        );

        await mintWithMercurialVaultDepositoryTest({
          collateralAmount,
          user,
          controller,
          depository,
          payer,
        });
      }
    );

    it(`Mint for 1 native unit ${collateralSymbol}`, async function () {
      const collateralAmount = Math.pow(
        10,
        -depository.collateralMint.decimals
      );

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithMercurialVaultDepositoryTest({
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
        `Should have failed - User cannot mint for 0 ${controller.redeemableMintSymbol} (happens due to precision loss and fees)`
      );
    });

    it(`Redeem for 1 native unit ${controller.redeemableMintSymbol}`, async function () {
      const redeemableAmount = Math.pow(10, -controller.redeemableMintDecimals);

      console.log(
        '[🧾 redeemableAmount',
        redeemableAmount,
        controller.redeemableMintSymbol,
        ']'
      );

      let failure = false;
      try {
        await redeemFromMercurialVaultDepositoryTest({
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
        `Should have failed - User cannot get 0 ${controller.redeemableMintSymbol} from redeem (happens due to precision loss and fees)`
      );
    });

    after(
      `Cleanup: Redeem all ${controller.redeemableMintSymbol} minted previously for ${collateralSymbol}`,
      async function () {
        const redeemableAccountBalance = await getBalance(userRedeemableATA);

        const previouslyMintedRedeemableAmount =
          redeemableAccountBalance - initialRedeemableAccountBalance;

        console.log(
          '[🧾 redeemableAmount',
          previouslyMintedRedeemableAmount,
          collateralSymbol,
          ']'
        );

        await redeemFromMercurialVaultDepositoryTest({
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

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${collateralSymbol} (should fail)`, async function () {
      const collateralAmount = 0.001;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithMercurialVaultDepositoryTest({
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

      await editMercurialVaultDepositoryTest({
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
      const collateralAmount = 0.001;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithMercurialVaultDepositoryTest({
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

    it(`Reset redeemable depository supply cap back to its original value`, async function () {
      const redeemableAmountUnderManagementCap = nativeToUi(
        initialRedeemableDepositorySupplyCap,
        controller.redeemableMintDecimals
      );

      await editMercurialVaultDepositoryTest({
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
    it('Disable minting on mercurial vault depository', async function () {
      await editMercurialVaultDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          mintingDisabled: true,
        },
      });
    });

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${collateralSymbol} (should fail)`, async function () {
      const collateralAmount = 0.001;

      console.log(
        '[🧾 collateralAmount',
        collateralAmount,
        collateralSymbol,
        ']'
      );

      let failure = false;
      try {
        await mintWithMercurialVaultDepositoryTest({
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

    it(`Re-enable minting for mercurial vault depository`, async function () {
      await editMercurialVaultDepositoryTest({
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
