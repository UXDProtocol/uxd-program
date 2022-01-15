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
import { provider } from "../provider";
import { getBalance, printDepositoryInfo, printUserInfo } from "../utils";

export const mangoDepositoryIntegrationSuite = (authority: Signer, user: Signer, controller: Controller, depository: MangoDepository) => {
    let mango: Mango;

    beforeEach("\n", () => { console.log("=============================================\n\n") });

    before("setup", async () => {
        mango = await createAndInitializeMango(provider.connection, CLUSTER);
    });

    it("Initialize Controller", async () => {
        await initializeControllerTest(authority, controller);
    });

    it("Initialize SOL Depository", async () => {
        await initializeMangoDepositoryTest(authority, controller, depository, mango);
    });

    // SET REDEEMABLE CAPS (as they should be by default at launch)
    it("Set Global Redeemable supply cap to 1_000_000", async () => {
        await setRedeemableGlobalSupplyCapTest(1_000_000, authority, controller);
    });

    it("Set MangoDepositories Redeemable Soft cap to 10_000", async () => {
        await setRedeemableSoftCapMangoDepositoryTest(10_000, authority, controller);
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

    // TEST MINT/REDEEM

    it("Mint 1 SOL worth of UXD (2% slippage) then redeem the outcome", async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(1, 20, user, controller, depository, mango);
        await redeemWithMangoDepositoryTest(mintedAmount, 20, user, controller, depository, mango);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it("Mint 5 SOL worth of UXD (2% slippage) then redeem the outcome", async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(5, 20, user, controller, depository, mango);
        await redeemWithMangoDepositoryTest(mintedAmount, 20, user, controller, depository, mango);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it("Mint 10 SOL worth of UXD (2% slippage) then redeem the outcome", async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(10, 20, user, controller, depository, mango);
        await redeemWithMangoDepositoryTest(mintedAmount, 20, user, controller, depository, mango);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it("Redeem 1_000 UXD (2% slippage) (should fail)", async () => {
        try {
            await redeemWithMangoDepositoryTest(1_000, 20, user, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - User's balance too low");
    });

    it("Mint 0 UXD (2% slippage) (should fail)", async () => {
        try {
            await mintWithMangoDepositoryTest(0, 20, user, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it("Redeem 0 UXD (2% slippage) (should fail)", async () => {
        try {
            await redeemWithMangoDepositoryTest(0, 20, user, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it("Mint 1 SOL worth of UXD (2% slippage) then redeem the outcome 10 times (stress test)", async () => {
        for (var _i = 0; _i < 10; _i++) {
            const mintedAmount = await mintWithMangoDepositoryTest(1, 20, user, controller, depository, mango);
            await redeemWithMangoDepositoryTest(mintedAmount, 20, user, controller, depository, mango);
        }
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it("Mint 1 SOL worth of UXD (2% slippage) 10 times then redeem the outcome", async () => {
        let mintedAmount: number = 0;
        for (var _i = 0; _i < 10; _i++) {
            mintedAmount += await mintWithMangoDepositoryTest(1, 20, user, controller, depository, mango);
        }
        await redeemWithMangoDepositoryTest(mintedAmount, 20, user, controller, depository, mango);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it("Mint 10 SOL worth of UXD (2% slippage) then redeem the outcome in 3 times", async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(10, 20, user, controller, depository, mango);

        const redeemAmountPartial = mintedAmount / 3;
        await redeemWithMangoDepositoryTest(redeemAmountPartial, 20, user, controller, depository, mango);
        await redeemWithMangoDepositoryTest(redeemAmountPartial, 20, user, controller, depository, mango);
        const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
        const remainingRedeemableAmount = await getBalance(userRedeemableATA);;
        await redeemWithMangoDepositoryTest(remainingRedeemableAmount, 20, user, controller, depository, mango);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    // TEST GLOBAL REDEEMABLE CAP

    it("Mint 2 SOL worth of UXD (2% slippage) then Set Global Redeemable supply cap to 0 and redeem", async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(2, 20, user, controller, depository, mango);
        await setRedeemableGlobalSupplyCapTest(0, authority, controller);
        await redeemWithMangoDepositoryTest(mintedAmount, 20, user, controller, depository, mango);
    });

    it("Set Global Redeemable supply cap to 500 then Mint 10 SOL worth of UXD (2% slippage) (should fail)", async () => {
        await setRedeemableGlobalSupplyCapTest(500, authority, controller);
        try {
            await mintWithMangoDepositoryTest(10, 20, user, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount beyond global supply cap");
    });

    it("Mint 0.5 sol then redeem with the Global Redeemable supply cap at 500", async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(0.5, 20, user, controller, depository, mango);
        await redeemWithMangoDepositoryTest(mintedAmount, 20, user, controller, depository, mango);
    });

    it("Reset Global Redeemable supply cap back to 1_000_000", async () => {
        await setRedeemableGlobalSupplyCapTest(1_000_000, authority, controller);
    });

    // TEST MANGO DEPOSITORIES SOFT CAP

    it("Mint 2 SOL worth of UXD (2% slippage) then set the MangoDepositories Redeemable Soft cap to 0 and redeem", async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(2, 20, user, controller, depository, mango);
        await setRedeemableSoftCapMangoDepositoryTest(0, authority, controller);
        await redeemWithMangoDepositoryTest(mintedAmount, 20, user, controller, depository, mango);
    });

    it("Set the MangoDepositories Redeemable Soft cap to 500 then Mint 10 SOL worth of UXD (2% slippage) (should fail)", async () => {
        await setRedeemableSoftCapMangoDepositoryTest(500, authority, controller);
        try {
            await mintWithMangoDepositoryTest(10, 20, user, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount beyond global supply cap");
    });

    it("Mint 0.5 sol then redeem with the MangoDepositories Redeemable Soft cap at 500", async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(0.5, 20, user, controller, depository, mango);
        await redeemWithMangoDepositoryTest(mintedAmount, 20, user, controller, depository, mango);
    });

    it("Reset MangoDepositories Redeemable Soft cap back to 10_000", async () => {
        await setRedeemableSoftCapMangoDepositoryTest(10_000, authority, controller);
    });

    // TEST INSURANCE WITHDRAWAL

    it("Withdraw 0 USDC of insurance (should fail)", async () => {
        try {
            await withdrawInsuranceMangoDepositoryTest(0, authority, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it("Withdraw 10_000 USDC of insurance (should fail)", async () => {
        try {
            await withdrawInsuranceMangoDepositoryTest(10_000, authority, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is too big");
    });

    // Due to mango health constraints we cannot remove the entirety 
    it("Withdraw 90 USDC of insurance", async () => {
        await depositInsuranceMangoDepositoryTest(90, authority, controller, depository, mango);
    });

    // TODO ACCOUNTING TESTS

};