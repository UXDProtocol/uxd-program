
import { Controller, Depository, Mango } from "@uxdprotocol/uxd-client";
import { TXN_OPTS, provider } from "./provider";
import { NodeWallet } from "@project-serum/anchor/dist/cjs/provider";
import { ControllerAccount } from "@uxdprotocol/uxd-client/dist/types/uxd-interfaces";
import { uxdClient } from "./test_integration_0_consts";


afterEach("", () => {
    console.log("\n=====================================\n");
});


// Utils Calls ----------------------------------------------------------------

export async function collateralUIPriceInMangoQuote(depository: Depository, mango: Mango): Promise<number> {
    return uxdClient.perpUIPriceInQuote(mango, depository)
}

export async function redeemableCirculatinSupply(controller: Controller): Promise<number> {
    return uxdClient.redeemableCirculatinSupply(controller, TXN_OPTS)
}

export async function getControllerAccount(controller: Controller): Promise<ControllerAccount> {
    return uxdClient.getControllerAccount(controller, TXN_OPTS)
}

// Permissionned Calls --------------------------------------------------------

export async function initializeController(authority: NodeWallet, controller: Controller): Promise<string> {
    return uxdClient.initializeController(controller, authority, TXN_OPTS);
}

export async function registerMangoDepository(authority: NodeWallet, controller: Controller, depository: Depository, mango: Mango): Promise<string> {
    return uxdClient.registerMangoDepository(controller, depository, mango, authority, TXN_OPTS);
}

export async function setRedeemableGlobalSupplyCap(authority: NodeWallet, controller: Controller, supplyCapUiAmount: number): Promise<string> {
    return uxdClient.setRedeemableGlobalSupplyCap(controller, authority, supplyCapUiAmount, TXN_OPTS);
}

// User Facing Permissionless Calls -------------------------------------------

export function mintWithMangoDepository(user: NodeWallet, slippage: number, collateralAmount: number, controller: Controller, depository: Depository, mango: Mango): Promise<string> {
    return uxdClient.mintWithMangoDepository(collateralAmount, slippage, controller, depository, mango, user, TXN_OPTS);
}

export function redeemFromMangoDepository(user: NodeWallet, slippage: number, amountRedeemable: number, controller: Controller, depository: Depository, mango: Mango): Promise<string> {
    return uxdClient.redeemFromMangoDepository(amountRedeemable, slippage, controller, depository, mango, user, TXN_OPTS);
}