import { Keypair, Signer } from '@solana/web3.js';
import {
  Controller,
  MercurialVaultDepository,
  MercurialVaultDepositoryAccount,
  nativeToUi,
} from '@uxd-protocol/uxd-client';
import { getConnection, TXN_OPTS } from '../connection';
import { editMercurialVaultDepositoryTest } from '../cases/editMercurialVaultDepositoryTest';
import { createMercurialVaultDepositoryDevnet } from '../utils';

export const editMercurialVaultDepositorySuite = async function ({
  authority,
  controller,
}: {
  authority: Signer;
  controller: Controller;
}) {
  let depository: MercurialVaultDepository;
  let beforeDepository: MercurialVaultDepositoryAccount;

  describe('Edit mint/redeem', () => {
    before(async () => {
      depository = await createMercurialVaultDepositoryDevnet();

      // Snapshot the initial depository values
      beforeDepository = await depository.getOnchainAccount(
        getConnection(),
        TXN_OPTS
      );
    });

    it('Edit mintingFeeInBps alone should work', async function () {
      const mintingFeeInBps = 50;

      console.log('[🧾 mintingFeeInBps', mintingFeeInBps, ']');

      await editMercurialVaultDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          mintingFeeInBps,
        },
      });
    });

    it('Edit redeemingFeeInBps alone should work', async function () {
      const redeemingFeeInBps = 50;

      console.log('[🧾 redeemingFeeInBps', redeemingFeeInBps, ']');

      await editMercurialVaultDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          redeemingFeeInBps,
        },
      });
    });

    it('Edit redeemableAmountUnderManagementCap alone should work', async function () {
      const redeemableAmountUnderManagementCap = 50;

      console.log(
        '[🧾 redeemableAmountUnderManagementCap',
        redeemableAmountUnderManagementCap,
        ']'
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

    it('Edit mintingDisabled alone should work', async function () {
      const mintingDisabled = true;

      console.log('[🧾 mintingDisabled', mintingDisabled, ']');

      await editMercurialVaultDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          mintingDisabled,
        },
      });
    });

    it(`Edit profitsBeneficiaryCollateral alone should work`, async function () {
      const profitsBeneficiaryCollateral = new Keypair().publicKey;

      console.log(
        '[🧾 profitsBeneficiaryCollateral',
        profitsBeneficiaryCollateral,
        ']'
      );

      await editMercurialVaultDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          profitsBeneficiaryCollateral,
        },
      });
    });

    it(`Edit redeemableAmountUnderManagementWeightBps alone should work`, async function () {
      const redeemableAmountUnderManagementWeightBps = 42;

      console.log(
        '[🧾 redeemableAmountUnderManagementWeightBps',
        redeemableAmountUnderManagementWeightBps,
        ']'
      );

      await editMercurialVaultDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          redeemableAmountUnderManagementWeightBps,
        },
      });
    });

    // Restore initial depository values there
    it(`Edit mintingFeeInBps/redeemingFeeInBps/redeemableAmountUnderManagementCap/profitsBeneficiaryCollateral should work`, async function () {
      const {
        redeemableAmountUnderManagementCap,
        mintingFeeInBps,
        redeemingFeeInBps,
        mintingDisabled,
        profitsBeneficiaryCollateral,
        redeemableAmountUnderManagementWeightBps,
      } = beforeDepository;

      const uiRedeemableAmountUnderManagementCap = nativeToUi(
        redeemableAmountUnderManagementCap,
        controller.redeemableMintDecimals
      );

      console.log('[🧾 mintingFeeInBps', mintingFeeInBps, ']');
      console.log('[🧾 redeemingFeeInBps', redeemingFeeInBps, ']');
      console.log(
        '[🧾 redeemableAmountUnderManagementCap',
        uiRedeemableAmountUnderManagementCap,
        ']'
      );
      console.log('[🧾 mintingDisabled', mintingDisabled, ']');
      console.log(
        '[🧾 profitsBeneficiaryCollateral',
        profitsBeneficiaryCollateral,
        ']'
      );
      console.log(
        '[🧾 redeemableAmountUnderManagementWeightBps',
        redeemableAmountUnderManagementWeightBps,
        ']'
      );

      await editMercurialVaultDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          redeemableAmountUnderManagementCap:
            uiRedeemableAmountUnderManagementCap,
          mintingFeeInBps,
          redeemingFeeInBps,
          mintingDisabled,
          redeemableAmountUnderManagementWeightBps,
        },
      });
    });
  });
};
