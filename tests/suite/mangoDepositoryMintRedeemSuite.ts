import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, Mango, createAndInitializeMango, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { initializeControllerTest } from "../cases/initializeControllerTest";
import { initializeMangoDepositoryTest } from "../cases/initializeMangoDepositoryTest";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "../cases/redeemWithMangoDepositoryTest";
import { bank, CLUSTER } from "../constants";
import { getConnection } from "../provider";
import { getBalance, printDepositoryInfo, printUserInfo } from "../utils";
import { MangoDepositoryTestSuiteParameters } from "./mangoDepositoryIntegrationSuite";

export const mangoDepositoryMintRedeemSuite = (authority: Signer, user: Signer, controller: Controller, depository: MangoDepository, params: MangoDepositoryTestSuiteParameters) => {
    let mango: Mango;

    beforeEach("\n", () => { console.log("=============================================\n\n") });

    before("setup", async () => {
        mango = await createAndInitializeMango(getConnection(), CLUSTER);
    });

    it("Initialize Controller", async () => {
        await initializeControllerTest(authority, controller);
    });

    it(`Initialize ${depository.collateralMintSymbol} Depository`, async () => {
        await initializeMangoDepositoryTest(authority, controller, depository, mango);
    });

    // TEST MINT/REDEEM

    it(`Mint 0.2 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then redeem the outcome`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(0.2, params.slippage, user, controller, depository, mango, bank);
        await redeemFromMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango, bank);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 2 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then redeem the outcome`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(2, params.slippage, user, controller, depository, mango, bank);
        await redeemFromMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango, bank);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 10 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then redeem the outcome`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(10, params.slippage, user, controller, depository, mango, bank);
        await redeemFromMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango, bank);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Redeem 1_000 UXD (${params.slippage} slippage) (should fail)`, async () => {
        try {
            await redeemFromMangoDepositoryTest(1_000, params.slippage, user, controller, depository, mango, bank);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - User's balance too low");
    });

    it(`Mint 0 UXD (${params.slippage} slippage) (should fail)`, async () => {
        try {
            await mintWithMangoDepositoryTest(0, params.slippage, user, controller, depository, mango, bank);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it(`Redeem 0 UXD (${params.slippage} slippage) (should fail)`, async () => {
        try {
            await redeemFromMangoDepositoryTest(0, params.slippage, user, controller, depository, mango, bank);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it(`Mint 1 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then redeem the outcome 5 times (stress test)`, async () => {
        for (var _i = 0; _i < 5; _i++) {
            const mintedAmount = await mintWithMangoDepositoryTest(1, params.slippage, user, controller, depository, mango, bank);
            await redeemFromMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango, bank);
        }
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 1 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) 5 times then redeem the outcome`, async () => {
        let mintedAmount: number = 0;
        for (var _i = 0; _i < 5; _i++) {
            mintedAmount += await mintWithMangoDepositoryTest(1, params.slippage, user, controller, depository, mango, bank);
        }
        await redeemFromMangoDepositoryTest(mintedAmount, params.slippage, user, controller, depository, mango, bank);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 10 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) then redeem the outcome in 3 times`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(10, params.slippage, user, controller, depository, mango, bank);

        const redeemAmountPartial = mintedAmount / 3;
        await redeemFromMangoDepositoryTest(redeemAmountPartial, params.slippage, user, controller, depository, mango, bank);
        await redeemFromMangoDepositoryTest(redeemAmountPartial, params.slippage, user, controller, depository, mango, bank);
        const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
        const remainingRedeemableAmount = await getBalance(userRedeemableATA);
        await redeemFromMangoDepositoryTest(remainingRedeemableAmount, params.slippage, user, controller, depository, mango, bank);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });
};