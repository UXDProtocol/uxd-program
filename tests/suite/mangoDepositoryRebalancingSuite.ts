
import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, PnLPolarity } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { rebalanceMangoDepositoryLiteTest } from "../cases/rebalanceMangoDepositoryLiteTest";
import { TXN_OPTS } from "../connection";
import { slippageBase } from "../constants";
import { mango } from "../fixtures";
import { printUserInfo, printDepositoryInfo, transferTokens, transferAllTokens } from "../utils";

export class MangoDepositoryRebalancingSuiteParameters {
    public slippage: number;

    public constructor(
        slippage: number,
    ) {
        this.slippage = slippage;
    }
}

export const mangoDepositoryRebalancingSuite = function (user: Signer, payer: Signer, controller: Controller, depository: MangoDepository, params: MangoDepositoryRebalancingSuiteParameters) {
    let rebalanceAmountSmall: number;
    let polarity: PnLPolarity;

    it("Calculate current unrealized PnL and derive PnLPolarity + funds user for it (in QUOTE or COLLATERAL)", async function () {
        const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);

        rebalanceAmountSmall = unrealizedPnl;//perpPrice * 2;
        polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;
        console.log("🔵 unrealizedPnl on ", depository.collateralMintSymbol, "depository:", unrealizedPnl, "| Polarity:", polarity);

        switch (polarity) {
            case `Positive`: {
                // Transfer COLLATERAL, will receive equivalent QUOTE back from the positive PNL
                const collateralAmount = unrealizedPnl / perpPrice
                await transferTokens(collateralAmount, depository.collateralMint, depository.collateralMintDecimals, payer, user.publicKey);
                console.log("🔵 transferring", collateralAmount, depository.collateralMintSymbol, "to user pre calling the rebalance");
                break;
            }
            case `Negative`: {
                // Transfer QUOTE to repay PNL, will receive equivalent COLLATERAL back
                const quoteAmountUi = unrealizedPnl;
                await transferTokens(quoteAmountUi, depository.quoteMint, depository.quoteMintDecimals, payer, user.publicKey);
                console.log("🔵 transferring", quoteAmountUi, depository.quoteMintSymbol, "to user pre calling the rebalance");
                break;
            }
        };

    });

    // it(`Rebalance 0 ${depository.quoteMintSymbol} (should fail)`, async function () {
    //     try {
    //         await rebalanceMangoDepositoryLiteTest(0, polarity, params.slippage, user, controller, depository, mango, payer);
    //     } catch {
    //         expect(true, "Failing as planned");
    //     }
    //     expect(false, "Should have failed - Cannot rebalance 0");
    // });

    it(`Rebalance ${rebalanceAmountSmall} ${depository.quoteMintSymbol} (${params.slippage / slippageBase} slippage and ${polarity} polarity)`, async function () {
        const rebalancedAmount = await rebalanceMangoDepositoryLiteTest(rebalanceAmountSmall, polarity, params.slippage, user, controller, depository, mango, payer);
        await printUserInfo(user.publicKey, controller, depository);
        await printDepositoryInfo(controller, depository, mango);
        return rebalancedAmount;
    });

    it("Return remaining balances from user back to the payer", async function () {
        await transferAllTokens(depository.quoteMint, depository.quoteMintDecimals, user, payer.publicKey);
        await transferAllTokens(depository.collateralMint, depository.collateralMintDecimals, user, payer.publicKey);
    });
};