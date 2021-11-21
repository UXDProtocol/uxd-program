import { expect } from "chai";
import { user } from "./identities";
import { controllerUXD, depositoryWSOL, mintWithMangoDepository, redeemFromMangoDepository, mango, collateralUIPriceInMangoQuote } from "./test_integration_0_setup_uxd_api";
import { printWorldInfo, printUserBalances, printDepositoryInfo, getBalance, userWSOLATA, userUXDATA } from "./integration_test_utils";

before("Initial world state", async () => {
    printWorldInfo();
    await printUserBalances();
});

describe(" ======= [Suite 2-2 : Mint then redeem all WSOL (2 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("", async () => {
        await printUserBalances();
        await printDepositoryInfo(depositoryWSOL, mango);
    });

    const slippageBase = 1000;
    const slippage = 10; // <=> 1%
    const slippagePercentage = slippage / slippageBase;

    // OP1
    let op1_amountUxdMinted: number;
    it(`Mint UXD worth 1 WSOL with ${slippagePercentage * 100}% max slippage`, async () => {
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
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(caller, depository, mango)) * collateralAmount;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        op1_amountUxdMinted = Number((_userUxdBalancePostOp - _userUxdBalancePreOp).toPrecision(controller.redeemableMintDecimals));
        let op1_amountWsolUsed = Number((_userWsolBalancePostOp - _userWsolBalancePreOp).toPrecision(depository.collateralMintdecimals));

        expect(op1_amountWsolUsed).equals(collateralAmount * -1, "The collateral amount paid doesn't match the user wallet delta");
        expect(op1_amountUxdMinted).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${op1_amountUxdMinted} for ${op1_amountWsolUsed} WSOL (prefect was ${maxAmountUxdMinted})]`);
    });

    // OP2
    let op2_amountUxdRedeemed: number;
    it(`Redeem ${op1_amountUxdMinted} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
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
        const maxAmountWsolReceived = Number((maxAmountUxdRedeemed / (await collateralUIPriceInMangoQuote(caller, depository, mango))).toPrecision(depository.collateralMintdecimals));
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        op2_amountUxdRedeemed = Number((_userUxdBalancePreOp - _userUxdBalancePostOp).toPrecision(controller.redeemableMintDecimals));
        let op2_amountWsolReceived = Number((_userWsolBalancePostOp - _userWsolBalancePreOp).toPrecision(depository.collateralMintdecimals));

        expect(op2_amountUxdRedeemed).closeTo(maxAmountUxdRedeemed, maxAmountUxdRedeemed * (slippage), "The UXD amount redeemed is out of the slippage range");
        expect(op2_amountWsolReceived).closeTo(maxAmountWsolReceived, maxAmountWsolReceived * (slippage), "The WSOL amount received is out of the slippage range");

        console.log(`    ==> [Redeemed ${op2_amountUxdRedeemed} UXD for ${op2_amountWsolReceived} WSOL (perfect was ${maxAmountWsolReceived})]`);
    });
});