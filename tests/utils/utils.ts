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
import { PythUtils } from "./pyth";

// Constants
export const BTC_DECIMAL = 6;
export const SOL_DECIMAL = 9;
export const UXD_DECIMAL = 6;

// Provider from the Anchor.toml provider variable
export const provider = Provider.env();
setProvider(provider);
export const wallet = provider.wallet as Wallet;
export const connection = provider.connection;

// // CLUSTERS
// export const MAINNET = "https://api.mainnet-beta.solana.com";
// export const DEVNET = "https://api.devnet.solana.com";
// export const TESTNET = "https://api.testnet.solana.com";
// export const LOCALNET = "https://api.testnet.solana.com";

// TXN prefight checks options
export const TXN_COMMIT: Commitment = "processed";
export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: false,
};

///////////////////////////////////////////////////////////////////////////////

export class TestToken extends Token {
  decimals: number;

  constructor(conn: Connection, token: Token, decimals: number) {
    super(conn, token.publicKey, token.programId, token.payer);
    this.decimals = decimals;
  }

  /**
   * Convert a token amount to the integer format for the mint
   * @param token The token mint
   * @param amount The amount of tokens
   */
  amount(amount: BN | number): BN {
    if (typeof amount == "number") {
      amount = new BN(amount);
    }

    const one_unit = new BN(10).pow(new BN(this.decimals));
    return amount.mul(one_unit);
  }
}

///////////////////////////////////////////////////////////////////////////////

export class Utils {
  public static readonly pythProgramId = PythUtils.programId;

  public pyth: PythUtils;
  public mango: Mango;
  private conn: Connection;
  private wallet: Wallet;
  private authority: Keypair;

  constructor(conn: Connection, funded: Wallet) {
    this.conn = conn;
    this.wallet = funded;
    this.authority = this.wallet.payer;
    this.pyth = new PythUtils(conn, funded);
    this.mango = new Mango(devnetCluster, devnetGroup);
  }

  async setupMango() {
    await this.mango.setupMangoGroup();
  }

  /**
   * Create a new SPL token
   * @param decimals The number of decimals for the token.
   * @param authority The account with authority to mint/freeze tokens.
   * @returns The new token
   */
  async createToken(decimals: number, authority: PublicKey = this.authority.publicKey): Promise<TestToken> {
    const token = await Token.createMint(this.conn, this.authority, authority, authority, decimals, TOKEN_PROGRAM_ID);

    return new TestToken(this.conn, token, decimals);
  }

  async createNativeToken(decimals: number = 9) {
    const token = new Token(this.conn, NATIVE_MINT, TOKEN_PROGRAM_ID, this.authority);

    return new TestToken(this.conn, token, decimals);
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

export async function createTokenEnv(decimals: number, price: bigint) {
  let pythPrice = await utils.pyth.createPriceAccount();
  let pythProduct = await utils.pyth.createProductAccount();

  await utils.pyth.updatePriceAccount(pythPrice, {
    exponent: -9,
    aggregatePriceInfo: {
      price: price * 1000000000n,
    },
  });
  await utils.pyth.updateProductAccount(pythProduct, {
    priceAccount: pythPrice.publicKey,
    attributes: {
      quote_currency: "USD",
    },
  });

  return {
    token: await utils.createToken(decimals),
    pythPrice,
    pythProduct,
  } as TokenEnv;
}
export interface TokenEnv {
  token: TestToken;
  pythPrice: Keypair;
  pythProduct: Keypair;
}

///////////////////////////////////////////////////////////////////////////////

export async function createTestUser(assets: Array<TokenEnv>): Promise<TestUser> {
  const userWallet = wallet.payer; //await testUtils.createWallet(1 * LAMPORTS_PER_SOL); I WISH TO use that.... but idk it doesn't sign
  // I think it would be neat to have 2 wallets to encure things are tighs, to not have only one GOD wallet that's also the user

  const createUserTokens = async (asset: TokenEnv) => {
    const tokenAccount = await asset.token.getOrCreateAssociatedAccountInfo(userWallet.publicKey);
    return tokenAccount.address;
  };

  let tokenAccounts: Record<string, PublicKey> = {};
  for (const asset of assets) {
    tokenAccounts[asset.token.publicKey.toBase58()] = await createUserTokens(asset);
  }

  return {
    wallet: userWallet,
    tokenAccounts,
  };
}
export interface TestUser {
  wallet: Keypair;
  tokenAccounts: Record<string, PublicKey>;
}

///////////////////////////////////////////////////////////////////////////////

// returns an instruction to create the associated account for a wallet and mint
export function createAssocTokenIx(wallet: PublicKey, account: PublicKey, mint: PublicKey): TransactionInstruction {
  return Token.createAssociatedTokenAccountInstruction(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    mint, // reedeemable mint PDA
    account, // user's reedeemable associated token account
    wallet, // owner
    wallet // payer
  );
}

export async function getRentExemption(size: number): Promise<number> {
  return await provider.connection.getMinimumBalanceForRentExemption(size, TXN_COMMIT);
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
