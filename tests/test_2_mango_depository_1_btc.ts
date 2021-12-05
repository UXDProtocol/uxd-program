import { expect } from "chai";
import { user } from "./identities";
import { mintWithMangoDepository, redeemFromMangoDepository, collateralUIPriceInMangoQuote } from "./test_0_uxd_api";
import { printUserBalances, printDepositoryInfo, getBalance, userBTCATA, userUXDATA } from "./integration_test_utils";
import { slippage } from "./test_2_consts";
import { depositoryBTC, mango, slippageBase, controllerUXD } from "./test_0_consts";

describe(" ======= [Suite 2-1 : Mint then redeem all BTC (2 op)] ======= ", () => {
  beforeEach("\n", async () => { });
  afterEach("", async () => {
    await printUserBalances();
    await printDepositoryInfo(depositoryBTC, mango);
  });


  it(`0 - initial state`, async () => { /* no-op */ });

  const slippagePercentage = slippage / slippageBase;

  // OP1
  let op1_amountUxdMinted: number;
  it(`1 - Mint UXD worth 0.01 BTC with ${slippagePercentage * 100}% max slippage`, async () => {
    // GIVEN
    const caller = user;
    const collateralAmount = 0.01; // in BTC
    const controller = controllerUXD;
    const depository = depositoryBTC;
    const _userUxdBalancePreOp = await getBalance(userUXDATA);
    const _userBtcBalancePreOp = await getBalance(userBTCATA);

    // WHEN
    await mintWithMangoDepository(caller, slippage, collateralAmount, controller, depository, mango);

    // Then
    // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
    const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmount;
    const _userUxdBalancePostOp = await getBalance(userUXDATA);
    const _userBtcBalancePostOp = await getBalance(userBTCATA);

    op1_amountUxdMinted = _userUxdBalancePostOp - _userUxdBalancePreOp;
    let op1_amountBtcUsed = _userBtcBalancePreOp - _userBtcBalancePostOp;

    expect(op1_amountBtcUsed).closeTo(collateralAmount, Math.pow(10, -controller.redeemableMintDecimals), "The collateral amount paid doesn't match the user wallet delta");
    expect(op1_amountUxdMinted).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

    console.log(`    ==> [Minted ${op1_amountUxdMinted} for ${op1_amountBtcUsed} BTC (approximperfect was ${maxAmountUxdMinted})]`);
  });

  // OP2
  let op2_amountUxdRedeemed: number;
  it(`2 - Redeem ${op1_amountUxdMinted} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
    // GIVEN
    const caller = user;
    const amountRedeemable = op1_amountUxdMinted; // In UXD
    const controller = controllerUXD;
    const depository = depositoryBTC;
    const _userUxdBalancePreOp = await getBalance(userUXDATA);
    const _userBtcBalancePreOp = await getBalance(userBTCATA);

    // WHEN
    await redeemFromMangoDepository(caller, slippage, amountRedeemable, controller, depository, mango);

    // THEN
    const maxAmountUxdRedeemed = op1_amountUxdMinted;
    const maxAmountBtcReceived = maxAmountUxdRedeemed / (await collateralUIPriceInMangoQuote(depository, mango));
    const _userUxdBalancePostOp = await getBalance(userUXDATA);
    const _userBtcBalancePostOp = await getBalance(userBTCATA);

    op2_amountUxdRedeemed = _userUxdBalancePreOp - _userUxdBalancePostOp;
    let op2_amountBtcReceived = _userBtcBalancePostOp - _userBtcBalancePreOp;
    // The amount of UXD that couldn't be redeemed due to odd lot size
    const unredeemedUXDAmount = amountRedeemable - op2_amountUxdRedeemed;

    expect(op2_amountUxdRedeemed).closeTo(maxAmountUxdRedeemed, maxAmountUxdRedeemed * (slippage), "The UXD amount redeemed is out of the slippage range");
    expect(op2_amountBtcReceived).closeTo(maxAmountBtcReceived, maxAmountBtcReceived * (slippage), "The BTC amount received is out of the slippage range");
    expect(_userUxdBalancePostOp).closeTo(_userUxdBalancePreOp - maxAmountUxdRedeemed + unredeemedUXDAmount, Math.pow(10, -controller.redeemableMintDecimals), "The amount of UXD carried over isn't right");

    console.log(`    ==> [Redeemed ${op2_amountUxdRedeemed} UXD for ${op2_amountBtcReceived} BTC (perfect was ${maxAmountBtcReceived}, returned UXD cause of odd lot ${unredeemedUXDAmount})]`);
  });
});