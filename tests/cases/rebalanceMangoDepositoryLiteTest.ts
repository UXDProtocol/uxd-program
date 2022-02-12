import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync, PnLPolarity } from "@uxdprotocol/uxd-client";
import { sleep } from "@uxdprotocol/uxd-client/node_modules/@blockworks-foundation/mango-client";
import { expect } from "chai";
import { rebalanceMangoDepositoryLite } from "../api";
import { TXN_OPTS } from "../connection";
import { CLUSTER, slippageBase } from "../constants";
import { getSolBalance, getBalance } from "../utils";

// Pretty unreliable test - this need a Rust test suit with fully under control environment.
export const rebalanceMangoDepositoryLiteTest = async (rebalancingMaxAmount: number, polarity: PnLPolarity, slippage: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer): Promise<number> => {
    console.group("🧭 rebalanceMangoDepositoryLiteTest");
    try {
        // GIVEN
        const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
        const userQuoteATA: PublicKey = findATAAddrSync(user.publicKey, depository.quoteMint)[0];
        const userQuoteBalance_pre = await getBalance(userQuoteATA);
        let userCollateralBalance_pre: number = await getBalance(userCollateralATA);
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            // use SOL + WSOL balance
            userCollateralBalance_pre += await getSolBalance(user.publicKey);
        }

        // WHEN
        // - Get the perp price at the same moment to have the less diff between exec and test price
        const mangoPerpPrice = await depository.getCollateralPerpPriceUI(mango);
        const txId = await rebalanceMangoDepositoryLite(user, payer ?? user, rebalancingMaxAmount, polarity, slippage, controller, depository, mango)
        console.log("🪙  perp price is", Number(mangoPerpPrice.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const userQuoteBalance_post = await getBalance(userQuoteATA);
        const quoteDelta = Math.abs(userQuoteBalance_post - userQuoteBalance_pre); // Rebalanced Amount

        let userCollateralBalance_post = await getBalance(userCollateralATA);
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            // use SOL + WSOL balance
            userCollateralBalance_post += await getSolBalance(user.publicKey);
        }
        const collateralDelta = Math.abs(userCollateralBalance_pre - userCollateralBalance_post);
        const collateralDeltaQuoteValue = collateralDelta * mangoPerpPrice;

        const mangoTakerFee = depository.getCollateralPerpTakerFees(mango);
        const estimatedTakerFees = mangoTakerFee * quoteDelta;

        const quoteDirection = (polarity == PnLPolarity.Positive ? "providing" : "receiving back");
        const baseDirection = (polarity == PnLPolarity.Negative ? "providing" : "receiving back");

        console.log(
            `🧾 Rebalanced`, Number(quoteDelta.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol,
            quoteDirection, Number(collateralDelta.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
            baseDirection, Number(quoteDelta.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol,
            "|| ~ estimated taker fees around ", Number(estimatedTakerFees.toFixed(depository.quoteMintDecimals))
        );

        const slippageDeltaMultiplier = slippage / slippageBase;
        expect(quoteDelta).closeTo(collateralDeltaQuoteValue, collateralDeltaQuoteValue * slippageDeltaMultiplier, "User should have gotten back the same value +/- slippage")
        switch (polarity) {
            case PnLPolarity.Positive: {
                expect(userQuoteBalance_post).greaterThan(userQuoteBalance_pre, "PnL is Positive, user should have gotten some quote back");
                expect(userCollateralBalance_post).lessThan(userCollateralBalance_pre, "PnL is Negative, user should have spent collateral");
            }
            case PnLPolarity.Negative: {
                expect(userQuoteBalance_post).lessThan(userQuoteBalance_pre, "PnL is Negative, user should have spent quote");
                expect(userCollateralBalance_post).greaterThan(userCollateralBalance_pre, "PnL is Negative, user should have gotten some collateral back");
            }
        };

        console.groupEnd();
        return quoteDelta;
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}