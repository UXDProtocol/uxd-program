import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxdprotocol/uxd-client";
import { migrateMangoDepositoryToV2 } from "../api";
import { CLUSTER, uxdHelpers } from "../constants";
import { getProvider, TXN_OPTS } from "../provider";

export const migrateMangoDepositoryToV2Test = async (authority: Signer, controller: Controller, depository: MangoDepository) => {
    console.group("🧭 migrateMangoDepositoryToV2Test");
    try {
        await getProvider().connection.getAccountInfo(controller.pda); // With throw if doesn't exist
        try {

            // WHEN
            if ((await uxdHelpers.getMangoDepositoryAccountNoProvider(getProvider().connection, depository, TXN_OPTS)).version != 1) {
                console.log("🚧 Already migrated.");
            } else {
                const txId = await migrateMangoDepositoryToV2(authority, controller, depository);
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
    } catch (error) {
        console.groupEnd();
        console.log("🚧 Depository not initialized. Nothing to migrate. All good.");
        return;
    }
}