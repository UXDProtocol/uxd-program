import { Signer } from "@solana/web3.js";
import { Controller, nativeToUi } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { setRedeemableGlobalSupplyCap } from "../api";
import { CLUSTER } from "../constants";
import { getConnection, TXN_OPTS } from "../provider";

export const setRedeemableGlobalSupplyCapTest = async (supplyCapAmount: number, authority: Signer, controller: Controller) => {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("🧭 setRedeemableGlobalSupplyCapTest");
    try {
        // GIVEN
        const controllerOnchainAccount = await controller.getOnchainAccount(getConnection(), options);
        const redeemableGlobalSupplyCap = nativeToUi(controllerOnchainAccount.redeemableGlobalSupplyCap.toNumber(), controller.redeemableMintDecimals);

        // WHEN
        const txId = await setRedeemableGlobalSupplyCap(authority, controller, supplyCapAmount);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const controllerOnchainAccount_post = await controller.getOnchainAccount(connection, options);
        const redeemableGlobalSupplyCap_post = nativeToUi(controllerOnchainAccount_post.redeemableGlobalSupplyCap.toNumber(), controller.redeemableMintDecimals);
        const redeemableCirculatingSupply = nativeToUi(controllerOnchainAccount_post.redeemableCirculatingSupply.toNumber(), controller.redeemableMintDecimals);

        expect(redeemableGlobalSupplyCap_post).equals(supplyCapAmount, "The redeemable global supply cap hasn't been updated.");
        console.log(`🧾 Previous global supply cap was`, redeemableGlobalSupplyCap, "now is", redeemableGlobalSupplyCap_post, "(circulating supply", redeemableCirculatingSupply, ")");
        controller.info();
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}