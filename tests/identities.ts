import { PublicKey, Keypair, Signer } from "@solana/web3.js";

// TESTING wallets for convenience (The user and admin). To remove when going open source

// aca3VWxwBeu8FTZowJ9hfSKGzntjX68EXh1N9xpE1PC
const aca3VWSeed = Uint8Array.from([
  here replace with the authority - Normally it's the DAO but I used my personnal one for the test
]);
// Eyh77zP5b7arPtPgpnCT8vsGmq9p5Z9HHnBSeQLnAFQi
const Eyh77Seed = Uint8Array.from([
  219, 139, 131, 236, 34, 125, 165, 13, 18, 248, 93, 160, 73, 236, 214, 251, 179, 235, 124, 126, 56, 47, 222, 28, 166,
  239, 130, 126, 66, 127, 26, 187, 207, 173, 205, 133, 48, 102, 2, 219, 20, 234, 72, 102, 53, 122, 175, 166, 198, 11,
  198, 248, 59, 40, 137, 208, 193, 138, 197, 171, 147, 124, 212, 175,
]);

// Identities - both of these are wallets that exists on devnet, we clone them each time and init from the privatekey
// This is us, the UXD deployment admins // aca3VWxwBeu8FTZowJ9hfSKGzntjX68EXh1N9xpE1PC
let adminKeypair = Keypair.fromSecretKey(aca3VWSeed);
export let authority: Signer = adminKeypair;
console.log(`CONTROLLER AUTHORITY KEY => ${authority.publicKey}`);
// This is the user //
let userKeypair = Keypair.fromSecretKey(Eyh77Seed);
export let user: Signer = userKeypair;
console.log(`USER KEY => ${user.publicKey}`);


export const WSOL = new PublicKey("So11111111111111111111111111111111111111112");

// Devnet
// export const USDC = new PublicKey("8FRFC6MoGGkMFQwngccyu69VnYbzykGeez7ignHVAFSN");
// export const BTC = new PublicKey("3UNBZ6o52WTWwjac2kPUb4FyodhU1vFkRJheu1Sh2TvU");

// Mainnet 
export const USDC = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
export const BTC = new PublicKey("9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E");
