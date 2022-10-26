import { BN } from "@project-serum/anchor";
import { Signer } from "@solana/web3.js";
import { Controller, MaplePoolDepository, uiToNative } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { editMaplePoolDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";

export const editMaplePoolDepositoryTest = async function (
  authority: Signer,
  controller: Controller,
  depository: MaplePoolDepository,
  uiFields: {
    redeemableAmountUnderManagementCap?: number;
    mintingFeeInBps?: number;
    redeemingFeeInBps?: number;
  }
) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group("🧭 editMaplePoolDepositoryTest");
  try {
    // GIVEN
    const depositoryOnchainAccount_pre = await depository.getOnchainAccount(connection, options);

    const {
      redeemableAmountUnderManagementCap: redeemableAmountUnderManagementCap_pre,
      mintingFeeInBps: mintingFeeInBps_pre,
      redeemingFeeInBps: redeemingFeeInBps_pre,
    } = depositoryOnchainAccount_pre;

    // WHEN
    const txId = await editMaplePoolDepository(authority, controller, depository, uiFields);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
    const {
      redeemableAmountUnderManagementCap: redeemableAmountUnderManagementCap_post,
      mintingFeeInBps: mintingFeeInBps_post,
      redeemingFeeInBps: redeemingFeeInBps_post,
    } = depositoryOnchainAccount_post;

    if (uiFields.redeemableAmountUnderManagementCap !== undefined) {
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
        redeemableAmountUnderManagementCap_pre.toString(),
        "now is",
        redeemableAmountUnderManagementCap_post.toString()
      );
    }
    if (uiFields.mintingFeeInBps !== undefined) {
      expect(mintingFeeInBps_post).equals(uiFields.mintingFeeInBps, "The minting fee has not changed.");
      console.log(`🧾 Previous minting fee was`, mintingFeeInBps_pre, "now is", mintingFeeInBps_post);
    }
    if (uiFields.redeemingFeeInBps !== undefined) {
      expect(redeemingFeeInBps_post).equals(uiFields.redeemingFeeInBps, "The redeeming fee has not changed.");
      console.log(`🧾 Previous redeeming fee was`, redeemingFeeInBps_pre, "now is", redeemingFeeInBps_post);
    }

    controller.info();
    console.groupEnd();
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
