"use strict";

/*
depository: tzK6m4Uswcqj4ZQFXeKxPxZnneJAFLgtTGvpQGquVA5
controller: 2jca3qi8Yb4mwCXTe4W9ukDYcLoUE8pRfLwENyKvA99k
uxd: B1dXqAh9h648sXgwK2S4DUSiEWSeYqRcCcHFazCrSgVa
fake btc: 7g96eHEa1QhjGMsApCVoUMH28fkh6Yv9AFCeZLZWfqza
*/

const anchor = require("@project-serum/anchor");
const spl = require("@solana/spl-token");

// const DEVNET = //"https://api.devnet.solana.com";
const CLUSTER = "http://127.0.0.1:8899";
// XXX temporary test token on devnet. ultimately we want to target btc/eth/sol
// const TEST_COIN_MINT = "7g96eHEa1QhjGMsApCVoUMH28fkh6Yv9AFCeZLZWfqza";
const TEST_COIN_MINT = process.env.COIN_MINT;
// const testCoinMintKey = new anchor.web3.PublicKey(TEST_COIN_MINT); 

// XXX temp?
const provider = anchor.Provider.local();
anchor.setProvider(provider);

// real constants we intend to keep
// TODO the addresses are probably in libraries tho check that later
const TXN_COMMIT = "confirmed";
const TXN_OPTS = {commitment: TXN_COMMIT, preflightCommitment: TXN_COMMIT, skipPreflight: false};

// static keys with no dependencies
const TOKEN_PROGRAM_ID = new anchor.web3.PublicKey(spl.TOKEN_PROGRAM_ID.toBase58());
const ASSOC_TOKEN_PROGRAM_ID = new anchor.web3.PublicKey(spl.ASSOCIATED_TOKEN_PROGRAM_ID.toBase58());

// Programs
const controller = anchor.workspace.Controller;
const depository = anchor.workspace.Depository;
const oracle = anchor.workspace.Oracle;

// controller derived keys
// XXX im not 100% on making the program create the uxd mint on mainnet, maybe it should be separate
const controlStateKey    = findAddr([Buffer.from("STATE")], controller.programId);
const uxdMintKey         = findAddr([Buffer.from("STABLECOIN")], controller.programId);

// depository programs and derived keys
// record key is technically controller but maps to depository
// XXX we cant just pull the address from the idl once we have multiple
// need to store privkeys somewhere and hardcode the pubkeys in files... so ugly ugh
//XXX const testDepositoryKey = new anchor.web3.PublicKey(depositoryIdl.metadata.address);
//
// (alex) From what I understand the depository program is used to interact with different depository's
// states, one of them being the one below. Why do we need to bother with private keys storage? We would
// have a initiator wallet that does instantiate several depositories for each collaterals, and keep track of which
// state pubkey for which collateral type, and use permisionless-ly by front end? Or maybe I miss something
// !! Maybe what's missing is to add the Token Mint to the hashing to these? :
//    let depositStateKey = findAddr([Buffer.from("STATE")], depository.programId);

const coinMintKey = new anchor.web3.PublicKey(TEST_COIN_MINT);

// const testCoinMintDepositoryKey = new anchor.web3.PublicKey("UXDDepTysvnvAhFyY7tfG793iQAJA8T4ZpyAZyrCLQ7");

const depositories = {};

depositories[TEST_COIN_MINT] = {
    key: depository.programId,
    stateKey: findAddr([Buffer.from("STATE"), coinMintKey.toBuffer()], depository.programId),
    redeemableMintKey: findAddr([Buffer.from("REDEEMABLE"), coinMintKey.toBuffer()], depository.programId),
    depositAccountKey: findAddr([Buffer.from("DEPOSIT"), coinMintKey.toBuffer()], depository.programId),
    recordKey: findAddr([Buffer.from("RECORD"), depository.programId.toBuffer()], controller.programId),
    oracleKey: findAddr([Buffer.from("BTCUSD")], oracle.programId),
};

// simple shorthand
function findAddr(seeds, programId) {
    return anchor.utils.publicKey.findProgramAddressSync(seeds, programId)[0];
}

// establish a devnet connection, returning the connection object
// transactions must be assembled and signed by the wallet, but this can be done with vanilla solana if desired
function instantiateConnection() {
    let conn = new anchor.web3.Connection(CLUSTER, TXN_OPTS);
    anchor.setProvider(new anchor.Provider(conn, null, null));

    return conn;
}

// derives the canonical token account address for a given wallet and mint
function findAssociatedTokenAddress(walletKey, mintKey) {
    return findAddr([walletKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mintKey.toBuffer()], ASSOC_TOKEN_PROGRAM_ID);
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
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: anchor.web3.SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
        ],
        programId: ASSOC_TOKEN_PROGRAM_ID,
        data: Buffer.alloc(0),
    })];
}

// returns an two-instruction array for depositing funds and minting uxd
// coinAmount should be a bn with the appropriate number of zeros (ie we dont normalize here)
// userCoinKey is optional and defaults to the associated account
// redeemable and uxd are always associated because we create these on behalf of the user
function mintUxd(walletKey, coinMintKey, coinAmount, userCoinKey) {
    let coinMintDepository = depositories[coinMintKey.toString()];
    if(!coinMintDepository) throw `no depository found for mint ${coinMintKey.toString()}`;

    if(!userCoinKey) userCoinKey = findAssociatedTokenAddress(walletKey, coinMintKey);
    let userRedeemableKey = findAssociatedTokenAddress(walletKey, coinMintDepository.redeemableMintKey);
    let userUxdKey = findAssociatedTokenAddress(walletKey, uxdMintKey);
    let coinPassthroughKey = findAddr([Buffer.from("PASSTHROUGH"), coinMintKey.toBuffer()], controller.programId);

    let depositIxn = depository.instruction.deposit(coinAmount, {
        accounts: {
            user: walletKey,
            state: coinMintDepository.stateKey,
            programCoin: coinMintDepository.depositAccountKey,
            redeemableMint: coinMintDepository.redeemableMintKey,
            userCoin: userCoinKey,
            userRedeemable: userRedeemableKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
        }
    });

    let mintIxn = controller.instruction.mintUxd(coinAmount, {
        accounts: {
            user: walletKey,
            state: controlStateKey,
            depository: coinMintDepository.key,
            depositoryRecord: coinMintDepository.recordKey,
            depositoryState: coinMintDepository.stateKey,
            depositoryCoin: coinMintDepository.depositAccountKey,
            coinMint: coinMintKey,
            coinPassthrough: coinPassthroughKey,
            redeemableMint: coinMintDepository.redeemableMintKey,
            userRedeemable: userRedeemableKey,
            userUxd: userUxdKey,
            uxdMint: uxdMintKey,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            // XXX FIXME temp
            oracle: coinMintDepository.oracleKey,
    }});

    return [depositIxn, mintIxn];
}

// returns a two-instruction array for burning uxd and withdrawing backing collateral
// uxdAmount is a dollar amount, again as a proper bn. otherwise args are the same
// frontend should check that the associated token account exists and make the user create it if not
function redeemUxd(walletKey, coinMintKey, uxdAmount) {
    let coinMintDepository = depositories[coinMintKey.toString()];
    if(!coinMintDepository) throw `no depository found for mint ${coinMintKey.toString()}`;

    let userCoinKey = findAssociatedTokenAddress(walletKey, coinMintKey);
    let userRedeemableKey = findAssociatedTokenAddress(walletKey, coinMintDepository.redeemableMintKey);
    let userUxdKey = findAssociatedTokenAddress(walletKey, uxdMintKey);
    let coinPassthroughKey = findAddr([Buffer.from("PASSTHROUGH"), coinMintKey.toBuffer()], controller.programId);

    let redeemIxn = controller.instruction.redeemUxd(uxdAmount, {
        accounts: {
            user: walletKey,
            state: controlStateKey,
            depository: coinMintDepository.key,
            depositoryRecord: coinMintDepository.recordKey,
            depositoryState: coinMintDepository.stateKey,
            depositoryCoin: coinMintDepository.depositAccountKey,
            coinMint: coinMintKey,
            coinPassthrough: coinPassthroughKey,
            redeemableMint: coinMintDepository.redeemableMintKey,
            userRedeemable: userRedeemableKey,
            userUxd: userUxdKey,
            uxdMint: uxdMintKey,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            // XXX FIXME temp
            oracle: coinMintDepository.oracleKey,
    }});

    let withdrawIxn = depository.instruction.withdraw(null, {
        accounts: {
            user: walletKey,
            state: coinMintDepository.stateKey,
            programCoin: coinMintDepository.depositAccountKey,
            redeemableMint: coinMintDepository.redeemableMintKey,
            userCoin: userCoinKey,
            userRedeemable: userRedeemableKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
    }});

    return [redeemIxn, withdrawIxn];
}

module.exports = {
    TEST_COIN_MINT: TEST_COIN_MINT,
    provider: provider,
    depositories: depositories,
    connect: instantiateConnection,
    findAssociatedTokenAddress: findAssociatedTokenAddress,
    createAssociatedTokenAccount: createAssociatedTokenAccount,
    mintUxd: mintUxd,
    redeemUxd: redeemUxd,
};
