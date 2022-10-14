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
    mangoDepositoriesQuoteRedeemableSoftCap?: {
      value: number;
      depository: MangoDepository;
    };
    mangoDepositoriesRedeemableSoftCap?: number;
    redeemableGlobalSupplyCap?: number;
  }
) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group("🧭 editControllerTest");
  try {
    // GIVEN
    const controllerOnChainAccount = await controller.getOnchainAccount(connection, options);

    const mangoDepositoriesQuoteRedeemableSoftCap_pre = controllerOnChainAccount.mangoDepositoriesQuoteRedeemableSoftCap;
    const mangoDepositoriesRedeemableSoftCap_pre = controllerOnChainAccount.mangoDepositoriesRedeemableSoftCap;
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

    const mangoDepositoriesQuoteRedeemableSoftCap_post = controllerOnChainAccount_post.mangoDepositoriesQuoteRedeemableSoftCap;
    const mangoDepositoriesRedeemableSoftCap_post = controllerOnChainAccount_post.mangoDepositoriesRedeemableSoftCap;
    const redeemableGlobalSupplyCap_post = controllerOnChainAccount_post.redeemableGlobalSupplyCap;

    if (uiFields.mangoDepositoriesQuoteRedeemableSoftCap) {
      const mangoDepositoriesQuoteRedeemableSoftCap_postUi = nativeToUi(
        mangoDepositoriesQuoteRedeemableSoftCap_post.toNumber(),
        uiFields.mangoDepositoriesQuoteRedeemableSoftCap.depository.collateralMintDecimals
      );
      expect(mangoDepositoriesQuoteRedeemableSoftCap_postUi).equals(
        uiFields.mangoDepositoriesQuoteRedeemableSoftCap.value,
        "Mango Depositories Quote Mint And Redeem SoftCap must be set"
      );
      console.log(
        `🧾 Previous mango depositories quote mint and redeem soft cap was`,
        mangoDepositoriesQuoteRedeemableSoftCap_pre,
        "now is",
        mangoDepositoriesQuoteRedeemableSoftCap_post
      );
    } else {
      expect(mangoDepositoriesQuoteRedeemableSoftCap_pre.cmp(mangoDepositoriesQuoteRedeemableSoftCap_post)).equals(
        0,
        "Mango Depositories Quote Mint And Redeem SoftCap must not have changed"
      );
    }

    if (uiFields.mangoDepositoriesRedeemableSoftCap !== undefined) {
      const mangoDepositoriesRedeemableSoftCap_postUi = nativeToUi(mangoDepositoriesRedeemableSoftCap_post.toNumber(), controller.redeemableMintDecimals);
      expect(mangoDepositoriesRedeemableSoftCap_postUi).equals(uiFields.mangoDepositoriesRedeemableSoftCap, "Mango Depositories Redeemable SoftCap must bet set");
      console.log(
        `🧾 Previous mango depositories soft cap was`,
        mangoDepositoriesRedeemableSoftCap_pre,
        "now is",
        mangoDepositoriesRedeemableSoftCap_post,
        "(circulating supply",
        redeemableCirculatingSupply,
        ")"
      );
    } else {
      expect(mangoDepositoriesRedeemableSoftCap_pre.cmp(mangoDepositoriesRedeemableSoftCap_post)).equals(0, "Mango Depositories Redeemable SoftCap must not have changed");
    }

    if (uiFields.redeemableGlobalSupplyCap !== undefined) {
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
