import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxdprotocol/uxd-client";
import { migrateMangoDepositoryToV2Test } from "../cases/migrateMangoDepositoryToV2Test";

export const mangoDepositoryMigrationsSuite = function (authority: Signer, payer: Signer, controller: Controller, depository: MangoDepository) {
    it(`Migrates ${depository.collateralMintSymbol} Depository`, async function () {
        await migrateMangoDepositoryToV2Test(authority, controller, depository, payer);
    });
};