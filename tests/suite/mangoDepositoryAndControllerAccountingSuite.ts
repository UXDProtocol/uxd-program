import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxdprotocol/uxd-client";
import { mintWithMangoDepositoryAccountingTest } from "../cases/mintWithMangoDepositoryAccountingTest";
import { redeemFromMangoDepositoryAccountingTest } from "../cases/redeemFromMangoDepositoryAccountingTest";
import { mango } from "../fixtures";
import { MangoDepositoryAndControllerInteractionsSuiteParameters } from "./mangoDepositoryAndControllerInteractionsSuite";

export const mangoDepositoryAndControllerAccountingSuite = function (authority: Signer, user: Signer, payer: Signer, controller: Controller, depository: MangoDepository, params: MangoDepositoryAndControllerInteractionsSuiteParameters) {

    it(`Mint 2 ${depository.collateralMintSymbol} worth of UXD (${params.slippage} slippage) and redeem`, async function () {
        let mintedAmount = await mintWithMangoDepositoryAccountingTest(2, params.slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryAccountingTest(mintedAmount, params.slippage, user, controller, depository, mango, payer);
    });
}