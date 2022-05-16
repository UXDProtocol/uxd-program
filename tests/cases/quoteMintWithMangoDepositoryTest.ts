import { utils } from "@project-serum/anchor";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, UXD_DECIMALS } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { quoteMintWithMangoDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
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
        let userQuoteBalance = await getBalance(userQuoteATA);
        let userRedeemableBalance = await getBalance(userRedeemableATA);
        if (isNaN(userQuoteBalance)) { userQuoteBalance = 0; }
        if (isNaN(userRedeemableBalance)) { userRedeemableBalance = 0; }

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

        const bps_pow = Math.pow(10, 4);
        const lessFeesMultiple = 1 - ((await depository.getOnchainAccount(getConnection(), TXN_OPTS)).quoteMintAndRedeemFee / bps_pow);
        const quoteNativeUnitPrecision = Math.pow(10, -depository.quoteMintDecimals);
        const redeemableNativeUnitPrecision = Math.pow(10, -controller.redeemableMintDecimals);

        expect(userQuoteBalance_post).closeTo(userQuoteBalance - quoteAmount, quoteNativeUnitPrecision);
        expect(userRedeemableBalance_post).closeTo(userRedeemableBalance + (quoteAmount * lessFeesMultiple), redeemableNativeUnitPrecision);

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}
