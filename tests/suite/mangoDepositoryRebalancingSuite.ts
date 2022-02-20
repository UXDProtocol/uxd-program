
import { NATIVE_MINT } from "@solana/spl-token";
import { Account, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, PnLPolarity, WSOL_DEVNET } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { rebalanceMangoDepositoryLiteTest } from "../cases/rebalanceMangoDepositoryLiteTest";
import { TXN_OPTS } from "../connection";
import { slippageBase } from "../constants";
import { mango } from "../fixtures";
import { printUserInfo, transferTokens, transferAllTokens, transferSol, printDepositoryInfo } from "../utils";

export class MangoDepositoryRebalancingSuiteParameters {
    public slippage: number;

    public constructor(
        slippage: number,
    ) {
        this.slippage = slippage;
    }
}

export const mangoDepositoryRebalancingSuite = function (user: Signer, payer: Signer, controller: Controller, depository: MangoDepository, params: MangoDepositoryRebalancingSuiteParameters) {

    it(`Rebalance 100 ${depository.quoteMintSymbol} without funding (should fail)`, async function () {
        try {
            await rebalanceMangoDepositoryLiteTest(100, PnLPolarity.Negative, params.slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - User not funded");
    });

    it(`Rebalance 0 ${depository.quoteMintSymbol} (should fail)`, async function () {
        try {
            await rebalanceMangoDepositoryLiteTest(0, PnLPolarity.Positive, params.slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Cannot rebalance 0");
    });

    it(`Rebalance with wrong PnLPolarity (should fail)`, async function () {
        const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
        const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

        const wrongPolarity = (polarity == PnLPolarity.Positive) ? PnLPolarity.Negative : PnLPolarity.Positive;
        try {
            await rebalanceMangoDepositoryLiteTest(1000, wrongPolarity, params.slippage, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Cannot rebalance 0");
    });

    it(`Rebalance a small amount of the depository unrealized PnL (${params.slippage / slippageBase * 100}% slippage)`, async function () {
        const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const minTradingSize = await depository.getMinTradingSizeQuoteUI(mango) * 1.5; // To not fail the CI on a sudden price change
        const rebalanceAmountSmall = Math.max(Math.abs(unrealizedPnl) / 10, minTradingSize);
        const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

        console.log("🔵 unrealizedPnl on ", depository.collateralMintSymbol, "depository:", unrealizedPnl, "| Polarity:", polarity);
        if (Math.abs(unrealizedPnl) < minTradingSize) {
            console.log("🔵  skipping rebalancing, unrealized pnl too small");
            return;
        }
        switch (polarity) {
            case `Positive`: {
                // Transfer COLLATERAL, will receive equivalent QUOTE back from the positive PNL
                const collateralAmount = rebalanceAmountSmall / perpPrice;
                // For Wsol we send sol, the API handle the wrapping before each minting
                if (depository.collateralMint.equals(WSOL_DEVNET)) {
                    await transferSol(collateralAmount, payer, user.publicKey);
                } else {
                    await transferTokens(collateralAmount, depository.collateralMint, depository.collateralMintDecimals, payer, user.publicKey);
                }
                console.log("🔵 transferring", collateralAmount, depository.collateralMintSymbol, "to user pre calling the rebalance");
                break;
            }
            case `Negative`: {
                // Transfer QUOTE to repay PNL, will receive equivalent COLLATERAL back
                const quoteAmountUi = rebalanceAmountSmall;
                await transferTokens(quoteAmountUi, depository.quoteMint, depository.quoteMintDecimals, payer, user.publicKey);
                console.log("🔵 transferring", quoteAmountUi, depository.quoteMintSymbol, "to user pre calling the rebalance");
                break;
            }
        };
        const rebalancedAmount = await rebalanceMangoDepositoryLiteTest(rebalanceAmountSmall, polarity, params.slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
        return rebalancedAmount;
    });

    it(`Rebalance 500$ of the depository unrealized PnL (${params.slippage / slippageBase * 100}% slippage)`, async function () {
        const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const minTradingSize = (await depository.getMinTradingSizeQuoteUI(mango)) * 1.5; // To not fail the CI on a sudden price change
        const rebalanceAmount = 500;
        const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

        console.log("🔵 unrealizedPnl on ", depository.collateralMintSymbol, "depository:", unrealizedPnl, "| Polarity:", polarity);
        if (Math.abs(unrealizedPnl) < minTradingSize) {
            console.log("🔵  skipping rebalancing, unrealized pnl too small");
            return;
        }
        switch (polarity) {
            case `Positive`: {
                // Transfer COLLATERAL, will receive equivalent QUOTE back from the positive PNL
                const collateralAmount = rebalanceAmount / perpPrice;
                // For Wsol we send sol, the API handle the wrapping before each minting
                if (depository.collateralMint.equals(WSOL_DEVNET)) {
                    await transferSol(collateralAmount, payer, user.publicKey);
                } else {
                    await transferTokens(collateralAmount, depository.collateralMint, depository.collateralMintDecimals, payer, user.publicKey);
                }
                console.log("🔵 transferring", collateralAmount, depository.collateralMintSymbol, "to user pre calling the rebalance");
                break;
            }
            case `Negative`: {
                // Transfer QUOTE to repay PNL, will receive equivalent COLLATERAL back
                const quoteAmountUi = rebalanceAmount;
                await transferTokens(quoteAmountUi, depository.quoteMint, depository.quoteMintDecimals, payer, user.publicKey);
                console.log("🔵 transferring", quoteAmountUi, depository.quoteMintSymbol, "to user pre calling the rebalance");
                break;
            }
        };
        const rebalancedAmount = await rebalanceMangoDepositoryLiteTest(rebalanceAmount, polarity, params.slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
        return rebalancedAmount;
    });

    it("Return remaining balances from user back to the payer", async function () {
        await transferAllTokens(depository.quoteMint, depository.quoteMintDecimals, user, payer.publicKey);
        await transferAllTokens(depository.collateralMint, depository.collateralMintDecimals, user, payer.publicKey);
    });

};