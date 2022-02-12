import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { redeemFromMangoDepository } from "../api";
import { CLUSTER } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const redeemFromMangoDepositoryTest = async function (redeemableAmount: number, slippage: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer): Promise<number> {
    console.group("🧭 redeemWithMangoDepositoryTest");
    try {
        // GIVEN
        const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
        const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
        const userRedeemableBalance = await getBalance(userRedeemableATA);
        let userCollateralBalance: number = await getBalance(userCollateralATA);
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            // use SOL + WSOL balance
            userCollateralBalance += await getSolBalance(user.publicKey);
        }

        // WHEN
        // - Get the perp price at the same moment to have the less diff between exec and test price
        const mangoPerpPrice = await depository.getCollateralPerpPriceUI(mango);
        const txId = await redeemFromMangoDepository(user, payer ?? user, slippage, redeemableAmount, controller, depository, mango);
        console.log("🪙  perp price is", Number(mangoPerpPrice.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const redeemableMintNativePrecision = Math.pow(10, -controller.redeemableMintDecimals);

        const maxCollateralDelta = redeemableAmount / mangoPerpPrice;

        const userRedeemableBalance_post = await getBalance(userRedeemableATA);
        let userCollateralBalance_post = await getBalance(userCollateralATA);
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            // use SOL + WSOL balance
            userCollateralBalance_post += await getSolBalance(user.publicKey);
        }

        const redeemableDelta = userRedeemableBalance - userRedeemableBalance_post;
        // There will be issues due to the TX fee + account creation fee, in some case that will fail the slippage test
        // So for now, until we implement a separate payer/user for mint and redeem, don't use tiny amounts for test where the 0.00203928 
        // could create a fail positive for wrong slippage
        const collateralDelta = userCollateralBalance_post - userCollateralBalance;
        const mangoTakerFee = depository.getCollateralPerpTakerFees(mango);
        const maxTakerFee = mangoTakerFee * redeemableAmount;
        // The amount of UXD that couldn't be redeemed due to odd lot size
        const unprocessedRedeemable = redeemableAmount - redeemableDelta;

        console.log(
            `🧾 Redeemed`, Number(redeemableDelta.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol,
            "for", Number(collateralDelta.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
            "(perfect was", Number(redeemableAmount.toFixed(controller.redeemableMintDecimals)),
            "|| ~ returned unprocessed Redeemable due to odd lot ", Number(unprocessedRedeemable.toFixed(controller.redeemableMintDecimals)),
            "|| ~ max taker fees were", Number(maxTakerFee.toFixed(controller.redeemableMintDecimals)),
            "|| ~ loss in slippage", Number((maxCollateralDelta - (collateralDelta)).toFixed(depository.collateralMintDecimals)),
            ")"
        );

        expect(redeemableDelta + unprocessedRedeemable).closeTo(redeemableAmount, redeemableMintNativePrecision, "Some Redeemable tokens are missing the count.");
        expect(redeemableDelta).closeTo(redeemableAmount - unprocessedRedeemable, maxTakerFee, "The Redeemable delta is out of odd lot range");
        expect(collateralDelta).closeTo(maxCollateralDelta, maxCollateralDelta * (slippage) + maxTakerFee, "The Collateral delta is out of the slippage range");
        expect(userRedeemableBalance_post).closeTo(userRedeemableBalance - redeemableAmount + unprocessedRedeemable, redeemableMintNativePrecision, "The amount of UnprocessedRedeemable carried over is wrong");
        console.groupEnd();
        return redeemableDelta;
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}