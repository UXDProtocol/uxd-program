"use strict";

/* XXX unnodejsify, this cjs shit is too smart for me idk
import * as anchor from "./node_modules/@project-serum/anchor";
import * as spl from "../node_modules/@solana/spl-token/lib/index.cjs.js";
import * as controllerIdlSpec from "./node_modules/controller.json";
import * as depositoryIdlSpec from "./node_modules/depository.json";
*/

const anchor = require("@project-serum/anchor");
const spl = require("@solana/spl-token");

// XXX this is temporary until their dns is fixed
const DEVNET = "http://128.0.113.156";
// XXX temporary test token on devnet. ultimately we want to target btc/eth/sol
const TEST_COIN_MINT = "GCyuZvK4RbemLBCqQgaAWFFt2TEDixXQBgC5PH42iKCW";
const testCoinMintKey = new anchor.web3.PublicKey(TEST_COIN_MINT); 

// real constants we intend to keep
// TODO the addresses are probably in libraries tho check that later
const TXN_COMMIT = "confirmed";
const TXN_OPTS = {commitment: TXN_COMMIT, preflightCommitment: TXN_COMMIT, skipPreflight: false};
const TOKEN_PROGRAM_ID = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ASSOC_TOKEN_PROGRAM_ID = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

// controller program
// XXX unnodejsify
const controller = anchor.workspace.Controller;

// depository program
// TODO we actually need several depository programs depending on allowed mints, sort this after tho
// XXX unnodejsify
const depository = anchor.workspace.Depository;

// static keys with no dependencies
const tokenProgramKey = new anchor.web3.PublicKey(TOKEN_PROGRAM_ID);
const assocTokenProgramKey = new anchor.web3.PublicKey(ASSOC_TOKEN_PROGRAM_ID);

// controller derived keys
// XXX im not 100% on making the program create the uxd mint on mainnet, maybe it should be separate
const controlStateKey    = findAddr([Buffer.from("STATE")], controllerKey);
const uxdMintKey         = findAddr([Buffer.from("STABLECOIN")], controllerKey);
const depositRecordKey   = findAddr([Buffer.from("RECORD"), depositoryKey.toBuffer()], controllerKey);

// depository derived keys
const depositStateKey   = findAddr([Buffer.from("STATE")], depositoryKey);
const redeemableMintKey = findAddr([Buffer.from("REDEEMABLE")], depositoryKey);
const depositAccountKey = findAddr([Buffer.from("DEPOSIT")], depositoryKey);
// TODO each depository will have a hardcoded oracle on devnet
const btcOracleDevnetKey = new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");

// simple shorthand
function findAddr(seeds, programId) {
    return anchor.utils.publicKey.findProgramAddressSync(seeds, programId)[0];
}

// establish a devnet connection, returning the connection object
// transactions must be assembled and signed by the wallet, but this can be done with vanilla solana if desired
function connect() {
    let conn = new anchor.web3.Connection(DEVNET, TXN_OPTS);
    anchor.setProvider(new anchor.Provider(conn, null, null));

    return conn;
}

// derives the canonical token account address for a given wallet and mint
function findAssociatedTokenAddress(walletKey, mintKey) {
    return findAddr([walletKey.toBuffer(), tokenProgramKey.toBuffer(), mintKey.toBuffer()], assocTokenProgramKey);
}

// returns an instruction to create the associated account for a wallet and mint
// frontend should check if such an account exists and create it for the user if not
// i have no opinions about how frontend should handle the case if a user has non-canonical account(s)
// actually i lied about having no opinions, frontend should check for and pull from non-canon accounts for deposits
// but always withdraw to the canonical account for withdrawals
function createAssociatedTokenAccount(walletKey, mintKey) {
    let assocKey = findAssociatedTokenAddress(walletKey, mintKey);

    return new anchor.web3.TransactionInstruction({
        keys: [
            {pubkey: walletKey, isSigner: true, isWritable: true},
            {pubkey: assocKey, isSigner: false, isWritable: true},
            {pubkey: walletKey, isSigner: false, isWritable: false},
            {pubkey: mintKey, isSigner: false, isWritable: false},
            {pubkey: anchor.web3.SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: tokenProgramKey, isSigner: false, isWritable: false},
            {pubkey: anchor.web3.SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
        ],
        programId: assocTokenProgramKey,
        data: Buffer.alloc(0),
    });
}

// returns an two-instruction array for depositing funds and minting uxd
// coinAmount should be a bn with the appropriate number of zeros (ie we dont normalize here)
// userCoinKey is optional and defaults to the associated account
// redeemable and uxd are always associated because we create these on behalf of the user
function mintUxd(walletKey, coinMintKey, coinAmount, userCoinKey) {
    if(!userCoinKey) userCoinKey = findAssociatedTokenAddress(walletKey, coinMintKey);
    let userRedeemableKey = findAssociatedTokenAddress(walletKey, redeemableMintKey);
    let userUxdKey = findAssociatedTokenAddress(walletKey, uxdMintKey);
    let coinPassthroughKey = findAddr([Buffer.from("PASSTHROUGH"), coinMintKey.toBuffer()], controllerKey);

    let depositIxn = depository.instruction.deposit(coinAmount, {
        accounts: {
            user: walletKey,
            state: depositStateKey,
            programCoin: depositAccountKey,
            redeemableMint: redeemableMintKey,
            userCoin: userCoinKey,
            userRedeemable: userRedeemableKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: tokenProgramKey,
            program: depositoryKey,
    }});

    let mintIxn = controller.instruction.mintUxd(coinAmount, {
        accounts: {
            user: walletKey,
            state: controlStateKey,
            depository: depositoryKey,
            depositoryRecord: depositRecordKey,
            depositoryState: depositStateKey,
            depositoryCoin: depositAccountKey,
            coinMint: coinMintKey,
            coinPassthrough: coinPassthroughKey,
            redeemableMint: redeemableMintKey,
            userRedeemable: userRedeemableKey,
            userUxd: userUxdKey,
            uxdMint: uxdMintKey,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: tokenProgramKey,
            program: controllerKey,
            // XXX FIXME temp
            programCoin: depositAccountKey,
            oracle: btcOracleDevnetKey,
    }});

    return [depositIxn, mintIxn];
}

// returns a two-instruction array for burning uxd and withdrawing backing collateral
// uxdAmount is a dollar amount, again as a proper bn. otherwise args are the same
// frontend should check that the associated token account exists and make the user create it if not
function redeemUxd(walletKey, coinMintKey, uxdAmount) {
    let userCoinKey = findAssociatedTokenAddress(walletKey, coinMintKey);
    let userRedeemableKey = findAssociatedTokenAddress(walletKey, redeemableMintKey);
    let userUxdKey = findAssociatedTokenAddress(walletKey, uxdMintKey);
    let coinPassthroughKey = findAddr([Buffer.from("PASSTHROUGH"), coinMintKey.toBuffer()], controllerKey);

    let redeemIxn = controller.instruction.redeemUxd(uxdAmount, {
        accounts: {
            user: walletKey,
            state: controlStateKey,
            depository: depositoryKey,
            depositoryRecord: depositRecordKey,
            depositoryState: depositStateKey,
            depositoryCoin: depositAccountKey,
            coinMint: coinMintKey,
            coinPassthrough: coinPassthroughKey,
            redeemableMint: redeemableMintKey,
            userRedeemable: userRedeemableKey,
            userUxd: userUxdKey,
            uxdMint: uxdMintKey,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: tokenProgramKey,
            program: controllerKey,
            // XXX FIXME temp
            programCoin: depositAccountKey,
            oracle: btcOracleDevnetKey,
    }});

    let withdrawIxn = depository.instruction.withdraw(null, {
        accounts: {
            user: walletKey,
            state: depositStateKey,
            programCoin: depositAccountKey,
            redeemableMint: redeemableMintKey,
            userCoin: userCoinKey,
            userRedeemable: userRedeemableKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: tokenProgramKey,
            program: depositoryKey,
    }});

    return [redeemIxn, withdrawIxn];
}

// XXX unnodejsify
//export { TEST_COIN_MINT, connect, findAssociatedTokenAddress, createAssociatedTokenAccount}
module.exports = {
    TEST_COIN_MINT: TEST_COIN_MINT,
    connect: connect,
    findAssociatedTokenAddress: findAssociatedTokenAddress,
    createAssociatedTokenAccount: createAssociatedTokenAccount,
    mintUxd: mintUxd,
    redeemUxd: redeemUxd,
};
