"use strict";

const fs = require("fs");
const anchor = require("@project-serum/anchor");
const spl = require("@solana/spl-token");

const depositoryIdl = JSON.parse(fs.readFileSync("/home/hana/work/soteria/solana-usds/target/idl/depository.json"));
const depositoryKey = new anchor.web3.PublicKey(depositoryIdl.metadata.address);
const depository = new anchor.Program(depositoryIdl, depositoryKey);

const TEST_MINT = "Kova7SyXBM216fpWEdPzdtbJ2gb9RTynBYjM9TaphMk";
const MINT_DECIMAL = 9;

// this is theoretically constant everywhere
const TOKEN_PROGRAM_ID = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ASSOC_TOKEN_PROGRAM_ID = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

const TXN_COMMIT = "processed";
const TXN_OPTS = {commitment: TXN_COMMIT, preflightCommitment: TXN_COMMIT, skipPreflight: true};

const depositMintKey = new anchor.web3.PublicKey(TEST_MINT);
const tokenProgramKey = new anchor.web3.PublicKey(TOKEN_PROGRAM_ID);
const assocTokenProgramKey = new anchor.web3.PublicKey(ASSOC_TOKEN_PROGRAM_ID);

const provider = anchor.Provider.local();
anchor.setProvider(provider);

// derives the canonical token account address for a given wallet and mint
async function findAssocTokenAddr(wallet, mint) {
    return (await anchor.web3.PublicKey.findProgramAddress([
        wallet.toBuffer(),
        tokenProgramKey.toBuffer(),
        mint.toBuffer(),
    ], assocTokenProgramKey))[0];
}

async function main() {
    // anchor insists on including wallet addresses in derived accounts
    // so i cant use their fucntions intended for this
    let depositStateKey = (await anchor.web3.PublicKey.findProgramAddress([Buffer.from("STATE")], depositoryKey))[0];
    let redeemableMintKey = (await anchor.web3.PublicKey.findProgramAddress([Buffer.from("REDEEMABLE")], depositoryKey))[0];
    let depositAccountKey = (await anchor.web3.PublicKey.findProgramAddress([Buffer.from("DEPOSIT")], depositoryKey))[0];

    // standard spl associated accounts
    let walletCoinKey = await findAssocTokenAddr(provider.wallet.publicKey, depositMintKey);
    let walletRedeemableKey = await findAssocTokenAddr(provider.wallet.publicKey, redeemableMintKey);

    async function printBalances(skipRed) {
        let userCoin = (await provider.connection.getTokenAccountBalance(walletCoinKey, TXN_COMMIT))["value"]["uiAmount"];
        let userRedeemable = skipRed ? 0 : (await provider.connection.getTokenAccountBalance(walletRedeemableKey, TXN_COMMIT))["value"]["uiAmount"];
        let programCoin = (await provider.connection.getTokenAccountBalance(depositAccountKey, TXN_COMMIT))["value"]["uiAmount"];

        console.log(`* user balance: ${userCoin}\n* user redeemable: ${userRedeemable}\n* program balance: ${programCoin}\n\n`);
    }

    console.log("payer:", provider.wallet.publicKey.toString());
    console.log("state:", depositStateKey.toString());
    console.log("redeemable mint:", redeemableMintKey.toString());
    console.log("program coin:", depositAccountKey.toString());
    console.log("coin mint:", depositMintKey.toString());
    console.log("depository id:", depositoryKey.toString());

    // set up the depository
    // im deploying to a new address each time soo this is fine
    await depository.rpc.new({
        accounts: {
            payer: provider.wallet.publicKey,
            state: depositStateKey,
            redeemableMint: redeemableMintKey,
            programCoin: depositAccountKey,
            coinMint: depositMintKey,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: tokenProgramKey,
            program: depositoryKey,
        },
        signers: [provider.wallet.payer],
        options: TXN_OPTS,
    });

    console.log("initialized!");

    // ok now uhh i just wanna deposit and withdraw. may or may not have to set up the redeemable acct
    let redeemState = await provider.connection.getAccountInfo(walletRedeemableKey);

    // anchor will error if you pass [] or null lol
    var depositIxns = undefined;
    if(!redeemState) {
        depositIxns = [
            new anchor.web3.TransactionInstruction({
                keys: [
                    {pubkey: provider.wallet.publicKey, isSigner: true, isWritable: true},
                    {pubkey: walletRedeemableKey, isSigner: false, isWritable: true},
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
            userCoin: walletCoinKey,
            userRedeemable: walletRedeemableKey,
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
            userCoin: walletCoinKey,
            userRedeemable: walletRedeemableKey,
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
