import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync, PnLPolarity } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { rebalanceMangoDepositoryLite } from "../api";
import { CLUSTER, slippageBase } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const rebalanceMangoDepositoryLiteTest = async (rebalancingMaxAmount: number, polarity: PnLPolarity, slippage: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer): Promise<number> => {
    console.group("🧭 rebalanceMangoDepositoryLiteTest");
    try {
        // GIVEN
        const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
        const userQuoteATA: PublicKey = findATAAddrSync(user.publicKey, depository.quoteMint)[0];
        const userQuoteBalance = await getBalance(userQuoteATA);
        let userCollateralBalance: number = await getBalance(userCollateralATA);
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            // use SOL + WSOL balance
            userCollateralBalance += await getSolBalance(user.publicKey);
        }

        // WHEN
        // - Get the perp price at the same moment to have the less diff between exec and test price
        const mangoPerpPrice = await depository.getCollateralPerpPriceUI(mango);
        const txId = await rebalanceMangoDepositoryLite(user, payer ?? user, rebalancingMaxAmount, polarity, slippage, controller, depository, mango)
        console.log("🪙  perp price is", Number(mangoPerpPrice.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const userQuoteBalance_post = await getBalance(userQuoteATA);
        const quoteDelta = userQuoteBalance_post - userQuoteBalance;
        const realRebalancingAmount = Math.abs(quoteDelta);
        expect(realRebalancingAmount).to.be.lessThanOrEqual(rebalancingMaxAmount)

        if (polarity == PnLPolarity.Positive) {
            // Send COLLATERAL, should retrieve equivalent amount of USDC minus the slippage
            let userCollateralBalance_post = await getBalance(userCollateralATA);
            if (NATIVE_MINT.equals(depository.collateralMint)) {
                // use SOL + WSOL balance
                userCollateralBalance_post += await getSolBalance(user.publicKey);
            }


            // The user should
            const expectedCollateralDelta = rebalancingMaxAmount / mangoPerpPrice;
            const collateralDelta = userCollateralBalance - userCollateralBalance_post;
            expect(collateralDelta).closeTo(expectedCollateralDelta, (expectedCollateralDelta * (slippage / slippageBase)), "The amount received back is invalid");
            console.log(
                `🧾 Rebalanced`, Number(realRebalancingAmount.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol);
        } else if (polarity == PnLPolarity.Negative) {
            // // Send USED, should retrieve equivalent amount of COLLATERAL minus the slippage
            // let userCollateralBalance_post = await getBalance(userCollateralATA);
            // if (NATIVE_MINT.equals(depository.collateralMint)) {
            //     // use SOL + WSOL balance
            //     userCollateralBalance_post += await getSolBalance(user.publicKey);
            // }


            // // The user should
            // const expectedCollateralDelta = rebalancingMaxAmount / mangoPerpPrice;
            // const collateralDelta = userCollateralBalance - userCollateralBalance_post;
            // expect(collateralDelta).closeTo(expectedCollateralDelta, (expectedCollateralDelta * (slippage / slippageBase)), "The amount received back is invalid");
            // console.log(
            //     `🧾 Rebalanced`, Number(realRebalancingAmount.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol);
        }


        // const mangoTakerFee = uxdHelpers.getMangoTakerFeeForPerp(depository, mango);
        // const maxTakerFee = mangoTakerFee.toNumber() * rebalancingMaxAmount;
        const collateralNativeUnitPrecision = Math.pow(10, -depository.collateralMintDecimals);

        console.log(
            `🧾 Rebalanced`, Number(quoteDelta.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol,
            // "for", Number(collateralDelta.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
            // "(perfect was", Number(maxRedeemableDelta.toFixed(controller.redeemableMintDecimals)),
            // "|| returned unprocessed collateral due to odd lot", Number(collateralLeftOver.toFixed(depository.collateralMintDecimals)),
            // "|| ~ max taker fees were", Number(maxTakerFee.toFixed(controller.redeemableMintDecimals)),
            // "|| ~ loss in slippage", Number((maxRedeemableDelta - (quoteDelta + maxTakerFee)).toFixed(controller.redeemableMintDecimals)),
            // ")"
        );
        // expect(collateralLeftOver + collateralDelta).closeTo(collateralAmount, collateralNativeUnitPrecision, "The amount of collateral used for redeem + carried over should be equal to the inputted amount.")
        // expect(quoteDelta).closeTo(maxRedeemableDelta, (maxRedeemableDelta * (slippage / slippageBase)) + maxTakerFee, "The amount minted is out of the slippage range");
        // expect(collateralDelta).closeTo(collateralAmount - collateralLeftOver, collateralNativeUnitPrecision, "The collateral amount paid doesn't match the user wallet delta");

        console.groupEnd();
        return realRebalancingAmount;
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}