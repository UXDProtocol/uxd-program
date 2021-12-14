import { expect } from "chai";
import { user } from "../identities";
import { collateralUIPriceInMangoQuote, redeemFromMangoDepository } from "../test_0_uxd_api";
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

    // OP2
    let amountUxdMinted = 1914;
    it(`1 - Redeem ${amountUxdMinted} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const amountRedeemable = amountUxdMinted; // In UXD
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

        // THEN
        const maxAmountUxdRedeemed = amountUxdMinted;
        const maxAmountWsolReceived = maxAmountUxdRedeemed / (await collateralUIPriceInMangoQuote(depository, mango));
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        let amountUxdRedeemed = _userUxdBalancePreOp - _userUxdBalancePostOp;
        const op_amountWsolReceived = _userWsolBalancePostOp - _userWsolBalancePreOp;
        // The amount of UXD that couldn't be redeemed due to odd lot size
        const unredeemedUXDAmount = amountRedeemable - amountUxdRedeemed;

        expect(amountUxdRedeemed).closeTo(maxAmountUxdRedeemed, maxAmountUxdRedeemed * (slippage), "The UXD amount redeemed is out of the slippage range");
        expect(op_amountWsolReceived).closeTo(maxAmountWsolReceived, maxAmountWsolReceived * (slippage), "The WSOL amount received is out of the slippage range");
        expect(_userUxdBalancePostOp).closeTo(_userUxdBalancePreOp - maxAmountUxdRedeemed + unredeemedUXDAmount, Math.pow(10, -controller.redeemableMintDecimals), "The amount of UXD carried over isn't right");

        console.log(`    ==> [Redeemed ${amountUxdRedeemed} UXD for ${op_amountWsolReceived} WSOL (perfect was ${maxAmountWsolReceived}, returned UXD cause of odd lot ${unredeemedUXDAmount})]`);
    });
});