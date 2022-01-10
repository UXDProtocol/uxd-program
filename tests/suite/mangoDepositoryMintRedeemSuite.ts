import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, Mango, createAndInitializeMango, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { initializeControllerTest } from "../cases/initializeControllerTest";
import { initializeMangoDepositoryTest } from "../cases/initializeMangoDepositoryTest";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { redeemWithMangoDepositoryTest } from "../cases/redeemWithMangoDepositoryTest";
import { CLUSTER } from "../constants";
import { provider } from "../provider";
import { getBalance, printDepositoryInfo, printUserInfo } from "../utils";

export const mangoDepositoryMintRedeemSuite = (authority: Signer, user: Signer, controller: Controller, depository: MangoDepository) => {
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

    // TEST MINT/REDEEM

    it("Mint 0.2 SOL worth of UXD (2% slippage) then redeem the outcome", async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(0.2, 20, user, controller, depository, mango);
        await redeemWithMangoDepositoryTest(mintedAmount, 20, user, controller, depository, mango);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it("Mint 2 SOL worth of UXD (2% slippage) then redeem the outcome", async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(2, 20, user, controller, depository, mango);
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

    it("Mint 1 SOL worth of UXD (2% slippage) then redeem the outcome 5 times (stress test)", async () => {
        for (var _i = 0; _i < 5; _i++) {
            const mintedAmount = await mintWithMangoDepositoryTest(1, 20, user, controller, depository, mango);
            await redeemWithMangoDepositoryTest(mintedAmount, 20, user, controller, depository, mango);
        }
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it("Mint 1 SOL worth of UXD (2% slippage) 5 times then redeem the outcome", async () => {
        let mintedAmount: number = 0;
        for (var _i = 0; _i < 5; _i++) {
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
        const remainingRedeemableAmount = await getBalance(userRedeemableATA);
        await redeemWithMangoDepositoryTest(remainingRedeemableAmount, 20, user, controller, depository, mango);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });
};