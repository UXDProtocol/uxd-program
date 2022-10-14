import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { editControllerTest } from "../cases/editControllerTest";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "../cases/redeemFromMangoDepositoryTest";
import { mango } from "../fixtures";

export class MangoDepositoryAndControllerInteractionsSuiteParameters {
  public globalSupplyCap: number;
  public globalSupplyCapLow: number;
  public mangoDepositoriesRedeemableSoftCap: number;
  public mangoDepositoriesRedeemableSoftCapLow: number;
  public slippage: number;

  public constructor(
    globalSupplyCap: number,
    globalSupplyCapLow: number,
    mangoDepositoriesRedeemableSoftCap: number,
    mangoDepositoriesRedeemableSoftCapLow: number,
    slippage: number
  ) {
    this.globalSupplyCap = globalSupplyCap;
    this.globalSupplyCapLow = globalSupplyCapLow;
    this.mangoDepositoriesRedeemableSoftCap = mangoDepositoriesRedeemableSoftCap;
    this.mangoDepositoriesRedeemableSoftCapLow = mangoDepositoriesRedeemableSoftCapLow;
    this.slippage = slippage;
  }
}

// Contain what can't be run in parallel due to having impact on the Controller

export const mangoDepositoryAndControllerInteractionsSuite = function (
  authority: Signer,
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: MangoDepository,
  params: MangoDepositoryAndControllerInteractionsSuiteParameters
) {
  it(`Mint 10 ${controller.redeemableMintSymbol} (${params.slippage} slippage) then Set Global Redeemable supply cap to 0 and redeem`, async function () {
    const perpPrice = await depository.getCollateralPerpPriceUI(mango);
    const amount = 10 / perpPrice;
    console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
    const mintedAmount = await mintWithMangoDepositoryTest(
      amount,
      params.slippage,
      user,
      controller,
      depository,
      mango,
      payer
    );

    await editControllerTest(authority, controller, {
      redeemableGlobalSupplyCap: 0,
    });

    await redeemFromMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango, payer);
  });

  it(`Set Global Redeemable supply cap to ${params.globalSupplyCapLow} then Mint 10 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) (should fail)`, async function () {
    await editControllerTest(authority, controller, {
      redeemableGlobalSupplyCap: params.globalSupplyCapLow,
    });

    try {
      await mintWithMangoDepositoryTest(10, params.slippage, user, controller, depository, mango, payer);
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - Amount beyond global supply cap");
  });

  it(`Reset Global Redeemable supply cap back to ${params.globalSupplyCap}`, async function () {
    await editControllerTest(authority, controller, {
      redeemableGlobalSupplyCap: params.globalSupplyCap,
    });
  });

  it(`Mint 10 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then set the MangoDepositories Redeemable Soft cap to 0 and redeem`, async function () {
    const perpPrice = await depository.getCollateralPerpPriceUI(mango);
    const amount = 10 / perpPrice;
    console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
    const mintedAmount = await mintWithMangoDepositoryTest(
      amount,
      params.slippage,
      user,
      controller,
      depository,
      mango,
      payer
    );

    await editControllerTest(authority, controller, {
      mangoDepositoriesRedeemableSoftCap: 0,
    });

    await redeemFromMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango, payer);
  });

  it(`Set the MangoDepositories Redeemable Soft cap to ${params.mangoDepositoriesRedeemableSoftCapLow} then Mint 10 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) (should fail)`, async function () {
    await editControllerTest(authority, controller, {
      mangoDepositoriesRedeemableSoftCap: params.mangoDepositoriesRedeemableSoftCapLow,
    });
    try {
      await mintWithMangoDepositoryTest(10, params.slippage, user, controller, depository, mango, payer);
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - Amount beyond global supply cap");
  });

  it(`Reset MangoDepositories Redeemable Soft cap back to ${params.mangoDepositoriesRedeemableSoftCap}`, () => editControllerTest(authority, controller, {
    mangoDepositoriesRedeemableSoftCap: params.mangoDepositoriesRedeemableSoftCap,
  }));
};
