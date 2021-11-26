
import { Controller, Depository, Mango, MangoDepository } from "@uxdprotocol/uxd-client";
import { TXN_OPTS } from "./provider";
import { NodeWallet } from "@project-serum/anchor/dist/cjs/provider";
import { ControllerAccount, MangoDepositoryAccount } from "@uxdprotocol/uxd-client/dist/types/uxd-interfaces";
import { uxdClient } from "./test_0_consts";

afterEach("", () => {
    console.log("\n=====================================\n");
});

// Utils Calls ----------------------------------------------------------------

export async function collateralUIPriceInMangoQuote(depository: Depository, mango: Mango): Promise<number> {
    return uxdClient.perpUIPriceInQuote(mango, depository);
}

export async function redeemableCirculatinSupply(controller: Controller): Promise<number> {
    return uxdClient.redeemableCirculatinSupply(controller, TXN_OPTS);
}

export async function getControllerAccount(controller: Controller): Promise<ControllerAccount> {
    return uxdClient.getControllerAccount(controller, TXN_OPTS);
}

export async function getmangoDepositoryAccount(mangoDepository: MangoDepository): Promise<MangoDepositoryAccount> {
    return uxdClient.getMangoDepositoryAccount(mangoDepository, TXN_OPTS);
}

// DOESNT WORK in uxd-client- to fix
export async function getMangoDepositoryCollateralBalance(mangoDepository: MangoDepository, mango: Mango): Promise<number> {
    return uxdClient.getMangoDepositoryCollateralBalance(mangoDepository, mango);
}

// DOESNT WORK in uxd-client- to fix
export async function getMangoDepositoryInsuranceBalance(mangoDepository: MangoDepository, mango: Mango): Promise<number> {
    return uxdClient.getMangoDepositoryInsuranceBalance(mangoDepository, mango);
}

// Permissionned Calls --------------------------------------------------------

export async function initializeController(authority: NodeWallet, controller: Controller): Promise<string> {
    return uxdClient.initializeController(controller, authority, TXN_OPTS);
}

export async function registerMangoDepository(authority: NodeWallet, controller: Controller, depository: Depository, mango: Mango): Promise<string> {
    return uxdClient.registerMangoDepository(controller, depository, mango, authority, TXN_OPTS);
}

export async function depositInsuranceToMangoDepository(authority: NodeWallet, amount: number, controller: Controller, depository: Depository, mango: Mango): Promise<string> {
    return uxdClient.depositInsuranceToMangoDepository(amount, controller, depository, mango, authority, TXN_OPTS);
}

export async function withdrawInsuranceFromMangoDepository(authority: NodeWallet, amount: number, controller: Controller, depository: Depository, mango: Mango): Promise<string> {
    return uxdClient.withdrawInsuranceFromMangoDepository(amount, controller, depository, mango, authority, TXN_OPTS);
}

export async function setRedeemableGlobalSupplyCap(authority: NodeWallet, controller: Controller, supplyCapUiAmount: number): Promise<string> {
    return uxdClient.setRedeemableGlobalSupplyCap(controller, authority, supplyCapUiAmount, TXN_OPTS);
}

export async function setMangoDepositoriesRedeemableSoftCap(authority: NodeWallet, controller: Controller, supplySoftCapUiAmount: number): Promise<string> {
    return uxdClient.setMangoDepositoriesRedeemableSoftCap(controller, authority, supplySoftCapUiAmount, TXN_OPTS);
}

// User Facing Permissionless Calls -------------------------------------------

export function mintWithMangoDepository(user: NodeWallet, slippage: number, collateralAmount: number, controller: Controller, depository: Depository, mango: Mango): Promise<string> {
    // Need to sign manually but on the front end it's done through the provider
    return uxdClient.mintWithMangoDepository(collateralAmount, slippage, controller, depository, mango, user, TXN_OPTS, [], [], [user.payer]);
}

export function redeemFromMangoDepository(user: NodeWallet, slippage: number, amountRedeemable: number, controller: Controller, depository: Depository, mango: Mango): Promise<string> {
    // Need to sign manually but on the front end it's done through the provider
    return uxdClient.redeemFromMangoDepository(amountRedeemable, slippage, controller, depository, mango, user, TXN_OPTS, [], [], [user.payer]);
}