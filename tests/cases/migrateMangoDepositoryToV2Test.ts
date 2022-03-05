import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { migrateMangoDepositoryToV2 } from "../api";
import { CLUSTER } from "../constants";
import { getConnection, TXN_OPTS } from "../connection";

export const migrateMangoDepositoryToV2Test = async function (authority: Signer, controller: Controller, depository: MangoDepository, payer?: Signer) {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("🧭 migrateMangoDepositoryToV2Test");
    try {
        await getConnection().getAccountInfo(controller.pda);
        try {
            // WHEN
            const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);
            if (depositoryOnchainAccount.version > 1) {
                console.log("🚧 Already migrated.");
            } else {
                try {
                    const txId = await migrateMangoDepositoryToV2(authority, payer ?? authority, controller, depository);
                    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);
                } catch (error) {
                    console.error("❌", error);
                    console.groupEnd();
                    throw error;
                }
            }

            // THEN
            const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
            expect(depositoryOnchainAccount_post.version).to.equals(2);
            console.log(`🧾 Initialized`, depository.collateralMintSymbol, "Depository");
            depository.info();
            console.groupEnd();
        } catch (error) {
            console.error("❌", error);
            console.groupEnd();
            throw error;
        }
    } catch (error) {
        console.groupEnd();
        console.log("🚧 Depository not initialized. Nothing to migrate. All good.");
        return;
    }
}