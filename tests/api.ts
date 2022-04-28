import { getConnection, TXN_OPTS } from "./connection";
import { uxdClient } from "./constants";
import { Keypair, PublicKey, Signer, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { NATIVE_MINT } from "@solana/spl-token";
import { prepareWrappedSolTokenAccount } from "./utils";
import { MangoDepository, Mango, Controller, PnLPolarity, ZoDepository, Zo, CONTROL_ACCOUNT_SIZE, createAssocTokenIx, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { BN, web3 } from "@project-serum/anchor";

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
    const depositInsuranceToMangoDepositoryIx = await uxdClient.createDepositInsuranceToMangoDepositoryInstruction(amount, controller, depository, mango, authority.publicKey, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(depositInsuranceToMangoDepositoryIx);
    signers.push(authority);

    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function depositInsuranceToZoDepository(authority: Signer, amount: number, controller: Controller, depository: ZoDepository, zo: Zo): Promise<string> {
    const depositInsuranceToZoDepositoryIx = await uxdClient.createDepositInsuranceToZoDepositoryInstruction(amount, controller, depository, zo, authority.publicKey, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    tx.instructions.push(depositInsuranceToZoDepositoryIx);
    signers.push(authority);

    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function withdrawInsuranceFromMangoDepository(amount: number, authority: Signer, controller: Controller, depository: MangoDepository, mango: Mango): Promise<string> {
    const withdrawInsuranceFromMangoDepository = uxdClient.createWithdrawInsuranceFromMangoDepositoryInstruction(amount, controller, depository, mango, authority.publicKey, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    const authorityQuoteAta = findATAAddrSync(authority.publicKey, depository.quoteMint)[0];
    if (!await getConnection().getAccountInfo(authorityQuoteAta)) {
        const createUserQuoteAtaIx = createAssocTokenIx(authority.publicKey, authorityQuoteAta, depository.quoteMint);
        tx.add(createUserQuoteAtaIx);
    }

    tx.instructions.push(withdrawInsuranceFromMangoDepository);
    signers.push(authority);

    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function withdrawInsuranceFromZoDepository(amount: number, authority: Signer, controller: Controller, depository: ZoDepository, zo: Zo): Promise<string> {
    const withdrawInsuranceFromZoDepository = await uxdClient.createWithdrawInsuranceFromZoDepositoryInstruction(amount, controller, depository, zo, authority.publicKey, TXN_OPTS);
    let signers = [];
    let tx = new Transaction();

    const authorityQuoteAta = findATAAddrSync(authority.publicKey, depository.quoteMint)[0];
    if (!await getConnection().getAccountInfo(authorityQuoteAta)) {
        const createUserQuoteAtaIx = createAssocTokenIx(authority.publicKey, authorityQuoteAta, depository.quoteMint);
        tx.add(createUserQuoteAtaIx);
    }

    tx.instructions.push(withdrawInsuranceFromZoDepository);
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

    const userRedeemableAta = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
    if (!await getConnection().getAccountInfo(userRedeemableAta)) {
        const createUserRedeemableAtaIx = createAssocTokenIx(user.publicKey, userRedeemableAta, controller.redeemableMintPda);
        tx.add(createUserRedeemableAtaIx);
    }

    if (depository.collateralMint.equals(NATIVE_MINT)) {
        const nativeAmount = collateralAmount * 10 ** depository.collateralMintDecimals;
        const prepareWrappedSolIxs = await prepareWrappedSolTokenAccount(
            getConnection(),
            payer.publicKey,
            user.publicKey,
            nativeAmount
        );
        tx.add(...prepareWrappedSolIxs);
    } else {
        console.log("Find user ata");
        const userCollateralAta = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
        if (!await getConnection().getAccountInfo(userCollateralAta)) {
            const createUserCollateralAtaIx = createAssocTokenIx(user.publicKey, userCollateralAta, depository.collateralMint);
            console.log("will create user ata");
            tx.add(createUserCollateralAtaIx);
        }
    }

    tx.add(mintWithMangoDepositoryIx);
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

    const userCollateralAta = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
    if (!await getConnection().getAccountInfo(userCollateralAta)) {
        const createUserCollateralAtaIx = createAssocTokenIx(user.publicKey, userCollateralAta, depository.collateralMint);
        tx.add(createUserCollateralAtaIx);
    }

    const userRedeemableAta = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
    if (!await getConnection().getAccountInfo(userRedeemableAta)) {
        const createUserCollateralAtaIx = createAssocTokenIx(user.publicKey, userRedeemableAta, controller.redeemableMintPda);
        tx.add(createUserCollateralAtaIx);
    }

    tx.add(redeemFromMangoDepositoryIx);
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
        tx.add(...prepareWrappedSolIxs);
    } else {
        // TEMPORARY - Also make a WSOL account to prevent program doing it and save some computing
        const createWSOLATAIxs = await prepareWrappedSolTokenAccount(
            getConnection(),
            payer.publicKey,
            user.publicKey,
            0
        );
        tx.add(...createWSOLATAIxs);
    }

    const userCollateralAta = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
    if (!await getConnection().getAccountInfo(userCollateralAta)) {
        const createUserCollateralAtaIx = createAssocTokenIx(user.publicKey, userCollateralAta, depository.collateralMint);
        tx.add(createUserCollateralAtaIx);
    }

    tx.add(rebalanceMangoDepositoryLiteIx);
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
    const mintWithZoDepositoryIx = await uxdClient.createMintWithZoDepositoryInstruction(collateralAmount, slippage, controller, depository, zo, user.publicKey, TXN_OPTS);
    let signers = [];
    signers.push(user);
    if (payer) {
        signers.push(payer);
    }

    let tx = new Transaction();

    // Compute budget request // Not sure this works but cannot test on devnet yet
    const data = Buffer.from(
        Uint8Array.of(0, ...new BN(256000).toArray("le", 4))
    );
    const additionalComputeBudgetInstruction = new TransactionInstruction({
        keys: [],
        programId: new PublicKey("ComputeBudget111111111111111111111111111111"),
        data,
    });
    tx.add(additionalComputeBudgetInstruction);

    if (depository.collateralMint.equals(NATIVE_MINT)) {
        const nativeAmount = collateralAmount * 10 ** depository.collateralMintDecimals;
        const prepareWrappedSolIxs = await prepareWrappedSolTokenAccount(
            getConnection(),
            payer.publicKey,
            user.publicKey,
            nativeAmount
        );
        tx.add(...prepareWrappedSolIxs);
    }

    const userRedeemableAta = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
    if (!await getConnection().getAccountInfo(userRedeemableAta)) {
        const createUserRedeemableAtaIx = createAssocTokenIx(user.publicKey, userRedeemableAta, controller.redeemableMintPda);
        tx.add(createUserRedeemableAtaIx);
    }

    // if (txPre.instructions.length != 0) {
    //     txPre.feePayer = payer.publicKey;
    //     await Promise.allSettled(await web3.sendAndConfirmTransaction(getConnection(), txPre, signers, TXN_OPTS));
    // }

    tx.add(mintWithZoDepositoryIx);
    tx.feePayer = payer.publicKey;
    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

export async function redeemFromZoDepository(user: Signer, payer: Signer, slippage: number, amountRedeemable: number, controller: Controller, depository: ZoDepository, zo: Zo): Promise<string> {
    const redeemFromZoDepositoryIx = await uxdClient.createRedeemFromZoDepositoryInstruction(amountRedeemable, slippage, controller, depository, zo, user.publicKey, TXN_OPTS);

    let signers = [];
    let tx = new Transaction();
    signers.push(user);
    if (payer) {
        signers.push(payer);
    }

    // Compute budget request
    const data = Buffer.from(
        Uint8Array.of(0, ...new BN(256000).toArray("le", 4))
    );
    const additionalComputeBudgetInstruction = new TransactionInstruction({
        keys: [],
        programId: new PublicKey("ComputeBudget111111111111111111111111111111"),
        data,
    });
    tx.add(additionalComputeBudgetInstruction);

    const userCollateralAta = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
    if (!await getConnection().getAccountInfo(userCollateralAta)) {
        const createUserCollateralAtaIx = createAssocTokenIx(user.publicKey, userCollateralAta, depository.collateralMint);
        tx.add(createUserCollateralAtaIx);
    }

    const userRedeemableAta = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
    if (!await getConnection().getAccountInfo(userRedeemableAta)) {
        const createUserCollateralAtaIx = createAssocTokenIx(user.publicKey, userRedeemableAta, controller.redeemableMintPda);
        tx.add(createUserCollateralAtaIx);
    }

    tx.add(redeemFromZoDepositoryIx);
    tx.feePayer = payer.publicKey;
    return web3.sendAndConfirmTransaction(getConnection(), tx, signers, TXN_OPTS);
}

// Non UXD API calls ----------------------------------------------------------

export async function settleDepositoryPnl(payer: Signer, depository: MangoDepository, mango: Mango): Promise<string> {
    return depository.settleMangoDepositoryMangoAccountPnl(payer as Keypair, mango);
}