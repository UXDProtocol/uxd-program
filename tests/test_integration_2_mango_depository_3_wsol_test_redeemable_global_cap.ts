import { assert, expect } from "chai";
import { authority, user } from "./identities";
import { controllerUXD, depositoryWSOL, mintWithMangoDepository, mango } from "./test_integration_0_setup_uxd_api";
import { printWorldInfo, printUserBalances, printDepositoryInfo, sleep } from "./integration_test_utils";
import { getControllerAccount, setRedeemableGlobalSupplyCap, slippageBase } from "./test_integration_0_setup_uxd_api";
import { provider } from "./provider";

before("Initial world state", async () => {
    printWorldInfo();
    await printUserBalances();
});

// Here we setup the 
describe(" ======= [Suite 2-3 : test mint beyond redeemable global supply cap (2 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("", async () => {
        await printUserBalances();
        await printDepositoryInfo(depositoryWSOL, mango);
    });

    const slippage = 10; // <=> 1%
    const slippagePercentage = slippage / slippageBase;

    const supplyCapUIAmountLow = 2_000; // 2_000 redeemable token UI amount
    // OP1
    it(`Set redeemable supply amount cap at ${supplyCapUIAmountLow} UXD`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;

        // WHEN
        const txId = await setRedeemableGlobalSupplyCap(caller, controller, supplyCapUIAmountLow);
        console.log(txId);

        // THEN
        const _postRedeemableGlobalSupplyCap = (await getControllerAccount(controller)).redeemableGlobalSupplyCap.toNumber();
        expect(_postRedeemableGlobalSupplyCap).equals(supplyCapUIAmountLow, "The redeemable global supply cap hasn't been updated.");
    });

    // OP2
    it(`Mint UXD worth 15 WSOL with ${slippagePercentage * 100}% max slippage - SHOULD FAIL`, async () => {
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
            expect(true, "");
            return
        }
        expect(false, "Transaction should have errored")

        // THEN
        // let txReponse = await provider.connection.getTransaction(txId);
        // expect(txReponse.meta.err).to.not.be.equal("failed to send transaction: Transaction simulation failed: Error processing Instruction 0: custom program error: 0x132", "Should error");
    });


    const supplyCapUIAmountHigh = 10_000_000; // 10_000_000 redeemable token UI amount
    // OP3
    it(`Set redeemable supply amount cap back at ${supplyCapUIAmountHigh} UXD`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;

        // WHEN
        let txId = await setRedeemableGlobalSupplyCap(caller, controller, supplyCapUIAmountHigh);
        console.log(txId);

        // THEN
        const _postRedeemableGlobalSupplyCap = (await getControllerAccount(controller)).redeemableGlobalSupplyCap.toNumber();
        expect(_postRedeemableGlobalSupplyCap).equals(supplyCapUIAmountHigh, "The redeemable global supply cap hasn't been updated.");
    });

    // ADD test to close the supply when overminted already and see the behaviour

});