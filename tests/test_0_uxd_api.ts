
import { Controller, MangoDepository, Mango, findATAAddrSync, createAssocTokenIx } from "@uxdprotocol/uxd-client";
import { provider, TXN_COMMIT, TXN_OPTS } from "./provider";
import { ControllerAccount, MangoDepositoryAccount } from "@uxdprotocol/uxd-client/dist/types/uxd-interfaces";
import { controllerUXD, uxdClient, uxdHelpers } from "./test_0_consts";
import { Signer, Transaction } from '@solana/web3.js';

afterEach("", () => {
    console.log("\n=====================================\n");
});

// Utils Calls ----------------------------------------------------------------

export async function collateralUIPriceInMangoQuote(depository: MangoDepository, mango: Mango): Promise<number> {
    return uxdHelpers.perpUIPriceInQuote(mango, depository);
}

export async function redeemableCirculatinSupply(controller: Controller): Promise<number> {
    return uxdHelpers.redeemableCirculatinSupply(provider, controller, TXN_OPTS);
}

export async function getControllerAccount(controller: Controller): Promise<ControllerAccount> {
    return uxdHelpers.getControllerAccount(provider, uxdClient.program, controller, TXN_OPTS);
}

export async function getmangoDepositoryAccount(mangoDepository: MangoDepository): Promise<MangoDepositoryAccount> {
    return uxdHelpers.getMangoDepositoryAccount(provider, uxdClient.program, mangoDepository, TXN_OPTS);
}

// DOESNT WORK in uxd-client- to fix
export async function getMangoDepositoryCollateralBalance(mangoDepository: MangoDepository, mango: Mango): Promise<number> {
    return uxdHelpers.getMangoDepositoryCollateralBalance(mangoDepository, mango);
}

// DOESNT WORK in uxd-client- to fix
export async function getMangoDepositoryInsuranceBalance(mangoDepository: MangoDepository, mango: Mango): Promise<number> {
    return uxdHelpers.getMangoDepositoryInsuranceBalance(mangoDepository, mango);
}

// Permissionned Calls --------------------------------------------------------

export async function initializeController(authority: Signer, controller: Controller): Promise<string> {
    const initControllerIx = uxdClient.createInitializeControllerInstruction(controller, authority, TXN_OPTS);

    const signers = [];
    const tx = new Transaction();

    tx.instructions.push(initControllerIx);
    signers.push(authority);

    return provider.send(tx, signers, TXN_OPTS);
}

export function registerMangoDepository(authority: Signer, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const registerMangoDepositoryIx = uxdClient.createRegisterMangoDepositoryInstruction(controller, depository, mango, authority, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(registerMangoDepositoryIx);
    signers.push(authority);

    return provider.send(tx, signers, TXN_OPTS);
}

export function depositInsuranceToMangoDepository(authority: Signer, amount: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const depositInsuranceToMangoDepositoryIx = uxdClient.createDepositInsuranceToMangoDepositoryInstruction(amount, controller, depository, mango, authority, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(depositInsuranceToMangoDepositoryIx);
    signers.push(authority);

    return provider.send(tx, signers, TXN_OPTS);
}

export function withdrawInsuranceFromMangoDepository(authority: Signer, amount: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const withdrawInsuranceFromMangoDepository = uxdClient.createWithdrawInsuranceFromMangoDepositoryInstruction(amount, controller, depository, mango, authority, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(withdrawInsuranceFromMangoDepository);
    signers.push(authority);

    return provider.send(tx, signers, TXN_OPTS);
}

export function setRedeemableGlobalSupplyCap(authority: Signer, controller: Controller, supplyCapUiAmount: number): Promise<string> {
    const setRedeemableGlobalSupplyCapIx = uxdClient.createSetRedeemableGlobalSupplyCapInstruction(controller, authority, supplyCapUiAmount, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(setRedeemableGlobalSupplyCapIx);
    signers.push(authority);

    return provider.send(tx, signers, TXN_OPTS);
}

export function setMangoDepositoriesRedeemableSoftCap(authority: Signer, controller: Controller, supplySoftCapUiAmount: number): Promise<string> {
    const setMangoDepositoriesRedeemableSoftCapIx = uxdClient.createSetMangoDepositoriesRedeemableSoftCapInstruction(controller, authority, supplySoftCapUiAmount, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(setMangoDepositoriesRedeemableSoftCapIx);
    signers.push(authority);

    return provider.send(tx, signers, TXN_OPTS);
}

// User Facing Permissionless Calls -------------------------------------------

export async function mintWithMangoDepository(user: Signer, slippage: number, collateralAmount: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const mintWithMangoDepositoryIx = uxdClient.createMintWithMangoDepositoryInstruction(collateralAmount, slippage, controller, depository, mango, user, TXN_OPTS);
    const mangoConsumeEventsIx = await mango.createConsumeEventInstruction(depository.mangoAccountPda, mango.getPerpMarketConfigFor(depository.collateralMintSymbol), `sell`);
    let signers = [];
    let tx = new Transaction();

    const userRedeemableATA = findATAAddrSync(user.publicKey, controllerUXD.redeemableMintPda)[0];
    if (!(await provider.connection.getAccountInfo(userRedeemableATA))) {
        tx.instructions.push(createAssocTokenIx(user.publicKey, userRedeemableATA, controllerUXD.redeemableMintPda));
    }

    tx.instructions.push(mintWithMangoDepositoryIx);
    // tx.instructions.push(mangoConsumeEventsIx);
    signers.push(user);

    return provider.send(tx, signers, TXN_OPTS);
}

export async function redeemFromMangoDepository(user: Signer, slippage: number, amountRedeemable: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const redeemFromMangoDepositoryIx = uxdClient.createRedeemFromMangoDepositoryInstruction(amountRedeemable, slippage, controller, depository, mango, user, TXN_OPTS);
    const mangoConsumeEventsIx = await mango.createConsumeEventInstruction(depository.mangoAccountPda, mango.getPerpMarketConfigFor(depository.collateralMintSymbol), `buy`);
    let signers = [];
    let tx = new Transaction();

    const userCollateralATA = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
    if (!(await provider.connection.getAccountInfo(userCollateralATA))) {
        tx.instructions.push(createAssocTokenIx(user.publicKey, userCollateralATA, depository.collateralMint));
    }

    tx.instructions.push(redeemFromMangoDepositoryIx);
    // tx.instructions.push(mangoConsumeEventsIx);
    signers.push(user);

    return provider.send(tx, signers, TXN_OPTS);
}