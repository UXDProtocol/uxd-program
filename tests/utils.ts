import * as anchor from "@project-serum/anchor";
import { NodeWallet } from "@project-serum/anchor/dist/cjs/provider";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  Token,
  AccountInfo,
} from "@solana/spl-token";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";

const provider = anchor.Provider.local();
anchor.setProvider(provider);
export const wallet = provider.wallet as NodeWallet;
export const connection = provider.connection;

export const MAINNET = "https://api.mainnet-beta.solana.com";
export const DEVNET = "https://api.devnet.solana.com";
export const TESTNET = "https://api.testnet.solana.com";
export const LOCALNET = "http://127.0.0.1:8899";

export const TXN_COMMIT = "processed";
export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: false,
};

// simple shorthand
export function findAddr(
  seeds: (Buffer | Uint8Array)[],
  programId: PublicKey
): PublicKey {
  return anchor.utils.publicKey.findProgramAddressSync(seeds, programId)[0];
}

// derives the canonical token account address for a given wallet and mint
export function findAssocTokenAddr(
  walletKey: PublicKey,
  mintKey: PublicKey
): PublicKey {
  return findAddr(
    [walletKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mintKey.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID
  );
}

// returns an instruction to create the associated account for a wallet and mint
export function createAssocIxn(
  walletKey: PublicKey,
  mintKey: PublicKey
): TransactionInstruction {
  let assocKey = findAssocTokenAddr(walletKey, mintKey);

  return new anchor.web3.TransactionInstruction({
    keys: [
      { pubkey: walletKey, isSigner: true, isWritable: true },
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

// handle the error when an account is uninitialized...
export function getTokenBalance(tokenKey: PublicKey): Promise<number> {
  return provider.connection
    .getTokenAccountBalance(tokenKey, TXN_COMMIT)
    .then((o) => o["value"]["uiAmount"])
    .catch(() => null);
}
