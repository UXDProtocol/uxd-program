import {
  getMarketByPublicKey,
  getTokenByMint,
  MangoAccount,
  MangoCache,
  MangoGroup,
  nativeI80F48ToUi,
  PerpMarket,
  PerpMarketConfig,
  QUOTE_INDEX,
  TokenConfig,
  zeroKey,
  ZERO_BN,
  ZERO_I80F48,
} from "@blockworks-foundation/mango-client";
import { BN } from "@project-serum/anchor";
import { ControllerUXD, Depository } from "@uxdprotocol/solana-usds-client";
import { controller } from "./test_integration_admin";
import { PublicKey } from "@solana/web3.js";
import { EOL } from "os";
import { provider } from "./provider";

export async function printMangoPDAInfo(depository: Depository) {
  const mango = controller.mango;
  const mangoPda = ControllerUXD.mangoPda(depository.collateralMint);
  const groupConfig = mango.groupConfig;
  const mangoGroup = mango.group;
  const cache = await mango.group.loadCache(mango.client.connection);
  const mangoAccount = await mango.client.getMangoAccount(mangoPda, mango.programId);
  const lines: string[] = [];

  // Bug cause idk some perp market name is undefined on devnet?
  // mangoAccount.toPrettyString(groupConfig, mangoGroup, mangoCache);

  // lines.push("MangoAccount " + mangoAccount.publicKey.toBase58());
  // lines.push("Owner: " + mangoAccount.owner.toBase58());
  lines.push(
    "Maint Health Ratio:                     " + mangoAccount.getHealthRatio(mangoGroup, cache, "Maint").toFixed(2)
  );
  // lines.push("Maint Health: " + mangoAccount.getHealth(mangoGroup, cache, "Maint").toFixed(4));
  // lines.push("Init Health: " + mangoAccount.getHealth(mangoGroup, cache, "Init").toFixed(4));
  lines.push("Equity:                                 " + mangoAccount.computeValue(mangoGroup, cache).toFixed(4));
  if (mangoAccount.isBankrupt) {
    lines.push("!!!!! isBankrupt: " + mangoAccount.isBankrupt);
  }
  if (mangoAccount.beingLiquidated) {
    lines.push("!!!!! beingLiquidated:    " + mangoAccount.beingLiquidated);
  }

  const quoteAdj = new BN(10).pow(new BN(mangoGroup.tokens[QUOTE_INDEX].decimals));

  for (let i = 0; i < mangoGroup.tokens.length; i++) {
    if (mangoGroup.tokens[i].mint.equals(zeroKey)) {
      continue;
    }
    const token = getTokenByMint(groupConfig, mangoGroup.tokens[i].mint) as TokenConfig;

    let baseInOrders = ZERO_BN;
    let quoteInOrders = ZERO_BN;
    const openOrders = i !== QUOTE_INDEX ? mangoAccount.spotOpenOrdersAccounts[i] : undefined;

    if (openOrders) {
      const baseAdj = new BN(10).pow(new BN(mangoGroup.tokens[i].decimals));

      baseInOrders = openOrders.baseTokenTotal.div(baseAdj);
      quoteInOrders = openOrders.quoteTokenTotal.add(openOrders["referrerRebatesAccrued"]).div(quoteAdj);
    }
    const net = nativeI80F48ToUi(mangoAccount.getNet(cache.rootBankCache[i], i), mangoGroup.tokens[i].decimals);

    if (net.eq(ZERO_I80F48) && baseInOrders.isZero() && quoteInOrders.isZero()) {
      continue;
    }

    lines.push(
      `SPOT - ${token.symbol}:     [Net Balance: ${net.toFixed(6)}] / [Base In Orders: ${baseInOrders
        .toNumber()
        .toFixed(4)}] / [Quote In Orders: ${quoteInOrders.toNumber().toFixed(4)}]`
    );

    for (let i = 0; i < 2 /*mangoAccount.perpAccounts.length*/; i++) {
      if (mangoGroup.perpMarkets[i].perpMarket.equals(zeroKey)) {
        continue;
      }
      const market = getMarketByPublicKey(groupConfig, mangoGroup.perpMarkets[i].perpMarket) as PerpMarketConfig;

      const perpAccount = mangoAccount.perpAccounts[i];
      const perpMarketInfo = mangoGroup.perpMarkets[i];
      lines.push(
        `PERP - ${market.name}:   [Base Pos: ${getBasePositionUiWithGroup(mangoAccount, i, mangoGroup).toFixed(
          6
        )}] / [Quote Pos: ${(
          perpAccount.getQuotePosition(cache.perpMarketCache[i]).toNumber() / quoteAdj.toNumber()
        ).toFixed(4)}] / [Unsettled Funding: ${(
          perpAccount.getUnsettledFunding(cache.perpMarketCache[i]).toNumber() / quoteAdj.toNumber()
        ).toFixed(4)}] / [Health: ${perpAccount
          .getHealth(
            perpMarketInfo,
            cache.priceCache[i].price,
            perpMarketInfo.maintAssetWeight,
            perpMarketInfo.maintLiabWeight,
            cache.perpMarketCache[i].longFunding,
            cache.perpMarketCache[i].shortFunding
          )
          .toFixed(2)}]`
      );
    }
    console.log(`        * [Depository ${depository.collateralSymbol} Mango Account informations]`);
    console.log(lines.map((line, index) => `        *       ${line.replace(":", ":  ")}`).join(EOL));
  }
}

/**
 * Return the open orders keys in basket and replace open orders not in basket with zero key
 */
export function getOpenOrdersKeysInBasket(account: MangoAccount): PublicKey[] {
  return account.spotOpenOrders.map((pk, i) => (account.inMarginBasket[i] ? pk : zeroKey));
}

/**
 *  Return the current position for the market at `marketIndex` in UI units
 *  e.g. if you buy 1 BTC in the UI, you're buying 1,000,000 native BTC,
 *  10,000 BTC-PERP contracts and exactly 1 BTC in UI
 *  Find the marketIndex in the ids.json list of perp markets
 */
export function getPerpPositionUi(account: MangoAccount, marketIndex: number, perpMarket: PerpMarket): number {
  return account.perpAccounts[marketIndex].getBasePositionUi(perpMarket);
}
/**
 *  Return the current position for the market at `marketIndex` in UI units
 *  e.g. if you buy 1 BTC in the UI, you're buying 1,000,000 native BTC,
 *  10,000 BTC-PERP contracts and exactly 1 BTC in UI
 *  Find the marketIndex in the ids.json list of perp markets
 */
export function getBasePositionUiWithGroup(account: MangoAccount, marketIndex: number, group: MangoGroup): number {
  return (
    account.perpAccounts[marketIndex].basePosition.mul(group.perpMarkets[marketIndex].baseLotSize).toNumber() /
    Math.pow(10, group.tokens[marketIndex].decimals)
  );
}

/**
 * Return the equity in standard UI numbers. E.g. if equity is $100, this returns 100
 */
export function getEquityUi(account: MangoAccount, mangoGroup: MangoGroup, mangoCache: MangoCache): number {
  return (
    account.computeValue(mangoGroup, mangoCache).toNumber() / Math.pow(10, mangoGroup.tokens[QUOTE_INDEX].decimals)
  );
}

/**
 * This is the init health divided by quote decimals
 */
export function getCollateralValueUi(account: MangoAccount, mangoGroup: MangoGroup, mangoCache: MangoCache): number {
  return (
    account.getHealth(mangoGroup, mangoCache, "Init").toNumber() / Math.pow(10, mangoGroup.tokens[QUOTE_INDEX].decimals)
  );
}
