"use strict";

const anchor = require("@project-serum/anchor");
const api = require("./api.js");
const assert = require("assert");

const TXN_COMMIT = "processed";
const TXN_OPTS = {commitment: TXN_COMMIT, preflightCommitment: TXN_COMMIT, skipPreflight: false};

describe("API Test", () => {
    // XXX these are fake im not actually calling out tho just making sure ixns look right
    // also lol it claims publicKey() on the keypair is not a function?? idgi
    //XXX const walletKey = new anchor.web3.PublicKey(new anchor.web3.Keypair()._keypair.publicKey);
    const wallet = api.provider.wallet;
    const mintKey = new anchor.web3.PublicKey(api.TEST_COIN_MINT);
    const coinAmount = new anchor.BN(1 * 10 ** 9);
    const uxdAmount = new anchor.BN(20000 * 10 ** 6);

    it("Mint ixs", async () => {
        let mintIxns = api.mintUxd(wallet.publicKey, mintKey, coinAmount);

        for (let ixn of mintIxns) {
            console.log(ixn);
        }

        let mintTxn = new anchor.web3.Transaction();
        mintTxn.add(mintIxns[0]);
        mintTxn.add(mintIxns[1]);
        // mintTxn.feePayer = wallet.publicKey;
        // mintTxn.recentBlockhash = (await api.provider.connection.getRecentBlockhash()).blockhash;
    
        // mintTxn = await wallet.signTransaction(mintTxn);

        let txId = api.provider.send(mintTxn);
    });

    it("Redeem ixns", async () => {
        let redeemIxns = api.redeemUxd(wallet.publicKey, mintKey, uxdAmount);
        for (let ixn of redeemIxns) {
            console.log(ixn);
        }

        let redeemTxn = new anchor.web3.Transaction();
        redeemTxn.add(redeemIxns[0]);
        redeemTxn.add(redeemIxns[1]);
        // redeemTxn.feePayer = wallet.publicKey;
        // redeemTxn.recentBlockhash = (await api.provider.connection.getRecentBlockhash()).blockhash;
      
        // redeemTxn = await wallet.signTransaction(redeemTxn);

        let txId = api.provider.send(redeemTxn);
    });
});