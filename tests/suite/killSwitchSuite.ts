import { NATIVE_MINT } from "@solana/spl-token";
import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, SafetyVault } from "@uxd-protocol/uxd-client";
import { initializeSafetyVaultTest } from "../cases/initializeSafetyVaultTest";
import { liquidationKillSwitchTest } from "../cases/liquidationKillSwitchTest";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "../cases/redeemFromMangoDepositoryTest";
import { slippageBase } from "../constants";
import { mango } from "../fixtures";
import { transferSol, transferTokens } from "../utils";

export const killSwitchSuite = function(authority: Signer, payer: Signer, controller: Controller, depository: MangoDepository, safetyVault: SafetyVault, slippage: number) {

    before(`Transfer 5,000${depository.quoteMintSymbol} from payer to user`, async function () {
        await transferTokens(5000, depository.quoteMint, depository.quoteMintDecimals, payer, authority.publicKey);
    });

    before(`Transfer 5,000 USD worth of ${depository.collateralMintSymbol} from payer to user`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 5_000 / perpPrice;
        console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
        // For Wsol we send sol, the API handle the wrapping before each minting
        if (depository.collateralMint.equals(NATIVE_MINT)) {
            await transferSol(amount, payer, authority.publicKey);
        } else {
            await transferTokens(amount, depository.collateralMint, depository.collateralMintDecimals, payer, authority.publicKey);
        }
    });

    before(`Mint 3000 ${controller.redeemableMintSymbol} (${20 / slippageBase * 100} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 3000 / perpPrice;
        console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
        await mintWithMangoDepositoryTest(amount, slippage, authority, controller, depository, mango, payer);
    });

    it(`Initialize safety vault for ${depository.collateralMintSymbol} depository`, async function () {
        await initializeSafetyVaultTest(authority, controller, depository, safetyVault, payer);
    });

    it(`Test redeem for odd lot`, async function () {
        await redeemFromMangoDepositoryTest(8, slippage, authority, controller, depository, mango, payer);
    });

    it(`Call Liquidation Kill Switch to x of current collateral`, async function () {
        await liquidationKillSwitchTest(1000, slippage, controller, depository, safetyVault, mango, authority, payer);
    });
}
