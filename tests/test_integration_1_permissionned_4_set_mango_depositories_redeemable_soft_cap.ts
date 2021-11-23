import { authority } from "./identities";
import { expect } from "chai";
import { setRedeemableGlobalSupplyCap, getControllerAccount, setMangoDepositoriesRedeemableSoftCap } from "./test_integration_0_uxd_api";
import { controllerUXD } from "./test_integration_0_consts";

before(" ======= [Suite 1-4 : Test setting the mango depositories redeemable soft cap (1 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("\n", async () => { });

    const supplySoftCapUIAmount = 100_000; // 100_000 redeemable token UI amount

    it(`1 - Set mango depositories redeemable soft cap to ${supplySoftCapUIAmount}`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;
        const _preRedeemableSoftCap = (await getControllerAccount(controller)).mangoDepositoriesRedeemableSoftCap.toNumber();

        // WHEN
        const txId = await setMangoDepositoriesRedeemableSoftCap(caller, controller, supplySoftCapUIAmount);
        console.log(`txId : ${txId}`);

        // THEN
        const controllerAccount = await getControllerAccount(controller);
        const _postRedeemableSoftCapUIAmount = controllerAccount.mangoDepositoriesRedeemableSoftCap.toNumber() / (10 ** controller.redeemableMintDecimals);
        expect(_postRedeemableSoftCapUIAmount).equals(supplySoftCapUIAmount, "The mango depositories redeemable soft cap hasn't been updated.");
        console.log(`    ==> Previous mango depositories redeemable soft cap was ${_preRedeemableSoftCap}, now is ${_postRedeemableSoftCapUIAmount}`);
        controller.info();
        console.log(controllerAccount);
    });
});