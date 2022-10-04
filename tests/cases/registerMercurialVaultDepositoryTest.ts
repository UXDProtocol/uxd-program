import { Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository } from "@uxd-protocol/uxd-client";
import { registerMercurialVaultDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection } from "../connection";

export const registerMercurialVaultDepositoryTest = async function (authority: Signer, controller: Controller, depository: MercurialVaultDepository, payer?: Signer) {
    console.group("🧭 initializeMercurialVaultDepositoryTest");
    try {
        // WHEN
        if (await getConnection().getAccountInfo(depository.pda)) {
            console.log("🚧 Already registered.");
        } else {
            const txId = await registerMercurialVaultDepository(authority, payer ?? authority, controller, depository);
            console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);
        }

        // THEN
        console.log(`🧾 Initialized`, depository.collateralMint.symbol, "Mercurial Vault Depository");
        depository.info();
        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}