import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { enableMangoDepositoryRedeemOnlyMode } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";

export const enableMangoDepositoryRedeemOnlyModeTest = async function (enable: boolean, authority: Signer, controller: Controller, depository: MangoDepository) {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("🧭 disableDepositoryRegularMintingTest");
    try {
        // GIVEN
        const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);
        const redeemOnlyModeEnabled = depositoryOnchainAccount.redeemOnlyModeEnabled;
        
        // WHEN
        const txId = await enableMangoDepositoryRedeemOnlyMode(authority, controller, depository, enable);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
        const redeemOnlyModeEnabled_post = depositoryOnchainAccount_post.redeemOnlyModeEnabled;

        expect(redeemOnlyModeEnabled_post).equals(enable, "Redeem only mode is set correctly");
        console.log(`🧾 Previous ${depository.collateralMintSymbol} redeem only mode is`, redeemOnlyModeEnabled, "now is", redeemOnlyModeEnabled_post);
        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}