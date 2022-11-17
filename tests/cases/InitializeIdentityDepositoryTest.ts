import { Signer } from "@solana/web3.js";
import { Controller, IdentityDepository } from "@uxd-protocol/uxd-client";
import { initializeIdentityDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection } from "../connection";

export const InitialiazeIdentityDepositoryTest = async function (
    authority: Signer,
    controller: Controller,
    depository: IdentityDepository,
    payer?: Signer,
) {
    console.group("🧭 InitialiazeIdentityDepositoryTest");
    try {
        // WHEN
        if (await getConnection().getAccountInfo(depository.pda)) {
            console.log("🚧 Already initialized.");
        } else {
            const txId = await initializeIdentityDepository(authority, payer ?? authority, controller, depository);
            console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);
        }

        // THEN
        console.log(`🧾 Initialized`, depository.collateralMint, "Identity depository");
        depository.info();
        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}