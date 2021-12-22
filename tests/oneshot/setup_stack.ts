import { depositoryBTC, depositoryWSOL } from "../test_0_consts";
import { registerMangoDepository } from "../test_0_uxd_api";
import { authority } from "../identities";
import { provider } from "../provider";
import { controllerUXD } from "../test_0_consts";
import { initializeController, getControllerAccount } from "../test_0_uxd_api";
import { createAndInitializeMango, Mango } from "@uxdprotocol/uxd-client";

export let mango: Mango;
// ----------------------------------------------------------------------------

describe(" ======= [Suite 0 : Initialize mango (1 op)] ======= ", async () => {
    mango = await createAndInitializeMango(provider, `devnet`);
});

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

describe(" ======= [Suite 1-2 : Test setup UXD controller (permissionned) (2 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("\n", async () => { });

    it("1 - Register BTC Depository to the Controller", async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;
        const depository = depositoryBTC;

        // WHEN
        if (await provider.connection.getAccountInfo(depository.mangoAccountPda)) {
            console.log("Already registered.");
        } else {
            const txId = await registerMangoDepository(caller, controller, depository, mango);
            console.log(`txId : ${txId}`);
        }

        // THEN
        depository.info();
    });

    it("1 - Register WSOL Depository to the Controller", async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;
        const depository = depositoryWSOL;

        // WHEN
        if (await provider.connection.getAccountInfo(depository.mangoAccountPda)) {
            console.log("Already registered.");
        } else {
            const txId = await registerMangoDepository(caller, controller, depository, mango);
            console.log(`txId : ${txId}`);
        }

        // THEN
        depository.info();
    });
});