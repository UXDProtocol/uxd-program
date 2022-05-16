import { utils } from "@project-serum/anchor";
import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync, uiToNative } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { mintWithMangoDepository, quoteMintWithMangoDepository, quoteRedeemFromMangoDepository } from "../api";
import { getConnection, TXN_COMMIT, TXN_OPTS } from "../connection";
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

        const bps_pow = Math.pow(10, 4);
        const lessFeesMultiple = 1 - ((await depository.getOnchainAccount(getConnection(), TXN_OPTS)).quoteMintAndRedeemFee / bps_pow);
        const quoteNativeUnitPrecision = Math.pow(10, -depository.quoteMintDecimals);
        const redeemableNativeUnitPrecision = Math.pow(10, -controller.redeemableMintDecimals);

        expect(userQuoteBalance_post).closeTo(userQuoteBalance + (redeemableAmount * lessFeesMultiple), quoteNativeUnitPrecision);
        expect(userRedeemableBalance_post).closeTo(userRedeemableBalance - redeemableAmount, redeemableNativeUnitPrecision);

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}
