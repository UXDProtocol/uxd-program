import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync, nativeToUi } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { mintWithMangoDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER, slippageBase } from "../constants";
import { getSolBalance, getBalance } from "../utils";
import { depositInsuranceMangoDepositoryTest } from "./depositInsuranceMangoDepositoryTest";

export const mintWithMangoDepositoryAccountingTest = async function (collateralAmount: number, slippage: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer): Promise<number> {
    console.group("🧭 mintWithMangoDepositoryTest");
    try {
        const connection = getConnection();
        const options = TXN_OPTS;

        // GIVEN
        const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
        const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
        const userRedeemableBalance = await getBalance(userRedeemableATA);
        const userCollateralBalance: number = await getBalance(userCollateralATA);

        const depositoryAccount = await depository.getOnchainAccount(connection, options);
        const depositoryRedeemable = nativeToUi(depositoryAccount.redeemableAmountUnderManagement.toNumber(), depository.quoteMintDecimals);
        const depositoryCollateral = nativeToUi(depositoryAccount.collateralAmountDeposited.toNumber(), depository.collateralMintDecimals);
        const depositoryTakerFees = nativeToUi(depositoryAccount.totalAmountPaidTakerFee.toNumber(), depository.quoteMintDecimals);
        const controllerAccount = await controller.getOnchainAccount(connection, options);
        const controllerRedeemable = nativeToUi(controllerAccount.redeemableCirculatingSupply.toNumber(), controller.redeemableMintDecimals);

        // Initial SOL is used to make the diff afterward as the instruction does unwrap
        const userStartingSolBalance = await getSolBalance(user.publicKey);

        // WHEN
        // - Get the perp price at the same moment to have the less diff between exec and test price
        // Simulates user experience from the front end
        const mangoPerpPrice = await depository.getCollateralPerpPriceUI(mango);
        const txId = await mintWithMangoDepository(user, payer ?? user, slippage, collateralAmount, controller, depository, mango);
        console.log("🪙  perp price is", Number(mangoPerpPrice.toFixed(depository.quoteMintDecimals)), depository.quoteMintSymbol);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const userRedeemableBalance_post = await getBalance(userRedeemableATA);
        let userCollateralBalance_post: number;
        if (NATIVE_MINT.equals(depository.collateralMint)) {
            // the TX unwrap the WSOL, so we need to remove the initial SOL balance. Payer takes the tx fees so we'r good.
            userCollateralBalance_post = await getSolBalance(user.publicKey) - userStartingSolBalance;
        } else {
            userCollateralBalance_post = await getBalance(userCollateralATA);
        }

        const mangoTakerFee = depository.getCollateralPerpTakerFees(mango);

        const redeemableDelta = userRedeemableBalance_post - userRedeemableBalance;
        const collateralDelta = userCollateralBalance - userCollateralBalance_post;
        const collateralOddLotLeftOver = Math.max(collateralAmount - collateralDelta, 0);
        const collateralProcessedByMinting = collateralAmount - collateralOddLotLeftOver;
        // The mango perp price in these might not be the exact same as the one in the transaction.
        const estimatedFrictionlessRedeemableDelta = collateralProcessedByMinting * mangoPerpPrice;
        const estimatedAmountRedeemableLostInTakerFees = mangoTakerFee * collateralProcessedByMinting * mangoPerpPrice;
        const collateralNativeUnitPrecision = Math.pow(10, -depository.collateralMintDecimals);
        const estimatedAmountRedeemableLostInSlippage = Math.abs(estimatedFrictionlessRedeemableDelta - redeemableDelta) - estimatedAmountRedeemableLostInTakerFees;
        // Get onchain depository and controller for post accounting
        const depositoryAccount_post = await depository.getOnchainAccount(connection, TXN_OPTS);
        const depositoryRedeemable_post = nativeToUi(depositoryAccount_post.redeemableAmountUnderManagement.toNumber(), depository.quoteMintDecimals);
        const depositoryCollateral_post = nativeToUi(depositoryAccount_post.collateralAmountDeposited.toNumber(), depository.collateralMintDecimals);
        const controllerAccount_post = await controller.getOnchainAccount(connection, TXN_OPTS);
        const controllerRedeemable_post = nativeToUi(controllerAccount_post.redeemableCirculatingSupply.toNumber(), controller.redeemableMintDecimals);
        const redeemableNativeUnitPrecision = Math.pow(10, -controller.redeemableMintDecimals);
        const depositoryTakerFees_post = nativeToUi(depositoryAccount_post.totalAmountPaidTakerFee.toNumber(), depository.quoteMintDecimals);


        console.log("Efficiency", Number(((redeemableDelta / estimatedFrictionlessRedeemableDelta) * 100).toFixed(2)), "%");
        console.log(
            `🧾 Minted`, Number(redeemableDelta.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol,
            "by locking", Number(collateralProcessedByMinting.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
            "(+~ takerFees =", Number(estimatedAmountRedeemableLostInTakerFees.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol,
            ", +~ slippage =", Number(estimatedAmountRedeemableLostInSlippage.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol, ")",
            "(frictionless minting would have been", Number(estimatedFrictionlessRedeemableDelta.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol, ")",
            "|| odd lot returns ", Number(collateralOddLotLeftOver.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
            ")"
        );
        // Accounting
        expect(depositoryRedeemable_post).closeTo(depositoryRedeemable + redeemableDelta, redeemableNativeUnitPrecision, "Depository RedeemableAmountUnderManagement is incorrect");
        expect(controllerRedeemable_post).closeTo(controllerRedeemable + redeemableDelta, redeemableNativeUnitPrecision, "Controller RedeemableCirculatingSupply is incorrect");
        expect(depositoryTakerFees_post).to.be.within(depositoryTakerFees, depositoryTakerFees + (mangoTakerFee * collateralDelta * mangoPerpPrice), "Depository TotalAmountPaidTakerFee is incorrect");
        expect(depositoryCollateral_post).closeTo(depositoryCollateral + collateralProcessedByMinting, collateralNativeUnitPrecision, "Depository CollateralAmountDeposited is incorrect");

        console.groupEnd();
        return redeemableDelta;
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}