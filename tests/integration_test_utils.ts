import { MangoDepository, findATAAddrSync, Mango, SOL_DECIMALS } from "@uxdprotocol/uxd-client";
import { BTC, user, WSOL } from "./identities";
import { provider, TXN_COMMIT, TXN_OPTS } from "./provider";
import { PublicKey } from "@solana/web3.js";
import { controllerUXD, uxdClient, uxdHelpers } from "./test_0_consts";
import { nativeI80F48ToUi, nativeToUi } from "@blockworks-foundation/mango-client";
import { BN } from "@project-serum/anchor";

// User's SPL Accounts
export const userBTCATA: PublicKey = findATAAddrSync(user.publicKey, BTC)[0];
export const userWSOLATA: PublicKey = findATAAddrSync(user.publicKey, WSOL)[0];
export const userUXDATA: PublicKey = findATAAddrSync(user.publicKey, controllerUXD.redeemableMintPda)[0];

export function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function getSolBalance(wallet: PublicKey): Promise<number> {
    const lamports = await provider.connection
        .getBalance(wallet, TXN_COMMIT);
    return nativeToUi(lamports, SOL_DECIMALS);
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
    //
    let controllerAccount = await uxdHelpers.getControllerAccount(provider, uxdClient.program, controllerUXD, TXN_OPTS);
    let depositoryAccount = await uxdHelpers.getMangoDepositoryAccount(provider, uxdClient.program, depository, TXN_OPTS);
    //
    const mangoAccount = await mango.load(depository.mangoAccountPda);
    await mango.printAccountInfo(mangoAccount);
    const pmi = mango.getPerpMarketConfig(SYM).marketIndex;
    const smi = mango.getSpotMarketConfig(SYM).marketIndex;
    const sti = mango.getTokenIndex(depository.collateralMint);
    const pa = mangoAccount.perpAccounts[pmi];
    const pm = await mango.getPerpMarket(SYM);
    const cache = await mango.group.loadCache(provider.connection);
    const accountValue = mangoAccount.computeValue(mango.group, cache).toBig();
    const accountingInsuranceDepositedValue = nativeToUi(depositoryAccount.insuranceAmountDeposited.toNumber(), 6);
    const collateralSpotAmount = mangoAccount.getNet(cache.rootBankCache[smi], sti);
    const collateralDepositInterests = new BN(collateralSpotAmount.toNumber()).sub(depositoryAccount.collateralAmountDeposited);
    const accountValueMinusInsurance = accountValue - accountingInsuranceDepositedValue;
    const redeemableUnderManagement = nativeToUi(depositoryAccount.redeemableAmountUnderManagement.toNumber(), 6);

    console.log("        * - underlying mango account -");
    console.log(`        *     value                                       ${accountValue.toFixed(6)} (${(accountValueMinusInsurance).toFixed(6)} minus insurance)`);
    console.log("        *");
    console.log(`        *     ${SYM}-SPOT BASE Pos                           ${nativeI80F48ToUi(collateralSpotAmount, 9).toFixed(9)}`);
    console.log("        *");
    console.log(`        *     ${SYM}-PERP BASE Pos                          ${nativeToUi(pm.baseLotsToNative(pa.basePosition).toNumber(), 9).toFixed(9)}`);
    console.log(`        *     ${SYM}-PERP Quote Pos (lot)                    ${nativeToUi(pa.quotePosition, 6).toFixed(6)}`);
    console.log(`        *     ${SYM}-PERP BASE Tak                          ${nativeToUi(pm.baseLotsToNative(pa.takerBase).toNumber(), 9).toFixed(9)} (pending settlement)`);
    console.log(`        *     ${SYM}-PERP Quote Tak (lot)                    ${nativeToUi(pa.takerQuote, 6).toFixed(6)} (pending settlement`);
    console.log("        *");
    console.log(`        *     ${SYM}-PERP Unsettled Funding                  ${nativeI80F48ToUi(pa.getUnsettledFunding(cache.perpMarketCache[pmi]), 6).toNumber().toFixed(6)}`);
    // console.log(`        *     getLiabsVal                                 ${nativeI80F48ToUi(pa.getLiabsVal(mango.group.perpMarkets[pmi], cache.priceCache[pmi].price, pm.shortFunding, pm.longFunding), 6).toNumber().toFixed(6)}`);
    // console.log(`        *     getAssetVal                                 ${nativeI80F48ToUi(pa.getAssetVal(mango.group.perpMarkets[pmi], cache.priceCache[pmi].price, pm.shortFunding, pm.longFunding), 6).toNumber().toFixed(6)}`);
    // console.log(`        *     ${SYM}-PERP PnL                                ${nativeI80F48ToUi(pa.getPnl(mango.group.perpMarkets[pmi], cache.perpMarketCache[pmi], cache.priceCache[pmi].price), 6).toNumber().toFixed(6)}            ((perp_quote_pos + unsettled_funding) - liabsVal)`);
    // console.log("takerBase ", pm.baseLotsToNative(pa.takerBase).toFixed(9));
    // console.log("takerQuote ", nativeToUi(pa.takerQuote.toNumber(), 6).toFixed(6));
    // console.log("- bidsQuantity ", nativeToUi(pa.bidsQuantity.toNumber(), 6));
    // console.log("- asksQuantity ", nativeToUi(pa.asksQuantity.toNumber(), 6));
    // console.log(`        *     mngoAccrued                                 ${nativeToUi(pa.mngoAccrued.toNumber(), 9)}`);
    // console.log(`        *     longSettledFunding                          ${nativeI80F48ToUi(pa.longSettledFunding, 6).toNumber().toFixed(6)}`);
    // console.log(`        *     shortSettledFunding                         ${nativeI80F48ToUi(pa.shortSettledFunding, 6).toNumber().toFixed(6)}`);
    console.log("        * - onchain accounting -");
    console.log(`        *     insuranceAmountDeposited                    ${accountingInsuranceDepositedValue}`);
    console.log(`        *     collateralAmountDeposited                   ${nativeToUi(depositoryAccount.collateralAmountDeposited.toNumber(), 9).toFixed(9)}`);
    console.log(`        *     redeemableAmountUnderManagement             ${redeemableUnderManagement} / ${nativeToUi(controllerAccount.redeemableCirculatingSupply.toNumber(), 6)} (controller.redeemableCirculatingSupply)`);
    console.log(`        *     totalAmountPaidTakerFee                     ${nativeToUi(depositoryAccount.totalAmountPaidTakerFee.toNumber(), 6)}`);
    console.log("        * - derived -");
    console.log(`        *     depository PnL                              ${(accountValueMinusInsurance - redeemableUnderManagement).toFixed(6)}`);
    console.log(`        *     collateral deposit interests                ${nativeToUi(collateralDepositInterests.toNumber(), 9).toFixed(9)}`);
}

export async function printUserBalances() {
    console.log(`\
        * [user]:
        *     BTC:                                        ${await getBalance(userBTCATA)}
        *     SOL:                                        ${await getSolBalance(user.publicKey)}
        *     WSOL:                                       ${await getBalance(userWSOLATA)}
        *     UXD:                                        ${await getBalance(userUXDATA)}`);
}

export function printWorldInfo() {
    console.log(`\
        * BTC mint:                                       ${BTC.toString()}
        * WSOL mint:                                      ${WSOL.toString()}
        * UXD mint:                                       ${controllerUXD.redeemableMintPda.toString()}`);
}