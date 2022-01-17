import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, Mango, createAndInitializeMango } from "@uxdprotocol/uxd-client";
import { depositInsuranceMangoDepositoryTest } from "../cases/depositInsuranceMangoDepositoryTest";
import { initializeControllerTest } from "../cases/initializeControllerTest";
import { initializeMangoDepositoryTest } from "../cases/initializeMangoDepositoryTest";
import { CLUSTER } from "../constants";
import { provider } from "../provider";

export const setupControllerAndMangoDepositorySolSuite = (authority: Signer, controller: Controller, depository: MangoDepository) => {
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

    it("Deposit 100 USDC of insurance", async () => {
        await depositInsuranceMangoDepositoryTest(100, authority, controller, depository, mango);
    });
};