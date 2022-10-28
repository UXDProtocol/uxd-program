import { nativeToUi } from "@blockworks-foundation/mango-client";
import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { editController } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";

export const editControllerTest = async function (
  authority: Signer,
  controller: Controller,
  uiFields: {
    quoteMintAndRedeemSoftCap?: {
      value: number;
      depository: MangoDepository;
    };
    redeemableSoftCap?: number;
    redeemableGlobalSupplyCap?: number;
  }
) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group("🧭 editControllerTest");
  try {
    // GIVEN
    const controllerOnChainAccount = await controller.getOnchainAccount(connection, options);

    const quoteMintAndRedeemSoftCap_pre = controllerOnChainAccount.mangoDepositoriesQuoteRedeemableSoftCap;
    const redeemableSoftCap_pre = controllerOnChainAccount.mangoDepositoriesRedeemableSoftCap;
    const redeemableGlobalSupplyCap_pre = controllerOnChainAccount.redeemableGlobalSupplyCap;

    // WHEN
    const txId = await editController(authority, controller, uiFields);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const controllerOnChainAccount_post = await controller.getOnchainAccount(connection, options);

    const redeemableCirculatingSupply = nativeToUi(
      controllerOnChainAccount_post.redeemableCirculatingSupply.toNumber(),
      controller.redeemableMintDecimals
    );

    const quoteMintAndRedeemSoftCap_post = controllerOnChainAccount_post.mangoDepositoriesQuoteRedeemableSoftCap;
    const redeemableSoftCap_post = controllerOnChainAccount_post.mangoDepositoriesRedeemableSoftCap;
    const redeemableGlobalSupplyCap_post = controllerOnChainAccount_post.redeemableGlobalSupplyCap;

    if (typeof uiFields.quoteMintAndRedeemSoftCap !== 'undefined') {
      const quoteMintAndRedeemSoftCap_postUi = nativeToUi(
        quoteMintAndRedeemSoftCap_post.toNumber(),
        uiFields.quoteMintAndRedeemSoftCap.depository.collateralMintDecimals
      );
      expect(quoteMintAndRedeemSoftCap_postUi).equals(
        uiFields.quoteMintAndRedeemSoftCap.value,
        "Quote Mint And Redeem SoftCap must be set"
      );
      console.log(
        `🧾 Previous quote mint and redeem soft cap was`,
        quoteMintAndRedeemSoftCap_pre,
        "now is",
        quoteMintAndRedeemSoftCap_post
      );
    } else {
      expect(quoteMintAndRedeemSoftCap_pre.cmp(quoteMintAndRedeemSoftCap_post)).equals(
        0,
        "Quote Mint And Redeem SoftCap must not have changed"
      );
    }

    if (typeof uiFields.redeemableSoftCap !== 'undefined') {
      const redeemableSoftCap_postUi = nativeToUi(redeemableSoftCap_post.toNumber(), controller.redeemableMintDecimals);
      expect(redeemableSoftCap_postUi).equals(uiFields.redeemableSoftCap, "Redeemable SoftCap must bet set");
      console.log(
        `🧾 Previous mango depositories soft cap was`,
        redeemableSoftCap_pre,
        "now is",
        redeemableSoftCap_post,
        "(circulating supply",
        redeemableCirculatingSupply,
        ")"
      );
    } else {
      expect(redeemableSoftCap_pre.cmp(redeemableSoftCap_post)).equals(0, "Redeemable SoftCap must not have changed");
    }

    if (typeof uiFields.redeemableGlobalSupplyCap !== 'undefined') {
      const redeemableGlobalSupplyCap_postUi = nativeToUi(
        redeemableGlobalSupplyCap_post.toNumber(),
        controller.redeemableMintDecimals
      );
      expect(redeemableGlobalSupplyCap_postUi).equals(
        uiFields.redeemableGlobalSupplyCap,
        "Redeemable Global Supply Cap must bet set"
      );
      console.log(
        `🧾 Previous global supply cap was`,
        redeemableGlobalSupplyCap_pre,
        "now is",
        redeemableGlobalSupplyCap_post,
        "(circulating supply",
        redeemableCirculatingSupply,
        ")"
      );
    } else {
      expect(redeemableGlobalSupplyCap_pre.cmp(redeemableGlobalSupplyCap_post)).equals(
        0,
        "Redeemable Global Supply Cap must not have changed"
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
