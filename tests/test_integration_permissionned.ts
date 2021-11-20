import { authority } from "./identities";
import { expect, util } from "chai";
import { provider } from "./provider";
import { createAndInitializeMango, Mango } from "@uxdprotocol/uxd-client";
import { depositoryBTC, controllerUXD, initializeControllerIfNeeded, registerMangoDepositoryIfNeeded, depositoryWSOL } from "./uxdApi";

// Util object to interact with Mango, dependency for MangoDepositories
let mango: Mango;

before("Load Mango + print perpMarketConfig for BTC", async () => {
  mango = await createAndInitializeMango(provider, `devnet`);
  const perpMarketConfigBTC = mango.getPerpMarketConfigFor(depositoryBTC.collateralMintSymbol);
  const perpMarketIndexBTC = perpMarketConfigBTC.marketIndex;
  const perpMarketBTC = await mango.group.loadPerpMarket(provider.connection, perpMarketIndexBTC, perpMarketConfigBTC.baseDecimals, perpMarketConfigBTC.quoteDecimals);
  console.log("--- Printing the Mango BTC perp market informations ---------------- ");
  console.log(perpMarketBTC.toPrettyString(mango.group, perpMarketConfigBTC));
  console.log("--------------------------- START TESTS ---------------------------- \n");
});

before("Permissionned operations", () => {

  it("Initialize UXD Controller", async () => {
    // GIVEN
    let caller = authority;
    let controller = controllerUXD;

    // WHEN
    await initializeControllerIfNeeded(caller, controller, mango);

    // THEN
    controller.info();
  });

  it("Register BTC Depository to the Controller", async () => {
    // GIVEN
    let caller = authority;
    let controller = controllerUXD;
    let depository = depositoryBTC;

    // WHEN
    let txId = await registerMangoDepositoryIfNeeded(caller, controller, depository, mango);

    // THEN
    console.log(`txId : ${txId}`);
    depository.info();
  });

  it("Register WSOL Depository to the Controller", async () => {
    // GIVEN
    let caller = authority;
    let controller = controllerUXD;
    let depository = depositoryWSOL;

    // WHEN
    let txId = await registerMangoDepositoryIfNeeded(caller, controller, depository, mango);

    // THEN
    console.log(`txId : ${txId}`);
    depository.info();
  });
});
