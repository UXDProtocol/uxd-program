import { Wallet } from "@project-serum/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";

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
let adminKeypair = Keypair.fromSecretKey(aca3VWSeed);
export let authority = new Wallet(adminKeypair);
console.log(`CONTROLLER AUTHORITY KEY => ${authority.publicKey}`);
// This is the user //
let userKeypair = Keypair.fromSecretKey(Eyh77Seed);
export let user = new Wallet(userKeypair);
console.log(`USER KEY => ${user.publicKey}`);

// Mints cloned from devnet to interact with mango
export const USDC = new PublicKey("8FRFC6MoGGkMFQwngccyu69VnYbzykGeez7ignHVAFSN");
export const BTC = new PublicKey("3UNBZ6o52WTWwjac2kPUb4FyodhU1vFkRJheu1Sh2TvU");
export const WSOL = new PublicKey("So11111111111111111111111111111111111111112");


// import {
//   getTokenByMint,
//   MangoAccount,
//   MangoCache,
//   MangoGroup,
//   nativeI80F48ToUi,
//   PerpMarket,
//   QUOTE_INDEX,
//   TokenConfig,
//   zeroKey,
//   ZERO_BN,
//   ZERO_I80F48,
// } from "@blockworks-foundation/mango-client";
// import { BN } from "@project-serum/anchor";
// import { PublicKey } from "@solana/web3.js";
// import { EOL } from "os";
// import { ControllerUXD, Depository } from "@uxdprotocol/uxd-client";

// export async function printMangoPDAInfo(depository: Depository, controller: ControllerUXD) {
//   const mango = controller.mango;
//   const mangoPda = controller.mangoAccountPda(depository.collateralMint)[0];
//   const groupConfig = mango.groupConfig;
//   const mangoGroup = mango.group;
//   const cache = await mango.group.loadCache(mango.client.connection);
//   const mangoAccount = await mango.client.getMangoAccount(mangoPda, mango.programId);
//   const lines: string[] = [];

//   // Bug cause idk some perp market name is undefined on devnet?
//   // mangoAccount.toPrettyString(groupConfig, mangoGroup, mangoCache);

//   // lines.push("MangoAccount " + mangoAccount.publicKey.toBase58());
//   // lines.push("Owner: " + mangoAccount.owner.toBase58());
//   lines.push(
//     "Maint Health Ratio:                     " + mangoAccount.getHealthRatio(mangoGroup, cache, "Maint").toFixed(2)
//   );
//   // lines.push("Maint Health: " + mangoAccount.getHealth(mangoGroup, cache, "Maint").toFixed(4));
//   // lines.push("Init Health: " + mangoAccount.getHealth(mangoGroup, cache, "Init").toFixed(4));
//   lines.push("Equity:                                 " + mangoAccount.computeValue(mangoGroup, cache).toFixed(4));
//   if (mangoAccount.isBankrupt) {
//     lines.push("!!!!! isBankrupt: " + mangoAccount.isBankrupt);
//   }
//   if (mangoAccount.beingLiquidated) {
//     lines.push("!!!!! beingLiquidated:    " + mangoAccount.beingLiquidated);
//   }

//   const quoteAdj = new BN(10).pow(new BN(mangoGroup.tokens[QUOTE_INDEX].decimals));

//   for (let i = 0; i < mangoGroup.tokens.length; i++) {
//     if (mangoGroup.tokens[i].mint.equals(zeroKey)) {
//       continue;
//     }
//     const token = getTokenByMint(groupConfig, mangoGroup.tokens[i].mint) as TokenConfig;

//     let baseInOrders = ZERO_BN;
//     let quoteInOrders = ZERO_BN;
//     const openOrders = i !== QUOTE_INDEX ? mangoAccount.spotOpenOrdersAccounts[i] : undefined;

//     if (openOrders) {
//       const baseAdj = new BN(10).pow(new BN(mangoGroup.tokens[i].decimals));

//       baseInOrders = openOrders.baseTokenTotal.div(baseAdj);
//       quoteInOrders = openOrders.quoteTokenTotal.add(openOrders["referrerRebatesAccrued"]).div(quoteAdj);
//     }
//     const net = nativeI80F48ToUi(mangoAccount.getNet(cache.rootBankCache[i], i), mangoGroup.tokens[i].decimals);

//     if (net.eq(ZERO_I80F48) && baseInOrders.isZero() && quoteInOrders.isZero()) {
//       continue;
//     }

//     lines.push(
//       `SPOT - ${token.symbol}:     [Net Balance: ${net.toFixed(6)}] / [Base In Orders: ${baseInOrders
//         .toNumber()
//         .toFixed(4)}] / [Quote In Orders: ${quoteInOrders.toNumber().toFixed(4)}]`
//     );

//     for (let i = 0; i < mangoAccount.perpAccounts.length; i++) {
//       if (mangoGroup.perpMarkets[i].perpMarket.equals(zeroKey)) {
//         continue;
//       }
//       // const market = getMarketByPublicKey(groupConfig, mangoGroup.perpMarkets[i].perpMarket) as PerpMarketConfig;
//       const perpAccount = mangoAccount.perpAccounts[i];
//       const perpMarketInfo = mangoGroup.perpMarkets[i];
//       lines.push(
//         //${market.name}: fail sometimes ...
//         `PERP -  [Base Pos: ${getBasePositionUiWithGroup(mangoAccount, i, mangoGroup).toFixed(6)}] / [Quote Pos: ${(
//           perpAccount.getQuotePosition(cache.perpMarketCache[i]).toNumber() / quoteAdj.toNumber()
//         ).toFixed(4)}] / [Unsettled Funding: ${(
//           perpAccount.getUnsettledFunding(cache.perpMarketCache[i]).toNumber() / quoteAdj.toNumber()
//         ).toFixed(4)}] / [Health: ${perpAccount
//           .getHealth(
//             perpMarketInfo,
//             cache.priceCache[i].price,
//             perpMarketInfo.maintAssetWeight,
//             perpMarketInfo.maintLiabWeight,
//             cache.perpMarketCache[i].longFunding,
//             cache.perpMarketCache[i].shortFunding
//           )
//           .toFixed(2)}]`
//       );
//     }
//     console.log(`        * [Depository ${depository.collateralSymbol} Mango Account informations]`);
//     console.log(lines.map((line, _index) => `        *       ${line.replace(":", ":  ")}`).join(EOL));
//   }
// }

// /**
//  * Return the open orders keys in basket and replace open orders not in basket with zero key
//  */
// export function getOpenOrdersKeysInBasket(account: MangoAccount): PublicKey[] {
//   return account.spotOpenOrders.map((pk, i) => (account.inMarginBasket[i] ? pk : zeroKey));
// }

// /**
//  *  Return the current position for the market at `marketIndex` in UI units
//  *  e.g. if you buy 1 BTC in the UI, you're buying 1,000,000 native BTC,
//  *  10,000 BTC-PERP contracts and exactly 1 BTC in UI
//  *  Find the marketIndex in the ids.json list of perp markets
//  */
// export function getPerpPositionUi(account: MangoAccount, marketIndex: number, perpMarket: PerpMarket): number {
//   return account.perpAccounts[marketIndex].getBasePositionUi(perpMarket);
// }
// /**
//  *  Return the current position for the market at `marketIndex` in UI units
//  *  e.g. if you buy 1 BTC in the UI, you're buying 1,000,000 native BTC,
//  *  10,000 BTC-PERP contracts and exactly 1 BTC in UI
//  *  Find the marketIndex in the ids.json list of perp markets
//  */
// export function getBasePositionUiWithGroup(account: MangoAccount, marketIndex: number, group: MangoGroup): number {
//   return (
//     account.perpAccounts[marketIndex].basePosition.mul(group.perpMarkets[marketIndex].baseLotSize).toNumber() /
//     Math.pow(10, group.tokens[marketIndex].decimals)
//   );
// }

// /**
//  * Return the equity in standard UI numbers. E.g. if equity is $100, this returns 100
//  */
// export function getEquityUi(account: MangoAccount, mangoGroup: MangoGroup, mangoCache: MangoCache): number {
//   return (
//     account.computeValue(mangoGroup, mangoCache).toNumber() / Math.pow(10, mangoGroup.tokens[QUOTE_INDEX].decimals)
//   );
// }

// /**
//  * This is the init health divided by quote decimals
//  */
// export function getCollateralValueUi(account: MangoAccount, mangoGroup: MangoGroup, mangoCache: MangoCache): number {
//   return (
//     account.getHealth(mangoGroup, mangoCache, "Init").toNumber() / Math.pow(10, mangoGroup.tokens[QUOTE_INDEX].decimals)
//   );
// }