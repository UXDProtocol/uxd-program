import { utils } from "@project-serum/anchor";
import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync, uiToNative, nativeToUi } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { mintWithMangoDepository, quoteMintWithMangoDepository, quoteRedeemFromMangoDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER, slippageBase } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const quoteRedeemFromMangoDepositoryAccountingTest = async function(redeemableAmount, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer) {
    console.group("🧭 quoteRedeemFromMangoDepositoryAccountingTest");
    try {
        const connection = getConnection();
        const options = TXN_OPTS;

        // GIVEN
        const depositoryAccount = await depository.getOnchainAccount(connection, options);
        const depositoryRedeemable = nativeToUi(depositoryAccount.redeemableAmountUnderManagement.toNumber(), depository.quoteMintDecimals);
        const controllerAccount = await controller.getOnchainAccount(connection, options);
        const controllerRedeemable = nativeToUi(controllerAccount.redeemableCirculatingSupply.toNumber(), controller.redeemableMintDecimals);
    
        // WHEN
        const txId = await quoteRedeemFromMangoDepository(user, payer ?? user, redeemableAmount, controller, depository, mango);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const depositoryAccount_post = await depository.getOnchainAccount(connection, options);
        const depositoryRedeemable_post = nativeToUi(depositoryAccount_post.redeemableAmountUnderManagement.toNumber(), depository.quoteMintDecimals);
        const controllerAccount_post = await controller.getOnchainAccount(connection, options);
        const controllerRedeemable_post = nativeToUi(controllerAccount_post.redeemableCirculatingSupply.toNumber(), controller.redeemableMintDecimals);
    
        // Accounting tests
        // Check depository and controller balances
        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}
