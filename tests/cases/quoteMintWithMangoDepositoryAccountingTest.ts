import { utils } from "@project-serum/anchor";
import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync, nativeToUi } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { mintWithMangoDepository, quoteMintWithMangoDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER, slippageBase } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const quoteMintWithMangoDepositoryAccountingTest = async function (quoteAmount: number, fee_amount: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer) {
    console.group("🧭 mintWithMangoDepositoryAccountingTest");
    try {
        const connection = getConnection();
        const options = TXN_OPTS;

        // GIVEN
        const userQuoteATA: PublicKey = await utils.token.associatedAddress({
            mint: depository.quoteMint,
            owner: user.publicKey,
        });
        const userRedeemableATA: PublicKey = await utils.token.associatedAddress({
            mint: controller.redeemableMintPda,
            owner: user.publicKey,
        });
        const userQuoteBalance = await getBalance(userQuoteATA);
        const userRedeemableBalance = await getBalance(userRedeemableATA);

        const depositoryAccount = await depository.getOnchainAccount(connection, options);
        const depositoryRedeemable = nativeToUi(depositoryAccount.redeemableAmountUnderManagement.toNumber(), depository.quoteMintDecimals);
        const controllerAccount = await controller.getOnchainAccount(connection, options);
        const controllerRedeemable = nativeToUi(controllerAccount.redeemableCirculatingSupply.toNumber(), controller.redeemableMintDecimals);
        
        // WHEN
        const txId = await quoteMintWithMangoDepository(user, payer ?? user, quoteAmount, controller, depository, mango);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);
        
        // THEN

    }

}