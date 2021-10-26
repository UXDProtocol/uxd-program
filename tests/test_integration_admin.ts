import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { ControllerUXD } from "./utils/controller";
import { Depository } from "./utils/depository";
import { utils, BTC, admin, provider, connection, BTC_DECIMALS } from "./utils/utils";
import { expect, util } from "chai";

// Depositories - They represent the business object that tie a mint to a depository
export let depositoryBTC: Depository;

before("Setup mints and depositories", async () => {
  // GIVEN
  await utils.setupMango(); // Async fetch of mango group

  let sig = await connection.requestAirdrop(admin.publicKey, 10 * LAMPORTS_PER_SOL);
  await connection.confirmTransaction(sig);

  depositoryBTC = new Depository(BTC, "BTC", BTC_DECIMALS);
});

before("Standard Administrative flow for UXD Controller and depositories", () => {
  it("Create UXD Controller", async () => {
    // WHEN
    // WHEN solana 1.8 and anchor 1.8 we can handle that program side, but not sure that's desirable? maybe less explicit
    if (await provider.connection.getAccountInfo(ControllerUXD.statePda)) {
      console.log("already initialized.");
    } else {
      await ControllerUXD.initialize(admin);
    }

    // THEN
    // XXX add asserts
  });

  it("Register BTC Depository with Controller", async () => {
    // GIVEN

    // WHEN
    if (
      await provider.connection.getAccountInfo(ControllerUXD.collateralPassthroughPda(depositoryBTC.collateralMint))
    ) {
      console.log("already registered.");
    } else {
      await ControllerUXD.register(depositoryBTC, admin);
    }
  });
});
