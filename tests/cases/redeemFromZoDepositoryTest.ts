import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Zo, ZoDepository, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { redeemFromZoDepository } from "../api";
import { CLUSTER, slippageBase } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const redeemFromZoDepositoryTest = async function (redeemableAmount: number, slippage: number, user: Signer, controller: Controller, depository: ZoDepository, zo: Zo, payer?: Signer): Promise<number> {
    console.group("🧭 redeemWithZoDepositoryTest");
    try {
        // GIVEN
        const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
        const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
        const userRedeemableBalance = await getBalance(userRedeemableATA);
        let userCollateralBalance: number;
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            // If WSOL, as the transaction unwraps
            userCollateralBalance = await getSolBalance(user.publicKey);
        } else {
            userCollateralBalance = await getBalance(userCollateralATA);
        }

        // WHEN
        // - Get the perp price at the same moment to have the less diff between exec and test price.
        // Simulates user experience from the front end
        const perpPrice = depository.getPerpPriceUI(zo);
        const txId = await redeemFromZoDepository(user, payer ?? user, slippage, redeemableAmount, controller, depository, zo);
        console.log("🪙  perp price is", Number(perpPrice.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const userRedeemableBalance_post = await getBalance(userRedeemableATA);
        let userCollateralBalance_post: number;
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            // the TX unwrap the WSOL. Payer takes the tx fees so we'r good.
            userCollateralBalance_post = await getSolBalance(user.publicKey);
        } else {
            userCollateralBalance_post = await getBalance(userCollateralATA);
        }

        const zoTakerFee = depository.getZoTakerFee();

        const redeemableDelta = userRedeemableBalance - userRedeemableBalance_post;
        const collateralDelta = userCollateralBalance_post - userCollateralBalance;
        const redeemableLeftOverDueToOddLot = Math.max(redeemableAmount - redeemableDelta, 0);
        const redeemableProcessedByRedeeming = redeemableAmount - redeemableLeftOverDueToOddLot;
        // The mango perp price in these might not be the exact same as the one in the transaction.
        const estimatedFrictionlessCollateralDelta = redeemableProcessedByRedeeming / perpPrice;
        const estimatedAmountRedeemableLostInTakerFees = zoTakerFee * redeemableProcessedByRedeeming;
        const redeemableNativeUnitPrecision = Math.pow(10, -controller.redeemableMintDecimals);
        const estimatedAmountRedeemableLostInSlippage = Math.abs(redeemableDelta - redeemableProcessedByRedeeming) - estimatedAmountRedeemableLostInTakerFees;
        // The worst price the user could get (lowest amount of UXD)
        const worthExecutionPriceCollateralDelta = (estimatedFrictionlessCollateralDelta * (slippage / slippageBase)) / perpPrice;

        console.log("Efficiency", Number(((collateralDelta / estimatedFrictionlessCollateralDelta) * 100).toFixed(2)), "%");
        console.log(
            `🧾 Redeemed`, Number(collateralDelta.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
            "by burning", Number(redeemableProcessedByRedeeming.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol,
            "(+~ takerFees =", Number(estimatedAmountRedeemableLostInTakerFees.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol,
            ", +~ slippage =", Number(estimatedAmountRedeemableLostInSlippage.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol, ")",
            "(frictionless redeeming would have been", Number(estimatedFrictionlessCollateralDelta.toFixed(controller.redeemableMintDecimals)), depository.collateralMintSymbol, ")",
            "|| odd lot returns ", Number(redeemableLeftOverDueToOddLot.toFixed(depository.collateralMintDecimals)), controller.redeemableMintSymbol,
            ")"
        );

        expect(redeemableAmount).closeTo(redeemableProcessedByRedeeming + redeemableLeftOverDueToOddLot, redeemableNativeUnitPrecision, "The amount of collateral left over + processed is not equal to the collateral amount inputted initially");
        expect(redeemableDelta).greaterThanOrEqual(worthExecutionPriceCollateralDelta, "The amount redeemed is out of the slippage range");
        // expect(redeemableLeftOverDueToOddLot).lessThanOrEqual(minTradingSizeQuote, "The redeemable odd lot returned is higher than the minTradingSize for that perp.");
        expect(redeemableDelta).to.be.lessThanOrEqual(redeemableAmount, "User paid more collateral than inputted amount");
        console.groupEnd();
        return redeemableDelta;
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}