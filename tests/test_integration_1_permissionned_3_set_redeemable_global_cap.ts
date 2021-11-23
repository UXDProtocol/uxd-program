import { authority } from "./identities";
import { expect } from "chai";
import { setRedeemableGlobalSupplyCap, getControllerAccount } from "./test_integration_0_uxd_api";
import { controllerUXD } from "./test_integration_0_consts";

before(" ======= [Suite 1-3 : Test setting the redeemable global supply cap (1 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("\n", async () => { });

    const supplyCapUIAmount = 100_000; // 100_000 redeemable token UI amount

    it(`Set redeemable global supply to ${supplyCapUIAmount}`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;
        const _preRedeemableGlobalSupplyCap = (await getControllerAccount(controller)).redeemableGlobalSupplyCap.toNumber();

        // WHEN
        const txId = await setRedeemableGlobalSupplyCap(caller, controller, supplyCapUIAmount);
        console.log(`txId : ${txId}`);

        // THEN
        const controllerAccount = await getControllerAccount(controller);
        const _postRedeemableGlobalSupplyCapUIAmount = controllerAccount.redeemableGlobalSupplyCap.toNumber() / (10 ** controller.redeemableMintDecimals);
        const _redeemableCirculatingSupplyUIAmount = controllerAccount.redeemableCirculatingSupply.toNumber() / (10 ** controller.redeemableMintDecimals);
        expect(_postRedeemableGlobalSupplyCapUIAmount).equals(supplyCapUIAmount, "The redeemable global supply cap hasn't been updated.");
        console.log(`    ==> Previous cap was ${_preRedeemableGlobalSupplyCap}, now is ${_postRedeemableGlobalSupplyCapUIAmount} (circulating supply ${_redeemableCirculatingSupplyUIAmount})`);
        controller.info();
        console.log(controllerAccount);
    });
});