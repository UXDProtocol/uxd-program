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
const testnetOracle = new anchor.web3.PublicKey("DJW6f4ZVqCnpYNN9rNuzqUcCvkVtBgixo8mq9FKSsCbJ");
const localOracle = anchor.utils.publicKey.findProgramAddressSync(["BTCUSD"], oracle.programId)[0];

async function main() {
    console.log("testnet btc price key:", testnetOracle.toString());
    console.log("local btc price key:", localOracle.toString());

    let testnetConn = new anchor.web3.Connection(TESTNET);
    let testnet_oracle_data = await testnetConn.getAccountInfo(testnetOracle);

    console.log(`DATA from TESTNET: ${testnet_oracle_data}`);

    console.log("init");
    await oracle.rpc.init({
        accounts: {
            wallet: provider.wallet.publicKey,
            buffer: localOracle,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [provider.wallet.payer],
        options: TXN_OPTS,
    }).catch(() => null);

    var i = 0;
    while(true) {
        let j = i + 512 > d.data.length ? d.data.length : i + 512;

        console.log(`put [${i}..${j}]`);
        await oracle.rpc.put(new anchor.BN(i), d.data.slice(i, j), {
            accounts: {
                buffer: localOracle,
            },
            options: TXN_OPTS,
        });

        i += 512;
        if(i > d.data.length) break;
    }


    let local_data = await testnetConn.getAccountInfo(testnetOracle);
    console.log(`DATA from LOCAL oracle (copied from test net): ${local_data}`);

    console.log("get");
    await oracle.rpc.get({
        accounts: {
            oracle: localOracle,
        },
        options: TXN_OPTS,
    });

}

main();
