import { Signer } from "@solana/web3.js";
import { Controller, ZoDepository, Zo } from "@uxdprotocol/uxd-client";
import { initializeZoDepository, registerZoDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection } from "../connection";

export const registerZoDepositoryTest = async (authority: Signer, controller: Controller, depository: ZoDepository, payer?: Signer) => {
    console.group("🧭 initializeMangoDepositoryTest");
    try {
        // WHEN
        if (await getConnection().getAccountInfo(depository.pda)) {
            console.log("🚧 Already registered.");
        } else {
            const txId = await registerZoDepository(authority, payer ?? authority, controller, depository);
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