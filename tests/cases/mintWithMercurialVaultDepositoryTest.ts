import { Signer } from "@solana/web3.js";
import { uiToNative } from "@uxd-protocol/uxd-client";
import { Controller, MercurialVaultDepository, findMultipleATAAddSync, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { mintWithMercurialVaultDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";
import { getBalance } from "../utils";

export const mintWithMercurialVaultDepositoryTest = async function ({
    collateralAmount,
    user,
    controller,
    depository,
    payer,
}: {
    collateralAmount: number;
    user: Signer;
    controller: Controller;
    depository: MercurialVaultDepository;
    payer?: Signer;
}): Promise<number> {
    console.group("🧭 mintWithMercurialVaultDepositoryTest");

    try {
        // GIVEN
        const [
            [userCollateralATA],
            [userRedeemableATA],
        ] = findMultipleATAAddSync(user.publicKey, [
            depository.collateralMint.mint,
            controller.redeemableMintPda,
        ]);

        const [
            userRedeemableBalance_pre,
            userCollateralBalance_pre,
            onchainController_pre,
            onChainDepository_pre,
        ] = await Promise.all([
            getBalance(userRedeemableATA),
            getBalance(userCollateralATA),
            controller.getOnchainAccount(getConnection(), TXN_OPTS),
            depository.getOnchainAccount(getConnection(), TXN_OPTS),
        ]);

        // WHEN
        // Simulates user experience from the front end
        const txId = await mintWithMercurialVaultDepository(user, payer ?? user, controller, depository, collateralAmount);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const [
            userRedeemableBalance_post,
            userCollateralBalance_post,
            onchainController_post,
            onChainDepository_post,
        ] = await Promise.all([
            getBalance(userRedeemableATA),
            getBalance(userCollateralATA),
            controller.getOnchainAccount(getConnection(), TXN_OPTS),
            depository.getOnchainAccount(getConnection(), TXN_OPTS),
        ]);

        // Use toFixed to avoid +0.010000000000000009 != than +0.01
        const collateralDelta = Number((userCollateralBalance_pre - userCollateralBalance_post).toFixed(depository.collateralMint.decimals));
        const redeemableDelta = Number((userRedeemableBalance_post - userRedeemableBalance_pre).toFixed(controller.redeemableMintDecimals));

        const collateralNativeUnitPrecision = Math.pow(10, -depository.collateralMint.decimals);
        const nativeCollateralAmount = uiToNative(collateralAmount, depository.collateralMint.decimals);

        // Do calculation in native units so we avoid javascript calculation issue with small numbers
        // i.e Javascript calculation precision 0.000005000000000000013
        const estimatedFeesPaid = nativeToUi(Math.floor(Math.ceil(nativeCollateralAmount.toNumber() - ((10_000 - onChainDepository_pre.mintingFeeInBps) * nativeCollateralAmount.toNumber() / 10_000))), depository.collateralMint.decimals);

        console.log(
            `🧾 Minted`, Number(redeemableDelta.toFixed(depository.mercurialVaultLpMint.decimals)), controller.redeemableMintSymbol,
            "by locking", Number(collateralDelta.toFixed(depository.collateralMint.decimals)), depository.collateralMint.symbol,
            "with", estimatedFeesPaid, "fees paid."
        );

        const estimatedRedeemableAmount = Number((collateralAmount - estimatedFeesPaid).toFixed(controller.redeemableMintDecimals));

        // Check used collateral
        expect(collateralDelta).equal(collateralAmount, "The amount of collateral used for the mint should be exactly the one specified by the user");

        // Check minted redeemable amount
        // handle precision loss
        expect(redeemableDelta)
            .lte(estimatedRedeemableAmount)
            .gte(Number((estimatedRedeemableAmount - collateralNativeUnitPrecision).toFixed(controller.redeemableMintDecimals)));

        // Check depository accounting
        expect(nativeToUi(onChainDepository_post.collateralAmountDeposited, depository.collateralMint.decimals))
            .equal(Number((nativeToUi(onChainDepository_pre.collateralAmountDeposited, depository.collateralMint.decimals) + collateralAmount).toFixed(depository.collateralMint.decimals)));

        expect(nativeToUi(onChainDepository_post.redeemableAmountUnderManagement, controller.redeemableMintDecimals))
            .equal(Number((nativeToUi(onChainDepository_pre.redeemableAmountUnderManagement, controller.redeemableMintDecimals) + redeemableDelta).toFixed(controller.redeemableMintDecimals)));

        expect(nativeToUi(onChainDepository_post.mintingFeeTotalAccrued, depository.collateralMint.decimals))
            .equal(Number((nativeToUi(onChainDepository_pre.mintingFeeTotalAccrued, depository.collateralMint.decimals) + estimatedFeesPaid).toFixed(controller.redeemableMintDecimals)));

        // Check controller accounting
        expect(nativeToUi(onchainController_post.redeemableCirculatingSupply, controller.redeemableMintDecimals))
            .equal(Number((nativeToUi(onchainController_pre.redeemableCirculatingSupply, controller.redeemableMintDecimals) + redeemableDelta).toFixed(controller.redeemableMintDecimals)));

        console.groupEnd();

        return redeemableDelta;
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}