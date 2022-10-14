import { Signer } from "@solana/web3.js";
import { Controller } from "@uxd-protocol/uxd-client";
import { editControllerTest } from "../cases/editControllerTest";
import { initializeControllerTest } from "../cases/initializeControllerTest";

export class controllerIntegrationSuiteParameters {
  public globalSupplyCap: number;
  public mangoDepositoriesRedeemableSoftCap: number;

  public constructor(globalSupplyCap: number, mangoDepositoriesRedeemableSoftCap: number) {
    this.globalSupplyCap = globalSupplyCap;
    this.mangoDepositoriesRedeemableSoftCap = mangoDepositoriesRedeemableSoftCap;
  }
}

export const controllerIntegrationSuite = function (
  authority: Signer,
  payer: Signer,
  controller: Controller,
  params: controllerIntegrationSuiteParameters
) {
  it("Initialize Controller", async function () {
    await initializeControllerTest(authority, controller, payer);
  });

  it(`Set Global Redeemable supply cap to ${params.globalSupplyCap}`, () => editControllerTest(authority, controller, {
    redeemableGlobalSupplyCap: params.globalSupplyCap,
  }));

  it(`Set Mango Depositories Redeemable soft cap to ${params.mangoDepositoriesRedeemableSoftCap}`, () => editControllerTest(authority, controller, {
    mangoDepositoriesRedeemableSoftCap: params.mangoDepositoriesRedeemableSoftCap,
  }));

  it(
    [
      `Set Mango Depositories Redeemable soft cap to ${params.mangoDepositoriesRedeemableSoftCap}`,
      `And set Global Redeemable supply cap to ${params.globalSupplyCap} at the same time`,
    ].join(" "),
    () => editControllerTest(authority, controller, {
      mangoDepositoriesRedeemableSoftCap: params.mangoDepositoriesRedeemableSoftCap,
      redeemableGlobalSupplyCap: params.globalSupplyCap,
    })
  );
};
