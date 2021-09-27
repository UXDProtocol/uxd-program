"use strict";

// DzmGB2YeeFbSL72cAxYtfQCQXzyyWW2xYPCJ1uSPtNiP

const fs = require("fs");
const anchor = require("@project-serum/anchor");

const provider = anchor.Provider.local("https://api.devnet.solana.com");
anchor.setProvider(provider);

const oracle = anchor.workspace.Oracle;

const btcPrice = new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");

async function main() {
    let s = await oracle.rpc.lol({
        accounts: {
            btcPrice: btcPrice,
        },
    });

    console.log("s:", s);
}

main();
