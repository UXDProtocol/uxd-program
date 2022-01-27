import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, Mango, createAndInitializeMango, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { depositInsuranceMangoDepositoryTest } from "../cases/depositInsuranceMangoDepositoryTest";
import { initializeControllerTest } from "../cases/initializeControllerTest";
import { initializeMangoDepositoryTest } from "../cases/initializeMangoDepositoryTest";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { redeemWithMangoDepositoryTest } from "../cases/redeemWithMangoDepositoryTest";
import { setRedeemableGlobalSupplyCapTest } from "../cases/setRedeemableGlobalSupplyCapTest";
import { setRedeemableSoftCapMangoDepositoryTest } from "../cases/setRedeemableSoftCapMangoDepositoryTest";
import { withdrawInsuranceMangoDepositoryTest } from "../cases/withdrawInsuranceMangoDepositoryTest";
import { CLUSTER } from "../constants";
import { getProvider } from "../provider";
import { getBalance, printDepositoryInfo, printUserInfo } from "../utils";

export class MangoDepositoryTestSuiteParameters {
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
        this.slippage = slippage;
    }
}

export const mangoDepositoryIntegrationSuite = (authority: Signer, user: Signer, controller: Controller, depository: MangoDepository, params: MangoDepositoryTestSuiteParameters) => {
    let mango: Mango;

    beforeEach("\n", () => { console.log("=============================================\n\n") });

    before("setup", async () => {
        mango = await createAndInitializeMango(getProvider().connection, CLUSTER);
    });

    it("Initialize Controller", async () => {
        await initializeControllerTest(authority, controller);
    });

    it(`Initialize ${depository.collateralMintSymbol} Depository`, async () => {
        await initializeMangoDepositoryTest(authority, controller, depository, mango);
    });

    // SET REDEEMABLE CAPS
    it(`Set Global Redeemable supply cap to ${params.globalSupplyCap}`, async () => {
        await setRedeemableGlobalSupplyCapTest(params.globalSupplyCap, authority, controller);
    });

    it(`Set MangoDepositories Redeemable Soft cap to ${params.mangoDepositoriesRedeemableSoftCap}`, async () => {
        await setRedeemableSoftCapMangoDepositoryTest(params.mangoDepositoriesRedeemableSoftCap, authority, controller);
    });

    // TEST INSURANCE DEPOSIT

    it("DEPOSIT 0 USDC of insurance (should fail)", async () => {
        try {
            await depositInsuranceMangoDepositoryTest(0, authority, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it("Deposit 100 USDC of insurance", async () => {
        await depositInsuranceMangoDepositoryTest(100, authority, controller, depository, mango);
    });

    // // TEST MINT/REDEEM

    it(`Redeem 100 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) when no mint has happened (should fail)`, async () => {
        try {
            await redeemWithMangoDepositoryTest(100, params.slippage, user, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - No collateral deposited yet");
    });

    it(`Mint 1 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then redeem the outcome`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(1, params.slippage, user, controller, depository, mango);
        await redeemWithMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Redeem 1_000_000 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) when not enough has been minted yet (should fail)`, async () => {
        try {
            await redeemWithMangoDepositoryTest(1_000_000, params.slippage, user, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Redeeming beyond the available collateral");
    });

    it(`Mint 5 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then redeem the outcome`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(5, params.slippage, user, controller, depository, mango);
        await redeemWithMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 10 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then redeem the outcome`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(10, params.slippage, user, controller, depository, mango);
        await redeemWithMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Redeem 1_000 UXD (${params.slippage} slippage) (should fail)`, async () => {
        try {
            await redeemWithMangoDepositoryTest(1_000, params.slippage, user, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - User's balance too low");
    });

    it(`Mint 0 UXD (${params.slippage} slippage) (should fail)`, async () => {
        try {
            await mintWithMangoDepositoryTest(0, params.slippage, user, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it(`Redeem 0 UXD (${params.slippage} slippage) (should fail)`, async () => {
        try {
            await redeemWithMangoDepositoryTest(0, params.slippage, user, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it(`Mint 1 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then redeem the outcome 10 times (stress test)`, async () => {
        for (var _i = 0; _i < 10; _i++) {
            const mintedAmount = await mintWithMangoDepositoryTest(1, params.slippage, user, controller, depository, mango);
            await redeemWithMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango);
        }
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 1 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) 10 times then redeem the outcome`, async () => {
        let mintedAmount: number = 0;
        for (var _i = 0; _i < 10; _i++) {
            mintedAmount += await mintWithMangoDepositoryTest(1, params.slippage, user, controller, depository, mango);
        }
        await redeemWithMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 10 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then redeem the outcome in 3 times`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(10, params.slippage, user, controller, depository, mango);
        const redeemAmountPartial = mintedAmount / 3;
        await redeemWithMangoDepositoryTest(redeemAmountPartial, params.slippage, user, controller, depository, mango);
        await redeemWithMangoDepositoryTest(redeemAmountPartial, params.slippage, user, controller, depository, mango);
        const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
        const remainingRedeemableAmount = await getBalance(userRedeemableATA);;
        await redeemWithMangoDepositoryTest(remainingRedeemableAmount, params.slippage, user, controller, depository, mango);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    // TEST GLOBAL REDEEMABLE CAP

    it(`Mint 2 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then Set Global Redeemable supply cap to 0 and redeem`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(2, params.slippage, user, controller, depository, mango);
        await setRedeemableGlobalSupplyCapTest(0, authority, controller);
        await redeemWithMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango);
    });

    it(`Set Global Redeemable supply cap to ${params.globalSupplyCapLow} then Mint 10 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) (should fail)`, async () => {
        await setRedeemableGlobalSupplyCapTest(params.globalSupplyCapLow, authority, controller);
        try {
            await mintWithMangoDepositoryTest(10, params.slippage, user, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount beyond global supply cap");
    });

    it(`Reset Global Redeemable supply cap back to ${params.globalSupplyCap}`, async () => {
        await setRedeemableGlobalSupplyCapTest(params.globalSupplyCap, authority, controller);
    });

    // TEST MANGO DEPOSITORIES SOFT CAP

    it(`Mint 2 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then set the MangoDepositories Redeemable Soft cap to 0 and redeem`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(2, params.slippage, user, controller, depository, mango);
        await setRedeemableSoftCapMangoDepositoryTest(0, authority, controller);
        await redeemWithMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango);
    });

    it(`Set the MangoDepositories Redeemable Soft cap to ${params.mangoDepositoriesRedeemableSoftCapLow} then Mint 10 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) (should fail)`, async () => {
        await setRedeemableSoftCapMangoDepositoryTest(params.mangoDepositoriesRedeemableSoftCapLow, authority, controller);
        try {
            await mintWithMangoDepositoryTest(10, params.slippage, user, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount beyond global supply cap");
    });

    it(`Reset MangoDepositories Redeemable Soft cap back to ${params.mangoDepositoriesRedeemableSoftCap}`, async () => {
        await setRedeemableSoftCapMangoDepositoryTest(params.mangoDepositoriesRedeemableSoftCap, authority, controller);
    });

    // TEST INSURANCE WITHDRAWAL

    it(`Withdraw 0 USDC of insurance (should fail)`, async () => {
        try {
            await withdrawInsuranceMangoDepositoryTest(0, authority, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it(`Withdraw 10_000 USDC of insurance (should fail)`, async () => {
        try {
            await withdrawInsuranceMangoDepositoryTest(10_000, authority, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is too big");
    });

    // Due to mango health constraints we cannot remove the entirety 
    it(`Withdraw 90 USDC of insurance`, async () => {
        await depositInsuranceMangoDepositoryTest(90, authority, controller, depository, mango);
    });

    // TODO ACCOUNTING TESTS

};
