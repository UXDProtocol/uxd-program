import { BN } from "@project-serum/anchor";
import { Signer } from "@solana/web3.js";
import { uiToNative } from "@uxd-protocol/uxd-client";
import { Controller, MangoDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { editMangoDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";

export const editMangoDepositoryTest = async function (
  authority: Signer,
  controller: Controller,
  depository: MangoDepository,
  uiFields: {
    quoteMintAndRedeemFee?: number;
    redeemableAmountUnderManagementCap?: number;
  }
) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group("🧭 editMangoDepositoryTest");
  try {
    // GIVEN
    const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);
    const {
      quoteMintAndRedeemFee,
      redeemableAmountUnderManagementCap,
    } = depositoryOnchainAccount;

    // WHEN
    const txId = await editMangoDepository(authority, controller, depository, uiFields);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);

    const {
      quoteMintAndRedeemFee: quoteMintAndRedeemFee_post,
      redeemableAmountUnderManagementCap: redeemableAmountUnderManagementCap_post,
    } = depositoryOnchainAccount_post;

    if (uiFields.quoteMintAndRedeemFee) {
      expect(quoteMintAndRedeemFee_post).equals(uiFields.quoteMintAndRedeemFee, "The quote fee has not changed.");
      console.log(`🧾 Previous quote fee was`, quoteMintAndRedeemFee, "now is", quoteMintAndRedeemFee_post);
    }
    if (uiFields.redeemableAmountUnderManagementCap) {
      expect(redeemableAmountUnderManagementCap_post.toString()).equals(uiToNative(uiFields.redeemableAmountUnderManagementCap, controller.redeemableMintDecimals).toString(), "The redeemable amount under management cap has not changed.");
      console.log(`🧾 Previous redeemable amount under management cap was`, redeemableAmountUnderManagementCap.toString(), "now is", redeemableAmountUnderManagementCap_post.toString());
    }

    controller.info();
    console.groupEnd();
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
