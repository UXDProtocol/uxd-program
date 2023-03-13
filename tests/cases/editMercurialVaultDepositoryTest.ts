import { PublicKey, Signer } from '@solana/web3.js';
import {
  Controller,
  MercurialVaultDepository,
  nativeToUi,
} from '@uxd-protocol/uxd-client';
import { expect } from 'chai';
import { editMercurialVaultDepository } from '../api';
import { getConnection, TXN_OPTS } from '../connection';
import { CLUSTER } from '../constants';

export const editMercurialVaultDepositoryTest = async function ({
  authority,
  controller,
  depository,
  uiFields,
}: {
  authority: Signer;
  controller: Controller;
  depository: MercurialVaultDepository;
  uiFields: {
    redeemableAmountUnderManagementCap?: number;
    mintingFeeInBps?: number;
    redeemingFeeInBps?: number;
    mintingDisabled?: boolean;
    profitsBeneficiaryCollateral?: PublicKey;
    redeemableAmountUnderManagementWeight?: number;
  };
}) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group('🧭 editMercurialVaultDepositoryTest');
  try {
    // GIVEN
    const depositoryOnchainAccount = await depository.getOnchainAccount(
      connection,
      options
    );

    const {
      redeemableAmountUnderManagementCap,
      mintingFeeInBps,
      redeemingFeeInBps,
      mintingDisabled,
      profitsBeneficiaryCollateral,
      redeemableAmountUnderManagementWeight,
    } = depositoryOnchainAccount;

    // WHEN
    const txId = await editMercurialVaultDepository({
      authority,
      controller,
      depository,
      uiFields,
    });
    console.log(
      `🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`
    );

    // THEN
    const depositoryOnchainAccount_post = await depository.getOnchainAccount(
      connection,
      options
    );
    const {
      redeemableAmountUnderManagementCap:
        redeemableAmountUnderManagementCap_post,
      mintingFeeInBps: mintingFeeInBps_post,
      redeemingFeeInBps: redeemingFeeInBps_post,
      mintingDisabled: mintingDisabled_post,
      profitsBeneficiaryCollateral: profitsBeneficiaryCollateral_post,
      redeemableAmountUnderManagementWeight:
        redeemableAmountUnderManagementWeight_post,
    } = depositoryOnchainAccount_post;

    if (uiFields.redeemableAmountUnderManagementCap) {
      const redeemableAmountUnderManagementCapUi = nativeToUi(
        redeemableAmountUnderManagementCap_post,
        controller.redeemableMintDecimals
      );
      expect(
        redeemableAmountUnderManagementCapUi.toFixed(
          controller.redeemableMintDecimals
        )
      ).equals(
        uiFields.redeemableAmountUnderManagementCap.toFixed(
          controller.redeemableMintDecimals
        ),
        'The redeemable depository supply cap has not changed.'
      );
      console.log(
        `🧾 Previous redeemable depository supply cap was`,
        redeemableAmountUnderManagementCap,
        'now is',
        redeemableAmountUnderManagementCap_post
      );
    }
    if (typeof uiFields.mintingFeeInBps !== 'undefined') {
      expect(mintingFeeInBps_post).equals(
        uiFields.mintingFeeInBps,
        'The minting fee has not changed.'
      );
      console.log(
        `🧾 Previous minting fee was`,
        mintingFeeInBps,
        'now is',
        mintingFeeInBps_post
      );
    }
    if (typeof uiFields.redeemingFeeInBps !== 'undefined') {
      expect(redeemingFeeInBps_post).equals(
        uiFields.redeemingFeeInBps,
        'The redeeming fee has not changed.'
      );
      console.log(
        `🧾 Previous redeeming fee was`,
        redeemingFeeInBps,
        'now is',
        redeemingFeeInBps_post
      );
    }
    if (typeof uiFields.mintingDisabled !== 'undefined') {
      expect(mintingDisabled_post).equals(
        uiFields.mintingDisabled,
        'The minting disabled state has not changed.'
      );
      console.log(
        `🧾 Previous minting disabled state was`,
        mintingDisabled,
        'now is',
        mintingDisabled_post
      );
    }
    if (typeof uiFields.profitsBeneficiaryCollateral !== 'undefined') {
      expect(profitsBeneficiaryCollateral_post.toBase58()).equals(
        uiFields.profitsBeneficiaryCollateral.toBase58(),
        'The profits beneficiary collateral state has not changed.'
      );
      console.log(
        `🧾 Previous profits beneficiary collateral state was`,
        profitsBeneficiaryCollateral.toBase58(),
        'now is',
        profitsBeneficiaryCollateral_post.toBase58()
      );
    }
    if (typeof uiFields.redeemableAmountUnderManagementWeight !== 'undefined') {
      expect(redeemableAmountUnderManagementWeight_post).equals(
        uiFields.redeemableAmountUnderManagementWeight,
        'The redeemable depository weight has not changed.'
      );
      console.log(
        `🧾 Previous redeemable depository weight was`,
        redeemableAmountUnderManagementWeight,
        'now is',
        redeemableAmountUnderManagementWeight_post
      );
    }

    controller.info();
    console.groupEnd();
  } catch (error) {
    console.error('❌', error);
    console.groupEnd();
    throw error;
  }
};
