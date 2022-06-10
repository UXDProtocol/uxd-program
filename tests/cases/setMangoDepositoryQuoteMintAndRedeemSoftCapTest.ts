import { nativeToUi } from "@blockworks-foundation/mango-client";
import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { setMangoDepositoryQuoteMintAndRedeemSoftCap } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";

export const setMangoDepositoryQuoteMintAndRedeemSoftCapTest = async function (softCap: number, authority: Signer, controller: Controller, depository: MangoDepository) {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("🧭 setMangoDepositoryQuoteMintAndRedeemSoftCapTest");
    try {
        // GIVEN
        const controllerOnChainAccount = await controller.getOnchainAccount(connection, options);
        const softCap_pre = controllerOnChainAccount.mangoDepositoriesQuoteRedeemableSoftCap;

        // WHEN
        const txId = await setMangoDepositoryQuoteMintAndRedeemSoftCap(authority, controller, depository, softCap);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const controllerOnChainAccount_post = await controller.getOnchainAccount(connection, options);
        const softCap_post = controllerOnChainAccount_post.mangoDepositoriesQuoteRedeemableSoftCap;
        const softCap_postUi = nativeToUi(softCap_post.toNumber(), depository.quoteMintDecimals);

        expect(softCap_postUi).equals(softCap, "The soft cap has not changed.");
        console.log(`🧾 Previous soft cap was`, softCap_pre, "now is", softCap_post);
        controller.info();
        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}
