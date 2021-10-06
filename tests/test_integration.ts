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

import * as oracle_utils from "./oracle_utils";

const TXN_COMMIT = "processed";
const TXN_OPTS = { commitment: TXN_COMMIT, preflightCommitment: TXN_COMMIT, skipPreflight: false };

// THE WALLET - Will follow the local env, you can test with any cluster
const provider = anchor.Provider.env();
anchor.setProvider(provider);

// Constants
const FAKE_BTC_MINT = process.env.FAKE_BTC_MINT;
const BTC_DECIMAL = 6;
const SOL_DECIMAL = 9;
const UXD_DECIMAL = 6;

// Programs
const depository = anchor.workspace.Depository;
const controller = anchor.workspace.Controller;
const oracle = anchor.workspace.Oracle;

// Keypairs
let payer: Keypair;
let mintAuthority: Keypair;

// Mints
let mintBTC: Token;
let mintSOL: Token;

// Accounts
let userBTCTokenAccount: PublicKey;
let userSOLTokenAccount: PublicKey;
let userUXDTokenAccount: PublicKey;
    
const cleanState = async () => {
    payer = anchor.web3.Keypair.generate();
    mintAuthority = anchor.web3.Keypair.generate();

    // Airdropping tokens to the payer.
    await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(payer.publicKey, 10_000_000_000),
        "confirmed"
    );

    // Setup BTC mint
    mintBTC = await Token.createMint(
        provider.connection,
        payer,
        mintAuthority.publicKey,
        null,
        BTC_DECIMAL,
        TOKEN_PROGRAM_ID
    );
    // Setup SOL mint
    mintSOL = await Token.createMint(
        provider.connection,
        payer,
        mintAuthority.publicKey,
        null,
        SOL_DECIMAL,
        TOKEN_PROGRAM_ID
    );

    // Following can be moved to specifics tests instead of applying to all - good for now

    // create token accounts
    userBTCTokenAccount = await mintBTC.createAccount(provider.wallet.publicKey);
    userSOLTokenAccount = await mintSOL.createAccount(provider.wallet.publicKey);

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
describe("Integration test | Setup && Mint && Redeem", () => {
    
    it("Setup", async () => {
        // tabula rasa
        await cleanState();
    });

    it("Fetch testnet oracle data and deploy localnet oracle", async () => {
        // BTC
        oracle_utils.create_localnet_oracle_mirrored_from_testnet(
            "BTCUSD",
            oracle_utils.testnetBTCOraclePriceAccountKey,
            oracle_utils.localBTCOraclePriceAccountKey,
            provider.wallet
        );

        // SOL
        oracle_utils.create_localnet_oracle_mirrored_from_testnet(
            "SOLUSD",
            oracle_utils.testnetSOLOraclePriceAccountKey,
            oracle_utils.localSOLOraclePriceAccountKey,
            provider.wallet
        );
    });

    // TODO - rewrite the index.js test here for isntance, can be split up in different parts
    // The above takes care of all the setup from the test.sh script (that was already doing what deploy.sh was doing, removing what anchor took care of)
    it("INDEX>JS", async () => {
        // TODO
    });
});

// // Fail flow A
// ///////////////////////////////////////////////////////////////////////////////
// describe("Depository ShouldFail tests", () => {
//     it("Setup", async () => {
//         // Do XYz inits
//     });

//     it("Step A", async () => {

//     });

//     it("Step B", async () => {
//     });

//     //...
// });