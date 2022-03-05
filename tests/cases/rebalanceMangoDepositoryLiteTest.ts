import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync, PnLPolarity } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { rebalanceMangoDepositoryLite } from "../api";
import { CLUSTER, slippageBase } from "../constants";
import { getSolBalance, getBalance } from "../utils";

// Pretty unreliable test - this need a Rust test suit with fully under control environment.
export const rebalanceMangoDepositoryLiteTest = async function (rebalancingMaxAmount: number, polarity: PnLPolarity, slippage: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer): Promise<number> {
    console.group("🧭 rebalanceMangoDepositoryLiteTest");
    try {
        let quoteDelta: number;
        switch (polarity) {
            case PnLPolarity.Positive:
                quoteDelta = await rebalancePositivePnL(rebalancingMaxAmount, slippage, user, controller, depository, mango, payer);
                break;
            case PnLPolarity.Negative:
                quoteDelta = await rebalanceNegativePnL(rebalancingMaxAmount, slippage, user, controller, depository, mango, payer);
                break;
        };
        console.groupEnd();
        return quoteDelta;
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}

// Send Collateral, get QUOTE back (test accounting similar to mint, although not redeemable but quotes)
const rebalancePositivePnL = async function (rebalancingMaxAmount: number, slippage: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer): Promise<number> {
    try {
        // GIVEN
        const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
        const userQuoteATA: PublicKey = findATAAddrSync(user.publicKey, depository.quoteMint)[0];
        const userQuoteBalance = await getBalance(userQuoteATA);
        let userCollateralBalance: number = await getBalance(userCollateralATA);
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            userCollateralBalance += await getSolBalance(user.publicKey);
        }

        // WHEN
        // - Get the perp price at the same moment to have the less diff between exec and test price
        const mangoPerpPrice = await depository.getCollateralPerpPriceUI(mango);
        const txId = await rebalanceMangoDepositoryLite(user, payer ?? user, rebalancingMaxAmount, PnLPolarity.Positive, slippage, controller, depository, mango)
        console.log("🪙  perp price is", Number(mangoPerpPrice.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const rebalancingMaxAmountCollateralEquivalent = rebalancingMaxAmount / mangoPerpPrice;
        const userQuoteBalance_post = await getBalance(userQuoteATA);
        let userCollateralBalance_post = await getBalance(userCollateralATA);
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            userCollateralBalance_post += await getSolBalance(user.publicKey);
        }

        const mangoTakerFee = depository.getCollateralPerpTakerFees(mango);

        const quoteDelta = Math.abs(userQuoteBalance_post - userQuoteBalance);
        const collateralDelta = Math.abs(userCollateralBalance_post - userCollateralBalance);
        // The mango perp price in these might not be the exact same as the one in the transaction.
        const estimatedFrictionlessQuoteDelta = collateralDelta * mangoPerpPrice;
        const estimatedAmountQuoteLostInTakerFees = mangoTakerFee * quoteDelta;
        const estimatedAmountQuoteLostInSlippage = estimatedFrictionlessQuoteDelta - quoteDelta - estimatedAmountQuoteLostInTakerFees;
        // The worst price the user could get (lowest amount of QUOTE for his collateral)
        const worthExecutionPriceQuoteDelta = estimatedFrictionlessQuoteDelta * (slippage / slippageBase)

        console.log("Efficiency", Number(((quoteDelta / estimatedFrictionlessQuoteDelta) * 100).toFixed(2)), "%");
        console.log(
            `🧾 Rebalanced (sent)`, Number(quoteDelta.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol,
            "received", Number(collateralDelta.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
            "(+~ takerFees =", Number(estimatedAmountQuoteLostInTakerFees.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol,
            ", +~ slippage =", Number(estimatedAmountQuoteLostInSlippage.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol, ")",
            "(frictionless rebalancing would have been", Number(estimatedFrictionlessQuoteDelta.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol, ")",
            ")"
        );
        expect(quoteDelta).to.be.lessThanOrEqual(rebalancingMaxAmount, "used more quote that initially intended");
        expect(quoteDelta).to.be.greaterThanOrEqual(worthExecutionPriceQuoteDelta, "The amount rebalanced is out of the slippage range");
        expect(collateralDelta).to.be.lessThanOrEqual(rebalancingMaxAmountCollateralEquivalent, "User paid more collateral than inputted amount");
        return quoteDelta;
    } catch (error) {
        throw error;
    }
}

// Send QUOTE, get Collateral back. (Similar testing to redeem, although it's quote instead of redeemable)
const rebalanceNegativePnL = async function (rebalancingMaxAmount: number, slippage: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer): Promise<number> {
    try {
        // GIVEN
        const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
        const userQuoteATA: PublicKey = findATAAddrSync(user.publicKey, depository.quoteMint)[0];
        const userQuoteBalance = await getBalance(userQuoteATA);
        let userCollateralBalance = await getBalance(userCollateralATA);
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            userCollateralBalance += await getSolBalance(user.publicKey);
        }

        // This instruction does not unwrap YET (computing limit)
        // Initial SOL is used to make the diff afterward as the instruction does unwrap
        // const userStartingSolBalance = await getSolBalance(user.publicKey);

        // WHEN
        // - Get the perp price at the same moment to have the less diff between exec and test price
        // Simulates user experience from the front end
        const mangoPerpPrice = await depository.getCollateralPerpPriceUI(mango);
        const txId = await rebalanceMangoDepositoryLite(user, payer ?? user, rebalancingMaxAmount, PnLPolarity.Negative, slippage, controller, depository, mango)
        console.log("🪙  perp price is", Number(mangoPerpPrice.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const userQuoteBalance_post = await getBalance(userQuoteATA);
        let userCollateralBalance_post = await getBalance(userCollateralATA);
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            // the instruction DOESN'T unwrap the WSOL yet
            userCollateralBalance_post += await getSolBalance(user.publicKey);
        }

        const mangoTakerFee = depository.getCollateralPerpTakerFees(mango);

        const quoteDelta = Math.abs(userQuoteBalance - userQuoteBalance_post);
        const collateralDelta = Math.abs(userCollateralBalance_post - userCollateralBalance);
        const quoteProcessedByRebalancing = quoteDelta;
        // The mango perp price in these might not be the exact same as the one in the transaction.
        const estimatedFrictionlessCollateralDelta = quoteProcessedByRebalancing / mangoPerpPrice;
        const estimatedAmountQuoteLostInTakerFees = mangoTakerFee * quoteProcessedByRebalancing;
        const collateralDeltaQuoteValue = collateralDelta * mangoPerpPrice;
        const estimatedAmountQuoteLostInSlippage = quoteDelta - collateralDeltaQuoteValue - estimatedAmountQuoteLostInTakerFees;
        // The worst price the user could get (lowest amount of UXD)
        const worthExecutionPriceQuoteDelta = (estimatedFrictionlessCollateralDelta * (slippage / slippageBase)) * mangoPerpPrice;

        console.log("Efficiency", Number(((collateralDelta / estimatedFrictionlessCollateralDelta) * 100).toFixed(2)), "%");
        console.log(
            `🧾 Rebalanced (sent)`, Number(quoteDelta.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol,
            "and received back", Number(collateralDelta.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
            "(+~ takerFees =", Number(estimatedAmountQuoteLostInTakerFees.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol,
            ", +~ slippage =", Number(estimatedAmountQuoteLostInSlippage.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol, ")",
            "(frictionless rebalancing would have been", Number(estimatedFrictionlessCollateralDelta.toFixed(depository.quoteMintDecimals)), depository.collateralMintSymbol, ")",
            ")"
        );
        expect(quoteDelta).greaterThanOrEqual(worthExecutionPriceQuoteDelta, "The amount minted is out of the slippage range");
        expect(quoteDelta).lessThanOrEqual(rebalancingMaxAmount, "The amount of quote consumed is more that initially planned");
        return quoteDelta;
    } catch (error) {
        throw error;
    }
}