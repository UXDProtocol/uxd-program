import { Signer } from "@solana/web3.js";
import { Controller } from "@uxdprotocol/uxd-client";
import { initializeControllerTest } from "../cases/initializeControllerTest";
import { setRedeemableGlobalSupplyCapTest } from "../cases/setRedeemableGlobalSupplyCapTest";
import { setRedeemableSoftCapMangoDepositoryTest } from "../cases/setRedeemableSoftCapMangoDepositoryTest";

export class controllerSuiteParameters {
    public globalSupplyCap: number;
    public mangoDepositoriesRedeemableSoftCap: number;

    public constructor(
        globalSupplyCap: number,
        mangoDepositoriesRedeemableSoftCap: number,
    ) {
        this.globalSupplyCap = globalSupplyCap;
        this.mangoDepositoriesRedeemableSoftCap = mangoDepositoriesRedeemableSoftCap;
    }
}

export const controllerIntegrationSuite = (authority: Signer, payer: Signer,  controller: Controller, params: controllerSuiteParameters) => {

    it("Initialize Controller", async () => {
        await initializeControllerTest(authority, controller, payer);
    });

    it(`Set Global Redeemable supply cap to ${params.globalSupplyCap}`, async () => {
        await setRedeemableGlobalSupplyCapTest(params.globalSupplyCap, authority, controller);
    });

    it(`Set Mango Depositories Redeemable soft cap to ${params.mangoDepositoriesRedeemableSoftCap}`, async () => {
        await setRedeemableSoftCapMangoDepositoryTest(params.mangoDepositoriesRedeemableSoftCap, authority, controller);
    });
}; 