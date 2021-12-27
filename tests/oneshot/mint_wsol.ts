import { expect } from "chai";
import { user } from "../identities";
import { mintWithMangoDepository, collateralUIPriceInMangoQuote, redeemFromMangoDepository, mangoConsumeEvents } from "../test_0_uxd_api";
import { printUserBalances, printDepositoryInfo, getBalance, userWSOLATA, userUXDATA } from "../integration_test_utils";
import { slippage } from "../test_2_consts";
import { depositoryWSOL, mango, slippageBase, controllerUXD } from "../test_0_consts";

const amountToMint = 1;

describe(` Just mint ${amountToMint} `, () => {
    beforeEach("\n", async () => { });
    afterEach("", async () => {
        await printUserBalances();
        await printDepositoryInfo(depositoryWSOL, mango);
    });

    it(`0 - initial state`, async () => { /* no-op */ });

    const slippagePercentage = slippage / slippageBase;

    const caller = user;
    const controller = controllerUXD;
    const depository = depositoryWSOL;

    // OP1
    let collateralAmount = amountToMint; // in WSOL
    let amountUxdMinted: number;
    it(`1 - Mint UXD worth ${collateralAmount} WSOL with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);

        // Then
        // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmount;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        amountUxdMinted = _userUxdBalancePostOp - _userUxdBalancePreOp;
        const op_amountWsolUsed = _userWsolBalancePostOp - _userWsolBalancePreOp;

        expect(op_amountWsolUsed).closeTo(collateralAmount * -1, collateralAmount * 0.05, "The collateral amount paid doesn't match the user wallet delta");
        expect(amountUxdMinted).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${amountUxdMinted} for ${op_amountWsolUsed} WSOL (prefect was ${maxAmountUxdMinted})]`);
        await mangoConsumeEvents(depository, mango);
    });

    it(`mint 10 times`, async () => {
        let amountRedeemable = 69.69;
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);

        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);

        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);

        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);

        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);

        await mangoConsumeEvents(depository, mango);
    });
});