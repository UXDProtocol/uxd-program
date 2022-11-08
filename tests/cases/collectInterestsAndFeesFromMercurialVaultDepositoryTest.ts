import { Signer } from "@solana/web3.js";
import { findATAAddrSync } from "@uxd-protocol/uxd-client";
import { Controller, MercurialVaultDepository, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { collectInterestsAndFeesFromMercurialVaultDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";
import { getBalance } from "../utils";

export const collectInterestsAndFeesFromMercurialVaultDepositoryTest = async function (
    interestsAndFeesRedeemAuthority: Signer,
    controller: Controller,
    depository: MercurialVaultDepository,
    payer?: Signer,
): Promise<number> {
    console.group("🧭 collectInterestsAndFeesFromMercurialVaultDepositoryTest");

    try {
        // GIVEN
        const [
            interestsAndFeesRedeemAuthorityCollateralATA,
        ] = findATAAddrSync(interestsAndFeesRedeemAuthority.publicKey, depository.collateralMint.mint);

        const [
            interestsAndFeesRedeemAuthorityCollateralBalance_pre,
            onChainDepository_pre,
            onChainController_pre,
        ] = await Promise.all([
            getBalance(interestsAndFeesRedeemAuthorityCollateralATA),
            depository.getOnchainAccount(getConnection(), TXN_OPTS),
            controller.getOnchainAccount(getConnection(), TXN_OPTS),
        ]);

        const estimatedInterestsAndFeesValue = await depository.calculateInterestsAndFeesValue(getConnection());

        const uiEstimatedInterestsAndFeesValue = nativeToUi(estimatedInterestsAndFeesValue.toNumber(), depository.collateralMint.decimals);

        // WHEN
        // Simulates user experience from the front end
        const txId = await collectInterestsAndFeesFromMercurialVaultDepository(interestsAndFeesRedeemAuthority, payer ?? interestsAndFeesRedeemAuthority, controller, depository);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const [
            interestsAndFeesRedeemAuthorityCollateralBalance_post,
            onChainDepository_post,
            onChainController_post,
        ] = await Promise.all([
            getBalance(interestsAndFeesRedeemAuthorityCollateralATA),
            depository.getOnchainAccount(getConnection(), TXN_OPTS),
            controller.getOnchainAccount(getConnection(), TXN_OPTS),
        ]);

        // Use toFixed to avoid +0.010000000000000009 != than +0.01
        const collateralDelta = Number((interestsAndFeesRedeemAuthorityCollateralBalance_pre - interestsAndFeesRedeemAuthorityCollateralBalance_post).toFixed(depository.collateralMint.decimals));

        console.log(
            `🧾 Collected`, collateralDelta, depository.collateralMint.symbol,
        );

        // Check used collateral
        expect(collateralDelta).equal(uiEstimatedInterestsAndFeesValue, "The amount of collected collateral should be close to the estimated amount");

        // Check depository accounting
        expect(nativeToUi(onChainDepository_post.interestsAndFeesTotalCollected, depository.collateralMint.decimals))
            .equal(Number((nativeToUi(onChainDepository_pre.interestsAndFeesTotalCollected, depository.collateralMint.decimals) + collateralDelta).toFixed(depository.collateralMint.decimals)));

        // Check controller accounting
        expect(nativeToUi(onChainController_post.interestsAndFeesTotalCollected, depository.collateralMint.decimals))
            .equal(Number((nativeToUi(onChainController_post.interestsAndFeesTotalCollected, depository.collateralMint.decimals) + collateralDelta).toFixed(depository.collateralMint.decimals)));


        console.groupEnd();

        return collateralDelta;
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}