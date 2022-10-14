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
    redeemableDepositorySupplyCap?: number;
    regularMintingDisabled?: boolean;
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
      redeemableDepositorySupplyCap,
      regularMintingDisabled,
    } = depositoryOnchainAccount;

    // WHEN
    const txId = await editMangoDepository(authority, controller, depository, uiFields);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);

    const {
      quoteMintAndRedeemFee: quoteMintAndRedeemFee_post,
      redeemableDepositorySupplyCap: redeemableDepositorySupplyCap_post,
      regularMintingDisabled: regularMintingDisabled_post,
    } = depositoryOnchainAccount_post;

    if (uiFields.quoteMintAndRedeemFee) {
      expect(quoteMintAndRedeemFee_post).equals(uiFields.quoteMintAndRedeemFee, "The quote fee has not changed.");
      console.log(`🧾 Previous quote fee was`, quoteMintAndRedeemFee, "now is", quoteMintAndRedeemFee_post);
    }
    if (uiFields.redeemableDepositorySupplyCap) {
      expect(redeemableDepositorySupplyCap_post.toString()).equals(uiToNative(uiFields.redeemableDepositorySupplyCap, controller.redeemableMintDecimals).toString(), "The redeemable depository supply cap has not changed.");
      console.log(`🧾 Previous redeemable depository supply cap was`, redeemableDepositorySupplyCap.toString(), "now is", redeemableDepositorySupplyCap_post.toString());
    }
    if (typeof uiFields.regularMintingDisabled !== 'undefined') {
      expect(regularMintingDisabled_post).equals(uiFields.regularMintingDisabled, "The regular minting disabled flag has not changed.");
      console.log(`🧾 Previous regular minting disabled flag was`, regularMintingDisabled, "now is", regularMintingDisabled_post);
    }

    controller.info();
    console.groupEnd();
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
