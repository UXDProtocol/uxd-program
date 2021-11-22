import { authority } from "./identities";
import { expect } from "chai";
import { controllerUXD, redeemableCirculatinSupply, setRedeemableGlobalSupplyCap, getControllerAccount } from "./test_integration_0_setup_uxd_api";

export const U64_MAX: number = 9223372036854775807;

before(" ======= [Suite 1-3 : Test setting the redeemable global supply cap (1 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("\n", async () => { });

    const supplyCapUIAmount = 100_000; // 100_000 redeemable token UI amount

    it(`Set redeemable global supply to ${supplyCapUIAmount}`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;
        // const _redeemableGlobalCirculatingSupply = await redeemableCirculatinSupply(controller);
        const _preRedeemableGlobalSupplyCap = (await getControllerAccount(controller)).redeemableGlobalSupplyCap.toNumber();

        // WHEN
        const txId = await setRedeemableGlobalSupplyCap(caller, controller, supplyCapUIAmount);
        console.log(`txId : ${txId}`);

        // THEN
        const controllerAccount = await getControllerAccount(controller);
        const _postRedeemableGlobalSupplyCap = controllerAccount.redeemableGlobalSupplyCap.toNumber();
        expect(_postRedeemableGlobalSupplyCap).equals(supplyCapUIAmount, "The redeemable global supply cap hasn't been updated.");
        console.log(`    ==> Previous cap was ${_preRedeemableGlobalSupplyCap}, now is ${_postRedeemableGlobalSupplyCap}`);
        controller.info();
        console.log(controllerAccount);
    });
});