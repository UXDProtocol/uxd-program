import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, findATAAddrSync } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "../cases/redeemFromMangoDepositoryTest";
import { slippageBase } from "../constants";
import { mango } from "../fixtures";
import { getBalance, printUserInfo, transferAllTokens, transferSol, transferTokens } from "../utils";

export const mangoDepositoryMintRedeemSuite = function (user: Signer, payer: Signer, controller: Controller, depository: MangoDepository, slippage: number) {

    before(`Transfer 50 USD worth of ${depository.collateralMintSymbol} from payer to user`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 50 / perpPrice;
        console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
        // For Wsol we send sol, the API handle the wrapping before each minting
        if (depository.collateralMint.equals(NATIVE_MINT)) {
            await transferSol(amount, payer, user.publicKey);
        } else {
            await transferTokens(amount, depository.collateralMint, depository.collateralMintDecimals, payer, user.publicKey);
        }
    });

    it(`Redeem 1 ${controller.redeemableMintSymbol} (${slippage / slippageBase} % slippage) when no mint has happened (should fail)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 1 / perpPrice;
        console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
        try {
            await redeemFromMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - No collateral deposited yet");
    });

    it(`Mint 1 ${controller.redeemableMintSymbol} then redeem the outcome (${slippage / slippageBase * 100} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 1 / perpPrice;
        console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
    });

    it(`Mint 10 ${controller.redeemableMintSymbol} then redeem the outcome (${slippage / slippageBase * 100} % slippage)`, async function () {
        printUserInfo(user.publicKey, controller, depository);
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 10 / perpPrice;
        console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
    });

    it(`Redeem 30 ${controller.redeemableMintSymbol} when not enough has been minted yet (should fail)`, async function () {
        try {
            await redeemFromMangoDepositoryTest(30, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Redeeming beyond the available redeemable under management");
    });

    it(`Mint 5 ${controller.redeemableMintSymbol} then redeem the outcome  (${slippage / slippageBase * 100} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 5 / perpPrice;
        console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
    });

    it(`Mint 10 ${controller.redeemableMintSymbol} then redeem the outcome (${slippage / slippageBase * 100} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 10 / perpPrice;
        console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
    });

    it(`Mint 30 ${controller.redeemableMintSymbol} then redeem the outcome (30% slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 30 / perpPrice;
        const mintedAmount = await mintWithMangoDepositoryTest(amount, 300, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        // await printUserInfo(user.publicKey, controller, depository);
        // await printDepositoryInfo(controller, depository, mango);
    });

    it(`Redeem 10 ${controller.redeemableMintSymbol} (0% slippage) (should fail)`, async function () {
        try {
            await redeemFromMangoDepositoryTest(10, 0, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - User's balance too low");
    });

    it.skip(`Mint -10 ${controller.redeemableMintSymbol} (${slippage / slippageBase} % slippage) (should fail)`, async function () {
        try {
            await mintWithMangoDepositoryTest(-10, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is negative");
    });

    it.skip(`Redeem -10 ${controller.redeemableMintSymbol} (${slippage / slippageBase} % slippage) (should fail)`, async function () {
        try {
            await redeemFromMangoDepositoryTest(-10, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is negative");
    });

    it(`Mint 0 ${controller.redeemableMintSymbol} (${slippage / slippageBase} % slippage) (should fail)`, async function () {
        try {
            await mintWithMangoDepositoryTest(0, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it(`Redeem 0 ${controller.redeemableMintSymbol} (${slippage / slippageBase} % slippage) (should fail)`, async function () {
        try {
            await redeemFromMangoDepositoryTest(0, slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Amount is 0");
    });

    it(`Mint 1 ${controller.redeemableMintSymbol} then redeem the outcome, 5 times (${slippage / slippageBase * 100} % slippage) (🌶 stress test)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 1 / perpPrice;
        console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
        for (var _i = 0; _i < 5; _i++) {
            const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
            await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        }
        // await printUserInfo(user.publicKey, controller, depository);
        // await printDepositoryInfo(controller, depository, mango);
    });

    it.skip(`Mint 1 ${controller.redeemableMintSymbol} 3 times then redeem the outcome (${slippage / slippageBase * 100} % slippage)  (🌶 stress test)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 1 / perpPrice;
        console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
        let mintedAmount: number = 0;
        for (var _i = 0; _i < 3; _i++) {
            mintedAmount += await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        }
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
        // await printUserInfo(user.publicKey, controller, depository);
        // await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint 10 ${controller.redeemableMintSymbol} then redeem the outcome in 3 times (${slippage / slippageBase * 100} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const collateralAmount = 10 / perpPrice;
        console.log("[🧾 amount", collateralAmount, depository.collateralMintSymbol, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(collateralAmount, slippage, user, controller, depository, mango, payer);
        const redeemAmountPartial = mintedAmount / 3;
        await redeemFromMangoDepositoryTest(redeemAmountPartial, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(redeemAmountPartial, slippage, user, controller, depository, mango, payer);
        const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
        const remainingRedeemableAmount = await getBalance(userRedeemableATA);;
        await redeemFromMangoDepositoryTest(remainingRedeemableAmount, slippage, user, controller, depository, mango, payer);
        // await printUserInfo(user.publicKey, controller, depository);
        // await printDepositoryInfo(controller, depository, mango);
    });

    it(`Mint minimal amount possible ${controller.redeemableMintSymbol} (${slippage / slippageBase * 100}% slippage)`, async function () {
        const minTradingSizeQuote = await depository.getMinTradingSizeQuoteUI(mango);
        const minTradingSize = await depository.getMinTradingSizeCollateralUI(mango);
        console.log("[🧾 amount", minTradingSize, depository.collateralMintSymbol, "]");
        console.log("[🧾 $ value", minTradingSizeQuote, "]");
        await mintWithMangoDepositoryTest(minTradingSize, slippage, user, controller, depository, mango, payer);
    });

    // Fees are taken from the input on the redeem (provide UXD amount, gets UXD amount minus fees converted back to collateral). 
    // So we need to factor in the fees
    it(`Mint twice min mint trading size, then redeem them (10% slippage)`, async function () {
        const minRedeemAmount = await depository.getMinRedeemSizeQuoteUI(mango);
        const minTradingSize = await depository.getMinTradingSizeCollateralUI(mango);

        await mintWithMangoDepositoryTest(minTradingSize * 2, 100, user, controller, depository, mango, payer);
        console.log("[🧾 $ value", minRedeemAmount, controller.redeemableMintSymbol, "]");
        await redeemFromMangoDepositoryTest(minRedeemAmount, 100, user, controller, depository, mango, payer);
    });

    it(`Return remaining ${depository.collateralMintSymbol} user's balance to the payer`, async function () {
        // SOL will remain on the account when using WSOL
        await transferAllTokens(depository.collateralMint, depository.collateralMintDecimals, user, payer.publicKey);
    });

};