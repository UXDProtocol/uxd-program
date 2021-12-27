import { expect } from "chai";
import { user } from "../identities";
import { collateralUIPriceInMangoQuote, redeemFromMangoDepository } from "../test_0_uxd_api";
import { printUserBalances, printDepositoryInfo, getBalance, userUXDATA, getSolBalance } from "../integration_test_utils";
import { slippage } from "../test_2_consts";
import { depositoryWSOL, mango, slippageBase, controllerUXD } from "../test_0_consts";

describe(` just redeem`, () => {
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
    let amountUxdMinted = 2574.439826;
    it(`1 - Redeem ${amountUxdMinted} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
        console.log(controllerUXD.redeemableMintPda.toString());
        // GIVEN
        const amountRedeemable = amountUxdMinted; // In UXD
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userSolBalancePreOp = await getSolBalance(caller.publicKey);

        // WHEN
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

        // THEN
        const maxAmountUxdRedeemed = amountUxdMinted;
        const maxAmountSolReceived = maxAmountUxdRedeemed / (await collateralUIPriceInMangoQuote(depository, mango));
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userSolBalancePostOp = await getSolBalance(caller.publicKey);

        let amountUxdRedeemed = _userUxdBalancePreOp - _userUxdBalancePostOp;
        const collateralAmountReceived = _userSolBalancePostOp - _userSolBalancePreOp
        // The amount of UXD that couldn't be redeemed due to odd lot size
        const unredeemedUXDAmount = amountRedeemable - amountUxdRedeemed;

        expect(amountUxdRedeemed).closeTo(maxAmountUxdRedeemed, maxAmountUxdRedeemed * (slippage), "The UXD amount redeemed is out of the slippage range");
        expect(collateralAmountReceived).closeTo(maxAmountSolReceived, maxAmountSolReceived * (slippage), "The SOL amount received is out of the slippage range");
        expect(_userUxdBalancePostOp).closeTo(_userUxdBalancePreOp - maxAmountUxdRedeemed + unredeemedUXDAmount, Math.pow(10, -controller.redeemableMintDecimals), "The amount of UXD carried over isn't right");

        console.log(`    ==> [Redeemed ${amountUxdRedeemed} UXD for ${collateralAmountReceived} SOL (perfect was ${maxAmountSolReceived}, returned UXD cause of odd lot ${unredeemedUXDAmount})]`);
    });
});