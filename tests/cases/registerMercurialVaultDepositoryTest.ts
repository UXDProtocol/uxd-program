import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository } from "@uxd-protocol/uxd-client";
import { registerMercurialVaultDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection } from "../connection";

export const registerMercurialVaultDepositoryTest = async function (
    authority: Signer,
    profitsRedeemAuthority: PublicKey,
    controller: Controller,
    depository: MercurialVaultDepository,
    mintingFeeInBps: number,
    redeemingFeeInBps: number,
    redeemableAmountUnderManagementCap: number,
    payer?: Signer,
) {
    console.group("🧭 initializeMercurialVaultDepositoryTest");
    try {
        // WHEN
        if (await getConnection().getAccountInfo(depository.pda)) {
            console.log("🚧 Already registered.");
        } else {
            const txId = await registerMercurialVaultDepository(authority, profitsRedeemAuthority, payer ?? authority, controller, depository, mintingFeeInBps, redeemingFeeInBps, redeemableAmountUnderManagementCap);
            console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);
        }

        // THEN
        console.log(`🧾 Initialized`, depository.collateralMint.symbol, "Mercurial Vault Depository");
        depository.info();
        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}