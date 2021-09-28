"use strict";

const anchor = require("@project-serum/anchor");
const spl = require("@solana/spl-token");

const provider = anchor.Provider.local();
anchor.setProvider(provider);

const controller = anchor.workspace.Controller;
const depository = anchor.workspace.Depository;
const oracle = anchor.workspace.Oracle;

const COIN_MINT = process.env.COIN_MINT;
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

// we should not need this on mainnet but note the addresses change per cluster
// XXX copy data to local
//const btcOracleDevnetKey = new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");

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
    let controlStateKey = findAddr([Buffer.from("STATE")], controller.programId);
    let uxdMintKey = findAddr([Buffer.from("STABLECOIN")], controller.programId);

    // keys for depository.new
    let depositStateKey = findAddr([Buffer.from("STATE")], depository.programId);
    let redeemableMintKey = findAddr([Buffer.from("REDEEMABLE")], depository.programId);
    let depositAccountKey = findAddr([Buffer.from("DEPOSIT")], depository.programId);

    // keys for controller.registerDepository
    let depositRecordKey = findAddr([Buffer.from("RECORD"), depository.programId.toBuffer()], controller.programId);
    let coinPassthroughKey = findAddr([Buffer.from("PASSTHROUGH"), coinMintKey.toBuffer()], controller.programId);

    // standard spl associated accounts
    let userCoinKey = findAssocTokenAddr(provider.wallet.publicKey, coinMintKey);
    let userRedeemableKey = findAssocTokenAddr(provider.wallet.publicKey, redeemableMintKey);
    let userUxdKey = findAssocTokenAddr(provider.wallet.publicKey, uxdMintKey);

    // localnet oracle
    let localOracleKey = findAddr([Buffer.from("BTCUSD")], oracle.programId);

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
* local oracle BTC key : ${localOracleKey}
`);
    }

    console.log("payer:", provider.wallet.publicKey.toString());
    console.log("redeemable mint:", redeemableMintKey.toString());
    console.log("program coin:", depositAccountKey.toString());
    console.log("coin mint:", coinMintKey.toString());
    console.log("uxd mint:", uxdMintKey.toString());
    console.log("controller id:", controller.programId.toString());
    console.log("controller state:", controlStateKey.toString());
    console.log("depository id:", depository.programId.toString());
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
                program: controller.programId,
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
        await depository.rpc.new(controller.programId, {
            accounts: {
                payer: provider.wallet.publicKey,
                state: depositStateKey,
                redeemableMint: redeemableMintKey,
                programCoin: depositAccountKey,
                coinMint: coinMintKey,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenProgram: tokenProgramKey,
                program: depository.programId,
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
        await controller.rpc.registerDepository(depository.programId, localOracleKey, {
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
                program: controller.programId,
            },
            signers: [provider.wallet.payer],
            options: TXN_OPTS,
        });

        console.log("depository registered!");
    }

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
            program: depository.programId,
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
            depository: depository.programId,
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
            program: controller.programId,
            // XXX FIXME temp
            programCoin: depositAccountKey,
            oracle: localOracleKey,
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
            depository: depository.programId,
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
            program: controller.programId,
            // XXX FIXME temp
            programCoin: depositAccountKey,
            oracle: localOracleKey,
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
            program: depository.programId,
        },
        signers: [provider.wallet.payer],
        options: TXN_OPTS,
    });

    console.log("AFTER WITHDRAW");
    await printBalances();

}

main();
