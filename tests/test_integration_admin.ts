import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { ControllerUXD } from "./solana-usds-client/controller";
import { Depository } from "./solana-usds-client/depository";
import { BTC, admin } from "./identities";
import { expect, util } from "chai";
import { BTC_DECIMALS } from "./solana-usds-client/utils";
import { TXN_OPTS, provider } from "./provider";

// Depositories - They represent the business object that tie a mint to a depository
export let depositoryBTC = new Depository(BTC, "BTC", BTC_DECIMALS);
export let controller = new ControllerUXD("devnet");

before("Airdrop and config", async () => {
  // GIVEN
  await controller.mango.setupMangoGroup(); // Async fetch of mango group

  let sig = await provider.connection.requestAirdrop(admin.publicKey, 10 * LAMPORTS_PER_SOL);
  await provider.connection.confirmTransaction(sig);
});

before("Standard Administrative flow for UXD Controller and depositories", () => {
  it("Create UXD Controller", async () => {
    // GIVEN

    // WHEN - solana 1.8 and anchor 1.8 we can handle that program side, but not sure that's desirable? maybe less explicit
    if (await provider.connection.getAccountInfo(ControllerUXD.statePda)) {
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
    if (
      await provider.connection.getAccountInfo(ControllerUXD.collateralPassthroughPda(depositoryBTC.collateralMint))
    ) {
      console.log("already registered.");
    } else {
      await controller.register(depositoryBTC, admin, TXN_OPTS);
    }
  });
});
