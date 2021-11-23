import { workspace } from "@project-serum/anchor";
import { Mango, Controller, UXD_DECIMALS, Depository, BTC_DECIMALS, SOL_DECIMALS, UXD, createAndInitializeMango } from "@uxdprotocol/uxd-client";
import { BTC, WSOL } from "./identities";
import { provider } from "./provider";

const uxdProgram = workspace.Uxd;

export const slippageBase = 1000;

export let mango: Mango;

console.log(`UXD PROGRAM ID == ${uxdProgram.programId}`);

// Controller - The UXD mint keeper
export const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgram.programId);

// Depositories - An account that manage a Collateral mint for the controller
export const depositoryBTC = new Depository(BTC, "BTC", BTC_DECIMALS, uxdProgram.programId);
export const depositoryWSOL = new Depository(WSOL, "SOL", SOL_DECIMALS, uxdProgram.programId);

// Client interface to the Web3 call to `UXD-Program`
export const uxdClient = new UXD(provider, uxdProgram);

// ----------------------------------------------------------------------------

before(" ======= [Suite 0 : Initialize mango (1 op)] ======= ", async () => {
    mango = await createAndInitializeMango(provider, `devnet`);
});

// ----------------------------------------------------------------------------

before("PerpMarketConfig for BTC", async () => {
    const perpMarketConfigBTC = mango.getPerpMarketConfigFor(depositoryBTC.collateralMintSymbol);
    const perpMarketIndexBTC = perpMarketConfigBTC.marketIndex;
    const perpMarketBTC = await mango.group.loadPerpMarket(provider.connection, perpMarketIndexBTC, perpMarketConfigBTC.baseDecimals, perpMarketConfigBTC.quoteDecimals);
    console.log("--- Printing the Mango BTC perp market informations ---------------- ");
    console.log(perpMarketBTC.toPrettyString(mango.group, perpMarketConfigBTC));
});

before("PerpMarketConfig for WSOL", async () => {
    const perpMarketConfigWSOL = mango.getPerpMarketConfigFor(depositoryWSOL.collateralMintSymbol);
    const perpMarketIndexWSOL = perpMarketConfigWSOL.marketIndex;
    const perpMarketWSOL = await mango.group.loadPerpMarket(provider.connection, perpMarketIndexWSOL, perpMarketConfigWSOL.baseDecimals, perpMarketConfigWSOL.quoteDecimals);
    console.log("--- Printing the Mango BTC perp market informations ---------------- ");
    console.log(perpMarketWSOL.toPrettyString(mango.group, perpMarketConfigWSOL));
});

// ----------------------------------------------------------------------------
