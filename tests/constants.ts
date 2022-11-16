import { Keypair, PublicKey, Signer } from "@solana/web3.js";
import { UXDClient } from "@uxd-protocol/uxd-client";
import * as jsonIdl from "../target/idl/uxd.json";

// TESTING wallets for convenience (The user and admin). To remove when going open source

// aca3VWxwBeu8FTZowJ9hfSKGzntjX68EXh1N9xpE1PC
const aca3VWSeed = Uint8Array.from([
    197, 246, 88, 131, 17, 216, 175, 8, 72, 13, 40, 236, 135, 104, 59, 108, 17, 106, 164, 234, 46, 136, 171, 148, 111,
    176, 32, 136, 59, 253, 224, 247, 8, 156, 98, 175, 196, 123, 178, 151, 182, 220, 253, 138, 191, 233, 135, 182, 173,
    175, 33, 68, 162, 191, 254, 166, 133, 219, 8, 10, 17, 154, 146, 223,
]);
// Eyh77zP5b7arPtPgpnCT8vsGmq9p5Z9HHnBSeQLnAFQi
const Eyh77Seed = Uint8Array.from([
    219, 139, 131, 236, 34, 125, 165, 13, 18, 248, 93, 160, 73, 236, 214, 251, 179, 235, 124, 126, 56, 47, 222, 28, 166,
    239, 130, 126, 66, 127, 26, 187, 207, 173, 205, 133, 48, 102, 2, 219, 20, 234, 72, 102, 53, 122, 175, 166, 198, 11,
    198, 248, 59, 40, 137, 208, 193, 138, 197, 171, 147, 124, 212, 175,
]);

// Identities - both of these are wallets that exists on devnet, we clone them each time and init from the privatekey
// This is us, the UXD deployment admins // aca3VWxwBeu8FTZowJ9hfSKGzntjX68EXh1N9xpE1PC
const adminKeypair = Keypair.fromSecretKey(aca3VWSeed);
export const authority: Signer = adminKeypair;
console.log(`CONTROLLER AUTHORITY => 🔗 https://solscan.io/account/${authority.publicKey}?cluster=devnet`);
// This is the user //
const bankKeypair = Keypair.fromSecretKey(Eyh77Seed);
export const bank: Signer = bankKeypair;
console.log(`BANK => 🔗https://solscan.io/account/${bank.publicKey}?cluster=devnet`);

// Get this from anchor.toml TODO
export const CLUSTER = 'devnet';

// ----------------------------------------------------------------------------
export const uxdProgramId: PublicKey = new PublicKey(jsonIdl["metadata"]["address"]);
console.debug(`UXD PROGRAM ID == ${uxdProgramId}`);
export const uxdClient = new UXDClient(uxdProgramId);

// Used in mercurial vault tests
export const SOLEND_USDC_DEVNET = new PublicKey('zVzi5VAf4qMEwzv7NXECVx5v2pQ7xnqVVjCXZwS9XzA');
export const SOLEND_USDC_DEVNET_DECIMALS = 6;

export const slippageBase = 1000;
