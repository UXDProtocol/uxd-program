"use strict";

const fs = require("fs");
const anchor = require("@project-serum/anchor");
const spl = require("@solana/spl-token");

// these i have to change based on the whims of solana
const PROGRAM_ID = process.argv[2];
if(!PROGRAM_ID) throw "specify program id";
const TEST_MINT = "8teNo9g6MtV5RW7J2fPj1ZrhBgaaTdinSoSPGcCWjxPL";

// this is theoretically constant everywhere
const TOKEN_PROGRAM_ID = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

const provider = anchor.Provider.local();
anchor.setProvider(provider);

async function main() {
    let idl = JSON.parse(fs.readFileSync("/home/hana/work/soteria/solana-usds/target/idl/depository.json"));
    let programKey = new anchor.web3.PublicKey(PROGRAM_ID);
    let program = new anchor.Program(idl, programKey);

    let mintKey = new anchor.web3.PublicKey(TEST_MINT);
    let tokenProgramKey = new anchor.web3.PublicKey(TOKEN_PROGRAM_ID);

    // anchor insists on including wallet addresses in derived accounts
    // so i cant use their fucntions intended for this
    let redeemableMintKey = (await anchor.web3.PublicKey.findProgramAddress([Buffer.from("REDEEMABLE")], programKey))[0];
    let depositAccountKey = (await anchor.web3.PublicKey.findProgramAddress([Buffer.from("DEPOSIT")], programKey))[0];

    await program.state.rpc.new({
        accounts: {
            payer: provider.wallet.publicKey,
            redeemableMint: redeemableMintKey,
            depositAccount: depositAccountKey,
            depositMint: mintKey,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            sys: anchor.web3.SystemProgram.programId,
            tok: tokenProgramKey,
            prog: programKey,
        },
        signers: [provider.wallet.payer],
        options: {commitment: "processed", preflightCommitment: "processed", skipPreflight: true},
        /*
        instructions: [
            anchor.web3.SystemProgram.createAccount({
                fromPubkey: provider.wallet.publicKey,
                newAccountPubkey: tokenKeypair.publicKey,
                space: spl.AccountLayout.span,
                lamports: await provider.connection.getMinimumBalanceForRentExemption(spl.AccountLayout.span),
                programId: tokenProgramKey,
            }),
        ],
        remainingAccounts: [{isSigner: false, isWritable: false, pubkey: anchor.web3.SystemProgram.programId}],
        */
    });

    console.log("initialized!");

    let state = await program.state.fetch();

    console.log("state:", state);
}

main();
