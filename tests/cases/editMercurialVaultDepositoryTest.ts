import { BN } from "@project-serum/anchor";
import { Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { editMercurialVaultDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";

export const editMercurialVaultDepositoryTest = async function (
  authority: Signer,
  controller: Controller,
  depository: MercurialVaultDepository,
  uiFields: {
    redeemableDepositorySupplyCap?: number;
    mintingFeeInBps?: number;
    redeemingFeeInBps?: number;
  }
) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group("🧭 editMangoDepositoryTest");
  try {
    // GIVEN
    const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);

    const {
      redeemableDepositorySupplyCap,
      mintingFeeInBps,
      redeemingFeeInBps,
    } = depositoryOnchainAccount;

    // WHEN
    const txId = await editMercurialVaultDepository(authority, controller, depository, uiFields);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
    const {
      redeemableDepositorySupplyCap: redeemableDepositorySupplyCap_post,
      mintingFeeInBps: mintingFeeInBps_post,
      redeemingFeeInBps: redeemingFeeInBps_post,
    } = depositoryOnchainAccount_post;

    if (uiFields.redeemableDepositorySupplyCap) {
      const nativeRedeemableDepositorySupplyCap = uiFields.redeemableDepositorySupplyCap * (10 ** controller.redeemableMintDecimals);
      expect(redeemableDepositorySupplyCap_post.toString()).equals(nativeRedeemableDepositorySupplyCap.toString(), "The redeemable depository supply cap has not changed.");
      console.log(`🧾 Previous redeemable depository supply cap was`, redeemableDepositorySupplyCap.toString(), "now is", redeemableDepositorySupplyCap_post.toString());
    }
    if (uiFields.mintingFeeInBps) {
      expect(mintingFeeInBps_post).equals(uiFields.mintingFeeInBps, "The minting fee has not changed.");
      console.log(`🧾 Previous minting fee was`, mintingFeeInBps, "now is", mintingFeeInBps_post);
    }
    if (uiFields.redeemingFeeInBps) {
      expect(redeemingFeeInBps_post).equals(uiFields.redeemingFeeInBps, "The redeeming fee has not changed.");
      console.log(`🧾 Previous redeeming fee was`, redeemingFeeInBps, "now is", redeemingFeeInBps_post);
    }

    controller.info();
    console.groupEnd();
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
