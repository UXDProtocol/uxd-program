import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, Mango } from "@uxdprotocol/uxd-client";
import { registerMangoDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection } from "../provider";

export const initializeMangoDepositoryTest = async (authority: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer) => {
    console.group("🧭 initializeMangoDepositoryTest");
    try {
        // WHEN
        if (await getConnection().getAccountInfo(depository.mangoAccountPda)) {
            console.log("🚧 Already registered.");
        } else {
            const txId = await registerMangoDepository(authority, payer ?? authority, controller, depository, mango);
            console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);
        }

        // THEN
        console.log(`🧾 Initialized`, depository.collateralMintSymbol, "Depository");
        depository.info();
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}