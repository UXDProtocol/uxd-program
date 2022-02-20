import { getConnection, TXN_OPTS } from "./connection";
import { CLUSTER, uxdClient } from "./constants";
import { Account, Signer, Transaction } from '@solana/web3.js';
import { NATIVE_MINT } from "@solana/spl-token";
import { prepareWrappedSolTokenAccount } from "./utils";
import { MangoDepository, Mango, Controller, PnLPolarity, } from "@uxdprotocol/uxd-client";
import { web3 } from "@project-serum/anchor";

// Permissionned Calls --------------------------------------------------------

export async function initializeController(authority: Signer, payer: Signer, controller: Controller): Promise<string> {
    const initControllerIx = uxdClient.createInitializeControllerInstruction(controller, authority.publicKey, TXN_OPTS, payer.publicKey);

    const signers = [];
    const tx = new Transaction();

    tx.instructions.push(initControllerIx);
    signers.push(authority);
    if (payer) {
        signers.push(payer);
    }

    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function registerMangoDepository(authority: Signer, payer: Signer, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const registerMangoDepositoryIx = uxdClient.createRegisterMangoDepositoryInstruction(controller, depository, mango, authority.publicKey, TXN_OPTS, payer.publicKey);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(registerMangoDepositoryIx);
    signers.push(authority);
    if (payer) {
        signers.push(payer);
    }

    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function migrateMangoDepositoryToV2(authority: Signer, payer: Signer, controller: Controller, depository: MangoDepository): Promise<string> {
    const migrateMangoDepositoryToV2Ix = uxdClient.createMigrateMangoDepositoryToV2Instruction(controller, depository, authority.publicKey, TXN_OPTS, payer.publicKey);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(migrateMangoDepositoryToV2Ix);
    signers.push(authority);
    if (payer) {
        signers.push(payer);
    }

    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function depositInsuranceToMangoDepository(authority: Signer, amount: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const depositInsuranceToMangoDepositoryIx = uxdClient.createDepositInsuranceToMangoDepositoryInstruction(amount, controller, depository, mango, authority.publicKey, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(depositInsuranceToMangoDepositoryIx);
    signers.push(authority);

    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function withdrawInsuranceFromMangoDepository(authority: Signer, amount: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const withdrawInsuranceFromMangoDepository = uxdClient.createWithdrawInsuranceFromMangoDepositoryInstruction(amount, controller, depository, mango, authority.publicKey, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(withdrawInsuranceFromMangoDepository);
    signers.push(authority);

    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function setRedeemableGlobalSupplyCap(authority: Signer, controller: Controller, supplyCapUiAmount: number): Promise<string> {
    const setRedeemableGlobalSupplyCapIx = uxdClient.createSetRedeemableGlobalSupplyCapInstruction(controller, authority.publicKey, supplyCapUiAmount, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(setRedeemableGlobalSupplyCapIx);
    signers.push(authority);

    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function setMangoDepositoriesRedeemableSoftCap(authority: Signer, controller: Controller, supplySoftCapUiAmount: number): Promise<string> {
    const setMangoDepositoriesRedeemableSoftCapIx = uxdClient.createSetMangoDepositoriesRedeemableSoftCapInstruction(controller, authority.publicKey, supplySoftCapUiAmount, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(setMangoDepositoriesRedeemableSoftCapIx);
    signers.push(authority);

    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

// Permissionless Calls -------------------------------------------------------

export async function mintWithMangoDepository(user: Signer, payer: Signer, slippage: number, collateralAmount: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const mintWithMangoDepositoryIx = uxdClient.createMintWithMangoDepositoryInstruction(collateralAmount, slippage, controller, depository, mango, user.publicKey, TXN_OPTS, payer.publicKey);
    let signers = [];
    let tx = new Transaction();

    if (depository.collateralMint.equals(NATIVE_MINT)) {
        const nativeAmount = collateralAmount * 10 ** depository.collateralMintDecimals;
        const prepareWrappedSolIxs = await prepareWrappedSolTokenAccount(
            getConnection(),
            user.publicKey,
            nativeAmount
        );
        tx.instructions.push(...prepareWrappedSolIxs);
    }

    tx.instructions.push(mintWithMangoDepositoryIx);
    signers.push(user);
    if (payer) {
        signers.push(payer);
    }

    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function redeemFromMangoDepository(user: Signer, payer: Signer, slippage: number, amountRedeemable: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const redeemFromMangoDepositoryIx = uxdClient.createRedeemFromMangoDepositoryInstruction(amountRedeemable, slippage, controller, depository, mango, user.publicKey, TXN_OPTS, payer.publicKey);

    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(redeemFromMangoDepositoryIx);
    signers.push(user);
    if (payer) {
        signers.push(payer);
    }

    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function rebalanceMangoDepositoryLite(user: Signer, payer: Signer, rebalancingMaxAmountQuote: number, polarity: PnLPolarity, slippage: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const rebalanceMangoDepositoryLiteIx = uxdClient.createRebalanceMangoDepositoryLiteInstruction(rebalancingMaxAmountQuote, slippage, polarity, controller, depository, mango, user.publicKey, TXN_OPTS, payer.publicKey);
    let signers = [];
    let tx = new Transaction();

    // Only when polarity is positive this is required 
    // - Negative polarity sends QUOTE, gets COLLATERAL back.
    // - Positive polarity sends COLLATERAL, gets QUOTE back.
    if (polarity == PnLPolarity.Positive && depository.collateralMint.equals(NATIVE_MINT)) {
        console.log("rebalancingMaxAmount :", rebalancingMaxAmountQuote);
        const mangoPerpPrice = await depository.getCollateralPerpPriceUI(mango);
        const rebalancingMaxAmountCollateral = rebalancingMaxAmountQuote / mangoPerpPrice;
        const nativeAmount = rebalancingMaxAmountCollateral * 10 ** depository.collateralMintDecimals;
        const prepareWrappedSolIxs = await prepareWrappedSolTokenAccount(
            getConnection(),
            user.publicKey,
            nativeAmount
        );
        tx.instructions.push(...prepareWrappedSolIxs);
    }

    tx.instructions.push(rebalanceMangoDepositoryLiteIx);
    signers.push(user);
    if (payer) {
        signers.push(payer);
    }

    let txId = web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);

    // PNL should be settled afterward to ensure we have no "borrow" to prevent paying interests
    // const settlePnlTxID = await settleDepositoryPnl(payer, depository, mango);
    // console.log("🔗 depository PnL settlement Tx:", `'https://explorer.solana.com/tx/${settlePnlTxID}?cluster=${CLUSTER}'`);

    return txId;
}

// Non UXD API calls ----------------------------------------------------------

export async function settleDepositoryPnl(payer: Signer, depository: MangoDepository, mango: Mango): Promise<string> {
    let payerAccount = new Account(payer.secretKey);
    return depository.settleMangoDepositoryMangoAccountPnl(payerAccount, mango);
}