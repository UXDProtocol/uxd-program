import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, Mango } from "@uxd-protocol/uxd-client";
import { registerMangoDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection } from "../connection";
import { BN } from "@project-serum/anchor";

export const registerMangoDepositoryTest = async function (authority: Signer, controller: Controller, depository: MangoDepository, mango: Mango, redeemableDepositorySupplyCap: number, payer?: Signer) {
    console.group("🧭 initializeMangoDepositoryTest");
    try {
        // WHEN
        if (await getConnection().getAccountInfo(depository.mangoAccountPda)) {
            console.log("🚧 Already registered.");
        } else {
            const txId = await registerMangoDepository(authority, payer ?? authority, controller, depository, mango, redeemableDepositorySupplyCap);
            console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);
        }

        // THEN
        console.log(`🧾 Initialized`, depository.collateralMintSymbol, "Mango Depository");
        depository.info();
        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}