import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxd-protocol/uxd-client";
import { mintWithMangoDepositoryAccountingTest } from "../cases/mintWithMangoDepositoryAccountingTest";
import { redeemFromMangoDepositoryAccountingTest } from "../cases/redeemFromMangoDepositoryAccountingTest";
import { mango } from "../fixtures";


export const mangoDepositoryAndControllerAccountingSuite = function (user: Signer, payer: Signer, controller: Controller, depository: MangoDepository, slippage: number, collateralUnitShift: number) {
    it(`Mint 2 ${depository.collateralMintSymbol} worth of UXD (${slippage} slippage) and redeem`, async function () {
        const mintedAmount = await mintWithMangoDepositoryAccountingTest(2, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryAccountingTest(mintedAmount, slippage, collateralUnitShift, user, controller, depository, mango, payer);
    });
}