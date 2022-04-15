import { utils } from "@project-serum/anchor";
import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync, uiToNative } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { mintWithMangoDepository, quoteMintWithMangoDepository, quoteRedeemFromMangoDepository } from "../api";
import { CLUSTER, slippageBase } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const quoteRedeemFromMangoDepositoryTest = async function (quoteAmount: number, fee_amount: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer) {
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
        const userQuoteBalance = await getBalance(userQuoteATA);
        const userRedeemableBalance = await getBalance(userRedeemableATA);

        // WHEN
        const txId = await quoteRedeemFromMangoDepository(user, payer ?? user, quoteAmount, controller, depository, mango);
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

        expect(userQuoteBalance_post).equals(userQuoteBalance + (quoteAmount * lessFeesMultiple));
        expect(userRedeemableBalance_post).equals(userRedeemableBalance - quoteAmount);

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}
