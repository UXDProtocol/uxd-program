import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Connection, sendAndConfirmTransaction, Signer, Transaction } from "@solana/web3.js";
import { findATAAddrSync, MercurialVaultDepository, uiToNative } from "@uxd-protocol/uxd-client";
import { getConnection, TXN_OPTS } from "./connection";

export async function mercurialVaultNativeDeposit({
    connection,
    user,
    tokenAmount,
    minimumLpTokenAmount,
    mercurialVaultDepository,
}: {
    connection: Connection;
    user: Signer;
    tokenAmount: number;
    minimumLpTokenAmount: number;
    mercurialVaultDepository: MercurialVaultDepository;
}) {
    const [userToken] = findATAAddrSync(user.publicKey, mercurialVaultDepository.collateralMint.mint);
    const [userLp] = findATAAddrSync(user.publicKey, mercurialVaultDepository.mercurialVaultLpMint.mint);

    const nativeTokenAmount = uiToNative(tokenAmount, mercurialVaultDepository.collateralMint.decimals);
    const nativeMinimumLpTokenAmount = uiToNative(minimumLpTokenAmount, mercurialVaultDepository.mercurialVaultLpMint.decimals);

    const transaction = await mercurialVaultDepository.mercurialVaultProgram.methods.deposit(nativeTokenAmount, nativeMinimumLpTokenAmount).accounts({
        vault: mercurialVaultDepository.mercurialVault,
        tokenVault: mercurialVaultDepository.mercurialVaultCollateralTokenSafe,
        lpMint: mercurialVaultDepository.mercurialVaultLpMint.mint,
        userToken,
        userLp,
        user: user.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
    }).transaction();

    transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    transaction.feePayer = user.publicKey;
    transaction.sign(user);

    const txId = await sendAndConfirmTransaction(connection, transaction, [user], TXN_OPTS);

    console.log(`Mercurial vault native deposit: 'https://explorer.solana.com/tx/${txId}?cluster=devnet'`);
}

export async function transferLpTokenToDepositoryLpVault({
    amount,
    depository,
    payer,
}: {
    amount: number;
    depository: MercurialVaultDepository;
    payer: Signer;
}) {
    const token = new Token(getConnection(), depository.mercurialVaultLpMint.mint, TOKEN_PROGRAM_ID, payer);
    const sender = await token.getOrCreateAssociatedAccountInfo(payer.publicKey);

    const transferTokensIx = Token.createTransferInstruction(
        TOKEN_PROGRAM_ID,
        sender.address,
        depository.depositoryLpTokenVault,
        payer.publicKey,
        [],
        uiToNative(amount, depository.mercurialVaultLpMint.decimals).toNumber()
    );

    const transaction = new Transaction().add(transferTokensIx);

    return sendAndConfirmTransaction(getConnection(), transaction, [payer], TXN_OPTS);
}