import { BN, Provider, setProvider, Wallet } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { TOKEN_PROGRAM_ID, Token, NATIVE_MINT, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  PublicKey,
  Connection,
  Keypair,
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
  Commitment,
  TransactionInstruction,
} from "@solana/web3.js";
import { devnetCluster, devnetGroup, Mango } from "./mango";

// Constants
export const BTC_DECIMALS = 6;
export const SOL_DECIMALS = 9;
export const UXD_DECIMALS = 6;

// aca3VWxwBeu8FTZowJ9hfSKGzntjX68EXh1N9xpE1PC
const aca3VWSeed = Uint8Array.from([
  197, 246, 88, 131, 17, 216, 175, 8, 72, 13, 40, 236, 135, 104, 59, 108, 17, 106, 164, 234, 46, 136, 171, 148, 111,
  176, 32, 136, 59, 253, 224, 247, 8, 156, 98, 175, 196, 123, 178, 151, 182, 220, 253, 138, 191, 233, 135, 182, 173,
  175, 33, 68, 162, 191, 254, 166, 133, 219, 8, 10, 17, 154, 146, 223,
]);
// Eyh77zP5b7arPtPgpnCT8vsGmq9p5Z9HHnBSeQLnAFQi
const Eyh77Seed = Uint8Array.from([
  219, 139, 131, 236, 34, 125, 165, 13, 18, 248, 93, 160, 73, 236, 214, 251, 179, 235, 124, 126, 56, 47, 222, 28, 166,
  239, 130, 126, 66, 127, 26, 187, 207, 173, 205, 133, 48, 102, 2, 219, 20, 234, 72, 102, 53, 122, 175, 166, 198, 11,
  198, 248, 59, 40, 137, 208, 193, 138, 197, 171, 147, 124, 212, 175,
]);

// Identities - both of these are wallets that exists on devnet, we clone them each time and init from the privatekey
// This is us, the UXD deployment admins // aca3VWxwBeu8FTZowJ9hfSKGzntjX68EXh1N9xpE1PC
let adminKeypair = Keypair.fromSecretKey(aca3VWSeed);
export let admin = new Wallet(adminKeypair);
console.log(`ADMIN KEY => ${admin.publicKey}`);
// This is the user //
let userKeypair = Keypair.fromSecretKey(Eyh77Seed);
export let user = new Wallet(userKeypair);
console.log(`USER KEY => ${user.publicKey}`);

// Mints cloned from devnet to interact with mango
export const USDC = new PublicKey("8FRFC6MoGGkMFQwngccyu69VnYbzykGeez7ignHVAFSN");
export const BTC = new PublicKey("3UNBZ6o52WTWwjac2kPUb4FyodhU1vFkRJheu1Sh2TvU");
export const WSOL = new PublicKey("So11111111111111111111111111111111111111112");

// Provider
export const provider = Provider.env();
setProvider(provider);
export const wallet = provider.wallet as Wallet;
export const connection = provider.connection;

// TXN prefight checks options
export const TXN_COMMIT: Commitment = "processed";
export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: false,
};

///////////////////////////////////////////////////////////////////////////////

export class Utils {
  public mango: Mango;
  private conn: Connection;
  private wallet: Wallet;
  private authority: Keypair;

  constructor(conn: Connection, funded: Wallet) {
    this.conn = conn;
    this.wallet = funded;
    this.authority = this.wallet.payer;
    this.mango = new Mango(devnetCluster, devnetGroup);
  }

  async setupMango() {
    await this.mango.setupMangoGroup();
  }

  /**
   * Create a new wallet with some initial funding.
   * @param lamports The amount of lamports to fund the wallet account with.
   * @returns The keypair for the new wallet.
   */
  async createWallet(lamports: number): Promise<Keypair> {
    const wallet = Keypair.generate();
    const fundTx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: this.wallet.publicKey,
        toPubkey: wallet.publicKey,
        lamports,
      })
    );

    await sendAndConfirmTransaction(this.conn, fundTx, [this.authority]);
    return wallet;
  }

  /**
   * Create a new token account with some initial funding.
   * @param token The token to create an account for
   * @param owner The account that should own these tokens
   * @param amount The initial amount of tokens to provide as funding
   * @returns The address for the created account
   */
  async createTokenAccount(token: Token, owner: PublicKey, amount: BN): Promise<PublicKey> {
    if (token.publicKey == NATIVE_MINT) {
      const account = await Token.createWrappedNativeAccount(
        this.conn,
        TOKEN_PROGRAM_ID,
        owner,
        this.authority,
        amount.toNumber()
      );
      return account;
    } else {
      const account = await token.createAccount(owner);
      await token.mintTo(account, this.authority, [], amount.toNumber());
      return account;
    }
  }

  /**
   * Find a program derived address
   * @param programId The program the address is being derived for
   * @param seeds The seeds to find the address
   * @returns The address found and the bump seed required
   */
  async findProgramAddress(
    programId: PublicKey,
    seeds: (HasPublicKey | ToBytes | Uint8Array | string)[]
  ): Promise<[PublicKey, number]> {
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
    return await PublicKey.findProgramAddress(seed_bytes, programId);
  }

  /**
   * Find a program derived address, but synchronous
   * @param programId The program the address is being derived for
   * @param seeds The seeds to find the address
   * @returns The address found and the bump seed required
   */
  findProgramAddressSync(
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

  /**
   * Find a associated token derived address, but synchronous
   * @param programId The program the address is being derived for
   * @param seeds The seeds to find the address
   * @returns The address found and the bump seed required
   */
  findAssocTokenAddressSync(wallet: Wallet, mintAdress: PublicKey): [PublicKey, number] {
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
}

///////////////////////////////////////////////////////////////////////////////

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

export function getBalance(tokenAccount: PublicKey): Promise<number> {
  return provider.connection
    .getTokenAccountBalance(tokenAccount, TXN_COMMIT)
    .then((o) => o["value"]["uiAmount"])
    .catch(() => null);
}

interface ToBytes {
  toBytes(): Uint8Array;
}

interface HasPublicKey {
  publicKey: PublicKey;
}

export const utils = new Utils(connection, wallet);
