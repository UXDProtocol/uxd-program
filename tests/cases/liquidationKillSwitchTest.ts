import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, SafetyVault } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { liquidationKillSwitch } from "../api";
import { CLUSTER } from "../constants";
import { getBalance, getSolBalance } from "../utils";

export const liquidationKillSwitchTest = async function(targetCollateral: number, slippage: number, controller: Controller, depository: MangoDepository, safetyVault: SafetyVault, mango: Mango, authority: Signer, payer?: Signer): Promise<number> {
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
        const collateralWithdrawn: number = safetyVaultCollateralBalance - targetCollateral;

        // WHEN
        const txId = await liquidationKillSwitch(authority, payer ?? authority, targetCollateral, slippage, controller, depository, safetyVault, mango);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        let safetyVaultCollateralBalance_post: number;
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            // If WSOL, as the transaction unwraps
            safetyVaultCollateralBalance_post = await getSolBalance(safetyVault.pda);
        } else {
            safetyVaultCollateralBalance_post = await getBalance(safetyVaultCollateralATA);
        }

        expect(safetyVaultCollateralBalance_post).equals(safetyVaultCollateralBalance + collateralWithdrawn);
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}