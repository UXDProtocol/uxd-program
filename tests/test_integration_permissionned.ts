import { ControllerUXD, Depository, BTC_DECIMALS, SOL_DECIMALS } from "@uxdprotocol/solana-usds-client";
import { BTC, admin, WSOL } from "./identities";
import { expect, util } from "chai";
import { TXN_OPTS, provider } from "./provider";
import { workspace } from "@project-serum/anchor";
import { NodeWallet } from "@project-serum/anchor/dist/cjs/provider";

const uxdProgram = workspace.Uxd;

// Depositories - They represent the business object that tie a mint to a depository
export const depositoryBTC = new Depository(BTC, "BTC", BTC_DECIMALS);
export const depositoryWSOL = new Depository(WSOL, "SOL", SOL_DECIMALS);
export const controllerUXD = new ControllerUXD(provider, uxdProgram, "devnet");

before("Initial configuration", async () => {
  await controllerUXD.mango.setupMangoGroup(); // Async fetch of mango group

  const perpMarketConfigBTC = controllerUXD.mango.getPerpMarketConfigFor(depositoryBTC.collateralSymbol);
  const perpMarketIndexBTC = perpMarketConfigBTC.marketIndex;
  const perpMarketBTC = await controllerUXD.mango.group.loadPerpMarket(
    controllerUXD.mango.client.connection,
    perpMarketIndexBTC,
    perpMarketConfigBTC.baseDecimals,
    perpMarketConfigBTC.quoteDecimals
  );
  console.log("--- Printing the Mango BTC perp market informations ---------------- ");
  console.log(perpMarketBTC.toPrettyString(perpMarketConfigBTC));
  console.log("-------------------------------------------------------------------- \n");
  console.log("--------------------------- START TESTS ---------------------------- \n");
});

before("Permissionned operations", () => {

  it("Initialize UXD Controller", async () => {
    // GIVEN
    let caller = admin;
    let controller = controllerUXD;

    // WHEN
    await initializeControllerIfNeeded(caller, controller);

    // THEN
  });

  it("Register BTC Depository to the Controller", async () => {
    // GIVEN
    let caller = admin;
    let controller = controllerUXD;
    let depository = depositoryBTC;

    // WHEN
    let txId = await registerDepositoryIfNeeded(caller, controller, depository);

    // THEN
    console.log(`txId : ${txId}`);
    depositoryBTC.info(controllerUXD);
  });

  it("Register WSOL Depository to the Controller", async () => {
    // GIVEN
    let caller = admin;
    let controller = controllerUXD;
    let depository = depositoryWSOL;

    // WHEN
    let txId = await registerDepositoryIfNeeded(caller, controller, depository);

    // THEN
    console.log(`txId : ${txId}`);
    depositoryWSOL.info(controllerUXD);
  });
});

async function initializeControllerIfNeeded(admin: NodeWallet, controller: ControllerUXD): Promise<string> {
      if (await provider.connection.getAccountInfo(controller.statePda)) {
      console.log("Already initialized.");
    } else {
      return controller.initialize(admin, TXN_OPTS);
    }
}

async function registerDepositoryIfNeeded(admin: NodeWallet, controller: ControllerUXD, depository: Depository): Promise<string> {
    if (await provider.connection.getAccountInfo(controller.mangoAccountPda(depository.collateralMint)[0])) {
      console.log("Already registered.");
    } else {
      return controller.registerDepository(depository, admin, TXN_OPTS);
    }
}