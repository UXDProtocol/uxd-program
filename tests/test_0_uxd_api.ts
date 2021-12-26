
import { Controller, MangoDepository, Mango, findATAAddrSync, createAssocTokenIx } from "@uxdprotocol/uxd-client";
import { provider, TXN_COMMIT, TXN_OPTS } from "./provider";
import { controllerUXD, uxdClient, uxdHelpers } from "./test_0_consts";
import { Account, Signer, Transaction } from '@solana/web3.js';
import { user } from "./identities";
import { NATIVE_MINT } from "@solana/spl-token";
import { ControllerAccount, MangoDepositoryAccount } from "@uxdprotocol/uxd-client/dist/types/uxd-interfaces";
import { prepareWrappedSolTokenAccount } from "./wsol-utils";

afterEach("", () => {
    console.log("\n=====================================\n");
});

// Utils Calls ----------------------------------------------------------------

export async function collateralUIPriceInMangoQuote(depository: MangoDepository, mango: Mango): Promise<number> {
    return uxdHelpers.perpUIPriceInQuote(mango, depository);
}

export async function redeemableCirculatingSupply(controller: Controller): Promise<number> {
    return uxdHelpers.redeemableCirculatingSupply(provider, controller, TXN_OPTS);
}

export async function getControllerAccount(controller: Controller): Promise<ControllerAccount> {
    return uxdHelpers.getControllerAccount(provider, uxdClient.program, controller, TXN_OPTS);
}

export async function getMangoDepositoryAccount(mangoDepository: MangoDepository): Promise<MangoDepositoryAccount> {
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

export async function settleMangoDepositoryMangoAccountPnl(depository: MangoDepository, mango: Mango): Promise<string> {
    const mangoAccount = await mango.load(depository.mangoAccountPda);
    const perpMarketConfig = mango.getPerpMarketConfig(depository.collateralMintSymbol);
    const cache = await mango.group.loadCache(provider.connection);
    const perpMarket = await mango.client.getPerpMarket(perpMarketConfig.publicKey, perpMarketConfig.baseDecimals, perpMarketConfig.quoteDecimals);
    const quoteRootBank = await mango.getQuoteRootBank();

    const caller = new Account(user.secretKey);

    return mango.client.settlePnl(mango.group, cache, mangoAccount, perpMarket, quoteRootBank, cache.priceCache[perpMarketConfig.marketIndex].price, caller);
}

export async function settleMangoDepositoryMangoAccountFees(depository: MangoDepository, mango: Mango): Promise<string> {
    const mangoAccount = await mango.load(depository.mangoAccountPda);
    const perpMarketConfig = mango.getPerpMarketConfig(depository.collateralMintSymbol);
    const perpMarket = await mango.client.getPerpMarket(perpMarketConfig.publicKey, perpMarketConfig.baseDecimals, perpMarketConfig.quoteDecimals);
    const quoteRootBank = await mango.getQuoteRootBank();

    const caller = new Account(user.secretKey);
    return mango.client.settleFees(mango.group, mangoAccount, perpMarket, quoteRootBank, caller);
}

// Permissionned Calls --------------------------------------------------------

export async function initializeController(authority: Signer, controller: Controller): Promise<string> {
    const initControllerIx = uxdClient.createInitializeControllerInstruction(controller, authority.publicKey, TXN_OPTS);

    const signers = [];
    const tx = new Transaction();

    tx.instructions.push(initControllerIx);
    signers.push(authority);

    return provider.send(tx, signers, TXN_OPTS);
}

export function registerMangoDepository(authority: Signer, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const registerMangoDepositoryIx = uxdClient.createRegisterMangoDepositoryInstruction(controller, depository, mango, authority.publicKey, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(registerMangoDepositoryIx);
    signers.push(authority);

    return provider.send(tx, signers, TXN_OPTS);
}

export function depositInsuranceToMangoDepository(authority: Signer, amount: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const depositInsuranceToMangoDepositoryIx = uxdClient.createDepositInsuranceToMangoDepositoryInstruction(amount, controller, depository, mango, authority.publicKey, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(depositInsuranceToMangoDepositoryIx);
    signers.push(authority);

    return provider.send(tx, signers, TXN_OPTS);
}

export function withdrawInsuranceFromMangoDepository(authority: Signer, amount: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const withdrawInsuranceFromMangoDepository = uxdClient.createWithdrawInsuranceFromMangoDepositoryInstruction(amount, controller, depository, mango, authority.publicKey, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(withdrawInsuranceFromMangoDepository);
    signers.push(authority);

    return provider.send(tx, signers, TXN_OPTS);
}

export function setRedeemableGlobalSupplyCap(authority: Signer, controller: Controller, supplyCapUiAmount: number): Promise<string> {
    const setRedeemableGlobalSupplyCapIx = uxdClient.createSetRedeemableGlobalSupplyCapInstruction(controller, authority.publicKey, supplyCapUiAmount, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(setRedeemableGlobalSupplyCapIx);
    signers.push(authority);

    return provider.send(tx, signers, TXN_OPTS);
}

export function setMangoDepositoriesRedeemableSoftCap(authority: Signer, controller: Controller, supplySoftCapUiAmount: number): Promise<string> {
    const setMangoDepositoriesRedeemableSoftCapIx = uxdClient.createSetMangoDepositoriesRedeemableSoftCapInstruction(controller, authority.publicKey, supplySoftCapUiAmount, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(setMangoDepositoriesRedeemableSoftCapIx);
    signers.push(authority);

    return provider.send(tx, signers, TXN_OPTS);
}

// User Facing Permissionless Calls -------------------------------------------

export async function mintWithMangoDepository(user: Signer, slippage: number, collateralAmount: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const mintWithMangoDepositoryIx = uxdClient.createMintWithMangoDepositoryInstruction(collateralAmount, slippage, controller, depository, mango, user.publicKey, TXN_OPTS);
    const mangoConsumeEventsIx = await mango.createPerpMarketConsumeEventInstruction(depository.mangoAccountPda, depository.collateralMintSymbol, `sell`);
    let signers = [];
    let tx = new Transaction();

    const userRedeemableATA = findATAAddrSync(user.publicKey, controllerUXD.redeemableMintPda)[0];
    if (!(await provider.connection.getAccountInfo(userRedeemableATA))) {
        tx.instructions.push(createAssocTokenIx(user.publicKey, userRedeemableATA, controllerUXD.redeemableMintPda));
    }

    if (depository.collateralMint.equals(NATIVE_MINT)) {
        const nativeAmount = collateralAmount * 10 ** depository.collateralMintDecimals;
        const prepareWrappedSolIxs = await prepareWrappedSolTokenAccount(
            provider.connection,
            user.publicKey,
            nativeAmount
        );
        tx.instructions.push(...prepareWrappedSolIxs);
    }

    tx.instructions.push(mintWithMangoDepositoryIx);
    signers.push(user);

    let txId = await provider.send(tx, signers, TXN_OPTS);

    let tx2 = new Transaction();
    tx2.instructions.push(mangoConsumeEventsIx);
    provider.send(tx2, [], TXN_OPTS);

    return txId;

    // tx.instructions.push(mangoConsumeEventsIx);

    // // To check the size. Currently 1101 bytes
    // tx.recentBlockhash = (await provider.connection.getRecentBlockhash(TXN_COMMIT)).blockhash;
    // tx.sign(...signers);
    // console.log(tx.serialize().byteLength);
    // return provider.send(tx, signers, TXN_OPTS);
}

export async function redeemFromMangoDepository(user: Signer, slippage: number, amountRedeemable: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const redeemFromMangoDepositoryIx = uxdClient.createRedeemFromMangoDepositoryInstruction(amountRedeemable, slippage, controller, depository, mango, user.publicKey, TXN_OPTS);
    const mangoConsumeEventsIx = await mango.createPerpMarketConsumeEventInstruction(depository.mangoAccountPda, depository.collateralMintSymbol, `buy`);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(redeemFromMangoDepositoryIx);
    signers.push(user);
    tx.instructions.push(mangoConsumeEventsIx);
    // To check the size. Currently 1101 bytes
    // tx.recentBlockhash = (await provider.connection.getRecentBlockhash(TXN_COMMIT)).blockhash;
    // tx.sign(...signers);
    // console.log(tx.serialize().byteLength);
    return provider.send(tx, signers, TXN_OPTS);
}