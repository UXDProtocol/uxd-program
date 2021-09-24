"use strict";

const anchor = require("@project-serum/anchor");
const api = require("./api.js");

// XXX these are fake im not actually calling out tho just making sure ixns look right
// also lol it claims publicKey() on the keypair is not a function?? idgi
const walletKey = new anchor.web3.PublicKey(new anchor.web3.Keypair()._keypair.publicKey);
const mintKey = new anchor.web3.PublicKey(new anchor.web3.Keypair()._keypair.publicKey);
const amount = new anchor.BN(1 * 10**9);

let ixns = api.mintUxd(walletKey, mintKey, amount);

console.log("HANA mint ixns:");
for (let ixn of ixns) {
    console.log(ixn);
}
