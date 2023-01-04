import { Signer } from "@solana/web3.js";
import { Controller, IdentityDepository, uiToNative } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { editIdentityDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";

export const editIdentityDepositoryTest = async function (
  authority: Signer,
  controller: Controller,
  depository: IdentityDepository,
  uiFields: {
    redeemableAmountUnderManagementCap?: number;
    mintingDisabled?: boolean;
  }
) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group("🧭 editIdentityDepositoryTest");
  try {
    // GIVEN
    const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);

    const { redeemableAmountUnderManagementCap, mintingDisabled } = depositoryOnchainAccount;

    // WHEN
    const txId = await editIdentityDepository(authority, controller, depository, uiFields);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
    const {
      redeemableAmountUnderManagementCap: redeemableAmountUnderManagementCap_post,
      mintingDisabled: mintingDisabled_post,
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
    if (typeof uiFields.mintingDisabled !== "undefined") {
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
