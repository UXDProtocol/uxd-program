import { authority } from "./identities";
import { expect } from "chai";
import { setRedeemableGlobalSupplyCap, getControllerAccount } from "./test_0_uxd_api";
import { controllerUXD } from "./test_0_consts";
import BN from "bn.js";

describe(" ======= [Suite 1-3 : Test setting the redeemable global supply cap (1 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("\n", async () => { });

    const supplyCapUIAmount = 10_000_000; // 100_000 redeemable token UI amount

    it(`1 - Set redeemable global supply cap to ${supplyCapUIAmount}`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;
        const _preRedeemableGlobalSupplyCap = (await getControllerAccount(controller)).redeemableGlobalSupplyCap.div(new BN(10 ** controller.redeemableMintDecimals));

        // WHEN
        const txId = await setRedeemableGlobalSupplyCap(caller, controller, supplyCapUIAmount);
        console.log(`txId : ${txId}`);

        // THEN
        const controllerAccount = await getControllerAccount(controller);
        const _postRedeemableGlobalSupplyCapUIAmount = controllerAccount.redeemableGlobalSupplyCap.div(new BN(10 ** controller.redeemableMintDecimals));
        const _redeemableCirculatingSupplyUIAmount = controllerAccount.redeemableCirculatingSupply.div(new BN(10 ** controller.redeemableMintDecimals));
        expect(_postRedeemableGlobalSupplyCapUIAmount.toNumber()).equals(supplyCapUIAmount, "The redeemable global supply cap hasn't been updated.");
        console.log(`    ==> Previous cap was ${_preRedeemableGlobalSupplyCap.toString()}, now is ${_postRedeemableGlobalSupplyCapUIAmount.toString()} (circulating supply ${_redeemableCirculatingSupplyUIAmount.toString()})`);
        controller.info();
        // console.log(controllerAccount);
    });
});