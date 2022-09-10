import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, findATAAddrSync, MercurialVaultDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { redeemFromMercurialVaultDepositoryTest } from "../cases/redeemFromMercurialVaultDepositoryTest";
import { slippageBase, SOLEND_USDC_DEVNET, SOLEND_USDC_DEVNET_DECIMALS } from "../constants";
import { mango } from "../fixtures";
import { getBalance, printUserInfo, transferAllTokens, transferSol, transferTokens } from "../utils";

export const mercurialVaultDepositoryMintRedeemSuite = function (user: Signer, payer: Signer, controller: Controller, depository: MercurialVaultDepository) {
    before(`Transfer 1 ${depository.collateralMint.symbol} from payer to user`, async function () {
        await transferTokens(0.1, depository.collateralMint.mint, depository.collateralMint.decimals, payer, user.publicKey);
    });


    it(`Redeem ${controller.redeemableMintSymbol} when no mint has happened (should fail)`, async function () {
        /*const redeemableAmount = 
         console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
         try {
             await redeemFromMercurialVaultDepositoryTest(redeemableAmount, user, controller, depository, payer);
         } catch {
             expect(true, "Failing as planned");
         }
         expect(false, "Should have failed - No collateral deposited yet");*/
    });

    it(`Return remaining ${depository.collateralMint.symbol} user's balance to the payer`, async function () {
        await transferAllTokens(depository.collateralMint.mint, depository.collateralMint.decimals, user, payer.publicKey);
    });

};