import { Signer } from "@solana/web3.js";
import { Controller, nativeToUi } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { setMangoDepositoriesRedeemableSoftCap } from "../api";
import { CLUSTER } from "../constants";
import { getConnection, TXN_OPTS } from "../connection";

export const setRedeemableSoftCapMangoDepositoryTest = async (softCapAmount: number, authority: Signer, controller: Controller) => {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("🧭 setRedeemableSoftCapMangoDepositoryTest");
    try {
        // GIVEN
        const controllerOnchainAccount = await controller.getOnchainAccount(connection, options);
        const mangoDepositoryRedeemableSoftCap = nativeToUi(controllerOnchainAccount.mangoDepositoriesRedeemableSoftCap.toNumber(), controller.redeemableMintDecimals);

        // WHEN
        const txId = await setMangoDepositoriesRedeemableSoftCap(authority, controller, softCapAmount);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const controllerOnchainAccount_post = await controller.getOnchainAccount(connection, options);
        const mangoDepositoryRedeemableSoftCap_post = nativeToUi(controllerOnchainAccount_post.mangoDepositoriesRedeemableSoftCap.toNumber(), controller.redeemableMintDecimals);
        const redeemableCirculatingSupply = nativeToUi(controllerOnchainAccount_post.redeemableCirculatingSupply.toNumber(), controller.redeemableMintDecimals);

        expect(mangoDepositoryRedeemableSoftCap_post).equals(softCapAmount, "The redeemable mango depository soft cap hasn't been updated.");
        console.log(`🧾 Previous mango depositories soft cap was`, mangoDepositoryRedeemableSoftCap, "now is", mangoDepositoryRedeemableSoftCap_post, "(circulating supply", redeemableCirculatingSupply, ")");
        controller.info();
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}