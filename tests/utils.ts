import { SOL_DECIMALS, findATAAddrSync, nativeToUi, uiToNative, MaplePoolDepository } from "@uxd-protocol/uxd-client";
import { PublicKey, Signer } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import { ASSOCIATED_TOKEN_PROGRAM_ID, NATIVE_MINT, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { getConnection, TXN_COMMIT, TXN_OPTS } from "./connection";
import { MAPLE_USDC_DEVNET, uxdProgramId } from "./constants";

const SOLANA_FEES_LAMPORT: number = 1238880;

export function ceilAtDecimals(number: number, decimals: number): number {
  return Number((Math.ceil(number * 10 ** decimals) / 10 ** decimals).toFixed(decimals));
}

export async function transferSol(amountUi: number, from: Signer, to: PublicKey): Promise<string> {
  const transaction = new anchor.web3.Transaction().add(
    anchor.web3.SystemProgram.transfer({
      fromPubkey: from.publicKey,
      toPubkey: to,
      lamports: anchor.web3.LAMPORTS_PER_SOL * amountUi,
    })
  );
  return await anchor.web3.sendAndConfirmTransaction(getConnection(), transaction, [from], TXN_OPTS);
}

export async function transferAllSol(from: Signer, to: PublicKey): Promise<string> {
  const fromBalance = await getSolBalance(from.publicKey);
  const transaction = new anchor.web3.Transaction().add(
    anchor.web3.SystemProgram.transfer({
      fromPubkey: from.publicKey,
      toPubkey: to,
      lamports: anchor.web3.LAMPORTS_PER_SOL * fromBalance - SOLANA_FEES_LAMPORT,
    })
  );
  return anchor.web3.sendAndConfirmTransaction(getConnection(), transaction, [from], TXN_OPTS);
}

export async function transferTokens(
  amountUI: number,
  mint: PublicKey,
  decimals: number,
  from: Signer,
  to: PublicKey
): Promise<string> {
  const token = new Token(getConnection(), mint, TOKEN_PROGRAM_ID, from);
  const sender = await token.getOrCreateAssociatedAccountInfo(from.publicKey);
  const receiver = await token.getOrCreateAssociatedAccountInfo(to);
  const transferTokensIx = Token.createTransferInstruction(
    TOKEN_PROGRAM_ID,
    sender.address,
    receiver.address,
    from.publicKey,
    [],
    uiToNative(amountUI, decimals).toNumber()
  );
  const transaction = new anchor.web3.Transaction().add(transferTokensIx);
  return anchor.web3.sendAndConfirmTransaction(getConnection(), transaction, [from], TXN_OPTS);
}

export async function transferAllTokens(
  mint: PublicKey,
  decimals: number,
  from: Signer,
  to: PublicKey
): Promise<string> {
  const sender = findATAAddrSync(from.publicKey, mint)[0];
  if (!(await getConnection().getAccountInfo(sender))) {
    return "No account";
  }
  const token = new Token(getConnection(), mint, TOKEN_PROGRAM_ID, from);
  const receiver = await token.getOrCreateAssociatedAccountInfo(to);
  const amount = await getBalance(sender);
  const transferTokensIx = Token.createTransferInstruction(
    TOKEN_PROGRAM_ID,
    sender,
    receiver.address,
    from.publicKey,
    [],
    uiToNative(amount, decimals).toNumber()
  );
  const transaction = new anchor.web3.Transaction().add(transferTokensIx);
  return anchor.web3.sendAndConfirmTransaction(getConnection(), transaction, [from], TXN_OPTS);
}

export async function getSolBalance(wallet: PublicKey): Promise<number> {
  const lamports = await getConnection().getBalance(wallet, TXN_COMMIT);
  return nativeToUi(lamports, SOL_DECIMALS);
}

export async function getBalance(tokenAccount: PublicKey): Promise<number> {
  try {
    const o = await getConnection().getTokenAccountBalance(tokenAccount, TXN_COMMIT);
    return o["value"]["uiAmount"];
  } catch {
    return 0;
  }
}

export const prepareWrappedSolTokenAccount = async (connection, payerKey, userKey, amountNative) => {
  const wsolTokenKey = findAssociatedTokenAddress(userKey, NATIVE_MINT);
  const tokenAccount = await connection.getParsedAccountInfo(wsolTokenKey);
  if (tokenAccount.value) {
    const balanceNative = Number(tokenAccount.value.data.parsed.info.tokenAmount.amount);
    if (balanceNative < amountNative) {
      return [
        transferSolItx(userKey, wsolTokenKey, amountNative - balanceNative),
        // @ts-expect-error not sure why but it's not in their interface
        Token.createSyncNativeInstruction(TOKEN_PROGRAM_ID, wsolTokenKey),
      ];
    } else {
      // no-op we have everything we need
    }
  } else {
    return createWrappedSolTokenAccount(connection, payerKey, userKey, amountNative);
  }
  return [];
};

// derives the canonical token account address for a given wallet and mint
function findAssociatedTokenAddress(walletKey, mintKey) {
  if (!walletKey || !mintKey) return;
  return findAddr([walletKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mintKey.toBuffer()], ASSOCIATED_TOKEN_PROGRAM_ID);
}

// simple shorthand
function findAddr(seeds, programId) {
  return anchor.utils.publicKey.findProgramAddressSync(seeds, programId)[0];
}

/**
 *
 * @param {anchor.web3.PublicKey} fromKey
 * @param {anchor.web3.PublicKey} toKey
 * @param {number} amountNative
 * @returns {anchor.web3.TransactionInstruction}
 */
const transferSolItx = (fromKey, toKey, amountNative) =>
  anchor.web3.SystemProgram.transfer({
    fromPubkey: fromKey,
    toPubkey: toKey,
    lamports: amountNative,
  });

const createWrappedSolTokenAccount = async (connection, payerKey, userKey, amountNative = 0) => {
  const assocTokenKey = findAssociatedTokenAddress(userKey, NATIVE_MINT);
  const balanceNeeded = await Token.getMinBalanceRentForExemptAccount(connection);

  const transferItx = transferSolItx(userKey, assocTokenKey, amountNative + balanceNeeded);
  const createItx = createAssociatedTokenAccountItx(payerKey, userKey, NATIVE_MINT);

  return [transferItx, createItx];
};

export function createAssociatedTokenAccountItx(payerKey, walletKey, mintKey) {
  const assocKey = findAssociatedTokenAddress(walletKey, mintKey);

  return new anchor.web3.TransactionInstruction({
    keys: [
      { pubkey: payerKey, isSigner: true, isWritable: true },
      { pubkey: assocKey, isSigner: false, isWritable: true },
      { pubkey: walletKey, isSigner: false, isWritable: false },
      { pubkey: mintKey, isSigner: false, isWritable: false },
      {
        pubkey: anchor.web3.SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      {
        pubkey: anchor.web3.SYSVAR_RENT_PUBKEY,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: ASSOCIATED_TOKEN_PROGRAM_ID,
    data: Buffer.alloc(0),
  });
}

export async function createMaplePoolDepositoryDevnetUSDC() {
  return await MaplePoolDepository.initialize({
    connection: getConnection(),
    uxdProgramId: uxdProgramId,
    syrupProgramId: new PublicKey("5D9yi4BKrxF8h65NkVE1raCCWFKUs5ngub2ECxhvfaZe"),
    collateralMint: new PublicKey("Doe9rajhwt18aAeaVe8vewzAsBk4kSQ2tTyZVUJhHjhY"),
    collateralSymbol: "USDC(MapleDevnet)",
    mapleGlobals: new PublicKey("BDMBzwZEisVTTJzd9HTFsEfHMFFtXqoNjyRtz1Sp6zKP"),
    maplePool: new PublicKey("FfTKtBGj3F6nRXQiWVqqyw1Z2XVz2icaqLnUFJC4Fzqm"),
    maplePoolLocker: new PublicKey("Gq7sVXvEEKPNapNF2PSGEyy7GmiyJyNRq5LbdenAGdWY"),
    mapleSharesMint: new PublicKey("8HvMWzFnmZxLsoNwUzj4fqwLmeu7JPgYkgUpUkBtKWue"),
  });
}
