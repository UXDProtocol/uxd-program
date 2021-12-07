import { authority } from "./identities";
import { provider } from "./provider";
import { controllerUXD } from "./test_0_consts";
import { initializeController, getControllerAccount } from "./test_0_uxd_api";

describe(" ======= [Suite 1-1 : Test setup UXD controller (permissionned) (1 op)] ======= ", () => {
  beforeEach("\n", async () => { });
  afterEach("\n", async () => { });

  it("1 - Initialize Controller for UXD", async () => {
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
    const controllerAccount = await getControllerAccount(controller);
    controller.info();
    console.log(controllerAccount);
  });
});
