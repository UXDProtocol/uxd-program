import { Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository, findATAAddrSync } from "@uxd-protocol/uxd-client";
import { mintWithMercurialVaultDepository } from "../api";
import { CLUSTER } from "../constants";
import { getBalance, getSolBalance } from "../utils";

export const mintWithMercurialVaultDepositoryTest = async function (
    collateralAmount: number,
    minimumLpTokenAmount: number,
    user: Signer,
    controller: Controller,
    depository: MercurialVaultDepository,
    payer?: Signer,
): Promise<void> {
    console.group("🧭 mintWithMercurialVaultDepositoryTest");

    try {
        // GIVEN
        const [userCollateralATA] = findATAAddrSync(user.publicKey, depository.collateralMint);
        const [userRedeemableATA] = findATAAddrSync(user.publicKey, controller.redeemableMintPda);
        const userRedeemableBalance_pre = await getBalance(userRedeemableATA);
        const userCollateralBalance_pre: number = await getBalance(userCollateralATA);

        console.log('Before minting information', {
            userRedeemableBalance_pre,
            userCollateralBalance_pre,
            collateralAmount,
            minimumLpTokenAmount,
        });

        // WHEN
        // Simulates user experience from the front end
        const txId = await mintWithMercurialVaultDepository(user, payer ?? user, controller, depository, collateralAmount, minimumLpTokenAmount);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const userRedeemableBalance_post = await getBalance(userRedeemableATA);
        const userCollateralBalance_post = await getBalance(userCollateralATA);

        const collateralPaid = userCollateralBalance_post - userCollateralBalance_pre;
        const minted = userRedeemableBalance_post - userRedeemableBalance_pre;

        console.log(
            `🧾 Minted`, Number(minted.toFixed(depository.mercurialVaultLpMinDecimals)), controller.redeemableMintSymbol,
            "by locking", Number(collateralPaid.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
        );

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}