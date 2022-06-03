import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, Mango, SafetyVault } from "@uxd-protocol/uxd-client";
import { TXN_OPTS, getConnection } from "../connection";


export const liquidationKillSwitchAccountingTest = async function (amountToLiquidate: number, slippage: number, controller: Controller, depository: MangoDepository, safetyVault: SafetyVault, mango: Mango, authority: Signer, payer?: Signer) {
    console.group("🧭 liquidationKillSwitchAccountingTest");
    try {
        const options = TXN_OPTS;

        // GIVEN
        const depositoryAccount = await depository.getOnchainAccount(getConnection(), options);
        const controllerAccount = await controller.getOnchainAccount(getConnection(), options);
        const safetyVaultAccount = await safetyVault.getOnchainAccount(getConnection(), options);

    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}