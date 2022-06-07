import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, I80F48, Mango, MangoDepository, SafetyVault } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { liquidationKillSwitch } from "../api";
import { CLUSTER } from "../constants";
import { getBalance, getSolBalance } from "../utils";

export const liquidationKillSwitchTest = async function(amountToLiquidate: number, slippage: number, controller: Controller, depository: MangoDepository, safetyVault: SafetyVault, mango: Mango, authority: Signer, payer?: Signer) {
    console.group("🧭 liquidationKillSwitchTest");
    try {
        // GIVEN
        const safetyVaultCollateralATA: PublicKey = safetyVault.collateralVaultPda;
        let safetyVaultCollateralBalance: number;
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            // If WSOL, as the transaction unwraps
            safetyVaultCollateralBalance = await getSolBalance(safetyVault.pda);
        } else {
            safetyVaultCollateralBalance = await getBalance(safetyVaultCollateralATA);
        }

        // WHEN
        // - Get the perp and limit price at the same moment to have the less diff between exec and test price.
        // Simulates user experience from the front end
        const mangoPerpPrice = await depository.getCollateralPerpPriceUI(mango);
        const limitPrice = (
            await depository.getLimitPrice(I80F48.fromNumber(slippage), 'long', mango)
        ).toNumber();
        const txId = await liquidationKillSwitch(authority, payer ?? authority, amountToLiquidate, slippage, controller, depository, safetyVault, mango);
        console.log("🪙  perp price is", Number(mangoPerpPrice.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        let safetyVaultCollateralBalance_post: number;
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            // If WSOL, as the transaction unwraps
            safetyVaultCollateralBalance_post = await getSolBalance(safetyVault.pda);
        } else {
            safetyVaultCollateralBalance_post = await getBalance(safetyVaultCollateralATA);
        }

        // const mangoTakerFee = depository.getCollateralPerpTakerFees(mango);

        const collateralDelta = safetyVaultCollateralBalance_post - safetyVaultCollateralBalance;
        const minBase = amountToLiquidate / limitPrice;

        expect(collateralDelta).greaterThanOrEqual(minBase, "The collateral delta is not larger or equal to the minimum base");
        console.log(collateralDelta);
        console.log(minBase);
        // expect(safetyVaultCollateralBalance_post).equals(safetyVaultCollateralBalance + collateralWithdrawn);
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}