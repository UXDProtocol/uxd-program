import { utils } from "@project-serum/anchor";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { quoteMintWithMangoDepository } from "../api";
import { CLUSTER } from "../constants";
import { getBalance } from "../utils";

export const quoteMintWithMangoDepositoryTest = async function (quoteAmount: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer) {
    console.group("🧭 quoteMintWithMangoDepositoryTest");
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
        // CHECK IF EXIST, ELSE 0 (TODO)
        const userQuoteBalance = await getBalance(userQuoteATA);
        const userRedeemableBalance = await getBalance(userRedeemableATA);

        // WHEN
        const txId = await quoteMintWithMangoDepository(user, payer ?? user, quoteAmount, controller, depository, mango);
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

        expect(userQuoteBalance_post).equals(userQuoteBalance - quoteAmount);
        expect(userRedeemableBalance_post).equals(userRedeemableBalance + (quoteAmount * lessFeesMultiple));

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}
