import { authority } from "./identities";
import { expect } from "chai";
import { getControllerAccount, setMangoDepositoriesRedeemableSoftCap } from "./test_0_uxd_api";
import { controllerUXD } from "./test_0_consts";
import { BN } from "@project-serum/anchor";

before(" ======= [Suite 1-4 : Test setting the mango depositories redeemable soft cap (1 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("\n", async () => { });

    const supplySoftCapUIAmount = 100_000; // 100_000 redeemable token UI amount

    it(`1 - Set mango depositories redeemable soft cap to ${supplySoftCapUIAmount}`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;
        const _preRedeemableSoftCap = (await getControllerAccount(controller)).mangoDepositoriesRedeemableSoftCap.div(new BN(10 ** controller.redeemableMintDecimals));

        // WHEN
        const txId = await setMangoDepositoriesRedeemableSoftCap(caller, controller, supplySoftCapUIAmount);
        console.log(`txId : ${txId}`);

        // THEN
        const controllerAccount = await getControllerAccount(controller);
        const _postRedeemableSoftCapUIAmount = controllerAccount.mangoDepositoriesRedeemableSoftCap.div(new BN(10 ** controller.redeemableMintDecimals));
        expect(_postRedeemableSoftCapUIAmount.toNumber()).equals(supplySoftCapUIAmount, "The mango depositories redeemable soft cap hasn't been updated.");
        console.log(`    ==> Previous mango depositories redeemable soft cap was ${_preRedeemableSoftCap.toString()}, now is ${_postRedeemableSoftCapUIAmount.toString()}`);
        controller.info();
        // console.log(controllerAccount);
    });
});