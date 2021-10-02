"use strict";

const anchor = require("@project-serum/anchor");
const api = require("./api.js");

// XXX these are fake im not actually calling out tho just making sure ixns look right
// also lol it claims publicKey() on the keypair is not a function?? idgi
//XXX const walletKey = new anchor.web3.PublicKey(new anchor.web3.Keypair()._keypair.publicKey);
const walletKey = api.provider.wallet.publicKey;
const mintKey = new anchor.web3.PublicKey(api.TEST_COIN_MINT);
const coinAmount = new anchor.BN(1 * 10**9);
const uxdAmount = new anchor.BN(20000 * 10**6);

let mintIxns = api.mintUxd(walletKey, mintKey, coinAmount);

console.log("HANA mint ixns:");
for (let ixn of mintIxns) {
    console.log(ixn);
}

let mintTxn = new anchor.web3.Transaction();
mintTxn.add(mintIxns[0]);
mintTxn.add(mintIxns[1]);

let redeemIxns = api.redeemUxd(walletKey, mintKey, uxdAmount);

console.log("HANA redeem ixns:");
for (let ixn of redeemIxns) {
    console.log(ixn);
}

let redeemTxn = new anchor.web3.Transaction();
redeemTxn.add(redeemIxns[0]);
redeemTxn.add(redeemIxns[1]);

api.provider.send(mintTxn)
.then((s) => {
    console.log("HANA s1:", s);
    return api.provider.send(redeemTxn);
})
.then((s) => console.log("HANA s2:", s));
