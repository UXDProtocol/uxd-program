import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxdprotocol/uxd-client";
import { mintWithMangoDepositoryAccountingTest } from "../cases/mintWithMangoDepositoryAccountingTest";
import { redeemFromMangoDepositoryAccountingTest } from "../cases/redeemFromMangoDepositoryAccountingTest";
import { mango } from "../fixtures";
import { MangoDepositoryAndControllerInteractionsSuiteParameters } from "./mangoDepositoryAndControllerInteractionsSuite";

export const mangoDepositoryAndControllerAccountingSuite = function (authority: Signer, user: Signer, payer: Signer, controller: Controller, depository: MangoDepository, params: MangoDepositoryAndControllerInteractionsSuiteParameters) {
    let mintedAmount: number;

    it(`Mint 2 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage)`, async function () {
        mintedAmount = await mintWithMangoDepositoryAccountingTest(2, params.slippage, user, controller, depository, mango, payer);
    });

    it(`Redeem ${mintedAmount} UXD (${params.slippage} slippage)`, async function () {
        await redeemFromMangoDepositoryAccountingTest(mintedAmount, params.slippage, user, controller, depository, mango, payer);
    });
}