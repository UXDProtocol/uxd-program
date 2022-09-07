import { Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository, findATAAddrSync } from "@uxd-protocol/uxd-client";
import { mintWithMercurialVaultDepository } from "../api";
import { CLUSTER } from "../constants";
import { getBalance } from "../utils";

export const mintWithMercurialVaultDepositoryTest = async function (
    collateralAmount: number,
    user: Signer,
    controller: Controller,
    depository: MercurialVaultDepository,
    payer?: Signer,
): Promise<void> {
    console.group("🧭 mintWithMercurialVaultDepositoryTest");

    try {
        // GIVEN
        const [userCollateralATA] = findATAAddrSync(user.publicKey, depository.collateralMint.mint);
        const [userRedeemableATA] = findATAAddrSync(user.publicKey, controller.redeemableMintPda);
        const userRedeemableBalance_pre = await getBalance(userRedeemableATA);
        const userCollateralBalance_pre: number = await getBalance(userCollateralATA);

        // WHEN
        // Simulates user experience from the front end
        const txId = await mintWithMercurialVaultDepository(user, payer ?? user, controller, depository, collateralAmount);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const userRedeemableBalance_post = await getBalance(userRedeemableATA);
        const userCollateralBalance_post = await getBalance(userCollateralATA);

        const collateralPaid = userCollateralBalance_pre - userCollateralBalance_post;
        const minted = userRedeemableBalance_post - userRedeemableBalance_pre;

        console.log(
            `🧾 Minted`, Number(minted.toFixed(depository.mercurialVaultLpMint.decimals)), controller.redeemableMintSymbol,
            "by locking", Number(collateralPaid.toFixed(depository.collateralMint.decimals)), depository.collateralMint.symbol,
        );

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}