"use strict";

// DzmGB2YeeFbSL72cAxYtfQCQXzyyWW2xYPCJ1uSPtNiP

const fs = require("fs");
const anchor = require("@project-serum/anchor");

const idl = JSON.parse(fs.readFileSync("/home/hana/work/soteria/solana-usds/target/idl/oracle.json"));
const programKey = new anchor.web3.PublicKey(idl.metadata.address);

const btcPrice = new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");

const provider = anchor.Provider.local("https://api.devnet.solana.com");
anchor.setProvider(provider);

const program = new anchor.Program(idl, programKey);

async function main() {
    let s = await program.rpc.lol({
        accounts: {
            btcPrice: btcPrice,
        },
    });

    console.log("s:", s);
}

main();
