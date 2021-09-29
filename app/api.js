"use strict";

const anchor = require("@project-serum/anchor");
const spl = require("@solana/spl-token");

const controllerIdl = require("../target/idl/controller.json");
const depositoryIdl = require("../target/idl/depository.json");

const DEVNET = "https://api.devnet.solana.com";
// XXX temporary test token on devnet. ultimately we want to target btc/eth/sol
const TEST_COIN_MINT = "7g96eHEa1QhjGMsApCVoUMH28fkh6Yv9AFCeZLZWfqza";
const testCoinMintKey = new anchor.web3.PublicKey(TEST_COIN_MINT); 

// real constants we intend to keep
// TODO the addresses are probably in libraries tho check that later
const TXN_COMMIT = "confirmed";
const TXN_OPTS = {commitment: TXN_COMMIT, preflightCommitment: TXN_COMMIT, skipPreflight: false};
const TOKEN_PROGRAM_ID = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ASSOC_TOKEN_PROGRAM_ID = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

// static keys with no dependencies
const tokenProgramKey = new anchor.web3.PublicKey(TOKEN_PROGRAM_ID);
const assocTokenProgramKey = new anchor.web3.PublicKey(ASSOC_TOKEN_PROGRAM_ID);

// controller program
const controllerKey = new anchor.web3.PublicKey(controllerIdl.metadata.address);
const controller    = new anchor.Program(controllerIdl, controllerKey, new anchor.Provider(null, null, null));

// controller derived keys
// XXX im not 100% on making the program create the uxd mint on mainnet, maybe it should be separate
const controlStateKey    = findAddr([Buffer.from("STATE")], controllerKey);
const uxdMintKey         = findAddr([Buffer.from("STABLECOIN")], controllerKey);

// depository programs and derived keys
// record key is technically controller but maps to depository
// XXX we cant just pull the address from the idl once we have multiple
// need to store privkeys somewhere and hardcode the pubkeys in files... so ugly ugh
const testDepositoryKey = new anchor.web3.PublicKey(depositoryIdl.metadata.address);
const depositories = {};
depositories[TEST_COIN_MINT] = {
    key: testDepositoryKey,
    program: new anchor.Program(depositoryIdl, testDepositoryKey, new anchor.Provider(null, null, null)),
    stateKey: findAddr([Buffer.from("STATE")], testDepositoryKey),
    redeemableMintKey: findAddr([Buffer.from("REDEEMABLE")], testDepositoryKey),
    depositAccountKey: findAddr([Buffer.from("DEPOSIT")], testDepositoryKey),
    recordKey: findAddr([Buffer.from("RECORD"), testDepositoryKey.toBuffer()], controllerKey),
    oracleKey: new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J"),
};

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

// returns a one-instruction array to create the associated account for a wallet and mint
// frontend should check if such an account exists and create it for the user if not
// i have no opinions about how frontend should handle the case if a user has non-canonical account(s)
// actually i lied about having no opinions, frontend should check for and pull from non-canon accounts for deposits
// but always withdraw to the canonical account for withdrawals
function createAssociatedTokenAccount(walletKey, mintKey) {
    let assocKey = findAssociatedTokenAddress(walletKey, mintKey);

    return [new anchor.web3.TransactionInstruction({
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
    })];
}

// returns an two-instruction array for depositing funds and minting uxd
// coinAmount should be a bn with the appropriate number of zeros (ie we dont normalize here)
// userCoinKey is optional and defaults to the associated account
// redeemable and uxd are always associated because we create these on behalf of the user
function mintUxd(walletKey, coinMintKey, coinAmount, userCoinKey) {
    let d = depositories[coinMintKey.toString()];
    if(!d) throw `no depository found for mint ${coinMintKey.toString()}`;

    if(!userCoinKey) userCoinKey = findAssociatedTokenAddress(walletKey, coinMintKey);
    let userRedeemableKey = findAssociatedTokenAddress(walletKey, d.redeemableMintKey);
    let userUxdKey = findAssociatedTokenAddress(walletKey, uxdMintKey);
    let coinPassthroughKey = findAddr([Buffer.from("PASSTHROUGH"), coinMintKey.toBuffer()], controllerKey);

    let depositIxn = d.program.instruction.deposit(coinAmount, {
        accounts: {
            user: walletKey,
            state: d.stateKey,
            programCoin: d.depositAccountKey,
            redeemableMint: d.redeemableMintKey,
            userCoin: userCoinKey,
            userRedeemable: userRedeemableKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: tokenProgramKey,
            program: d.key,
    }});

    let mintIxn = controller.instruction.mintUxd(coinAmount, {
        accounts: {
            user: walletKey,
            state: controlStateKey,
            depository: d.key,
            depositoryRecord: d.recordKey,
            depositoryState: d.stateKey,
            depositoryCoin: d.depositAccountKey,
            coinMint: coinMintKey,
            coinPassthrough: coinPassthroughKey,
            redeemableMint: d.redeemableMintKey,
            userRedeemable: userRedeemableKey,
            userUxd: userUxdKey,
            uxdMint: uxdMintKey,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: tokenProgramKey,
            program: controllerKey,
            // XXX FIXME temp
            oracle: d.oracleKey,
    }});

    return [depositIxn, mintIxn];
}

// returns a two-instruction array for burning uxd and withdrawing backing collateral
// uxdAmount is a dollar amount, again as a proper bn. otherwise args are the same
// frontend should check that the associated token account exists and make the user create it if not
function redeemUxd(walletKey, coinMintKey, uxdAmount) {
    let d = depositories[coinMintKey.toString()];
    if(!d) throw `no depository found for mint ${coinMintKey.toString()}`;

    let userCoinKey = findAssociatedTokenAddress(walletKey, coinMintKey);
    let userRedeemableKey = findAssociatedTokenAddress(walletKey, d.redeemableMintKey);
    let userUxdKey = findAssociatedTokenAddress(walletKey, uxdMintKey);
    let coinPassthroughKey = findAddr([Buffer.from("PASSTHROUGH"), coinMintKey.toBuffer()], controllerKey);

    let redeemIxn = controller.instruction.redeemUxd(uxdAmount, {
        accounts: {
            user: walletKey,
            state: controlStateKey,
            depository: d.key,
            depositoryRecord: d.recordKey,
            depositoryState: d.stateKey,
            depositoryCoin: d.depositAccountKey,
            coinMint: coinMintKey,
            coinPassthrough: coinPassthroughKey,
            redeemableMint: d.redeemableMintKey,
            userRedeemable: userRedeemableKey,
            userUxd: userUxdKey,
            uxdMint: uxdMintKey,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: tokenProgramKey,
            program: controllerKey,
            // XXX FIXME temp
            oracle: d.oracleKey,
    }});

    let withdrawIxn = d.program.instruction.withdraw(null, {
        accounts: {
            user: walletKey,
            state: d.stateKey,
            programCoin: d.depositAccountKey,
            redeemableMint: d.redeemableMintKey,
            userCoin: userCoinKey,
            userRedeemable: userRedeemableKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: tokenProgramKey,
            program: d.key,
    }});

    return [redeemIxn, withdrawIxn];
}

module.exports = {
    TEST_COIN_MINT: TEST_COIN_MINT,
    depositories: depositories,
    connect: connect,
    findAssociatedTokenAddress: findAssociatedTokenAddress,
    createAssociatedTokenAccount: createAssociatedTokenAccount,
    mintUxd: mintUxd,
    redeemUxd: redeemUxd,
};
