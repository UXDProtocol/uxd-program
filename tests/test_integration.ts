import { TOKEN_PROGRAM_ID, Token, AccountInfo } from "@solana/spl-token";
import assert from "assert";
import * as anchor from "@project-serum/anchor";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  TransactionInstruction,
} from "@solana/web3.js";

import {
  BTC_USD,
  create_localnet_oracle_mirrored_from_testnet,
  localBTCOraclePriceAccountKey,
  localSOLOraclePriceAccountKey,
  oracle,
  SOL_USD,
  testnetBTCOraclePriceAccountKey,
  testnetSOLOraclePriceAccountKey,
} from "./oracle_utils";
import { Controller } from "./controller_utils";
import { connection, wallet } from "./utils";
import { Depository } from "./depository_utils";

const TXN_COMMIT = "processed";
const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: false,
};

// Constants
const FAKE_BTC_MINT = process.env.FAKE_BTC_MINT;
const BTC_DECIMAL = 6;
const SOL_DECIMAL = 9;
const UXD_DECIMAL = 6;

// Keypairs
let payer: Keypair;
let mintAuthority: Keypair;

// Mints
let mintBTC: Token;
let mintSOL: Token;

// Depositories - They represent the business object that tie a mint to a depository
let depositoryBTC: Depository;
let depositorySOL: Depository;
// Controller
let controllerUXD: Controller;

// Accounts
let userBTCTokenAccount: PublicKey;
let userSOLTokenAccount: PublicKey;
let userUXDTokenAccount: PublicKey;

const cleanState = async () => {
  payer = anchor.web3.Keypair.generate();
  mintAuthority = anchor.web3.Keypair.generate();

  // Airdropping tokens to the payer.
  await connection.confirmTransaction(
    await connection.requestAirdrop(payer.publicKey, 10_000_000_000),
    "confirmed"
  );

  // Setup BTC mint
  mintBTC = await Token.createMint(
    connection,
    payer,
    mintAuthority.publicKey,
    null,
    BTC_DECIMAL,
    TOKEN_PROGRAM_ID
  );
  // Setup SOL mint
  mintSOL = await Token.createMint(
    connection,
    payer,
    mintAuthority.publicKey,
    null,
    SOL_DECIMAL,
    TOKEN_PROGRAM_ID
  );

  // Following can be moved to specifics tests instead of applying to all - good for now

  // create token accounts
  userBTCTokenAccount = await mintBTC.createAccount(wallet.publicKey);
  userSOLTokenAccount = await mintSOL.createAccount(wallet.publicKey);

  // mint some tokens
  await mintBTC.mintTo(
    userBTCTokenAccount,
    mintAuthority.publicKey,
    [mintAuthority],
    100
  );
  await mintSOL.mintTo(
    userSOLTokenAccount,
    mintAuthority.publicKey,
    [mintAuthority],
    100
  );
};

// Setup Mint Redeem flow
///////////////////////////////////////////////////////////////////////////////
describe("UXD full flow (WIP)", () => {
  it("Setup", async () => {
    // tabula rasa
    await cleanState();
  });

  it("Fetch testnet oracle data and deploy localnet oracle", async () => {
    // BTC
    await create_localnet_oracle_mirrored_from_testnet(
      BTC_USD,
      testnetBTCOraclePriceAccountKey,
      localBTCOraclePriceAccountKey
    );

    // SOL
    await create_localnet_oracle_mirrored_from_testnet(
      SOL_USD,
      testnetSOLOraclePriceAccountKey,
      localSOLOraclePriceAccountKey
    );
  });

  it("Setup BTC and SOL Collateral business objects ", async () => {
    depositoryBTC = new Depository(mintBTC, localBTCOraclePriceAccountKey);

    depositorySOL = new Depository(mintSOL, localSOLOraclePriceAccountKey);
  });

  it("Initializing controller", async () => {
    controllerUXD = new Controller();

    await controllerUXD.program.rpc.new({
      accounts: {
        authority: wallet.publicKey,
        state: controllerUXD.statePda,
        uxdMint: controllerUXD.uxdMintPda,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer], // Payer does exist, just a problem of discovery?
      options: TXN_OPTS,
    });

    // Add some asserts ...
  });

  it("Create BTC depository", async () => {
    await depositoryBTC.program.rpc.new(Controller.ProgramId, {
      accounts: {
        payer: wallet.publicKey,
        state: depositoryBTC.statePda,
        redeemableMint: depositoryBTC.redeemableMintPda,
        programCoin: depositoryBTC.depositPda,
        coinMint: depositoryBTC.mint.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
    });
    // Add some asserts ...
  });

  it("Create SOL depository", async () => {
    await depositorySOL.program.rpc.new(Controller.ProgramId, {
      accounts: {
        payer: wallet.publicKey,
        state: depositorySOL.statePda,
        redeemableMint: depositorySOL.redeemableMintPda,
        programCoin: depositorySOL.depositPda,
        coinMint: depositorySOL.mint.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet.payer],
      options: TXN_OPTS,
    });
    // Add some asserts ...
  });

  it("Register BTC depository with Controller", async () => {
    await controllerUXD.program.rpc.registerDepository(
      Depository.ProgramId,
      depositoryBTC.oraclePriceAccount,
      {
        accounts: {
          authority: wallet.publicKey,
          state: controllerUXD.statePda,
          depositoryRecord: controllerUXD.depositoryRecordPda(depositoryBTC),
          depositoryState: depositoryBTC.statePda,
          coinMint: depositoryBTC.mint.publicKey,
          coinPassthrough: controllerUXD.coinPassthroughPda(depositoryBTC.mint),
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [wallet.payer],
        options: TXN_OPTS,
      }
    );
    // Add some asserts ...
  });

  it("Register SOL depository with Controller", async () => {
    await controllerUXD.program.rpc.registerDepository(
      Depository.ProgramId,
      depositorySOL.oraclePriceAccount,
      {
        accounts: {
          authority: wallet.publicKey,
          state: controllerUXD.statePda,
          depositoryRecord: controllerUXD.depositoryRecordPda(depositorySOL),
          depositoryState: depositorySOL.statePda,
          coinMint: depositorySOL.mint.publicKey,
          coinPassthrough: controllerUXD.coinPassthroughPda(depositorySOL.mint),
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [wallet.payer],
        options: TXN_OPTS,
      }
    );
    // Add some asserts ...
  });

  // Keep going from index.js line 280

});
