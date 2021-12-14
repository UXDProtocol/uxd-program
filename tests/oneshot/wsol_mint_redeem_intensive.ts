import { expect } from "chai";
import { user } from "../identities";
import { mintWithMangoDepository, redeemFromMangoDepository, collateralUIPriceInMangoQuote } from "../test_0_uxd_api";
import { printUserBalances, printDepositoryInfo, getBalance, userWSOLATA, userUXDATA } from "../integration_test_utils";
import { slippage } from "../test_2_consts";
import { depositoryWSOL, mango, slippageBase, controllerUXD } from "../test_0_consts";

describe(" ============== ", () => {
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
    let collateralAmount = 5; // in WSOL
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

        expect(op_amountWsolUsed).closeTo(collateralAmount * -1, Math.pow(10, -depository.collateralMintdecimals), "The collateral amount paid doesn't match the user wallet delta");
        expect(amountUxdMinted).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${amountUxdMinted} for ${op_amountWsolUsed} WSOL (prefect was ${maxAmountUxdMinted})]`);
    });

    // OP2
    it(`2 - Redeem ${amountUxdMinted} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
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

    // OP3
    let collateralAmountB = 6; // in WSOL
    let amountUxdMintedB: number;
    it(`3 - Mint UXD worth ${collateralAmountB} WSOL with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await mintWithMangoDepository(caller, slippage, collateralAmountB, controller, depository, mango);

        // Then
        // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmountB;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        amountUxdMintedB = _userUxdBalancePostOp - _userUxdBalancePreOp;
        const op_amountWsolUsed = _userWsolBalancePostOp - _userWsolBalancePreOp;

        expect(op_amountWsolUsed).closeTo(collateralAmountB * -1, collateralAmountB * 0.05, "The collateral amount paid doesn't match the user wallet delta");
        expect(amountUxdMintedB).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${amountUxdMintedB} for ${op_amountWsolUsed} WSOL (prefect was ${maxAmountUxdMinted})]`);
    });

    // OP4
    it(`4 - Redeem ${amountUxdMintedB} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const amountRedeemable = amountUxdMintedB; // In UXD
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

        // THEN
        const maxAmountUxdRedeemed = amountUxdMintedB;
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

    // OP5
    let collateralAmountC = 2.89; // in WSOL
    let amountUxdMintedC: number;
    it(`5 - Mint UXD worth ${collateralAmountC} WSOL with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await mintWithMangoDepository(caller, slippage, collateralAmountC, controller, depository, mango);

        // Then
        // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmountC;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        amountUxdMintedC = _userUxdBalancePostOp - _userUxdBalancePreOp;
        const op_amountWsolUsed = _userWsolBalancePostOp - _userWsolBalancePreOp;

        expect(op_amountWsolUsed).closeTo(collateralAmountC * -1, collateralAmountC * 0.05, "The collateral amount paid doesn't match the user wallet delta");
        expect(amountUxdMintedC).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${amountUxdMintedC} for ${op_amountWsolUsed} WSOL (prefect was ${maxAmountUxdMinted})]`);
    });

    // OP6
    it(`6 - Redeem ${amountUxdMintedC} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const amountRedeemable = amountUxdMintedC; // In UXD
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

        // THEN
        const maxAmountUxdRedeemed = amountUxdMintedC;
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

    // OP7
    let collateralAmountD = 0.69; // in WSOL
    let amountUxdMintedD: number;
    it(`7 - Mint UXD worth ${collateralAmountD} WSOL with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await mintWithMangoDepository(caller, slippage, collateralAmountD, controller, depository, mango);

        // Then
        // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmountD;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        amountUxdMintedD = _userUxdBalancePostOp - _userUxdBalancePreOp;
        const op_amountWsolUsed = _userWsolBalancePostOp - _userWsolBalancePreOp;

        expect(op_amountWsolUsed).closeTo(collateralAmountD * -1, collateralAmountD * 0.05, "The collateral amount paid doesn't match the user wallet delta");
        expect(amountUxdMintedD).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${amountUxdMintedD} for ${op_amountWsolUsed} WSOL (prefect was ${maxAmountUxdMinted})]`);
    });

    // OP8
    it(`8 - Redeem ${amountUxdMintedD} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const amountRedeemable = amountUxdMintedD; // In UXD
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

        // THEN
        const maxAmountUxdRedeemed = amountUxdMintedD;
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

    // OP9
    let collateralAmountE = 12.696969690; // in WSOL
    let amountUxdMintedE: number;
    it(`9 - Mint UXD worth ${collateralAmountE} WSOL with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await mintWithMangoDepository(caller, slippage, collateralAmountE, controller, depository, mango);

        // Then
        // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmountE;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        amountUxdMintedE = _userUxdBalancePostOp - _userUxdBalancePreOp;
        const op_amountWsolUsed = _userWsolBalancePostOp - _userWsolBalancePreOp;

        expect(op_amountWsolUsed).closeTo(collateralAmountE * -1, collateralAmountE * 0.05, "The collateral amount paid doesn't match the user wallet delta");
        expect(amountUxdMintedE).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${amountUxdMintedE} for ${op_amountWsolUsed} WSOL (prefect was ${maxAmountUxdMinted})]`);
    });

    // OP10
    it(`10 - Redeem ${amountUxdMintedE} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const amountRedeemable = amountUxdMintedE; // In UXD
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

        // THEN
        const maxAmountUxdRedeemed = amountUxdMintedE;
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

    ///////
    it(` - final state after 5 mint/redeem cycle`, async () => {
        await printUserBalances();
        await printDepositoryInfo(depositoryWSOL, mango);
    });
    ///////

    // OP11
    let collateralAmountF = 10; // in WSOL
    let amountUxdMintedF: number;
    it(`11 - Mint UXD worth ${collateralAmountF} WSOL with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await mintWithMangoDepository(caller, slippage, collateralAmountF, controller, depository, mango);

        // Then
        // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmountF;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        amountUxdMintedF = _userUxdBalancePostOp - _userUxdBalancePreOp;
        const op_amountWsolUsed = _userWsolBalancePostOp - _userWsolBalancePreOp;

        expect(op_amountWsolUsed).closeTo(collateralAmountF * -1, collateralAmountF * 0.05, "The collateral amount paid doesn't match the user wallet delta");
        expect(amountUxdMintedF).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${amountUxdMintedF} for ${op_amountWsolUsed} WSOL (prefect was ${maxAmountUxdMinted})]`);
    });

    // OP12
    let collateralAmountG = 8; // in WSOL
    let amountUxdMintedG: number;
    it(`12 - Mint UXD worth ${collateralAmountG} WSOL with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await mintWithMangoDepository(caller, slippage, collateralAmountG, controller, depository, mango);

        // Then
        // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmountG;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        amountUxdMintedG = _userUxdBalancePostOp - _userUxdBalancePreOp;
        const op_amountWsolUsed = _userWsolBalancePostOp - _userWsolBalancePreOp;

        expect(op_amountWsolUsed).closeTo(collateralAmountG * -1, collateralAmountG * 0.05, "The collateral amount paid doesn't match the user wallet delta");
        expect(amountUxdMintedG).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${amountUxdMintedG} for ${op_amountWsolUsed} WSOL (prefect was ${maxAmountUxdMinted})]`);
    });

    // OP13
    let collateralAmountH = 4.206969; // in WSOL
    let amountUxdMintedH: number;
    it(`13 - Mint UXD worth ${collateralAmountH} WSOL with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userWsolBalancePreOp = await getBalance(userWSOLATA);

        // WHEN
        await mintWithMangoDepository(caller, slippage, collateralAmountH, controller, depository, mango);

        // Then
        // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmountH;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userWsolBalancePostOp = await getBalance(userWSOLATA);

        amountUxdMintedH = _userUxdBalancePostOp - _userUxdBalancePreOp;
        const op_amountWsolUsed = _userWsolBalancePostOp - _userWsolBalancePreOp;

        expect(op_amountWsolUsed).closeTo(collateralAmountH * -1, collateralAmountH * 0.05, "The collateral amount paid doesn't match the user wallet delta");
        expect(amountUxdMintedH).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${amountUxdMintedH} for ${op_amountWsolUsed} WSOL (prefect was ${maxAmountUxdMinted})]`);
    });
});

