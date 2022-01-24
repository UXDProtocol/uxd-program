import { Signer } from "@solana/web3.js";
import { Controller, ZoDepository, Zo } from "@uxdprotocol/uxd-client";
import { registerZoDepository } from "../api";
import { CLUSTER } from "../constants";
import { getProvider } from "../provider";

export const initializeZoDepositoryTest = async (authority: Signer, controller: Controller, depository: ZoDepository, zo: Zo) => {
    console.group("🧭 initializeMangoDepositoryTest");
    try {
        // WHEN
        if (await getProvider().connection.getAccountInfo(depository.zoAccountPda)) {
            console.log("🚧 Already registered.");
        } else {
            const txId = await registerZoDepository(authority, controller, depository, zo);
            console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);
        }

        // THEN
        console.log(`🧾 Initialized`, depository.collateralMintSymbol, "Zo Depository");
        depository.info();
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}