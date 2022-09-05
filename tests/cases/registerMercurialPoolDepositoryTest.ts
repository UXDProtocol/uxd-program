import { Signer } from "@solana/web3.js";
import { Controller, MercurialPoolDepository } from "@uxd-protocol/uxd-client";
import { registerMercurialPoolDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection } from "../connection";

export const registerMercurialPoolDepositoryTest = async function (authority: Signer, controller: Controller, depository: MercurialPoolDepository, payer?: Signer) {
    console.group("🧭 initializeMercurialPoolDepositoryTest");
    try {
        // WHEN
        if (await getConnection().getAccountInfo(depository.pda)) {
            console.log("🚧 Already registered.");
        } else {
            const txId = await registerMercurialPoolDepository(authority, payer ?? authority, controller, depository);
            console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);
        }

        // THEN
        console.log(`🧾 Initialized`, depository.collateralMint.symbol, "Mercurial Pool Depository");
        depository.info();
        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}