import { Signer } from "@solana/web3.js";
import { Controller } from "@uxdprotocol/uxd-client";
import { setMangoDepositoriesRedeemableSoftCap } from "../api";
import { initializeControllerTest } from "../cases/initializeControllerTest";
import { setRedeemableGlobalSupplyCapTest } from "../cases/setRedeemableGlobalSupplyCapTest";

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

export const controllerIntegrationSuite = (authority: Signer, controller: Controller, params: controllerSuiteParameters) => {

    it("Initialize Controller", async () => {
        await initializeControllerTest(authority, controller);
    });

    it(`Set Global Redeemable supply cap to ${params.globalSupplyCap}`, async () => {
        await setRedeemableGlobalSupplyCapTest(authority, controller, params.globalSupplyCap);
    });

    it(`Set Mango Depositories Redeemable soft cap to ${params.mangoDepositoriesRedeemableSoftCap}`, async () => {
        await setMangoDepositoriesRedeemableSoftCap(authority, controller, params.mangoDepositoriesRedeemableSoftCap);
    });
};