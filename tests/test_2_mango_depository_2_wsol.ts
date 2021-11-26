import { expect } from "chai";
import { user } from "./identities";
import { mintWithMangoDepository, redeemFromMangoDepository, collateralUIPriceInMangoQuote } from "./test_0_uxd_api";
import { printUserBalances, printDepositoryInfo, getBalance, userWSOLATA, userUXDATA } from "./integration_test_utils";
import { slippage } from "./test_2_consts";
import { depositoryWSOL, mango, slippageBase, controllerUXD } from "./test_0_consts";

describe(" ======= [Suite 2-2 : Mint then redeem all WSOL (4 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("", async () => {
        await printUserBalances();
        await printDepositoryInfo(depositoryWSOL, mango);
    });

    it(`0 - initial state`, async () => { /* no-op */ });

    const slippagePercentage = slippage / slippageBase;

    // OP1
    let op1_amountUxdMinted: number;
    it(`1 - Mint UXD worth 1 WSOL with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const caller = user;
        const collateralAmount = 1; // in WSOL
        const controller = controllerUXD;
        const depository = depositoryWSOL;
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);

        // Then
        // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmount;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        console.log(`BALANCE pre ${_userUxdBalancePreOp} -- post ${_userUxdBalancePostOp} -- delta ${_userUxdBalancePostOp - _userUxdBalancePreOp}`);
        op1_amountUxdMinted = _userUxdBalancePostOp - _userUxdBalancePreOp;
        const op_amountWsolUsed = _userWsolBalancePostOp - _userWsolBalancePreOp;

        expect(op_amountWsolUsed).equals(collateralAmount * -1, "The collateral amount paid doesn't match the user wallet delta");
        expect(op1_amountUxdMinted).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${op1_amountUxdMinted} for ${op_amountWsolUsed} WSOL (prefect was ${maxAmountUxdMinted})]`);
    });

    // OP2
    let op2_amountUxdRedeemed: number;
    it(`2 - Redeem ${op1_amountUxdMinted} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const caller = user;
        const amountRedeemable = op1_amountUxdMinted; // In UXD
        const controller = controllerUXD;
        const depository = depositoryWSOL;
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

        // THEN
        const maxAmountUxdRedeemed = op1_amountUxdMinted;
        const maxAmountWsolReceived = maxAmountUxdRedeemed / (await collateralUIPriceInMangoQuote(depository, mango));
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        op2_amountUxdRedeemed = _userUxdBalancePreOp - _userUxdBalancePostOp;
        const op_amountWsolReceived = _userWsolBalancePostOp - _userWsolBalancePreOp;
        // The amount of UXD that couldn't be redeemed due to odd lot size
        const unredeemedUXDAmount = amountRedeemable - op2_amountUxdRedeemed;

        expect(op2_amountUxdRedeemed).closeTo(maxAmountUxdRedeemed, maxAmountUxdRedeemed * (slippage), "The UXD amount redeemed is out of the slippage range");
        expect(op_amountWsolReceived).closeTo(maxAmountWsolReceived, maxAmountWsolReceived * (slippage), "The WSOL amount received is out of the slippage range");
        expect(_userUxdBalancePostOp).closeTo(_userUxdBalancePreOp - maxAmountUxdRedeemed + unredeemedUXDAmount, Math.pow(10, -controller.redeemableMintDecimals), "The amount of UXD carried over isn't right");

        console.log(`    ==> [Redeemed ${op2_amountUxdRedeemed} UXD for ${op_amountWsolReceived} WSOL (perfect was ${maxAmountWsolReceived}, returned UXD cause of odd lot ${unredeemedUXDAmount})]`);
    });

    // OP3
    let op3_amountUxdMinted: number;
    it(`3 - Mint UXD worth 10 WSOL with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const caller = user;
        const collateralAmount = 10; // in WSOL
        const controller = controllerUXD;
        const depository = depositoryWSOL;
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);

        // Then
        // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmount;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        op3_amountUxdMinted = _userUxdBalancePostOp - _userUxdBalancePreOp;
        const op_amountWsolUsed = _userWsolBalancePostOp - _userWsolBalancePreOp;

        expect(op_amountWsolUsed).closeTo(collateralAmount * -1, Math.pow(10, -depository.collateralMintdecimals), "The collateral amount paid doesn't match the user wallet delta");
        expect(op3_amountUxdMinted).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${op3_amountUxdMinted} for ${op_amountWsolUsed} WSOL (prefect was ${maxAmountUxdMinted})]`);
    });

    // OP4
    let op4_amountUxdRedeemed: number;
    it(`4 - Redeem ${op3_amountUxdMinted} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const caller = user;
        const amountRedeemable = op3_amountUxdMinted; // In UXD
        const controller = controllerUXD;
        const depository = depositoryWSOL;
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

        // THEN
        const maxAmountUxdRedeemed = op3_amountUxdMinted;
        const maxAmountWsolReceived = maxAmountUxdRedeemed / (await collateralUIPriceInMangoQuote(depository, mango));
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        op4_amountUxdRedeemed = _userUxdBalancePreOp - _userUxdBalancePostOp;
        const op_amountWsolReceived = _userWsolBalancePostOp - _userWsolBalancePreOp;
        // The amount of UXD that couldn't be redeemed due to odd lot size
        const unredeemedUXDAmount = amountRedeemable - op4_amountUxdRedeemed;


        expect(op4_amountUxdRedeemed).closeTo(maxAmountUxdRedeemed, maxAmountUxdRedeemed * (slippage), "The UXD amount redeemed is out of the slippage range");
        expect(op_amountWsolReceived).closeTo(maxAmountWsolReceived, maxAmountWsolReceived * (slippage), "The WSOL amount received is out of the slippage range");
        expect(_userUxdBalancePostOp).closeTo(_userUxdBalancePreOp - maxAmountUxdRedeemed + unredeemedUXDAmount, Math.pow(10, -controller.redeemableMintDecimals), "The amount of UXD carried over isn't right");

        console.log(`    ==> [Redeemed ${op4_amountUxdRedeemed} UXD for ${op_amountWsolReceived} WSOL (perfect was ${maxAmountWsolReceived}, returned UXD cause of odd lot ${unredeemedUXDAmount})]`);
    });
});