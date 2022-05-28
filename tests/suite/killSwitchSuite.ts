import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, SafetyVault } from "@uxd-protocol/uxd-client";
import { initializeSafetyVaultTest } from "../cases/initializeSafetyVaultTest";
import { liquidationKillSwitchTest } from "../cases/liquidationKillSwitchTest";
import { mango } from "../fixtures";

export const killSwitchSuite = function(authority: Signer, payer: Signer, controller: Controller, depository: MangoDepository, safetyVault: SafetyVault, slippage: number) {
    it(`Initialize safety vault for ${depository.collateralMintSymbol} depository`, async function () {
        await initializeSafetyVaultTest(authority, controller, depository, safetyVault, payer);
    });

    it(`Call Liquidation Kill Switch to half of current collateral`, async function () {
        await liquidationKillSwitchTest(5, slippage, controller, depository, safetyVault, mango, authority, payer);
    });
}