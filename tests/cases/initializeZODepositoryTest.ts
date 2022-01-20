import { Signer } from "@solana/web3.js";
import { Controller, ZoDepository, Zo } from "@uxdprotocol/uxd-client";
import { initializeZoDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection } from "../connection";

export const initializeZoDepositoryTest = async (authority: Signer, controller: Controller, depository: ZoDepository, zo: Zo, payer?: Signer) => {
    console.group("🧭 initializeZoDepositoryTest");
    try {
        // WHEN
        if (await getConnection().getAccountInfo(depository.zoAccountPda)) {
            console.log("🚧 Open Orders pda already created.");
        } else {
            const txId = await initializeZoDepository(authority, payer ?? authority, controller, depository, zo);
            console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);
        }

        // THEN
        console.log(`🧾 Initialized`, depository.collateralMintSymbol, "Zo Depository Open orders account");
        depository.info();
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}