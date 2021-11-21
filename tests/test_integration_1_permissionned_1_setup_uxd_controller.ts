import { authority } from "./identities";
import { expect, util } from "chai";
import { provider } from "./provider";
import { depositoryBTC, controllerUXD, initializeController, registerMangoDepository, depositoryWSOL, mango } from "./test_integration_0_setup_uxd_api";

before(" ======= [Suite 1-1 : Test setup UXD controller (permissionned) (1 op)] ======= ", () => {
  beforeEach("\n", async () => { });
  afterEach("\n", async () => { });

  it("Initialize Controller for UXD", async () => {
    // GIVEN
    const caller = authority;
    const controller = controllerUXD;

    // WHEN
    if (await provider.connection.getAccountInfo(controller.pda)) {
      console.log("Already initialized.");
    } else {
      const txId = await initializeController(caller, controller);
      console.log(`txId : ${txId}`);
    }

    // THEN
    controller.info();
  });
});
