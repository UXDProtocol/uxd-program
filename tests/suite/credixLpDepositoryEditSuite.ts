import { PublicKey, Signer } from '@solana/web3.js';
import {
  Controller,
  CredixLpDepository,
  nativeToUi,
} from '@uxd-protocol/uxd-client';
import { getConnection, TXN_OPTS } from '../connection';
import { editCredixLpDepositoryTest } from '../cases/editCredixLpDepositoryTest';

export const credixLpDepositoryEditSuite = async function (
  authority: Signer,
  controller: Controller,
  depository: CredixLpDepository
) {
  describe('Edit mint/redeem', async () => {
    let beforeDepository = await depository.getOnchainAccount(
      getConnection(),
      TXN_OPTS
    );

    it(`Edit mintingFeeInBps alone should work`, async function () {
      const mintingFeeInBps = 50;

      console.log('[🧾 mintingFeeInBps', mintingFeeInBps, ']');

      await editCredixLpDepositoryTest(authority, controller, depository, {
        mintingFeeInBps,
      });
    });

    it(`Edit redeemingFeeInBps alone should work`, async function () {
      const redeemingFeeInBps = 50;

      console.log('[🧾 redeemingFeeInBps', redeemingFeeInBps, ']');

      await editCredixLpDepositoryTest(authority, controller, depository, {
        redeemingFeeInBps,
      });
    });

    it(`Edit redeemableAmountUnderManagementCap alone should work`, async function () {
      const redeemableAmountUnderManagementCap = 50;

      console.log(
        '[🧾 redeemableAmountUnderManagementCap',
        redeemableAmountUnderManagementCap,
        ']'
      );

      await editCredixLpDepositoryTest(authority, controller, depository, {
        redeemableAmountUnderManagementCap,
      });
    });

    it(`Edit mintingDisabled alone should work`, async function () {
      const mintingDisabled = true;

      console.log('[🧾 mintingDisabled', mintingDisabled, ']');

      await editCredixLpDepositoryTest(authority, controller, depository, {
        mintingDisabled,
      });
    });

    it(`Edit profitsBeneficiaryCollateral alone should work`, async function () {
      const profitsBeneficiaryCollateral = new PublicKey('42');

      console.log(
        '[🧾 profitsBeneficiaryCollateral',
        profitsBeneficiaryCollateral,
        ']'
      );

      await editCredixLpDepositoryTest(authority, controller, depository, {
        profitsBeneficiaryCollateral,
      });
    });

    // Restore initial depository values there
    it(`Edit mintingFeeInBps/redeemingFeeInBps/redeemableAmountUnderManagementCap should work`, async function () {
      const {
        mintingFeeInBps,
        redeemingFeeInBps,
        redeemableAmountUnderManagementCap,
        mintingDisabled,
        profitsBeneficiaryCollateral,
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

      await editCredixLpDepositoryTest(authority, controller, depository, {
        mintingFeeInBps,
        redeemingFeeInBps,
        redeemableAmountUnderManagementCap:
          uiRedeemableAmountUnderManagementCap,
        mintingDisabled,
        profitsBeneficiaryCollateral,
      });
    });
  });
};
