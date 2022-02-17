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
            case PnLPolarity.Positive: {
                quoteDelta = await rebalancePositivePnL(rebalancingMaxAmount, slippage, user, controller, depository, mango, payer);
            }
            case PnLPolarity.Negative: {
                quoteDelta = await rebalanceNegativePnL(rebalancingMaxAmount, slippage, user, controller, depository, mango, payer);
            }
        };
        console.groupEnd();
        return quoteDelta;
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}

// Send Collateral, get QUOTE back (test accounting similar to mint, although not redeemable but quotes)
const rebalancePositivePnL = async function (rebalancingMaxAmount: number, slippage: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer): Promise<number> {
    // GIVEN
    const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
    const userQuoteATA: PublicKey = findATAAddrSync(user.publicKey, depository.quoteMint)[0];
    const userQuoteBalance = await getBalance(userQuoteATA);
    const userCollateralBalance: number = await getBalance(userCollateralATA);

    // Initial SOL is used to make the diff afterward as the instruction does unwrap
    const userStartingSolBalance = await getSolBalance(user.publicKey);

    // WHEN
    // - Get the perp price at the same moment to have the less diff between exec and test price
    const mangoPerpPrice = await depository.getCollateralPerpPriceUI(mango);
    const txId = await rebalanceMangoDepositoryLite(user, payer ?? user, rebalancingMaxAmount, PnLPolarity.Positive, slippage, controller, depository, mango)
    console.log("🪙  perp price is", Number(mangoPerpPrice.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const rebalancingMaxAmountCollateralEquivalent = rebalancingMaxAmount / mangoPerpPrice;
    const userQuoteBalance_post = await getBalance(userQuoteATA);
    let userCollateralBalance_post: number;
    if (NATIVE_MINT.equals(depository.collateralMint)) {
        // the TX unwrap the WSOL, so we need to remove the initial SOL balance. Payer takes the tx fees so we'r good.
        userCollateralBalance_post = await getSolBalance(user.publicKey) - userStartingSolBalance;
    } else {
        userCollateralBalance_post = await getBalance(userCollateralATA);
    }

    const mangoTakerFee = depository.getCollateralPerpTakerFees(mango);

    const quoteDelta = userQuoteBalance_post - userQuoteBalance;
    const collateralDelta = userCollateralBalance - userCollateralBalance_post;
    const collateralOddLotLeftOver = rebalancingMaxAmountCollateralEquivalent - collateralDelta;
    const collateralProcessedByRebalancing = rebalancingMaxAmountCollateralEquivalent - collateralOddLotLeftOver;
    // The mango perp price in these might not be the exact same as the one in the transaction.
    const estimatedFrictionlessQuoteDelta = collateralProcessedByRebalancing * mangoPerpPrice;
    const estimatedAmountQuoteLostInTakerFees = mangoTakerFee * collateralProcessedByRebalancing * mangoPerpPrice;
    const collateralNativeUnitPrecision = Math.pow(10, -depository.collateralMintDecimals);
    const estimatedAmountQuoteLostInSlippage = Math.abs(estimatedFrictionlessQuoteDelta - quoteDelta) - estimatedAmountQuoteLostInTakerFees;
    // The worst price the user could get (lowest amount of QUOTE for his collateral)
    const worthExecutionPriceQuoteDelta = estimatedFrictionlessQuoteDelta * (slippage / slippageBase)

    console.log("Efficiency", Number(((quoteDelta / estimatedFrictionlessQuoteDelta) * 100).toFixed(2)), "%");
    console.log(
        `🧾 Rebalanced (received)`, Number(quoteDelta.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol,
        "by sending", Number(collateralProcessedByRebalancing.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
        "(+~ takerFees =", Number(estimatedAmountQuoteLostInTakerFees.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol,
        ", +~ slippage =", Number(estimatedAmountQuoteLostInSlippage.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol, ")",
        "(frictionless rebalancing would have been", Number(estimatedFrictionlessQuoteDelta.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol, ")",
        "|| odd lot returns ", Number(collateralOddLotLeftOver.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
        ")"
    );
    expect(rebalancingMaxAmountCollateralEquivalent).closeTo(collateralProcessedByRebalancing + collateralOddLotLeftOver, collateralNativeUnitPrecision, "The amount of collateral left over + processed is not equal to the collateral amount inputted initially");
    expect(quoteDelta).greaterThanOrEqual(worthExecutionPriceQuoteDelta, "The amount rebalanced is out of the slippage range");
    expect(collateralDelta).lessThanOrEqual(rebalancingMaxAmountCollateralEquivalent - collateralOddLotLeftOver, "User paid more collateral than inputted amount");
    return quoteDelta;
}

// Send QUOTE, get Collateral back. (Similar testing to redeem, although it's quote instead of redeemable)
const rebalanceNegativePnL = async function (rebalancingMaxAmount: number, slippage: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer): Promise<number> {
    // GIVEN
    const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
    const userQuoteATA: PublicKey = findATAAddrSync(user.publicKey, depository.quoteMint)[0];
    const userQuoteBalance = await getBalance(userQuoteATA);
    let userCollateralBalance: number;
    // if (NATIVE_MINT.equals(depository.collateralMint)) {
    //     // If WSOL, as the transaction DOESN't unwraps due to computing issues (temporary)
    //     userCollateralBalance = await getSolBalance(user.publicKey);
    // } else {
    // As long as it doesn't unwrap use this always
        userCollateralBalance = await getBalance(userCollateralATA);
    // }

    // This instruction does not unwrap YET (computing limit)
    // Initial SOL is used to make the diff afterward as the instruction does unwrap
    // const userStartingSolBalance = await getSolBalance(user.publicKey);

    // WHEN
    // - Get the perp price at the same moment to have the less diff between exec and test price
    const mangoPerpPrice = await depository.getCollateralPerpPriceUI(mango);
    const txId = await rebalanceMangoDepositoryLite(user, payer ?? user, rebalancingMaxAmount, PnLPolarity.Negative, slippage, controller, depository, mango)
    console.log("🪙  perp price is", Number(mangoPerpPrice.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const userQuoteBalance_post = await getBalance(userQuoteATA);
    let userCollateralBalance_post: number;
    // if (NATIVE_MINT.equals(depository.collateralMint)) {
    //     // the TX <DOESN'T> unwrap the WSOL. Payer takes the tx fees so we'r good.
    //     userCollateralBalance_post = await getSolBalance(user.publicKey);
    // } else {
        userCollateralBalance_post = await getBalance(userCollateralATA);
    // }

    const mangoTakerFee = depository.getCollateralPerpTakerFees(mango);

    const quoteDelta = userQuoteBalance - userQuoteBalance_post;
    const maxQuoteDelta = rebalancingMaxAmount / mangoPerpPrice;
    const collateralDelta = userCollateralBalance_post - userCollateralBalance;
    const quoteLeftOverDueToOddLot = rebalancingMaxAmount - quoteDelta;
    const quoteProcessedByRebalancing = rebalancingMaxAmount - quoteLeftOverDueToOddLot;
    // The mango perp price in these might not be the exact same as the one in the transaction.
    const estimatedFrictionlessCollateralDelta = quoteProcessedByRebalancing / mangoPerpPrice;
    const estimatedAmountQuoteLostInTakerFees = mangoTakerFee * quoteProcessedByRebalancing;
    const quoteNativeUnitPrecision = Math.pow(10, -depository.quoteMintDecimals);
    const estimatedAmountQuoteLostInSlippage = Math.abs(quoteDelta - quoteProcessedByRebalancing) - estimatedAmountQuoteLostInTakerFees;
    // The worst price the user could get (lowest amount of UXD)
    const worthExecutionPriceCollateralDelta = (estimatedFrictionlessCollateralDelta * (slippage / slippageBase)) / mangoPerpPrice;

    console.log("Efficiency", Number(((collateralDelta / estimatedFrictionlessCollateralDelta) * 100).toFixed(2)), "%");
    console.log(
        `🧾 Rebalanced (sent)`, Number(quoteProcessedByRebalancing.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol,
        "and received back", Number(collateralDelta.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
        "(+~ takerFees =", Number(estimatedAmountQuoteLostInTakerFees.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol,
        ", +~ slippage =", Number(estimatedAmountQuoteLostInSlippage.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol, ")",
        "(frictionless rebalancing would have been", Number(estimatedFrictionlessCollateralDelta.toFixed(depository.quoteMintDecimals)), depository.collateralMintSymbol, ")",
        "|| odd lot returns ", Number(quoteLeftOverDueToOddLot.toFixed(depository.collateralMintDecimals)), depository.quoteMintSymbol,
        ")"
    );

    expect(rebalancingMaxAmount).closeTo(quoteProcessedByRebalancing + quoteLeftOverDueToOddLot, quoteNativeUnitPrecision, "The amount of collateral left over + processed is not equal to the collateral amount inputted initially");
    expect(quoteDelta).greaterThanOrEqual(worthExecutionPriceCollateralDelta, "The amount minted is out of the slippage range");
    expect(collateralDelta).lessThanOrEqual(maxQuoteDelta, "The amount of sol consumed is more that initially planned");
    return quoteDelta;
}