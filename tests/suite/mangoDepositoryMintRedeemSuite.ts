import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "../cases/redeemFromMangoDepositoryTest";
import { mango } from "../fixtures";
import { getBalance, printDepositoryInfo, printUserInfo } from "../utils";

export const mangoDepositoryMintRedeemSuite = (user: Signer, payer: Signer, controller: Controller, depository: MangoDepository, slippage: number) => {

    it(`Redeem 100 ${depository.collateralMintSymbol} worth of UXD (${slippage} slippage) when no mint has happened (should fail)`, async () => {
        try {
            await redeemFromMangoDepositoryTest(100, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - No collateral deposited yet");
    });

    it(`Mint 1 ${depository.collateralMintSymbol} worth of UXD (${slippage} slippage) then redeem the outcome`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(1, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 1 ${depository.collateralMintSymbol} worth of UXD (${slippage} slippage) then redeem the outcome (With a separate Payer)`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(1, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Redeem 1_000_000 ${depository.collateralMintSymbol} worth of UXD (${slippage} slippage) when not enough has been minted yet (should fail)`, async () => {
        try {
            await redeemFromMangoDepositoryTest(1_000_000, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Redeeming beyond the available collateral");
    });

    it(`Mint 5 ${depository.collateralMintSymbol} worth of UXD (${slippage} slippage) then redeem the outcome`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(5, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 10 ${depository.collateralMintSymbol} worth of UXD (${slippage} slippage) then redeem the outcome`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(10, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Redeem 1_000 UXD (${slippage} slippage) (should fail)`, async () => {
        try {
            await redeemFromMangoDepositoryTest(1_000, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - User's balance too low");
    });

    it(`Mint 0 UXD (${slippage} slippage) (should fail)`, async () => {
        try {
            await mintWithMangoDepositoryTest(0, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it(`Redeem 0 UXD (${slippage} slippage) (should fail)`, async () => {
        try {
            await redeemFromMangoDepositoryTest(0, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it(`Mint 1 ${depository.collateralMintSymbol} worth of UXD (${slippage} slippage) then redeem the outcome 10 times (stress test)`, async () => {
        for (var _i = 0; _i < 10; _i++) {
            const mintedAmount = await mintWithMangoDepositoryTest(1, slippage, user, controller, depository, mango, payer);
            await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        }
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 1 ${depository.collateralMintSymbol} worth of UXD (${slippage} slippage) 10 times then redeem the outcome`, async () => {
        let mintedAmount: number = 0;
        for (var _i = 0; _i < 10; _i++) {
            mintedAmount += await mintWithMangoDepositoryTest(1, slippage, user, controller, depository, mango, payer);
        }
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 10 ${depository.collateralMintSymbol} worth of UXD (${slippage} slippage) then redeem the outcome in 3 times`, async () => {
        const mintedAmount = await mintWithMangoDepositoryTest(10, slippage, user, controller, depository, mango, payer);
        const redeemAmountPartial = mintedAmount / 3;
        await redeemFromMangoDepositoryTest(redeemAmountPartial, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(redeemAmountPartial, slippage, user, controller, depository, mango, payer);
        const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
        const remainingRedeemableAmount = await getBalance(userRedeemableATA);;
        await redeemFromMangoDepositoryTest(remainingRedeemableAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

};