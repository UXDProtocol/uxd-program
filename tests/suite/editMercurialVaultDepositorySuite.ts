import { Signer } from '@solana/web3.js';
import {
  Controller,
  MercurialVaultDepository,
  MercurialVaultDepositoryAccount,
  nativeToUi,
} from '@uxd-protocol/uxd-client';
import { getConnection, TXN_OPTS } from '../connection';
import { editMercurialVaultDepositoryTest } from '../cases/editMercurialVaultDepositoryTest';

export const editMercurialVaultDepositorySuite = async function (
  controllerAuthority: Signer,
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: MercurialVaultDepository
) {
  let beforeDepository: MercurialVaultDepositoryAccount;

  describe('Edit mint/redeem', () => {
    // Snapshot the initial depository values
    before(async () => {
      beforeDepository = await depository.getOnchainAccount(
        getConnection(),
        TXN_OPTS
      );
    });

    it(`Edit mintingFeeInBps alone should work`, async function () {
      const mintingFeeInBps = 50;

      console.log('[🧾 mintingFeeInBps', mintingFeeInBps, ']');

      await editMercurialVaultDepositoryTest(
        controllerAuthority,
        controller,
        depository,
        {
          mintingFeeInBps,
        }
      );
    });

    it(`Edit redeemingFeeInBps alone should work`, async function () {
      const redeemingFeeInBps = 50;

      console.log('[🧾 redeemingFeeInBps', redeemingFeeInBps, ']');

      await editMercurialVaultDepositoryTest(
        controllerAuthority,
        controller,
        depository,
        {
          redeemingFeeInBps,
        }
      );
    });

    it(`Edit redeemableAmountUnderManagementCap alone should work`, async function () {
      const redeemableAmountUnderManagementCap = 50;

      console.log(
        '[🧾 redeemableAmountUnderManagementCap',
        redeemableAmountUnderManagementCap,
        ']'
      );

      await editMercurialVaultDepositoryTest(
        controllerAuthority,
        controller,
        depository,
        {
          redeemableAmountUnderManagementCap,
        }
      );
    });

    it(`Edit mintingDisabled alone should work`, async function () {
      const mintingDisabled = true;

      console.log('[🧾 mintingDisabled', mintingDisabled, ']');

      await editMercurialVaultDepositoryTest(
        controllerAuthority,
        controller,
        depository,
        {
          mintingDisabled,
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

      await editMercurialVaultDepositoryTest(
        controllerAuthority,
        controller,
        depository,
        {
          mintingFeeInBps,
          redeemingFeeInBps,
          mintingDisabled,
          redeemableAmountUnderManagementCap:
            uiRedeemableAmountUnderManagementCap,
        }
      );
    });
  });
};
