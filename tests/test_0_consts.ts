// import { workspace } from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mango, Controller, UXD_DECIMALS, MangoDepository, BTC_DECIMALS, SOL_DECIMALS, UXD, createAndInitializeMango, USDC_DECIMALS, UXDHelpers } from "@uxdprotocol/uxd-client";
import { IDL } from "../target/types/uxd";
import { BTC, USDC, WSOL } from "./identities";
import { provider } from "./provider";
import * as jsonIdl from "../target/idl/uxd.json";

// const uxdProgram = workspace.Uxd;
const uxdProgram = new Program(IDL, jsonIdl["metadata"]["address"], provider); // Used for anchor test because case is not the same in idl and types.

export const slippageBase = 1000;

export let mango: Mango;

console.log(`UXD PROGRAM ID == ${uxdProgram.programId}`);

// Controller - The UXD mint keeper
export const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgram.programId);

// Depositories - An account that manage a Collateral mint for the controller
export const depositoryBTC = new MangoDepository(BTC, "BTC", BTC_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgram.programId);
export const depositoryWSOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgram.programId);

// Client interface to the Web3 call to `UXD-Program`
export const uxdClient = new UXD(uxdProgram);
export const uxdHelpers = new UXDHelpers();

export const accountUpdateSleepingInterval = 2500; // In milliseconds

// ----------------------------------------------------------------------------

before(" ======= [Suite 0 : Initialize mango (1 op)] ======= ", async () => {
    mango = await createAndInitializeMango(provider, `devnet`);
});

// ----------------------------------------------------------------------------

// before("PerpMarketConfig for BTC", async () => {
//     const perpMarketConfigBTC = mango.getPerpMarketConfigFor(depositoryBTC.collateralMintSymbol);
//     const perpMarketIndexBTC = perpMarketConfigBTC.marketIndex;
//     const perpMarketBTC = await mango.group.loadPerpMarket(provider.connection, perpMarketIndexBTC, perpMarketConfigBTC.baseDecimals, perpMarketConfigBTC.quoteDecimals);
//     console.log("--- Printing the Mango BTC perp market informations ---------------- ");
//     console.log(perpMarketBTC.toPrettyString(mango.group, perpMarketConfigBTC));
// });

// before("PerpMarketConfig for WSOL", async () => {
//     const perpMarketConfigWSOL = mango.getPerpMarketConfigFor(depositoryWSOL.collateralMintSymbol);
//     const perpMarketIndexWSOL = perpMarketConfigWSOL.marketIndex;
//     const perpMarketWSOL = await mango.group.loadPerpMarket(provider.connection, perpMarketIndexWSOL, perpMarketConfigWSOL.baseDecimals, perpMarketConfigWSOL.quoteDecimals);
//     console.log("--- Printing the Mango BTC perp market informations ---------------- ");
//     console.log(perpMarketWSOL.toPrettyString(mango.group, perpMarketConfigWSOL));
// });

// ----------------------------------------------------------------------------
