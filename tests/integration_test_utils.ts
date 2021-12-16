import { MangoDepository, findATAAddrSync, Mango } from "@uxdprotocol/uxd-client";
import { BTC, user, WSOL } from "./identities";
import { provider, TXN_COMMIT, TXN_OPTS } from "./provider";
import { Account, PublicKey } from "@solana/web3.js";
import { controllerUXD, uxdClient, uxdHelpers } from "./test_0_consts";
import { nativeI80F48ToUi, nativeToUi } from "@blockworks-foundation/mango-client";

// User's SPL Accounts
export const userBTCATA: PublicKey = findATAAddrSync(user.publicKey, BTC)[0];
export const userWSOLATA: PublicKey = findATAAddrSync(user.publicKey, WSOL)[0];
export const userUXDATA: PublicKey = findATAAddrSync(user.publicKey, controllerUXD.redeemableMintPda)[0];

export function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

export function getBalance(tokenAccount: PublicKey): Promise<number> {
    return provider.connection
        .getTokenAccountBalance(tokenAccount, TXN_COMMIT)
        .then((o) => o["value"]["uiAmount"])
        .catch(() => null);
}

export async function printDepositoryInfo(depository: MangoDepository, mango: Mango) {
    const SYM = depository.collateralMintSymbol;
    console.log(`\
        * [Depository ${SYM}]
        *     collateral_passthrough:                     ${await getBalance(depository.collateralPassthroughPda)}`);

    let controllerAccount = await uxdHelpers.getControllerAccount(provider, uxdClient.program, controllerUXD, TXN_OPTS);
    let depositoryAccount = await uxdHelpers.getMangoDepositoryAccount(provider, uxdClient.program, depository, TXN_OPTS);

    console.log("        * - onchain accounting -")
    console.log(`        *     accounting deltaNeutralQuotePosition        ${depositoryAccount.deltaNeutralQuotePosition.toNumber()} (${depositoryAccount.redeemableAmountUnderManagement.toNumber()} + ${depositoryAccount.deltaNeutralQuoteFeeOffset.toNumber()} <> redeemableAmountUnderManagement + deltaNeutralQuoteFeeOffset)`);
    console.log(`        *     collateralAmountDeposited                   ${depositoryAccount.collateralAmountDeposited.toNumber()}`);
    console.log(`        *     deltaNeutralQuoteFeeOffset                  ${depositoryAccount.deltaNeutralQuoteFeeOffset.toNumber()}`);
    console.log(`        *     redeemableAmountUnderManagement             ${depositoryAccount.redeemableAmountUnderManagement.toNumber()} / ${controllerAccount.redeemableCirculatingSupply.toNumber()} (controller.redeemableCirculatingSupply)`);
    console.log(`        *     insuranceAmountDeposited                    ${depositoryAccount.insuranceAmountDeposited.toNumber()}`);

    const mangoAccount = await mango.load(depository.mangoAccountPda);
    // await mango.printAccountInfo(mangoAccount);
    const pmi = mango.getPerpMarketConfig(SYM).marketIndex;
    const smi = mango.getSpotMarketConfig(SYM).marketIndex;
    const sti = mango.getTokenIndex(depository.collateralMint);
    const pa = mangoAccount.perpAccounts[pmi];
    const pm = await mango.getPerpMarket(SYM);
    const cache = await mango.group.loadCache(provider.connection);

    console.log("        * - underlying mango account -");
    console.log(`        *      quote value                                ${nativeI80F48ToUi(mangoAccount.computeValue(mango.group, cache), 6).toFixed(6)}`);
    console.log(`        *      ${SYM} SPOT Pos                               ${nativeI80F48ToUi(mangoAccount.getNet(cache.rootBankCache[smi], sti), 9).toFixed(9)}`);
    console.log(`        *      ${SYM}-PERP Base Pos                          ${nativeToUi(pm.baseLotsToNative(pa.basePosition).toNumber(), 9).toFixed(9)}`);
    console.log(`        *      ${SYM}-PERP Quote Pos                         ${nativeI80F48ToUi(pa.quotePosition, 6).toNumber().toFixed(6)}`);
    console.log(`        *      ${SYM}-PERP Unsettled Funding                 ${nativeI80F48ToUi(pa.getUnsettledFunding(cache.perpMarketCache[pmi]), 6).toNumber().toFixed(6)}`);
    console.log(`        *      ${SYM}-PERP PnL                               ${nativeI80F48ToUi(pa.getPnl(mango.group.perpMarkets[pmi], cache.perpMarketCache[pmi], cache.priceCache[pmi].price), 6).toNumber().toFixed(6)}`);
    // console.log("takerBase ", pm.baseLotsToNative(pa.takerBase).toFixed(9));
    // console.log("takerQuote ", nativeToUi(pa.takerQuote.toNumber(), 6).toFixed(6));
    // console.log("- bidsQuantity ", nativeToUi(pa.bidsQuantity.toNumber(), 6));
    // console.log("- asksQuantity ", nativeToUi(pa.asksQuantity.toNumber(), 6));
    console.log(`        *     mngoAccrued ${nativeToUi(pa.mngoAccrued.toNumber(), 9)}`);
    console.log(`        *     longSettledFunding                          ${nativeI80F48ToUi(pa.longSettledFunding, 6).toNumber().toFixed(6)}`);
    console.log(`        *     shortSettledFunding                         ${nativeI80F48ToUi(pa.shortSettledFunding, 6).toNumber().toFixed(6)}`);
    console.log(`        *     getAssetVal                                 ${nativeI80F48ToUi(pa.getAssetVal(mango.group.perpMarkets[pmi], cache.priceCache[pmi].price, pm.shortFunding, pm.longFunding), 6).toNumber().toFixed(6)}`);
}

export async function printUserBalances() {
    console.log(`\
        * [user]:
        *     BTC:                                        ${await getBalance(userBTCATA)}
        *     WSOL:                                       ${await getBalance(userWSOLATA)}
        *     UXD:                                        ${await getBalance(userUXDATA)}`);
}

export function printWorldInfo() {
    console.log(`\
        * BTC mint:                                       ${BTC.toString()}
        * WSOL mint:                                      ${WSOL.toString()}
        * UXD mint:                                       ${controllerUXD.redeemableMintPda.toString()}`);
}