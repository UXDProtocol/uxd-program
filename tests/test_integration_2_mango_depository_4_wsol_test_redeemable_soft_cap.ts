import { expect } from "chai";
import { authority, user } from "./identities";
import { collateralUIPriceInMangoQuote, mintWithMangoDepository } from "./test_integration_0_uxd_api";
import { printWorldInfo, printUserBalances, printDepositoryInfo, sleep, getBalance, userUXDATA, userWSOLATA } from "./integration_test_utils";
import { getControllerAccount, setRedeemableGlobalSupplyCap } from "./test_integration_0_uxd_api";
import { slippage } from "./test_integration_2_consts";
import { depositoryWSOL, mango, slippageBase, controllerUXD } from "./test_integration_0_consts";

before("Initial world state", async () => {
    printWorldInfo();
    await printUserBalances();
});

// Here we setup the 
describe(" ======= [Suite 2-4 : test mango depositories redeemable soft cap (4 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("", async () => {
        await printUserBalances();
        await printDepositoryInfo(depositoryWSOL, mango);
    });

    const slippagePercentage = slippage / slippageBase;

    const supplyCapUIAmountLow = 100; // redeemable token UI amount
    // OP1
    it(`1 - Set mango depositories redeemable soft cap to ${supplyCapUIAmountLow}`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;
        const _preRedeemableSoftCap = (await getControllerAccount(controller)).mangoDepositoriesRedeemableSoftCap.toNumber();

        // WHEN
        const txId = await setRedeemableGlobalSupplyCap(caller, controller, supplyCapUIAmountLow);
        console.log(`txId : ${txId}`);

        // THEN
        const controllerAccount = await getControllerAccount(controller);
        const _postRedeemableSoftCapUIAmount = controllerAccount.mangoDepositoriesRedeemableSoftCap.toNumber() / (10 ** controller.redeemableMintDecimals);
        expect(_postRedeemableSoftCapUIAmount).equals(supplyCapUIAmountLow, "The redeemable global supply cap hasn't been updated.");
        console.log(`    ==> Previous soft cap was ${_preRedeemableSoftCap}, now is ${_postRedeemableSoftCapUIAmount}`);
        controller.info();
        console.log(controllerAccount);
    });

    const validCollateralAmount = 0.5 // in WSOL
    // OP2
    it(`2 - Mint UXD worth ${validCollateralAmount} WSOL with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const caller = user;
        const controller = controllerUXD;
        const depository = depositoryWSOL;
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await mintWithMangoDepository(caller, slippage, validCollateralAmount, controller, depository, mango);

        // Then
        // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * validCollateralAmount;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        let amountUxdMinted = Number((_userUxdBalancePostOp - _userUxdBalancePreOp).toPrecision(controller.redeemableMintDecimals));
        let amountWsolUsed = Number((_userWsolBalancePostOp - _userWsolBalancePreOp).toPrecision(depository.collateralMintdecimals));

        expect(amountWsolUsed).equals(validCollateralAmount * -1, "The collateral amount paid doesn't match the user wallet delta");
        expect(amountUxdMinted).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${amountUxdMinted} for ${amountWsolUsed} WSOL (prefect was ${maxAmountUxdMinted})]`);
    });

    const invalidCollateralAmount = 5; // in WSOL
    // OP3
    it(`3 - Mint UXD worth ${invalidCollateralAmount} WSOL with ${slippagePercentage * 100}% max slippage - SHOULD FAIL`, async () => {
        // GIVEN
        const caller = user;
        const controller = controllerUXD;
        const depository = depositoryWSOL;

        // WHEN
        let txId: string;

        try {
            txId = await mintWithMangoDepository(caller, slippage, invalidCollateralAmount, controller, depository, mango);
        } catch {
            console.log(txId);
            expect(true, "");
            return
        }
        console.log(txId);
        expect(false, "Transaction should have errored")

        // THEN
    });

    const supplyCapUIAmountHigh = 10_000; // redeemable token UI amount
    // OP4
    it(`4 - Set mango depositories redeemable soft cap to ${supplyCapUIAmountHigh}`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;
        const _preRedeemableSoftCap = (await getControllerAccount(controller)).mangoDepositoriesRedeemableSoftCap.toNumber();

        // WHEN
        const txId = await setRedeemableGlobalSupplyCap(caller, controller, supplyCapUIAmountHigh);
        console.log(`txId : ${txId}`);

        // THEN
        const controllerAccount = await getControllerAccount(controller);
        const _postRedeemableSoftCapUIAmount = controllerAccount.mangoDepositoriesRedeemableSoftCap.toNumber() / (10 ** controller.redeemableMintDecimals);
        expect(_postRedeemableSoftCapUIAmount).equals(supplyCapUIAmountHigh, "The redeemable global supply cap hasn't been updated.");
        console.log(`    ==> Previous soft cap was ${_preRedeemableSoftCap}, now is ${_postRedeemableSoftCapUIAmount}`);
        controller.info();
        console.log(controllerAccount);
    });

    // ADD test to close the supply when overminted already and see the behaviour

});
