import { utils } from "@project-serum/anchor";
import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync, uiToNative, nativeToUi } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { mintWithMangoDepository, quoteMintWithMangoDepository, setMangoDepositoryQuoteMintAndRedeemFee } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER, slippageBase } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const setMangoDepositoryQuoteMintAndRedeemFeeTest = async function (quoteFee: number, authority: Signer, controller: Controller, depository: MangoDepository) {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("🧭 setMangoDepositoryQuoteMintAndRedeemFeeTest");
    try {
        // GIVEN
        const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);
        const quoteMintAndRedeemFee = nativeToUi(depositoryOnchainAccount.quoteMintAndRedeemFee.toNumber());
        
        // WHEN
        const txId = await setMangoDepositoryQuoteMintAndRedeemFee(authority, controller, depository, quoteFee);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
        const quoteMintAndRedeemFee_post = nativeToUi(depositoryOnchainAccount_post.quoteMintAndRedeemFee.toNumber());

        expect(quoteMintAndRedeemFee_post).equals(quoteFee, "The quote fee has not changed.");
        console.log(`🧾 Previous quote fee was`, quoteMintAndRedeemFee, "now is", quoteMintAndRedeemFee_post);
        controller.info();
        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}