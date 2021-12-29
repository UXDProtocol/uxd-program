import { expect } from "chai";
import { authority, user } from "./identities";
import { mintWithMangoDepository } from "./test_0_uxd_api";
import { printUserBalances, printDepositoryInfo, sleep } from "./integration_test_utils";
import { getControllerAccount, setRedeemableGlobalSupplyCap } from "./test_0_uxd_api";
import { slippage } from "./test_2_consts";
import { depositoryWSOL, mango, slippageBase, controllerUXD, accountUpdateSleepingInterval } from "./test_0_consts";
import { BN } from "@project-serum/anchor";

// Here we setup the 
describe(" ======= [Suite 2-3 : test mint beyond redeemable global supply cap (3 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("", async () => {
        await printUserBalances();
        await printDepositoryInfo(depositoryWSOL, mango);
    });

    it(`0 - initial state`, async () => { /* no-op */ });

    const slippagePercentage = slippage / slippageBase;

    const supplyCapUIAmountLow = 2_000; // UI amount
    // OP1
    it(`1 - Set redeemable supply amount cap at ${supplyCapUIAmountLow} UXD`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;

        // WHEN
        const txId = await setRedeemableGlobalSupplyCap(caller, controller, supplyCapUIAmountLow);
        console.log(txId);

        // THEN
        const _postRedeemableGlobalSupplyCapUIAmount = (await getControllerAccount(controller)).redeemableGlobalSupplyCap.div(new BN(10 ** controller.redeemableMintDecimals));
        expect(_postRedeemableGlobalSupplyCapUIAmount.toNumber()).equals(supplyCapUIAmountLow, "The redeemable global supply cap hasn't been updated.");
    });

    // OP2
    it(`2 - Mint UXD worth 15 WSOL with ${slippagePercentage * 100}% max slippage - SHOULD FAIL`, async () => {
        // GIVEN
        const caller = user;
        const collateralAmount = 15; // in WSOL
        const controller = controllerUXD;
        const depository = depositoryWSOL;

        // WHEN
        let txId: string;

        try {
            txId = await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        } catch {
            console.log(txId);
            expect(true, "");
            return
        }
        console.log(txId);
        expect(false, "Transaction should have errored")

        // THEN
    });


    const supplyCapUIAmountHigh = 10_000_000; // 10_000_000 redeemable token UI amount
    // OP3
    it(`3 - Set redeemable supply amount cap back at ${supplyCapUIAmountHigh} UXD`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;
        const _preRedeemableGlobalSupplyCapUIAmount = (await getControllerAccount(controller)).redeemableGlobalSupplyCap.div(new BN(10 ** controller.redeemableMintDecimals));

        // WHEN
        let txId = await setRedeemableGlobalSupplyCap(caller, controller, supplyCapUIAmountHigh);
        console.log(txId);

        // THEN
        await sleep(accountUpdateSleepingInterval);
        const _postRedeemableGlobalSupplyCapUIAmount = (await getControllerAccount(controller)).redeemableGlobalSupplyCap.div(new BN(10 ** controller.redeemableMintDecimals));
        console.log(`    ==> Previous global supply cap was ${_preRedeemableGlobalSupplyCapUIAmount.toString()}, now is ${_postRedeemableGlobalSupplyCapUIAmount.toString()}`);
        expect(_postRedeemableGlobalSupplyCapUIAmount.toNumber()).equals(supplyCapUIAmountHigh, "The redeemable global supply cap hasn't been updated.");
    });

    // ADD test to close the supply when overminted already and see the behaviour

});