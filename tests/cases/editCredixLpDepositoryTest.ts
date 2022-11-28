import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, CredixLpDepository, uiToNative } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { editCredixLpDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";

export const editCredixLpDepositoryTest = async function (
  authority: Signer,
  controller: Controller,
  depository: CredixLpDepository,
  uiFields: {
    redeemableAmountUnderManagementCap?: number;
    mintingFeeInBps?: number;
    redeemingFeeInBps?: number;
    mintingDisabled?: boolean;
    profitTreasuryCollateral?: PublicKey;
  }
) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group("🧭 editCredixLpDepositoryTest");
  try {
    // GIVEN
    const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);

    const {
      redeemableAmountUnderManagementCap,
      mintingFeeInBps,
      redeemingFeeInBps,
      mintingDisabled,
      profitTreasuryCollateral,
    } = depositoryOnchainAccount;

    // WHEN
    const txId = await editCredixLpDepository(authority, controller, depository, uiFields);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
    const {
      redeemableAmountUnderManagementCap: redeemableAmountUnderManagementCap_post,
      mintingFeeInBps: mintingFeeInBps_post,
      redeemingFeeInBps: redeemingFeeInBps_post,
      mintingDisabled: mintingDisabled_post,
      profitTreasuryCollateral: profitTreasuryCollateral_post,
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
    if (typeof uiFields.profitTreasuryCollateral !== "undefined") {
      expect(profitTreasuryCollateral_post.toString()).equals(
        uiFields.profitTreasuryCollateral.toString(),
        "The profit treasury collateral address has not changed."
      );
      console.log(
        `🧾 Previous profit treasury collateral address was`,
        profitTreasuryCollateral.toString(),
        "now is",
        profitTreasuryCollateral_post.toString()
      );
    }

    controller.info();
    console.groupEnd();
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
