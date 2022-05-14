import { utils } from "@project-serum/anchor";
import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync, uiToNative } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { mintWithMangoDepository, quoteMintWithMangoDepository, quoteRedeemFromMangoDepository } from "../api";
import { CLUSTER, slippageBase } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const quoteRedeemFromMangoDepositoryTest = async function (redeemableAmount: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer) {
    console.group("🧭 quoteRedeemFromMangoDepositoryTest");
    try {
        // GIVEN
        const userQuoteATA: PublicKey = await utils.token.associatedAddress({
            mint: depository.quoteMint,
            owner: user.publicKey,
        });
        const userRedeemableATA: PublicKey = await utils.token.associatedAddress({
            mint: controller.redeemableMintPda,
            owner: user.publicKey,
        });
        let userQuoteBalance = await getBalance(userQuoteATA);
        let userRedeemableBalance = await getBalance(userRedeemableATA);
        if (isNaN(userQuoteBalance)) { userQuoteBalance = 0; }
        if (isNaN(userRedeemableBalance)) { userRedeemableBalance = 0; }

        // WHEN
        const txId = await quoteRedeemFromMangoDepository(user, payer ?? user, redeemableAmount, controller, depository, mango);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const userQuoteATA_post: PublicKey = await utils.token.associatedAddress({
            mint: depository.quoteMint,
            owner: user.publicKey,
        });
        const userRedeemableATA_post: PublicKey = await utils.token.associatedAddress({
            mint: controller.redeemableMintPda,
            owner: user.publicKey,
        });
        const userQuoteBalance_post = await getBalance(userQuoteATA_post);
        const userRedeemableBalance_post = await getBalance(userRedeemableATA_post);

        // const quoteAmountNative = uiToNative(quoteAmount, depository.quoteMintDecimals);
        const lessFeesMultiple = 1 - depository.quoteMintAndRedeemFee;

        console.log(userQuoteBalance);

        expect(userQuoteBalance_post).equals(userQuoteBalance + (redeemableAmount * lessFeesMultiple));
        expect(userRedeemableBalance_post).equals(userRedeemableBalance - redeemableAmount);

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}
