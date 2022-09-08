import { Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository, findATAAddrSync } from "@uxd-protocol/uxd-client";
import { redeemFromMercurialVaultDepository } from "../api";
import { CLUSTER } from "../constants";
import { getBalance } from "../utils";

export const redeemFromMercurialVaultDepositoryTest = async function (
    redeemableAmount: number,
    user: Signer,
    controller: Controller,
    depository: MercurialVaultDepository,
    payer?: Signer,
): Promise<void> {
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

        const collateralReceived = userCollateralBalance_post - userCollateralBalance_pre;
        const redeemable = userRedeemableBalance_pre - userRedeemableBalance_post;

        console.log(
            `🧾 Redeemed`, Number(collateralReceived.toFixed(depository.collateralMint.decimals)), depository.collateralMint.symbol,
            "for", Number(redeemable.toFixed(depository.mercurialVaultLpMint.decimals)), controller.redeemableMintSymbol,
        );

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}