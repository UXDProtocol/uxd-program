import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { setMangoDepositoryQuoteMintAndRedeemFee } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";

export const editMangoDepository = async function (
  authority: Signer,
  controller: Controller,
  depository: MangoDepository,
  uiFields: {
    quoteMintAndRedeemFee: number;
  }
) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group("🧭 editMangoDepository");
  try {
    // GIVEN
    const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);
    const quoteMintAndRedeemFee = depositoryOnchainAccount.quoteMintAndRedeemFee;

    // WHEN
    const txId = await editMangoDepository(authority, controller, depository, uiFields);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
    const quoteMintAndRedeemFee_post = depositoryOnchainAccount_post.quoteMintAndRedeemFee;

    expect(quoteMintAndRedeemFee_post).equals(uiFields.quoteMintAndRedeemFee, "The quote fee has not changed.");
    console.log(`🧾 Previous quote fee was`, quoteMintAndRedeemFee, "now is", quoteMintAndRedeemFee_post);
    controller.info();
    console.groupEnd();
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
