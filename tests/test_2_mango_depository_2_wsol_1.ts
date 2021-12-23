import { expect } from "chai";
import { user } from "./identities";
import { mintWithMangoDepository, redeemFromMangoDepository, collateralUIPriceInMangoQuote } from "./test_0_uxd_api";
import { printUserBalances, printDepositoryInfo, getBalance, userUXDATA, getSolBalance } from "./integration_test_utils";
import { slippage } from "./test_2_consts";
import { depositoryWSOL, mango, slippageBase, controllerUXD } from "./test_0_consts";

describe(" ======= [Suite 2-2-1 : Mint then redeem all SOL (4 op)] ======= ", () => {
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
    let collateralAmount = 1;
    let amountUxdMinted: number;
    it(`1 - Mint UXD worth ${collateralAmount} SOL with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userSolBalancePreOp = await getSolBalance(caller.publicKey);

        // WHEN
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);

        // Then
        // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmount;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userSolBalancePostOp = await getSolBalance(caller.publicKey);

        amountUxdMinted = _userUxdBalancePostOp - _userUxdBalancePreOp;
        const solUsed = _userSolBalancePreOp - _userSolBalancePostOp;

        // + 0.00204 to create wsol ata
        expect(solUsed).lessThanOrEqual(collateralAmount + 0.00204, "The collateral amount paid doesn't match the user wallet delta");
        expect(amountUxdMinted).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${amountUxdMinted} for ${solUsed} SOL (perfect was ${maxAmountUxdMinted})]`);
    });

    // OP2
    it(`2 - Redeem ${amountUxdMinted} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
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
        const op_amountSolReceived = _userSolBalancePostOp - _userSolBalancePreOp
        // The amount of UXD that couldn't be redeemed due to odd lot size
        const unredeemedUXDAmount = amountRedeemable - amountUxdRedeemed;

        expect(amountUxdRedeemed).closeTo(maxAmountUxdRedeemed, maxAmountUxdRedeemed * (slippage), "The UXD amount redeemed is out of the slippage range");
        expect(op_amountSolReceived).closeTo(maxAmountSolReceived, maxAmountSolReceived * (slippage), "The SOL amount received is out of the slippage range");
        expect(_userUxdBalancePostOp).closeTo(_userUxdBalancePreOp - maxAmountUxdRedeemed + unredeemedUXDAmount, Math.pow(10, -controller.redeemableMintDecimals), "The amount of UXD carried over isn't right");

        console.log(`    ==> [Redeemed ${amountUxdRedeemed} UXD for ${op_amountSolReceived} SOL (perfect was ${maxAmountSolReceived}, returned UXD cause of odd lot ${unredeemedUXDAmount})]`);
    });

    // OP1
    let collateralAmountB = 10;
    let amountUxdMintedB: number;
    it(`1 - Mint UXD worth ${collateralAmountB} SOL with ${slippagePercentage * 100}% max slippage`, async () => {
        // GIVEN
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userSolBalancePreOp = await getSolBalance(caller.publicKey);

        // WHEN
        await mintWithMangoDepository(caller, slippage, collateralAmountB, controller, depository, mango);

        // Then
        // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
        const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmountB;
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userSolBalancePostOp = await getSolBalance(caller.publicKey);

        amountUxdMintedB = _userUxdBalancePostOp - _userUxdBalancePreOp;
        const solUsed = _userSolBalancePreOp - _userSolBalancePostOp;

        // + 0.00204 to create wsol assoc accounts
        expect(solUsed).lessThanOrEqual(collateralAmountB + 0.00204, "The collateral amount paid doesn't match the user wallet delta");
        expect(amountUxdMintedB).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${amountUxdMintedB} for ${solUsed} SOL (perfect was ${maxAmountUxdMinted})]`);
    });

    // OP4
    it(`4 - Redeem ${amountUxdMintedB} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
        console.log(controllerUXD.redeemableMintPda.toString());
        // GIVEN
        const amountRedeemable = amountUxdMintedB; // In UXD
        const _userUxdBalancePreOp = await getBalance(userUXDATA);
        const _userSolBalancePreOp = await getSolBalance(caller.publicKey);

        // WHEN
        await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

        // THEN
        const maxAmountUxdRedeemed = amountUxdMintedB;
        const maxAmountSolReceived = maxAmountUxdRedeemed / (await collateralUIPriceInMangoQuote(depository, mango));
        const _userUxdBalancePostOp = await getBalance(userUXDATA);
        const _userSolBalancePostOp = await getSolBalance(caller.publicKey);

        let amountUxdRedeemed = _userUxdBalancePreOp - _userUxdBalancePostOp;
        const op_amountSolReceived = _userSolBalancePostOp - _userSolBalancePreOp
        // The amount of UXD that couldn't be redeemed due to odd lot size
        const unredeemedUXDAmount = amountRedeemable - amountUxdRedeemed;

        expect(amountUxdRedeemed).closeTo(maxAmountUxdRedeemed, maxAmountUxdRedeemed * (slippage), "The UXD amount redeemed is out of the slippage range");
        expect(op_amountSolReceived).closeTo(maxAmountSolReceived, maxAmountSolReceived * (slippage), "The SOL amount received is out of the slippage range");
        expect(_userUxdBalancePostOp).closeTo(_userUxdBalancePreOp - maxAmountUxdRedeemed + unredeemedUXDAmount, Math.pow(10, -controller.redeemableMintDecimals), "The amount of UXD carried over isn't right");

        console.log(`    ==> [Redeemed ${amountUxdRedeemed} UXD for ${op_amountSolReceived} SOL  (perfect was ${maxAmountSolReceived}, returned UXD cause of odd lot ${unredeemedUXDAmount})]`);
    });
});