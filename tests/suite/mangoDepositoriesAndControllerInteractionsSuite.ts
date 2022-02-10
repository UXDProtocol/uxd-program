import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "../cases/redeemFromMangoDepositoryTest";
import { setRedeemableGlobalSupplyCapTest } from "../cases/setRedeemableGlobalSupplyCapTest";
import { setRedeemableSoftCapMangoDepositoryTest } from "../cases/setRedeemableSoftCapMangoDepositoryTest";
import { mango } from "../fixtures";

export class MangoDepositoriesAndControllerInteractionsSuiteParameters {
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
        slippage: number,
    ) {
        this.globalSupplyCap = globalSupplyCap;
        this.globalSupplyCapLow = globalSupplyCapLow;
        this.mangoDepositoriesRedeemableSoftCap = mangoDepositoriesRedeemableSoftCap;
        this.mangoDepositoriesRedeemableSoftCapLow = mangoDepositoriesRedeemableSoftCapLow;
        this.slippage = slippage
    }
}

// Contain what can't be run in parallel due to having impact on the Controller

export const mangoDepositoriesAndControllerInteractionsSuite = (authority: Signer, user: Signer, payer: Signer, controller: Controller, depository: MangoDepository, params: MangoDepositoriesAndControllerInteractionsSuiteParameters) => {

    it(`Mint 2 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then Set Global Redeemable supply cap to 0 and redeem`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(2, params.slippage, user, controller, depository, mango, payer);
        await setRedeemableGlobalSupplyCapTest(0, authority, controller);
        await redeemFromMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango, payer);
    });

    it(`Set Global Redeemable supply cap to ${params.globalSupplyCapLow} then Mint 10 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) (should fail)`, async () => {
        await setRedeemableGlobalSupplyCapTest(params.globalSupplyCapLow, authority, controller);
        try {
            await mintWithMangoDepositoryTest(10, params.slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount beyond global supply cap");
    });

    it(`Reset Global Redeemable supply cap back to ${params.globalSupplyCap}`, async () => {
        await setRedeemableGlobalSupplyCapTest(params.globalSupplyCap, authority, controller);
    });

    // TEST REEDEMABLE SOFT CAP

    it(`Mint 2 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then set the MangoDepositories Redeemable Soft cap to 0 and redeem`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(2, params.slippage, user, controller, depository, mango, payer);
        await setRedeemableSoftCapMangoDepositoryTest(0, authority, controller);
        await redeemFromMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango, payer);
    });

    it(`Set the MangoDepositories Redeemable Soft cap to ${params.mangoDepositoriesRedeemableSoftCapLow} then Mint 10 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) (should fail)`, async () => {
        await setRedeemableSoftCapMangoDepositoryTest(params.mangoDepositoriesRedeemableSoftCapLow, authority, controller);
        try {
            await mintWithMangoDepositoryTest(10, params.slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount beyond global supply cap");
    });

    it(`Reset MangoDepositories Redeemable Soft cap back to ${params.mangoDepositoriesRedeemableSoftCap}`, async () => {
        await setRedeemableSoftCapMangoDepositoryTest(params.mangoDepositoriesRedeemableSoftCap, authority, controller);
    });

    // Accounting in controller test (TODO)
};