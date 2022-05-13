import { Connection, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, Mango, PnLPolarity } from "@uxd-protocol/uxd-client";
import { quoteMintWithMangoDepository, quoteRedeemFromMangoDepository, setMangoDepositoryQuoteMintAndRedeemFee } from "../api";
import { quoteMintWithMangoDepositoryTest } from "../cases/quoteMintWithMangoDepositoryTest";
import { quoteRedeemFromMangoDepositoryTest } from "../cases/quoteRedeemFromMangoDepositoryTest";
import { setMangoDepositoryQuoteMintAndRedeemFeeTest } from "../cases/setMangoDepositoryQuoteMintAndRedeemFeeTest";
import { getConnection, TXN_OPTS } from "../connection";


export const quoteMintAndRedeemSuite = function (authority: Signer, user: Signer, payer: Signer, controller: Controller, depository: MangoDepository, mango: Mango) {
    // it("Quote mint 0 (should fail)", async function () {

    // });

    it(`Change the quote mint and redeem fees to 0`, async function () {
        await setMangoDepositoryQuoteMintAndRedeemFeeTest(0, authority, controller, depository);
    });


    it(`Quote mint or redeem a small amount (without fees)`, async function () {
        const offsetUnrealizedPnl = await depository.getOffsetUnrealizedPnl(mango, TXN_OPTS);
        const polarity = offsetUnrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;
        
        if (Math.abs(offsetUnrealizedPnl) < 10) { // add something here
            console.log("🔵  skipping rebalancing, unrealized pnl too small");
            return;
        }
        switch (polarity) {
            case `Positive`: {
                console.log("Quote Redeem!");
                await quoteRedeemFromMangoDepositoryTest(10, user, controller, depository, mango, payer);
            }
            case `Negative` : {
                console.log("Quote Mint!");
                await quoteMintWithMangoDepositoryTest(10, user, controller, depository, mango, payer);
            }
        }
    });

    it(`Change the quote mint and redeem fees to 5 bps`, async function () {
        await setMangoDepositoryQuoteMintAndRedeemFeeTest(5, authority, controller, depository);
    });

    it(`Quote mint or redeem a small amount (with fees)`, async function () {
        const offsetUnrealizedPnl = await depository.getOffsetUnrealizedPnl(mango, TXN_OPTS);
        const polarity = offsetUnrealizedPnl > 0 ? PnLPolarity.Positive : PnLPolarity.Negative;
        
        if (Math.abs(offsetUnrealizedPnl) < 10) { // add something here
            console.log("🔵  skipping rebalancing, unrealized pnl too small");
            return;
        }
        switch (polarity) {
            case `Positive`: {
                console.log("Quote Redeem!");
                await quoteRedeemFromMangoDepositoryTest(10, user, controller, depository, mango, payer);
            }
            case `Negative` : {
                console.log("Quote Mint!");
                await quoteMintWithMangoDepositoryTest(10, user, controller, depository, mango, payer);
            }
        }
    });



}