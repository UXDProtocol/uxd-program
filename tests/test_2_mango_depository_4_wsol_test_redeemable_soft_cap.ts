import { expect } from "chai";
import { authority, user } from "./identities";
import { collateralUIPriceInMangoQuote, mintWithMangoDepository, setMangoDepositoriesRedeemableSoftCap } from "./test_0_uxd_api";
import { printUserBalances, printDepositoryInfo, getBalance, userUXDATA, userWSOLATA } from "./integration_test_utils";
import { getControllerAccount } from "./test_0_uxd_api";
import { slippage } from "./test_2_consts";
import { depositoryWSOL, mango, slippageBase, controllerUXD } from "./test_0_consts";
import { BN } from "@project-serum/anchor";

// Here we setup the 
describe(" ======= [Suite 2-4 : test mango depositories redeemable soft cap (4 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("", async () => {
        await printUserBalances();
        await printDepositoryInfo(depositoryWSOL, mango);
    });

    it(`0 - initial state`, async () => { /* no-op */ });

    const slippagePercentage = slippage / slippageBase;

    const supplyCapUIAmountLow = 100; // redeemable token UI amount
    // OP1
    it(`1 - Set mango depositories redeemable soft cap to ${supplyCapUIAmountLow}`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;
        const _preRedeemableSoftCap = (await getControllerAccount(controller)).mangoDepositoriesRedeemableSoftCap.div(new BN(10 ** controller.redeemableMintDecimals));

        // WHEN
        const txId = await setMangoDepositoriesRedeemableSoftCap(caller, controller, supplyCapUIAmountLow);
        console.log(`txId : ${txId}`);

        // THEN
        const controllerAccount = await getControllerAccount(controller);
        const _postRedeemableSoftCapUIAmount = controllerAccount.mangoDepositoriesRedeemableSoftCap.div(new BN(10 ** controller.redeemableMintDecimals));
        expect(_postRedeemableSoftCapUIAmount.toNumber()).equals(supplyCapUIAmountLow, "The redeemable soft cap hasn't been updated.");
        console.log(`    ==> Previous soft cap was ${_preRedeemableSoftCap.toString()}, now is ${_postRedeemableSoftCapUIAmount.toString()}`);
        // controller.info();
        // console.log(controllerAccount);
    });

    const validCollateralAmount = 0.2 // in WSOL
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

        let amountUxdMinted = _userUxdBalancePostOp - _userUxdBalancePreOp;
        let amountWsolUsed = _userWsolBalancePreOp - _userWsolBalancePostOp;

        expect(amountWsolUsed).closeTo(validCollateralAmount, Math.pow(10, -depository.collateralMintdecimals), "The collateral amount paid doesn't match the user wallet delta");
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
        const _preRedeemableSoftCap = (await getControllerAccount(controller)).mangoDepositoriesRedeemableSoftCap.div(new BN(10 ** controller.redeemableMintDecimals));

        // WHEN
        const txId = await setMangoDepositoriesRedeemableSoftCap(caller, controller, supplyCapUIAmountHigh);
        console.log(`txId : ${txId}`);

        // 
        const controllerAccount = await getControllerAccount(controller);
        const _postRedeemableSoftCapUIAmount = controllerAccount.mangoDepositoriesRedeemableSoftCap.div(new BN(10 ** controller.redeemableMintDecimals));
        expect(_postRedeemableSoftCapUIAmount.toNumber()).equals(supplyCapUIAmountHigh, "The redeemable soft cap hasn't been updated.");
        console.log(`    ==> Previous soft cap was ${_preRedeemableSoftCap.toString()}, now is ${_postRedeemableSoftCapUIAmount.toString()}`);
        // controller.info();
        // console.log(controllerAccount);
    });

    // ADD test to close the supply when overminted already and see the behaviour

});
