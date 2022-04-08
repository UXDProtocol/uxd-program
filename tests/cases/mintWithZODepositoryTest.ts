import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Zo, ZoDepository, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { mintWithZoDepository } from "../api";
import { CLUSTER, slippageBase } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const mintWithZoDepositoryTest = async function (collateralAmount: number, slippage: number, user: Signer, controller: Controller, depository: ZoDepository, zo: Zo, payer?: Signer): Promise<number> {
    console.group("🧭 mintWithZoDepositoryTest");
    try {
        // GIVEN
        const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
        const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
        const userRedeemableBalance = await getBalance(userRedeemableATA);
        const userCollateralBalance: number = await getBalance(userCollateralATA);

        // Initial SOL is used to make the diff afterward as the instruction does unwrap
        const userStartingSolBalance = await getSolBalance(user.publicKey);

        // WHEN
        // - Get the perp price at the same moment to have the less diff between exec and test price
        // Simulates user experience from the front end
        const perpPrice = depository.getPerpPriceUI(zo);
        const txId = await mintWithZoDepository(user, payer ?? user, slippage, collateralAmount, controller, depository, zo);
        console.log("🪙  perp price is", Number(perpPrice.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const userRedeemableBalance_post = await getBalance(userRedeemableATA);
        let userCollateralBalance_post: number;
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            // the TX unwrap the WSOL, so we need to remove the initial SOL balance. Payer takes the tx fees so we'r good.
            userCollateralBalance_post = await getSolBalance(user.publicKey) - userStartingSolBalance;
        } else {
            userCollateralBalance_post = await getBalance(userCollateralATA);
        }


        const zoTakerFee = depository.getZoTakerFee();

        const redeemableDelta = userRedeemableBalance_post - userRedeemableBalance;
        const collateralDelta = Number((userCollateralBalance - userCollateralBalance_post).toFixed(depository.collateralMintDecimals));
        const collateralOddLotLeftOver = Number(Math.max(collateralAmount - collateralDelta, 0).toFixed(depository.collateralMintDecimals));
        const collateralProcessedByMinting = collateralAmount - collateralOddLotLeftOver;
        // The zo perp price in these might not be the exact same as the one in the transaction.
        const estimatedFrictionlessRedeemableDelta = collateralProcessedByMinting * perpPrice;
        const collateralNativeUnitPrecision = Math.pow(10, -depository.collateralMintDecimals);
        const estimatedAmountRedeemableLostInTakerFees = zoTakerFee * collateralProcessedByMinting * perpPrice;
        const estimatedAmountRedeemableLostInSlippage = Math.abs(estimatedFrictionlessRedeemableDelta - redeemableDelta);
        // The worst price the user could get (lowest amount of UXD)
        const worthExecutionPriceRedeemableDelta = estimatedFrictionlessRedeemableDelta * (slippage / slippageBase)


        console.log("Efficiency", Number(((redeemableDelta / estimatedFrictionlessRedeemableDelta) * 100).toFixed(2)), "%");
        console.log(
            `🧾 Minted`, Number(redeemableDelta.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol,
            "by locking", Number(collateralProcessedByMinting.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
            "(+~ takerFees =", Number(estimatedAmountRedeemableLostInTakerFees.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol,
            "(+~  slippage =", Number(estimatedAmountRedeemableLostInSlippage.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol, ")",
            "(frictionless minting would have been", Number(estimatedFrictionlessRedeemableDelta.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol, ")",
            "|| odd lot returns ", Number(collateralOddLotLeftOver.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
            ")"
        );
        expect(collateralAmount).closeTo(collateralProcessedByMinting + collateralOddLotLeftOver, collateralNativeUnitPrecision, "The amount of collateral left over + processed is not equal to the collateral amount inputted initially");
        expect(redeemableDelta).greaterThanOrEqual(worthExecutionPriceRedeemableDelta, "The amount minted is out of the slippage range");
        expect(collateralDelta).to.be.lessThanOrEqual(collateralAmount, "User paid more collateral than inputted amount");

        console.groupEnd();
        return redeemableDelta;
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}