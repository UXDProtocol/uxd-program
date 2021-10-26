import { TOKEN_PROGRAM_ID, Token, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { PublicKey, Connection, Commitment, TransactionInstruction } from "@solana/web3.js";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { Wallet } from "@project-serum/anchor";

// Constants
export const BTC_DECIMALS = 6;
export const SOL_DECIMALS = 9;
export const UXD_DECIMALS = 6;

// returns an instruction to create the associated account for a wallet and mint
export function createAssocTokenIx(wallet: PublicKey, account: PublicKey, mint: PublicKey): TransactionInstruction {
  return Token.createAssociatedTokenAccountInstruction(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    mint,
    account,
    wallet, // owner
    wallet // payer
  );
}

export function findPDA(
  programId: PublicKey,
  seeds: (HasPublicKey | ToBytes | Uint8Array | string)[]
): [PublicKey, number] {
  const seed_bytes = seeds.map((s) => {
    if (typeof s == "string") {
      return Buffer.from(s);
    } else if ("publicKey" in s) {
      return s.publicKey.toBytes();
    } else if ("toBytes" in s) {
      return s.toBytes();
    } else {
      return s;
    }
  });
  return findProgramAddressSync(seed_bytes, programId);
}

export function findAssocTokenAddressSync(wallet: Wallet, mintAdress: PublicKey): [PublicKey, number] {
  const seeds = [wallet, TOKEN_PROGRAM_ID, mintAdress];
  const seed_bytes = seeds.map((s) => {
    if (typeof s == "string") {
      return Buffer.from(s);
    } else if ("publicKey" in s) {
      return s.publicKey.toBytes();
    } else if ("toBytes" in s) {
      return s.toBytes();
    } else {
      return s;
    }
  });
  return findProgramAddressSync(seed_bytes, ASSOCIATED_TOKEN_PROGRAM_ID);
}

export function getBalance(connection: Connection, tokenAccount: PublicKey, commitment?: Commitment): Promise<number> {
  return connection
    .getTokenAccountBalance(tokenAccount, commitment)
    .then((o) => o["value"]["uiAmount"])
    .catch(() => null);
}

interface ToBytes {
  toBytes(): Uint8Array;
}

interface HasPublicKey {
  publicKey: PublicKey;
}
