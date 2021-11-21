import { expect } from "chai";
import { user } from "./identities";
import { controllerUXD, depositoryBTC, mintWithMangoDepository, redeemFromMangoDepository, mango, collateralUIPriceInMangoQuote } from "./uxdApi";
import { printWorldInfo, printUserBalances, printDepositoryInfo, getBalance, userBTCATA, userUXDATA } from "./integration_test_utils";

before("Initial world state", async () => {
  printWorldInfo();
  await printUserBalances();
});

describe(" ======= [Suite one : Mint then redeem all BTC (2 op)] ======= ", () => {

  afterEach("", async () => {
    await printUserBalances();
    await printDepositoryInfo(depositoryBTC, mango);
  });

  const slippageBase = 1000;
  const slippage = 10; // <=> 1%
  const slippagePercentage = slippage / slippageBase;

  // OP1
  let op1_amountUxdMinted: number;
  it(`Mint UXD worth 0.01 BTC with ${slippagePercentage * 100}% max slippage`, async () => {
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
    const maxAmountUxdMinted = (await collateralUIPriceInMangoQuote(caller, depository, mango)) * collateralAmount;
    const _userUxdBalancePostOp = await getBalance(userUXDATA);
    const _userBtcBalancePostOp = await getBalance(userBTCATA);

    op1_amountUxdMinted = Number((_userUxdBalancePostOp - _userUxdBalancePreOp).toPrecision(controller.redeemableMintDecimals));
    let op1_amountBtcUsed = Number((_userBtcBalancePostOp - _userBtcBalancePreOp).toPrecision(depository.collateralMintdecimals));

    expect(op1_amountBtcUsed).equals(collateralAmount * -1, "The collateral amount paid doesn't match the user wallet delta");
    expect(op1_amountUxdMinted).closeTo(maxAmountUxdMinted, maxAmountUxdMinted * (slippage), "The amount minted is out of the slippage range");

    console.log(`<<<<<>>>>> ==> [Minted ${op1_amountUxdMinted} for ${op1_amountBtcUsed} BTC (prefect was ${maxAmountUxdMinted})]`);
  });

  // OP2
  let op2_amountUxdRedeemed: number;
  it(`Redeem ${op1_amountUxdMinted} UXD with ${slippagePercentage * 100}% max slippage`, async () => {
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
    const maxAmountBtcReceived = Number((maxAmountUxdRedeemed / (await collateralUIPriceInMangoQuote(caller, depository, mango))).toPrecision(depository.collateralMintdecimals));
    const _userUxdBalancePostOp = await getBalance(userUXDATA);
    const _userBtcBalancePostOp = await getBalance(userBTCATA);

    op2_amountUxdRedeemed = Number((_userUxdBalancePreOp - _userUxdBalancePostOp).toPrecision(controller.redeemableMintDecimals));
    let op2_amountBtcReceived = Number((_userBtcBalancePostOp - _userBtcBalancePreOp).toPrecision(depository.collateralMintdecimals));

    expect(op2_amountUxdRedeemed).closeTo(maxAmountUxdRedeemed, maxAmountUxdRedeemed * (slippage), "The UXD amount redeemed is out of the slippage range");
    expect(op2_amountBtcReceived).closeTo(maxAmountBtcReceived, maxAmountBtcReceived * (slippage), "The BTC amount received is out of the slippage range");

    console.log(`<<<<<>>>>> ==> [Redeemed ${op2_amountUxdRedeemed} UXD for ${op2_amountBtcReceived} BTC (perfect was ${maxAmountBtcReceived})]`);
  });
});