"use strict";

// DzmGB2YeeFbSL72cAxYtfQCQXzyyWW2xYPCJ1uSPtNiP

const anchor = require("@project-serum/anchor");

//const DEVNET = "https://api.devnet.solana.com";
// const DEVNET = "http://128.0.113.156";
const TESTNET = "https://api.testnet.solana.com"
const TXN_COMMIT = "processed";
const TXN_OPTS = { commitment: TXN_COMMIT, preflightCommitment: TXN_COMMIT, skipPreflight: true };

const provider = anchor.Provider.local();
anchor.setProvider(provider);
const oracle = anchor.workspace.Oracle;

// const devnetOracle = new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");
const testnetOraclePriceAccountKey = new anchor.web3.PublicKey("DJW6f4ZVqCnpYNN9rNuzqUcCvkVtBgixo8mq9FKSsCbJ");
const localOraclePriceAccountKey = anchor.utils.publicKey.findProgramAddressSync(["BTCUSD"], oracle.programId)[0];

async function main() {
    console.log("testnet btc price key:", testnetOraclePriceAccountKey.toString());
    console.log("local btc price key:", localOraclePriceAccountKey.toString());

    let testnetConn = new anchor.web3.Connection(TESTNET);
    let testnet_oracle_account = await testnetConn.getAccountInfo(testnetOraclePriceAccountKey);

    console.log("DATA FROM TESTNET");
    console.log(testnet_oracle_account);

    console.log("init");
    await oracle.rpc.init({
        accounts: {
            wallet: provider.wallet.publicKey,
            buffer: localOraclePriceAccountKey,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [provider.wallet.payer],
        options: TXN_OPTS,
    }).catch(() => null);

    var i = 0;
    while(true) {
        let j = i + 512 > testnet_oracle_account.data.length ? testnet_oracle_account.data.length : i + 512;

        console.log(`put [${i}..${j}]`);
        await oracle.rpc.put(new anchor.BN(i), testnet_oracle_account.data.slice(i, j), {
            accounts: {
                buffer: localOraclePriceAccountKey,
            },
            options: TXN_OPTS,
        });

        i += 512;
        if(i > testnet_oracle_account.data.length) break;
    }

    // let localConn = new anchor.web3.Connection("http://127.0.0.1");
    // let local_oracle_account = await localConn.getAccountInfo(localOraclePriceAccountKey);
    // console.log("DATA FROM LOCAL ORACLE (copied from test net)");
    // console.log(local_oracle_account);

    console.log("get");
    await oracle.rpc.get({
        accounts: {
            oracle: localOraclePriceAccountKey,
        },
        options: TXN_OPTS,
    });

}

main();
