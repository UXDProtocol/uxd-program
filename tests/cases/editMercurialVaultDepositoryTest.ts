import { PublicKey, Signer } from "@solana/web3.js";
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
    profitsRedeemAuthority?: PublicKey;
  }
) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group("🧭 editMercurialVaultDepositoryTest");
  try {
    // GIVEN
    const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);

    const { redeemableAmountUnderManagementCap, mintingFeeInBps, redeemingFeeInBps, mintingDisabled, profitsRedeemAuthority, } =
      depositoryOnchainAccount;

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
      profitsRedeemAuthority: profitsRedeemAuthority_post,
    } = depositoryOnchainAccount_post;

    if (uiFields.redeemableAmountUnderManagementCap) {
      const nativeRedeemableDepositorySupplyCap = uiToNative(
        uiFields.redeemableAmountUnderManagementCap,
        controller.redeemableMintDecimals
      );
      expect(redeemableAmountUnderManagementCap_post.toString()).equals(
        nativeRedeemableDepositorySupplyCap.toString(),
        "The redeemable depository supply cap has not changed."
      );
      console.log(
        `🧾 Previous redeemable depository supply cap was`,
        redeemableAmountUnderManagementCap.toString(),
        "now is",
        redeemableAmountUnderManagementCap_post.toString()
      );
    }
    if (typeof uiFields.mintingFeeInBps !== "undefined") {
      expect(mintingFeeInBps_post).equals(uiFields.mintingFeeInBps, "The minting fee has not changed.");
      console.log(`🧾 Previous minting fee was`, mintingFeeInBps, "now is", mintingFeeInBps_post);
    }
    if (typeof uiFields.redeemingFeeInBps !== "undefined") {
      expect(redeemingFeeInBps_post).equals(uiFields.redeemingFeeInBps, "The redeeming fee has not changed.");
      console.log(`🧾 Previous redeeming fee was`, redeemingFeeInBps, "now is", redeemingFeeInBps_post);
    }
    if (typeof uiFields.mintingDisabled !== "undefined") {
      expect(mintingDisabled_post).equals(uiFields.mintingDisabled, "The minting disabled state has not changed.");
      console.log(`🧾 Previous minting disabled state was`, mintingDisabled, "now is", mintingDisabled_post);
    }
    if (typeof uiFields.profitsRedeemAuthority !== 'undefined') {
      expect(profitsRedeemAuthority_post).equals(uiFields.profitsRedeemAuthority, "The profits redeem authority state has not changed.");
      console.log(`🧾 Previous profits redeem authority state was`, profitsRedeemAuthority, "now is", profitsRedeemAuthority_post);
    }

    controller.info();
    console.groupEnd();
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
