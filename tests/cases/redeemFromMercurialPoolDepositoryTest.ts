import { Signer } from "@solana/web3.js";
import { Controller, MercurialPoolDepository, findATAAddrSync } from "@uxd-protocol/uxd-client";
import { redeemFromMercurialPoolDepository } from "../api";
import { CLUSTER } from "../constants";
import { getBalance } from "../utils";

export const redeemFromMercurialPoolDepositoryTest = async function (
    redeemableAmount: number,
    user: Signer,
    controller: Controller,
    depository: MercurialPoolDepository,
    payer?: Signer,
): Promise<void> {
    console.group("🧭 redeemFromMercurialPoolDepositoryTest");

    try {
        // GIVEN
        const [userCollateralATA] = findATAAddrSync(user.publicKey, depository.collateralMint.mint);
        const [userRedeemableATA] = findATAAddrSync(user.publicKey, controller.redeemableMintPda);
        const userRedeemableBalance_pre = await getBalance(userRedeemableATA);
        const userCollateralBalance_pre: number = await getBalance(userCollateralATA);

        // WHEN
        // Simulates user experience from the front end
        const txId = await redeemFromMercurialPoolDepository(user, payer ?? user, controller, depository, redeemableAmount);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const userRedeemableBalance_post = await getBalance(userRedeemableATA);
        const userCollateralBalance_post = await getBalance(userCollateralATA);

        const collateralPaid = userCollateralBalance_pre - userCollateralBalance_post;
        const minted = userRedeemableBalance_post - userRedeemableBalance_pre;

        console.log(
            `🧾 Redeemed`, Number(minted.toFixed(depository.mercurialPoolLpMint.decimals)), controller.redeemableMintSymbol,
            "by locking", Number(collateralPaid.toFixed(depository.collateralMint.decimals)), depository.collateralMint.symbol,
        );

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}