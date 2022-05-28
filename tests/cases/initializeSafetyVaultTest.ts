import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxd-protocol/uxd-client";
import { SafetyVault } from "@uxd-protocol/uxd-client/dist/types/mango/safetyVault";
import { initializeSafetyVault } from "../api";
import { getConnection } from "../connection";

export const initializeSafetyVaultTest = async function(authority: Signer, controller: Controller, depository: MangoDepository, safetyVault: SafetyVault, payer?: Signer) {
    console.group("⏱ initializeSafetyVaultTest");
    try {
        // WHEN
        if (await getConnection().getAccountInfo(safetyVault.pda)) {
            console.log("🚧 Already initialized.");
        } else {
            const txId = await initializeSafetyVault(authority, payer ?? authority, controller, depository, safetyVault);
            console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);
        }

        // THEN
        console.log(`🧾 Initialized`, depository.collateralMint, "Safety Vault");
        safetyVault.info();
        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}