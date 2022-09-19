import { nativeToUi } from "@blockworks-foundation/mango-client";
import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { setMangoDepositoryQuoteMintAndRedeemSoftCap } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";

export const setMangoDepositoryQuoteMintAndRedeemSoftCapTest = async function (
  quoteMintAndRedeemSoftCap: number,
  authority: Signer,
  controller: Controller,
  depository: MangoDepository
) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group("🧭 setMangoDepositoryQuoteMintAndRedeemSoftCapTest");
  try {
    // GIVEN
    const controllerOnChainAccount = await controller.getOnchainAccount(connection, options);
    const quoteMintAndRedeemSoftCap_pre = controllerOnChainAccount.mangoDepositoriesQuoteRedeemableSoftCap;

    // WHEN
    const txId = await setMangoDepositoryQuoteMintAndRedeemSoftCap(
      authority,
      controller,
      depository,
      quoteMintAndRedeemSoftCap
    );
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const controllerOnChainAccount_post = await controller.getOnchainAccount(connection, options);
    const quoteMintAndRedeemSoftCap_post = controllerOnChainAccount_post.mangoDepositoriesQuoteRedeemableSoftCap;
    const quoteMintAndRedeemSoftCap_postUi = nativeToUi(
      quoteMintAndRedeemSoftCap_post.toNumber(),
      depository.quoteMintDecimals
    );

    console.log(`🧾 Previous soft cap was`, quoteMintAndRedeemSoftCap_pre, "now is", quoteMintAndRedeemSoftCap_post);
    expect(quoteMintAndRedeemSoftCap_postUi).equals(quoteMintAndRedeemSoftCap, "The soft cap must be set.");
    controller.info();
    console.groupEnd();
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
