import { expect } from "chai";
import { user } from "../identities";
import { mintWithMangoDepository, collateralUIPriceInMangoQuote, redeemFromMangoDepository } from "../test_0_uxd_api";
import { printUserBalances, printDepositoryInfo, getBalance, userUXDATA, getSolBalance } from "../integration_test_utils";
import { slippage } from "../test_2_consts";
import { depositoryWSOL, mango, slippageBase, controllerUXD } from "../test_0_consts";

const amountToMint = 1;

describe(` mint ${amountToMint} SOL`, () => {
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
    let collateralAmount = amountToMint; // in SOL
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

        expect(solUsed).closeTo(collateralAmount, Math.pow(10, -depository.insuranceMintDecimals), "The collateral amount paid doesn't match the user wallet delta");
        expect(amountUxdMinted).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

        console.log(`    ==> [Minted ${amountUxdMinted} for ${solUsed} SOL (perfect was ${maxAmountUxdMinted})]`);
    });

    it(`mint 10 times`, async () => {
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
        await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);
    });
    
});