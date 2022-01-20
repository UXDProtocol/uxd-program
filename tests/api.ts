import { getConnection, TXN_OPTS } from "./connection";
import { CLUSTER, uxdClient } from "./constants";
import { Account, Keypair, Signer, SystemProgram, Transaction } from '@solana/web3.js';
import { NATIVE_MINT } from "@solana/spl-token";
import { prepareWrappedSolTokenAccount } from "./utils";
import { MangoDepository, Mango, Controller, PnLPolarity, ZoDepository, Zo, CONTROL_ACCOUNT_SIZE } from "@uxdprotocol/uxd-client";
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
    tx.feePayer = payer.publicKey;
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
    tx.feePayer = payer.publicKey;
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
    tx.feePayer = payer.publicKey;
    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function registerZoDepository(authority: Signer, payer: Signer, controller: Controller, depository: ZoDepository): Promise<string> {
    const registerZoDepositoryIx = await uxdClient.createRegisterZoDepositoryInstruction(controller, depository, authority.publicKey, TXN_OPTS, payer.publicKey);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(registerZoDepositoryIx);
    signers.push(authority);
    signers.push(payer);
    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function initializeZoDepository(authority: Signer, payer: Signer, controller: Controller, depository: ZoDepository, zo: Zo): Promise<string> {
    // Was done on chain, but the instruction is limited in term of stack size so we do it here (to free up space to create the openOrderAccount PDA on chain)
    const control = new Keypair();
    const initializeZoDepositoryIx = await uxdClient.createInitializeZoDepositoryInstruction(controller, depository, zo, control.publicKey, authority.publicKey, TXN_OPTS, payer.publicKey);
    let signers = [];
    let tx = new Transaction();

    // Create control account
    const controlLamports = await getConnection().getMinimumBalanceForRentExemption(CONTROL_ACCOUNT_SIZE);
    const createControlAccount = SystemProgram.createAccount({
        fromPubkey: payer.publicKey ?? authority.publicKey,
        newAccountPubkey: control.publicKey,
        lamports: controlLamports,
        space: CONTROL_ACCOUNT_SIZE,
        programId: zo.program.programId,
    });

    tx.instructions.push(createControlAccount);
    tx.instructions.push(initializeZoDepositoryIx);

    signers.push(authority);
    signers.push(payer);
    signers.push(control);
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
    const mintWithMangoDepositoryIx = await uxdClient.createMintWithMangoDepositoryInstruction(collateralAmount, slippage, controller, depository, mango, user.publicKey, TXN_OPTS, payer.publicKey);
    let signers = [];
    let tx = new Transaction();

    if (depository.collateralMint.equals(NATIVE_MINT)) {
        const nativeAmount = collateralAmount * 10 ** depository.collateralMintDecimals;
        const prepareWrappedSolIxs = await prepareWrappedSolTokenAccount(
            getConnection(),
            payer.publicKey,
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
    tx.feePayer = payer.publicKey;
    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function redeemFromMangoDepository(user: Signer, payer: Signer, slippage: number, amountRedeemable: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const redeemFromMangoDepositoryIx = await uxdClient.createRedeemFromMangoDepositoryInstruction(amountRedeemable, slippage, controller, depository, mango, user.publicKey, TXN_OPTS, payer.publicKey);

    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(redeemFromMangoDepositoryIx);
    signers.push(user);
    if (payer) {
        signers.push(payer);
    }

    tx.feePayer = payer.publicKey;
    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function rebalanceMangoDepositoryLite(user: Signer, payer: Signer, rebalancingMaxAmountQuote: number, polarity: PnLPolarity, slippage: number, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const rebalanceMangoDepositoryLiteIx = await uxdClient.createRebalanceMangoDepositoryLiteInstruction(rebalancingMaxAmountQuote, slippage, polarity, controller, depository, mango, user.publicKey, TXN_OPTS, payer.publicKey);
    let signers = [];
    let tx = new Transaction();

    // Only when polarity is positive this is required 
    // - Negative polarity sends QUOTE, gets COLLATERAL back.
    // - Positive polarity sends COLLATERAL, gets QUOTE back.
    if (polarity == PnLPolarity.Positive && depository.collateralMint.equals(NATIVE_MINT)) {
        const mangoPerpPrice = await depository.getCollateralPerpPriceUI(mango);
        const rebalancingMaxAmountCollateral = rebalancingMaxAmountQuote / mangoPerpPrice;
        const nativeAmount = rebalancingMaxAmountCollateral * 10 ** depository.collateralMintDecimals;
        const prepareWrappedSolIxs = await prepareWrappedSolTokenAccount(
            getConnection(),
            payer.publicKey,
            user.publicKey,
            nativeAmount
        );
        tx.instructions.push(...prepareWrappedSolIxs);
    } else {
        // TEMPORARY - Also make a WSOL account to prevent program doing it and save some computing
        const createWSOLATAIxs = await prepareWrappedSolTokenAccount(
            getConnection(),
            payer.publicKey,
            user.publicKey,
            0
        );
        tx.instructions.push(...createWSOLATAIxs);
    }

    tx.instructions.push(rebalanceMangoDepositoryLiteIx);
    signers.push(user);
    if (payer) {
        signers.push(payer);
    }

    tx.feePayer = payer.publicKey;
    let txId = web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);

    // PNL should be settled afterward to ensure we have no "borrow" to prevent paying interests
    // const settlePnlTxID = await settleDepositoryPnl(payer, depository, mango);
    // console.log("🔗 depository PnL settlement Tx:", `'https://explorer.solana.com/tx/${settlePnlTxID}?cluster=${CLUSTER}'`);

    return txId;
}

export async function mintWithZoDepository(user: Signer, payer: Signer, slippage: number, collateralAmount: number, controller: Controller, depository: ZoDepository, zo: Zo): Promise<string> {
    const mintWithZoDepositoryIx = await uxdClient.createMintWithZoDepositoryInstruction(collateralAmount, slippage, controller, depository, zo, user.publicKey, TXN_OPTS, payer.publicKey);
    let signers = [];
    let tx = new Transaction();

    if (depository.collateralMint.equals(NATIVE_MINT)) {
        const nativeAmount = collateralAmount * 10 ** depository.collateralMintDecimals;
        const prepareWrappedSolIxs = await prepareWrappedSolTokenAccount(
            getConnection(),
            payer.publicKey,
            user.publicKey,
            nativeAmount
        );
        tx.instructions.push(...prepareWrappedSolIxs);
    }

    tx.instructions.push(mintWithZoDepositoryIx);
    signers.push(user);
    if (payer) {
        signers.push(payer);
    }
    tx.feePayer = payer.publicKey;
    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}
// Non UXD API calls ----------------------------------------------------------

export async function settleDepositoryPnl(payer: Signer, depository: MangoDepository, mango: Mango): Promise<string> {
    let payerAccount = new Account(payer.secretKey);
    return depository.settleMangoDepositoryMangoAccountPnl(payerAccount, mango);
}