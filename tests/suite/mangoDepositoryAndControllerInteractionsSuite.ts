import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { controllerAccountingMangoDepositoryTest } from "../cases/controllerAccountingMangoDepositoryTest";
import { depositoryAccountingMangoDepositoryTest } from "../cases/depositoryAccountingMangoDepositoryTest";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "../cases/redeemFromMangoDepositoryTest";
import { setRedeemableGlobalSupplyCapTest } from "../cases/setRedeemableGlobalSupplyCapTest";
import { setRedeemableSoftCapMangoDepositoryTest } from "../cases/setRedeemableSoftCapMangoDepositoryTest";
import { getConnection, TXN_OPTS } from "../connection";
import { mango } from "../fixtures";
import { DepositoryAccountingInfo } from "../utils";

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

export const mangoDepositoryAndControllerInteractionsSuite = function (authority: Signer, user: Signer, payer: Signer, controller: Controller, depository: MangoDepository, params: MangoDepositoryAndControllerInteractionsSuiteParameters) {
    let depositoryAccounting: DepositoryAccountingInfo;
    before(`Get initial accounting data`, async () => {
        // Will need one for each depository
        const depositoryAccount = await depository.getOnchainAccount(getConnection(), TXN_OPTS);
        depositoryAccounting = {
            depository: depository,
            insuranceInitial: depositoryAccount.insuranceAmountDeposited.toNumber(),
            insuranceDelta: 0,
            collateralInitial: depositoryAccount.collateralAmountDeposited.toNumber(),
            collateralDelta: 0,
            redeemableInitial: depositoryAccount.redeemableAmountUnderManagement.toNumber(),
            redeemableDelta: 0,
        }
    });

    it(`Mint 2 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then Set Global Redeemable supply cap to 0 and redeem`, async function () {
        const mintedAmount = await mintWithMangoDepositoryTest(2, params.slippage, user, controller, depository, mango, payer);
        await setRedeemableGlobalSupplyCapTest(0, authority, controller);
        await redeemFromMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango, payer);
        // &*^ add AUM
    });

    it(`Set Global Redeemable supply cap to ${params.globalSupplyCapLow} then Mint 10 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) (should fail)`, async function () {
        await setRedeemableGlobalSupplyCapTest(params.globalSupplyCapLow, authority, controller);
        try {
            await mintWithMangoDepositoryTest(10, params.slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount beyond global supply cap");
    });

    it(`Reset Global Redeemable supply cap back to ${params.globalSupplyCap}`, async function () {
        await setRedeemableGlobalSupplyCapTest(params.globalSupplyCap, authority, controller);
    });

    it(`Mint 2 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then set the MangoDepositories Redeemable Soft cap to 0 and redeem`, async function () {
        const mintedAmount = await mintWithMangoDepositoryTest(2, params.slippage, user, controller, depository, mango, payer);
        await setRedeemableSoftCapMangoDepositoryTest(0, authority, controller);
        await redeemFromMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango, payer);
        // &*^ add AUM
    });

    it(`Set the MangoDepositories Redeemable Soft cap to ${params.mangoDepositoriesRedeemableSoftCapLow} then Mint 10 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) (should fail)`, async function () {
        await setRedeemableSoftCapMangoDepositoryTest(params.mangoDepositoriesRedeemableSoftCapLow, authority, controller);
        try {
            await mintWithMangoDepositoryTest(10, params.slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount beyond global supply cap");
    });

    it(`Reset MangoDepositories Redeemable Soft cap back to ${params.mangoDepositoriesRedeemableSoftCap}`, async function () {
        await setRedeemableSoftCapMangoDepositoryTest(params.mangoDepositoriesRedeemableSoftCap, authority, controller);
    });

    // after(`Check Controller Accounting`, async function () {
    //     await controllerAccountingMangoDepositoryTest(controller, [depository]);
    // });

    // after(`Check Depository Accounting`, async function () {
    //     await depositoryAccountingMangoDepositoryTest([depository], [depositoryAccounting]);
    // });
    // INSTEAD ^^
    // insert expects in mint and redeem calls
    // then confierm
    // then add to register and all tests check for accountiung (all cases)
};