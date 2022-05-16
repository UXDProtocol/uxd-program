import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, PnLPolarity } from "@uxd-protocol/uxd-client";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { quoteMintWithMangoDepositoryTest } from "../cases/quoteMintWithMangoDepositoryTest";
import { quoteRedeemFromMangoDepositoryTest } from "../cases/quoteRedeemFromMangoDepositoryTest";
import { setMangoDepositoryQuoteMintAndRedeemFeeTest } from "../cases/setMangoDepositoryQuoteMintAndRedeemFeeTest";
import { TXN_OPTS } from "../connection";
import { mango } from "../fixtures";


export const quoteMintAndRedeemSuite = function (authority: Signer, user: Signer, payer: Signer, controller: Controller, depository: MangoDepository) {
    it(`Ensure user has UXD`, async function () {
        await mintWithMangoDepositoryTest(0.01, 500, user, controller, depository, mango);
    });

    it(`Change the quote mint and redeem fees to 0`, async function () {
        await setMangoDepositoryQuoteMintAndRedeemFeeTest(0, authority, controller, depository);
    });

    it(`Quote mint or redeem a small amount (without fees)`, async function () {

        const offsetUnrealizedPnl = await depository.getOffsetUnrealizedPnl(mango, TXN_OPTS);
        const polarity = offsetUnrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

        if (Math.abs(offsetUnrealizedPnl) < 1) {
            console.log("🔵  skipping rebalancing, unrealized pnl too small");
            return;
        }
        switch (polarity) {
            case `Positive`: {
                console.log("Quote Redeem!");
                await quoteRedeemFromMangoDepositoryTest(1, user, controller, depository, mango, payer);
                break;
            }
            case `Negative`: {
                console.log("Quote Mint!");
                await quoteMintWithMangoDepositoryTest(1, user, controller, depository, mango, payer);
                break;
            }
        }
    });

    it(`Change the quote mint and redeem fees to 5 bps`, async function () {
        await setMangoDepositoryQuoteMintAndRedeemFeeTest(5, authority, controller, depository);
    });

    it(`Quote mint or redeem a small amount (with fees)`, async function () {
        const offsetUnrealizedPnl = await depository.getOffsetUnrealizedPnl(mango, TXN_OPTS);
        const polarity = offsetUnrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;

        if (Math.abs(offsetUnrealizedPnl) < 1) {
            console.log("🔵  skipping rebalancing, unrealized pnl too small");
            return;
        }
        switch (polarity) {
            case `Positive`: {
                console.log("Quote Redeem!");
                await quoteRedeemFromMangoDepositoryTest(1, user, controller, depository, mango, payer);
                break;
            }
            case `Negative`: {
                console.log("Quote Mint!");
                await quoteMintWithMangoDepositoryTest(1, user, controller, depository, mango, payer);
                break;
            }
        }
    });



}