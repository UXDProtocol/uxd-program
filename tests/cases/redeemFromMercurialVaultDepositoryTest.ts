import { Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository, findATAAddrSync } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { redeemFromMercurialVaultDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";
import { getBalance, ceilAtDecimals } from "../utils";

export const redeemFromMercurialVaultDepositoryTest = async function (
    redeemableAmount: number,
    user: Signer,
    controller: Controller,
    depository: MercurialVaultDepository,
    payer?: Signer,
): Promise<number> {
    console.group("🧭 redeemFromMercurialVaultDepositoryTest");

    try {
        // GIVEN
        const [userCollateralATA] = findATAAddrSync(user.publicKey, depository.collateralMint.mint);
        const [userRedeemableATA] = findATAAddrSync(user.publicKey, controller.redeemableMintPda);
        const userRedeemableBalance_pre = await getBalance(userRedeemableATA);
        const userCollateralBalance_pre: number = await getBalance(userCollateralATA);

        // WHEN
        // Simulates user experience from the front end
        const txId = await redeemFromMercurialVaultDepository(user, payer ?? user, controller, depository, redeemableAmount);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const userRedeemableBalance_post = await getBalance(userRedeemableATA);
        const userCollateralBalance_post = await getBalance(userCollateralATA);

        const collateralDelta = Number((userCollateralBalance_post - userCollateralBalance_pre).toFixed(depository.collateralMint.decimals));
        const redeemableDelta = Number((userRedeemableBalance_pre - userRedeemableBalance_post).toFixed(controller.redeemableMintDecimals));

        const collateralNativeUnitPrecision = Math.pow(10, -depository.collateralMint.decimals);

        const onchainDepository = await depository.getOnchainAccount(getConnection(), TXN_OPTS);

        const estimatedFeesPaid = ceilAtDecimals(redeemableAmount - ((10_000 - onchainDepository.redeemingFeeInBps) * redeemableAmount / 10_000), controller.redeemableMintDecimals);

        console.log(
            `🧾 Redeemed`, Number(collateralDelta.toFixed(depository.collateralMint.decimals)), depository.collateralMint.symbol,
            "for", Number(redeemableDelta.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol,
            "with", estimatedFeesPaid, "fees paid."
        );

        const estimatedCollateralAmount = Number((redeemableAmount - estimatedFeesPaid).toFixed(depository.collateralMint.decimals));

        // Check used redeemable
        expect(redeemableDelta).equal(redeemableAmount, "The amount of redeemable used for redeem should be exactly the one specified by the user");

        // Check redeemed collateral amount
        // handle precision loss
        expect(collateralDelta)
            .lte(estimatedCollateralAmount)
            .gte(Number((estimatedCollateralAmount - collateralNativeUnitPrecision).toFixed(controller.redeemableMintDecimals)));

        console.groupEnd();

        return collateralDelta;
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}