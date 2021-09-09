"use strict";

const fs = require("fs");
const anchor = require("@project-serum/anchor");
const spl = require("@solana/spl-token");

const controllerIdl = JSON.parse(fs.readFileSync("/home/hana/work/soteria/solana-usds/target/idl/controller.json"));
const controllerKey = new anchor.web3.PublicKey(controllerIdl.metadata.address);
const controller = new anchor.Program(controllerIdl, controllerKey);

const depositoryIdl = JSON.parse(fs.readFileSync("/home/hana/work/soteria/solana-usds/target/idl/depository.json"));
const depositoryKey = new anchor.web3.PublicKey(depositoryIdl.metadata.address);
const depository = new anchor.Program(depositoryIdl, depositoryKey);

const COIN_MINT = process.argv[2];
if(!COIN_MINT) throw "specify coin mint";
const MINT_DECIMAL = 9;

// this is theoretically constant everywhere
const TOKEN_PROGRAM_ID = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ASSOC_TOKEN_PROGRAM_ID = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

const TXN_COMMIT = "processed";
const TXN_OPTS = {commitment: TXN_COMMIT, preflightCommitment: TXN_COMMIT, skipPreflight: false};

const coinMintKey = new anchor.web3.PublicKey(COIN_MINT);
const tokenProgramKey = new anchor.web3.PublicKey(TOKEN_PROGRAM_ID);
const assocTokenProgramKey = new anchor.web3.PublicKey(ASSOC_TOKEN_PROGRAM_ID);

const provider = anchor.Provider.local();
anchor.setProvider(provider);

// simple shorthand
function findAddr(seeds, programId) {
    return anchor.utils.publicKey.findProgramAddressSync(seeds, programId)[0];
}

// derives the canonical token account address for a given wallet and mint
function findAssocTokenAddr(wallet, mint) {
    return findAddr([wallet.toBuffer(), tokenProgramKey.toBuffer(), mint.toBuffer()], assocTokenProgramKey);
}

async function main() {
    // keys for controller.new
    let controlStateKey = findAddr([Buffer.from("STATE")], controllerKey);
    let uxdMintKey = findAddr([Buffer.from("STABLECOIN")], controllerKey);

    // keys for depository.new
    let depositStateKey = findAddr([Buffer.from("STATE")], depositoryKey);
    let redeemableMintKey = findAddr([Buffer.from("REDEEMABLE")], depositoryKey);
    let depositAccountKey = findAddr([Buffer.from("DEPOSIT")], depositoryKey);

    // keys for controller.registerDepository
    let depositRecordKey = findAddr([Buffer.from("RECORD"), depositoryKey.toBuffer()], controllerKey);
    let coinPassthroughKey = findAddr([Buffer.from("PASSTHROUGH"), coinMintKey.toBuffer()], controllerKey);

    // standard spl associated accounts
    let userCoinKey = findAssocTokenAddr(provider.wallet.publicKey, coinMintKey);
    let userRedeemableKey = findAssocTokenAddr(provider.wallet.publicKey, redeemableMintKey);

    async function printBalances(skipRed) {
        let userCoin = (await provider.connection.getTokenAccountBalance(userCoinKey, TXN_COMMIT))["value"]["uiAmount"];
        let userRedeemable = skipRed ? 0 : (await provider.connection.getTokenAccountBalance(userRedeemableKey, TXN_COMMIT))["value"]["uiAmount"];
        let programCoin = (await provider.connection.getTokenAccountBalance(depositAccountKey, TXN_COMMIT))["value"]["uiAmount"];

        console.log(`* user balance: ${userCoin}\n* user redeemable: ${userRedeemable}\n* program balance: ${programCoin}\n\n`);
    }

    console.log("payer:", provider.wallet.publicKey.toString());
    console.log("redeemable mint:", redeemableMintKey.toString());
    console.log("program coin:", depositAccountKey.toString());
    console.log("coin mint:", coinMintKey.toString());
    console.log("uxd mint:", uxdMintKey.toString());
    console.log("controller id:", controllerKey.toString());
    console.log("controller state:", controlStateKey.toString());
    console.log("depository id:", depositoryKey.toString());
    console.log("depository state:", depositStateKey.toString());
    console.log("\n");

    // set up the controller
    if(await provider.connection.getAccountInfo(controlStateKey)) {
        console.log("controller already initialized...");
    } else {
        await controller.rpc.new({
            accounts: {
                authority: provider.wallet.publicKey,
                state: controlStateKey,
                uxdMint: uxdMintKey,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenProgram: tokenProgramKey,
                program: controllerKey,
            },
            signers: [provider.wallet.payer],
            options: TXN_OPTS,
        });

        console.log("controller initialized!");
    }

    // and set up the depository
    if(await provider.connection.getAccountInfo(depositStateKey)) {
        console.log("depository already initialized...");
    } else {
        await depository.rpc.new(controllerKey, {
            accounts: {
                payer: provider.wallet.publicKey,
                state: depositStateKey,
                redeemableMint: redeemableMintKey,
                programCoin: depositAccountKey,
                coinMint: coinMintKey,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenProgram: tokenProgramKey,
                program: depositoryKey,
            },
            signers: [provider.wallet.payer],
            options: TXN_OPTS,
        });

        console.log("depository initialized!");
    }

    // aaand register it with the controller
    if(await provider.connection.getAccountInfo(depositRecordKey)) {
        console.log("depository already registered...");
    } else {
        await controller.rpc.registerDepository(depositoryKey, {
            accounts: {
                authority: provider.wallet.publicKey,
                state: controlStateKey,
                depositoryRecord: depositRecordKey,
                depositoryState: depositStateKey,
                coinMint: coinMintKey,
                coinPassthrough: coinPassthroughKey,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenProgram: tokenProgramKey,
                program: controllerKey,
            },
            signers: [provider.wallet.payer],
            options: TXN_OPTS,
        });

        console.log("depository registered!");
    }

    // ok now uhh i just wanna deposit and withdraw. may or may not have to set up the redeemable acct
    let redeemState = await provider.connection.getAccountInfo(userRedeemableKey);

    // anchor will error if you pass [] or null lol
    var depositIxns = undefined;
    if(!redeemState) {
        depositIxns = [
            new anchor.web3.TransactionInstruction({
                keys: [
                    {pubkey: provider.wallet.publicKey, isSigner: true, isWritable: true},
                    {pubkey: userRedeemableKey, isSigner: false, isWritable: true},
                    {pubkey: provider.wallet.publicKey, isSigner: false, isWritable: false},
                    {pubkey: redeemableMintKey, isSigner: false, isWritable: false},
                    {pubkey: anchor.web3.SystemProgram.programId, isSigner: false, isWritable: false},
                    {pubkey: tokenProgramKey, isSigner: false, isWritable: false},
                    {pubkey: anchor.web3.SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
                ],
                programId: assocTokenProgramKey,
                data: Buffer.alloc(0),
            }),
        ]
    }

    console.log("BEFORE DEPOSIT");
    printBalances(true);

    await depository.rpc.deposit(new anchor.BN(1 * 10**MINT_DECIMAL), {
        accounts: {
            user: provider.wallet.publicKey,
            state: depositStateKey,
            programCoin: depositAccountKey,
            redeemableMint: redeemableMintKey,
            userCoin: userCoinKey,
            userRedeemable: userRedeemableKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: tokenProgramKey,
            program: depositoryKey,
        },
        signers: [provider.wallet.payer],
        options: TXN_OPTS,
        instructions: depositIxns,
    });

    console.log("AFTER DEPOSIT");
    printBalances();

    await depository.rpc.withdraw(new anchor.BN(1 * 10**MINT_DECIMAL), {
        accounts: {
            user: provider.wallet.publicKey,
            state: depositStateKey,
            programCoin: depositAccountKey,
            redeemableMint: redeemableMintKey,
            userCoin: userCoinKey,
            userRedeemable: userRedeemableKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: tokenProgramKey,
            program: depositoryKey,
        },
        signers: [provider.wallet.payer],
        options: TXN_OPTS,
    });

    console.log("AFTER WITHDRAW");
    printBalances();

}

main();
