import { utils } from "@project-serum/anchor";
import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync, uiToNative, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { disableDepositoryRegularMinting, mintWithMangoDepository, quoteMintWithMangoDepository, setMangoDepositoryQuoteMintAndRedeemFee } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER, slippageBase } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const disableDepositoryRegularMintingTest = async function (disableMinting: boolean, authority: Signer, controller: Controller, depository: MangoDepository) {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("🧭 disableDepositoryRegularMintingTest");
    try {
        // GIVEN
        const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);
        const regularMintingDisabled = depositoryOnchainAccount.regularMintingDisabled;
        
        // WHEN
        const txId = await disableDepositoryRegularMinting(authority, controller, depository, disableMinting);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
        const regularMintingDisabled_post = depositoryOnchainAccount_post.regularMintingDisabled;

        expect(regularMintingDisabled_post).equals(disableMinting, "Regular minting disabled status is updated");
        console.log(`🧾 Previous ${depository.collateralMintSymbol} minting is`, regularMintingDisabled, "now is", regularMintingDisabled_post);
        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}