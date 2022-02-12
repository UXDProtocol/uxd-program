import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "../cases/redeemFromMangoDepositoryTest";
import { slippageBase } from "../constants";
import { mango } from "../fixtures";
import { getBalance, printDepositoryInfo, printUserInfo, transferAllTokens, transferTokens } from "../utils";

export const mangoDepositoryMintRedeemSuite = (user: Signer, payer: Signer, controller: Controller, depository: MangoDepository, slippage: number) => {

    it(`Transfer 300,000 USD worth of ${depository.collateralMintSymbol} from payer to user`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 300_000 / perpPrice;
        console.log("[🧾 amount", amount, "]");
        await transferTokens(amount, depository.collateralMint, depository.collateralMintDecimals, payer, user.publicKey);
    });

    it(`Redeem 100 UXD (${slippage / slippageBase} % slippage) when no mint has happened (should fail)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 100 / perpPrice;
        console.log("[🧾 amount", amount, "]");
        try {
            await redeemFromMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - No collateral deposited yet");
    });

    it(`Mint minTradingSize UXD then redeem the outcome (0.1% slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const minTradingSize = await depository.getMinTradingSizeUi(mango);
        const amount = (minTradingSize * 1.1) / perpPrice; // + 10%
        console.log("[🧾 amount", amount, "]");
        console.log("[🧾 $ value", amount * perpPrice, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(amount, 1, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 10 UXD then redeem the outcome (${slippage / slippageBase} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 10 / perpPrice;
        console.log("[🧾 amount", amount, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 100 UXD then redeem the outcome (${slippage / slippageBase} % slippage)`, async function () {
        printUserInfo(user.publicKey, controller, depository);
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 100 / perpPrice;
        console.log("[🧾 amount", amount, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Redeem 100,000 UXD when not enough has been minted yet (should fail)`, async function () {
        try {
            await redeemFromMangoDepositoryTest(100_000, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Redeeming beyond the available redeemable under management");
    });

    it(`Mint 500 UXD then redeem the outcome  (${slippage / slippageBase} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 500 / perpPrice;
        console.log("[🧾 amount", amount, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 1000 UXD then redeem the outcome (${slippage / slippageBase} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 1000 / perpPrice;
        console.log("[🧾 amount", amount, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 100,000 UXD then redeem the outcome (${slippage / slippageBase} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 100_000 / perpPrice;
        const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Redeem 1000 UXD (0% slippage) (should fail)`, async function () {
        try {
            await redeemFromMangoDepositoryTest(1_000, 0, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - User's balance too low");
    });

    it(`Mint -10 UXD (${slippage / slippageBase} % slippage) (should fail)`, async function () {
        try {
            await mintWithMangoDepositoryTest(-10, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is negative");
    });

    it(`Redeem -10 UXD (${slippage / slippageBase} % slippage) (should fail)`, async function () {
        try {
            await redeemFromMangoDepositoryTest(-10, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is negative");
    });

    it(`Mint 0 UXD (${slippage / slippageBase} % slippage) (should fail)`, async function () {
        try {
            await mintWithMangoDepositoryTest(0, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it(`Redeem 0 UXD (${slippage / slippageBase} % slippage) (should fail)`, async function () {
        try {
            await redeemFromMangoDepositoryTest(0, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it(`Mint 100 UXD then redeem the outcome, 10 times (${slippage / slippageBase} % slippage) (🌶 stress test)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 100 / perpPrice;
        console.log("[🧾 amount", amount, "]");
        for (var _i = 0; _i < 10; _i++) {
            const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
            await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        }
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 100 UXD 5 times then redeem the outcome (${slippage / slippageBase} % slippage)  (🌶 stress test)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 100 / perpPrice;
        console.log("[🧾 amount", amount, "]");
        let mintedAmount: number = 0;
        for (var _i = 0; _i < 5; _i++) {
            mintedAmount += await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        }
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 1000 UXD then redeem the outcome in 3 times (${slippage / slippageBase} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 1000 / perpPrice;
        console.log("[🧾 amount", amount, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        const redeemAmountPartial = mintedAmount / 3;
        await redeemFromMangoDepositoryTest(redeemAmountPartial, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(redeemAmountPartial, slippage, user, controller, depository, mango, payer);
        const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
        const remainingRedeemableAmount = await getBalance(userRedeemableATA);;
        await redeemFromMangoDepositoryTest(remainingRedeemableAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 0.6969 ${depository.collateralMintSymbol} worth of UXD then redeem the outcome (${slippage / slippageBase} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        console.log("[🧾 $ value", 0.6969 * perpPrice, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(0.6969, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 1 ${depository.collateralMintSymbol} worth of UXD then redeem the outcome (${slippage / slippageBase} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        console.log("[🧾 $ value", 1 * perpPrice, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(1, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
    });

    it(`Return remaining ${depository.collateralMintSymbol} user's balance to the payer`, async function () {
        await transferAllTokens(depository.collateralMint, depository.collateralMintDecimals, user, payer.publicKey);
    });

};