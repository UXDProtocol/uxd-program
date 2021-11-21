import { authority } from "./identities";
import { expect, util } from "chai";
import { provider } from "./provider";
import { depositoryBTC, controllerUXD, initializeControllerIfNeeded, registerMangoDepositoryIfNeeded, depositoryWSOL, mango } from "./uxdApi";

// before("PerpMarketConfig for BTC", async () => {
//   const perpMarketConfigBTC = mango.getPerpMarketConfigFor(depositoryBTC.collateralMintSymbol);
//   const perpMarketIndexBTC = perpMarketConfigBTC.marketIndex;
//   const perpMarketBTC = await mango.group.loadPerpMarket(provider.connection, perpMarketIndexBTC, perpMarketConfigBTC.baseDecimals, perpMarketConfigBTC.quoteDecimals);
//   console.log("--- Printing the Mango BTC perp market informations ---------------- ");
//   console.log(perpMarketBTC.toPrettyString(mango.group, perpMarketConfigBTC));
//   console.log("--------------------------- START TESTS ---------------------------- \n");
// });

before("PerpMarketConfig for WSOL", async () => {
  const perpMarketConfigWSOL = mango.getPerpMarketConfigFor(depositoryWSOL.collateralMintSymbol);
  const perpMarketIndexWSOL = perpMarketConfigWSOL.marketIndex;
  const perpMarketWSOL = await mango.group.loadPerpMarket(provider.connection, perpMarketIndexWSOL, perpMarketConfigWSOL.baseDecimals, perpMarketConfigWSOL.quoteDecimals);
  console.log("--- Printing the Mango BTC perp market informations ---------------- ");
  console.log(perpMarketWSOL.toPrettyString(mango.group, perpMarketConfigWSOL));
  console.log("--------------------------- START TESTS ---------------------------- \n");
});

before("Permissionned operations", () => {

  beforeEach("\n", async () => { });
  afterEach("\n", async () => { });

  it("Initialize UXD Controller", async () => {
    // GIVEN
    const caller = authority;
    const controller = controllerUXD;

    // WHEN
    const txId = await initializeControllerIfNeeded(caller, controller);

    // THEN
    console.log(`txId : ${txId}`);
    controller.info();
  });

  it("Register BTC Depository to the Controller", async () => {
    // GIVEN
    const caller = authority;
    const controller = controllerUXD;
    const depository = depositoryBTC;

    // WHEN
    const txId = await registerMangoDepositoryIfNeeded(caller, controller, depository, mango);

    // THEN
    console.log(`txId : ${txId}`);
    depository.info();
  });

  it("Register WSOL Depository to the Controller", async () => {
    // GIVEN
    const caller = authority;
    const controller = controllerUXD;
    const depository = depositoryWSOL;

    // WHEN
    const txId = await registerMangoDepositoryIfNeeded(caller, controller, depository, mango);

    // THEN
    console.log(`txId : ${txId}`);
    depository.info();
  });
});
