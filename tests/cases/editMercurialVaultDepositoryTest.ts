import { Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository, uiToNative } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { editMercurialVaultDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";

export const editMercurialVaultDepositoryTest = async function (
  authority: Signer,
  controller: Controller,
  depository: MercurialVaultDepository,
  uiFields: {
    redeemableAmountUnderManagementCap?: number;
    mintingFeeInBps?: number;
    redeemingFeeInBps?: number;
    mintingDisabled?: boolean;
  },
) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group("🧭 editMangoDepositoryTest");
  try {
    // GIVEN
    const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);

    const {
      redeemableAmountUnderManagementCap,
      mintingFeeInBps,
      redeemingFeeInBps,
      mintingDisabled,
    } = depositoryOnchainAccount;

    // WHEN
    const txId = await editMercurialVaultDepository(authority, controller, depository, uiFields);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
    const {
      redeemableAmountUnderManagementCap: redeemableAmountUnderManagementCap_post,
      mintingFeeInBps: mintingFeeInBps_post,
      redeemingFeeInBps: redeemingFeeInBps_post,
      mintingDisabled: mintingDisabled_post,
    } = depositoryOnchainAccount_post;

    if (uiFields.redeemableAmountUnderManagementCap) {
      const nativeRedeemableAmountUnderManagementCap = uiToNative(uiFields.redeemableAmountUnderManagementCap, controller.redeemableMintDecimals);
      expect(redeemableAmountUnderManagementCap_post.toString()).equals(nativeRedeemableAmountUnderManagementCap.toString(), "The redeemable amount under management cap has not changed.");
      console.log(`🧾 Previous redeemable amount under management cap was`, redeemableAmountUnderManagementCap.toString(), "now is", redeemableAmountUnderManagementCap_post.toString());
    }
    if (uiFields.mintingFeeInBps) {
      expect(mintingFeeInBps_post).equals(uiFields.mintingFeeInBps, "The minting fee has not changed.");
      console.log(`🧾 Previous minting fee was`, mintingFeeInBps, "now is", mintingFeeInBps_post);
    }
    if (uiFields.redeemingFeeInBps) {
      expect(redeemingFeeInBps_post).equals(uiFields.redeemingFeeInBps, "The redeeming fee has not changed.");
      console.log(`🧾 Previous redeeming fee was`, redeemingFeeInBps, "now is", redeemingFeeInBps_post);
    }
    if (uiFields.mintingDisabled) {
      expect(mintingDisabled_post).equals(uiFields.mintingDisabled, "The minting disabled state has not changed.");
      console.log(`🧾 Previous minting disabled state was`, mintingDisabled, "now is", mintingDisabled_post);
    }

    controller.info();
    console.groupEnd();
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
