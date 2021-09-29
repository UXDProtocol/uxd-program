"use strict";

const anchor = require("@project-serum/anchor");
const spl = require("@solana/spl-token");

const COIN_MINT = process.argv[2];
if(!COIN_MINT) throw "specify coin mint";
const MINT_DECIMAL = 9;

const DEVNET = process.argv[3] == "devnet" ? "https://api.devnet.solana.com" : false;

// this is theoretically constant everywhere
const TOKEN_PROGRAM_ID = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ASSOC_TOKEN_PROGRAM_ID = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

const TXN_COMMIT = "processed";
const TXN_OPTS = {commitment: TXN_COMMIT, preflightCommitment: TXN_COMMIT, skipPreflight: false};

const coinMintKey = new anchor.web3.PublicKey(COIN_MINT);
const tokenProgramKey = new anchor.web3.PublicKey(TOKEN_PROGRAM_ID);
const assocTokenProgramKey = new anchor.web3.PublicKey(ASSOC_TOKEN_PROGRAM_ID);

const provider = anchor.Provider.local(DEVNET || undefined);
anchor.setProvider(provider);

const controllerIdl = require("../target/idl/controller.json");
const controllerKey = new anchor.web3.PublicKey(controllerIdl.metadata.address);
const controller = new anchor.Program(controllerIdl, controllerKey);

const depositoryIdl = require("../target/idl/depository.json");
const depositoryKey = new anchor.web3.PublicKey(depositoryIdl.metadata.address);
const depository = new anchor.Program(depositoryIdl, depositoryKey);

// we should not need this on mainnet but note the addresses change per cluster
// oracleprogram is for if we copied data to localnet
const oracleProgramKey = new anchor.web3.PublicKey(require("../target/idl/oracle.json").metadata.address);
const devnetOracleKey = new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");

// simple shorthand
function findAddr(seeds, programId) {
    return anchor.utils.publicKey.findProgramAddressSync(seeds, programId)[0];
}

// derives the canonical token account address for a given wallet and mint
function findAssocTokenAddr(walletKey, mintKey) {
    return findAddr([walletKey.toBuffer(), tokenProgramKey.toBuffer(), mintKey.toBuffer()], assocTokenProgramKey);
}

// returns an instruction to create the associated account for a wallet and mint
function createAssocIxn(walletKey, mintKey) {
    let assocKey = findAssocTokenAddr(walletKey, mintKey);

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

// handle the error when an account is uninitialized...
function getTokenBalance(tokenKey) {
    return provider.connection.getTokenAccountBalance(tokenKey, TXN_COMMIT)
           .then(o => o["value"]["uiAmount"])
           .catch(() => null);
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
    let userUxdKey = findAssocTokenAddr(provider.wallet.publicKey, uxdMintKey);

    // btcusd oracle
    let localOracleKey = findAddr([Buffer.from("BTCUSD")], oracleProgramKey);
    let oracleKey = DEVNET ? devnetOracleKey : localOracleKey;

    async function printBalances() {
        let userCoin = await getTokenBalance(userCoinKey);
        let depositCoin = await getTokenBalance(depositAccountKey);
        let coinPassthrough = await getTokenBalance(coinPassthroughKey);
        let userRedeemable = await getTokenBalance(userRedeemableKey);
        let userUxd = await getTokenBalance(userUxdKey);

        console.log(
`* user balance: ${userCoin}
* depository balance: ${depositCoin}
* controller balance: ${coinPassthrough}
* user redeemable: ${userRedeemable}
* user uxd: ${userUxd}
`);
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
    // at the moment i am using one, for a mint we control, plus the btc oracle
    // devnet we can do real testing with sol (theres no btc/eth faucets, dunno about markets)
    // but i need to write wrap/unwrap logic
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
        await controller.rpc.registerDepository(depositoryKey, oracleKey, {
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

    if(!DEVNET) {
        // create user account for redeemables if it doesnt exist
        // note anchor will error if you pass [] or null for the extra ixns
        let depositIxns = await provider.connection.getAccountInfo(userRedeemableKey)
                        ? undefined
                        : [createAssocIxn(provider.wallet.publicKey, redeemableMintKey)];

        console.log("BEFORE DEPOSIT");
        await printBalances();

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
        await printBalances();

        // XXX TODO here i need to...
        // * create user account for uxd
        // * call mint
        // * call redeem
        // * impl proxy xfer

        // create user account for uxd if needed
        let mintIxns = await provider.connection.getAccountInfo(userUxdKey)
                       ? undefined
                       : [createAssocIxn(provider.wallet.publicKey, uxdMintKey)];

        await controller.rpc.mintUxd(new anchor.BN(1 * 10**MINT_DECIMAL), {
            accounts: {
                user: provider.wallet.publicKey,
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
                oracle: oracleKey,
            },
            signers: [provider.wallet.payer],
            options: TXN_OPTS,
            instructions: mintIxns,
        });

        console.log("AFTER MINT");
        await printBalances();

        await controller.rpc.redeemUxd(new anchor.BN(20000 * 10**MINT_DECIMAL), {
            accounts: {
                user: provider.wallet.publicKey,
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
                oracle: oracleKey,
            },
            signers: [provider.wallet.payer],
            options: TXN_OPTS,
        });

        console.log("AFTER REDEEM");
        await printBalances();

        await depository.rpc.withdraw(null, {
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
        await printBalances();
    }
}

main();
