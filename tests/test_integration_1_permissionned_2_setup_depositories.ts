import { authority } from "./identities";
import { provider } from "./provider";
import { depositoryBTC, controllerUXD, registerMangoDepository, depositoryWSOL, mango } from "./test_integration_0_setup_uxd_api";

before(" ======= [Suite 1-2 : Test setup UXD controller (permissionned) (1 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("\n", async () => { });

    it("Register BTC Depository to the Controller", async () => {
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

    it("Register WSOL Depository to the Controller", async () => {
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