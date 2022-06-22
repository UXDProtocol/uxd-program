import { NATIVE_MINT } from "@solana/spl-token";
import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, PnLPolarity } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { quoteMintWithMangoDepositoryAccountingTest } from "../cases/quoteMintWithMangoDepositoryAccountingTest";
import { quoteMintWithMangoDepositoryTest } from "../cases/quoteMintWithMangoDepositoryTest";
import { quoteRedeemFromMangoDepositoryAccountingTest } from "../cases/quoteRedeemFromMangoDepositoryAccountingTest";
import { quoteRedeemFromMangoDepositoryTest } from "../cases/quoteRedeemFromMangoDepositoryTest";
import { setMangoDepositoryQuoteMintAndRedeemFeeTest } from "../cases/setMangoDepositoryQuoteMintAndRedeemFeeTest";
import { setMangoDepositoryQuoteMintAndRedeemSoftCapTest } from "../cases/setMangoDepositoryQuoteMintAndRedeemSoftCapTest";
import { TXN_OPTS } from "../connection";
import { slippageBase } from "../constants";
import { mango } from "../fixtures";
import { transferSol, transferTokens } from "../utils";


export const quoteMintAndRedeemSuite = function (authority: Signer, user: Signer, payer: Signer, controller: Controller, depository: MangoDepository) {

    before(`Transfer 5,000${depository.quoteMintSymbol} from payer to user`, async function () {
        await transferTokens(5000, depository.quoteMint, depository.quoteMintDecimals, payer, user.publicKey);
    });

    before(`Transfer 5,000 USD worth of ${depository.collateralMintSymbol} from payer to user`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 5_000 / perpPrice;
        console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
        // For Wsol we send sol, the API handle the wrapping before each minting
        if (depository.collateralMint.equals(NATIVE_MINT)) {
            await transferSol(amount, payer, user.publicKey);
        } else {
            await transferTokens(amount, depository.collateralMint, depository.collateralMintDecimals, payer, user.publicKey);
        }
    });

    before(`Mint 3000 ${controller.redeemableMintSymbol} (${20 / slippageBase * 100} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 3000 / perpPrice;
        console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
        await mintWithMangoDepositoryTest(amount, 20, user, controller, depository, mango, payer);
    });

    it(`Change the quote mint and redeem soft cap to 1_000_000`, async function () {
        await setMangoDepositoryQuoteMintAndRedeemSoftCapTest(1_000_000, authority, controller, depository);
    });

    it(`Change the quote mint and redeem fees to 0`, async function () {
        await setMangoDepositoryQuoteMintAndRedeemFeeTest(0, authority, controller, depository);
    });

    it(`Quote mint or redeem 10$ (without fees)`, async function () {

        const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
        const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

        if (Math.abs(unrealizedPnl) < 10) {
            console.log("🔵  skipping mint/redeem, unrealized pnl too small");
            return;
        }
        switch (polarity) {
            case `Positive`: {
                await quoteRedeemFromMangoDepositoryTest(10, user, controller, depository, mango, payer);
                break;
            }
            case `Negative`: {
                await quoteMintWithMangoDepositoryTest(10, user, controller, depository, mango, payer);
                break;
            }
        }
    });

    it(`Accounting test for quote mint or redeem 10$ (without fees)`, async function () {

        const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
        const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

        if (Math.abs(unrealizedPnl) < 10) {
            console.log("🔵  skipping quote mint/redeem, unrealized pnl too small");
            return;
        }
        switch (polarity) {
            case `Positive`: {
                await quoteRedeemFromMangoDepositoryAccountingTest(10, user, controller, depository, mango, payer);
                break;
            }
            case `Negative`: {
                await quoteMintWithMangoDepositoryAccountingTest(10, user, controller, depository, mango, payer);
                break;
            }
        }
    });

    it(`Change the quote mint and redeem fees to 5 bps`, async function () {
        await setMangoDepositoryQuoteMintAndRedeemFeeTest(5, authority, controller, depository);
    });

    it(`Quote mint or redeem 10$ (with fees)`, async function () {

        const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
        const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

        if (Math.abs(unrealizedPnl) < 10) {
            console.log("🔵  skipping mint/redeem, unrealized pnl too small");
            return;
        }
        switch (polarity) {
            case `Positive`: {
                await quoteRedeemFromMangoDepositoryTest(10, user, controller, depository, mango, payer);
                break;
            }
            case `Negative`: {
                await quoteMintWithMangoDepositoryTest(10, user, controller, depository, mango, payer);
                break;
            }
        }
    });

    it(`Accounting test for quote mint or redeem 10$ (with fees)`, async function () {

        const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
        const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

        if (Math.abs(unrealizedPnl) < 10) {
            console.log("🔵  skipping quote mint/redeem, unrealized pnl too small");
            return;
        }
        switch (polarity) {
            case `Positive`: {
                await quoteRedeemFromMangoDepositoryAccountingTest(10, user, controller, depository, mango, payer);
                break;
            }
            case `Negative`: {
                await quoteMintWithMangoDepositoryAccountingTest(10, user, controller, depository, mango, payer);
                break;
            }
        }
    });

    it(`Quote mint or redeem with the wrong polarity (should fail)`, async function () {

        const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
        const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

        if (Math.abs(unrealizedPnl) < 10) {
            console.log("🔵  skipping mint/redeem, unrealized pnl too small");
            return;
        }
        try {
            switch (polarity) {
                case `Negative`: {
                    await quoteRedeemFromMangoDepositoryTest(10, user, controller, depository, mango, payer);
                    break;
                }
                case `Positive`: {
                    await quoteMintWithMangoDepositoryTest(10, user, controller, depository, mango, payer);
                    break;
                }
            }
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Did the wrong instruction given polarity");
    });

    it(`Quote mint or redeem more than is available to mint (should fail)`, async function () {

        const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
        const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

        const amountToMintOrRedeem = Math.abs(unrealizedPnl) * 1.5

        try {
            switch (polarity) {
                case `Positive`: {
                    await quoteRedeemFromMangoDepositoryTest(amountToMintOrRedeem, user, controller, depository, mango, payer);
                    break;
                }
                case `Negative`: {
                    await quoteMintWithMangoDepositoryTest(amountToMintOrRedeem, user, controller, depository, mango, payer);
                    break;
                }
            }
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Minting or redeeming more than available");
    });

    it(`Quote mint or redeem 0 (should fail)`, async function () {

        const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
        const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

        try {
            switch (polarity) {
                case `Positive`: {
                    await quoteRedeemFromMangoDepositoryTest(0, user, controller, depository, mango, payer);
                    break;
                }
                case `Negative`: {
                    await quoteMintWithMangoDepositoryTest(0, user, controller, depository, mango, payer);
                    break;
                }
            }
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - Tried minting or redeeming 0");
    });

    it(`Change the quote mint and redeem soft cap to 5_000`, async function () {
        await setMangoDepositoryQuoteMintAndRedeemSoftCapTest(5_000, authority, controller, depository);
    });

    it(`Quote mint or redeem 1000$ (with fees)`, async function () {

        const unrealizedPnl = await depository.getUnrealizedPnl(mango, TXN_OPTS);
        const polarity = unrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

        if (Math.abs(unrealizedPnl) < 1000) {
            console.log("🔵  skipping mint/redeem, unrealized pnl too small");
            return;
        }
        switch (polarity) {
            case `Positive`: {
                await quoteRedeemFromMangoDepositoryTest(1000, user, controller, depository, mango, payer);
                break;
            }
            case `Negative`: {
                await quoteMintWithMangoDepositoryTest(1000, user, controller, depository, mango, payer);
                break;
            }
        }
    });

    it(`Quote redeem 10_000$ (should fail)`, async function () {
        try {
            await quoteRedeemFromMangoDepositoryTest(10_000, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - No collateral deposited yet");
    });

    it(`Quote mint 10_000$ (should fail)`, async function () {
        try {
            await quoteMintWithMangoDepositoryTest(10_000, user, controller, depository, mango, payer);
        } catch {
            expect(true, "Failing as planned");
        }
        expect(false, "Should have failed - No collateral deposited yet");
    });

}
