import { authority } from "./identities";
import { provider } from "./provider";
import { controllerUXD, depositoryBTC, mango, depositoryWSOL } from "./test_0_consts";
import { registerMangoDepository } from "./test_0_uxd_api";

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