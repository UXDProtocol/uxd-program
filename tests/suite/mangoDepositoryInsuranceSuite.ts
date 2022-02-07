import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { depositInsuranceMangoDepositoryTest } from "../cases/depositInsuranceMangoDepositoryTest";
import { withdrawInsuranceMangoDepositoryTest } from "../cases/withdrawInsuranceMangoDepositoryTest";
import { mango } from "../fixtures";

export const mangoDepositoryInsuranceSuite = (authority: Signer, controller: Controller, depository: MangoDepository) => {

    it("DEPOSIT 0 USDC of insurance (should fail)", async () => {
        try {
            await depositInsuranceMangoDepositoryTest(0, authority, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it(`Withdraw 0 USDC of insurance (should fail)`, async () => {
        try {
            await withdrawInsuranceMangoDepositoryTest(0, authority, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });


    it(`Deposit 1 USDC of insurance`, async () => {
        await depositInsuranceMangoDepositoryTest(1, authority, controller, depository, mango);
    });

    it(`Withdraw 1 USDC of insurance (should fail)`, async () => {
        try {
            await withdrawInsuranceMangoDepositoryTest(1, authority, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is too big");
    });

    it(`Deposit 100_000 USDC of insurance`, async () => {
        await depositInsuranceMangoDepositoryTest(100_000, authority, controller, depository, mango);
    });

    it(`Withdraw 1 USDC of insurance`, async () => {
        await withdrawInsuranceMangoDepositoryTest(1, authority, controller, depository, mango);
    });

    it(`Withdraw 500_000 USDC of insurance (should fail)`, async () => {
        try {
            await withdrawInsuranceMangoDepositoryTest(500_000, authority, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is too big");
    });

    it(`Withdraw -1 USDC of insurance (should fail)`, async () => {
        try {
            await withdrawInsuranceMangoDepositoryTest(-1, authority, controller, depository, mango);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is negative");
    });

    // Due to mango health constraints we cannot remove the entirety 
    it(`Withdraw 99_900 USDC of insurance`, async () => {
        await depositInsuranceMangoDepositoryTest(99_900, authority, controller, depository, mango);
    });
};
