import { utils } from "@project-serum/anchor";
import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync, uiToNative, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { disableDepositoryMinting, mintWithMangoDepository, quoteMintWithMangoDepository, setMangoDepositoryQuoteMintAndRedeemFee } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER, slippageBase } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const disableDepositoryMintingTest = async function (disable: boolean, authority: Signer, controller: Controller, depository: MangoDepository) {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("🧭 disableDepositoryMintingTest");
    try {
        // GIVEN
        const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);
        const mintingDisabled = depositoryOnchainAccount.disabled;
        
        // WHEN
        const txId = await disableDepositoryMinting(authority, controller, depository, disable);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
        const mintingDisabled_post = depositoryOnchainAccount_post.disabled;

        expect(mintingDisabled_post).equals(disable, "The quote fee has not changed.");
        console.log(`🧾 Previous ${depository.collateralMintSymbol} minting is`, mintingDisabled, "now is", mintingDisabled_post);
        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}