import { ControllerUXD, Depository, BTC_DECIMALS } from "@uxdprotocol/solana-usds-client";
import { BTC, admin } from "./identities";
import { expect, util } from "chai";
import { TXN_OPTS, provider } from "./provider";
import { Program, workspace } from "@project-serum/anchor";

// Depositories - They represent the business object that tie a mint to a depository
const controllerProgram = workspace.Controller as Program;
export let depositoryBTC = new Depository(BTC, "BTC", BTC_DECIMALS);
console.log(controllerProgram.programId);
export let controller = new ControllerUXD(provider, controllerProgram, "devnet");

before("Airdrop and config", async () => {
  // GIVEN
  await controller.mango.setupMangoGroup(); // Async fetch of mango group

  let perpMarketConfigBTC = controller.mango.getPerpMarketConfigFor(depositoryBTC.collateralSymbol);
  let perpMarketIndexBTC = perpMarketConfigBTC.marketIndex;
  let perpMarketBTC = await controller.mango.group.loadPerpMarket(
    controller.mango.client.connection,
    perpMarketIndexBTC,
    perpMarketConfigBTC.baseDecimals,
    perpMarketConfigBTC.quoteDecimals
  );
  console.log("--- Printing the Mango BTC perp market informations ---------------- ");
  console.log(perpMarketBTC.toPrettyString(perpMarketConfigBTC));
  console.log("-------------------------------------------------------------------- \n");
  console.log("--------------------------- START TESTS ---------------------------- \n");
});

before("Standard Administrative flow for UXD Controller and depositories", () => {
  it("Create UXD Controller", async () => {
    // GIVEN

    // WHEN - solana 1.8 and anchor 1.8 we can handle that program side, but not sure that's desirable? maybe less explicit
    if (await provider.connection.getAccountInfo(controller.statePda)) {
      console.log("already initialized.");
    } else {
      await controller.initialize(admin, TXN_OPTS);
    }

    // THEN
    // XXX add asserts
  });

  it("Register BTC Depository with Controller", async () => {
    // GIVEN

    // WHEN
    if (await provider.connection.getAccountInfo(controller.collateralPassthroughPda(depositoryBTC.collateralMint))) {
      console.log("already registered.");
    } else {
      await controller.register(depositoryBTC, admin, TXN_OPTS);
    }
  });
});
