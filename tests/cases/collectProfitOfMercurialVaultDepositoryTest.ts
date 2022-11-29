import { Signer } from "@solana/web3.js";
import { findATAAddrSync, Controller, MercurialVaultDepository, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { collectProfitOfMercurialVaultDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";
import { getBalance } from "../utils";

export const collectProfitOfMercurialVaultDepositoryTest = async function ({
    authority,
    controller,
    depository,
    payer,
}: {
    authority: Signer;
    controller: Controller;
    depository: MercurialVaultDepository;
    payer?: Signer;
}): Promise<number> {
    console.group("🧭 collectProfitOfMercurialVaultDepositoryTest");

    try {
        // GIVEN
        const [
            profitsRedeemAuthorityCollateralATA,
        ] = findATAAddrSync(authority.publicKey, depository.collateralMint.mint);

        const [
            profitsRedeemAuthorityCollateralBalance_pre,
            onChainDepository_pre,
            onChainController_pre,
        ] = await Promise.all([
            getBalance(profitsRedeemAuthorityCollateralATA),
            depository.getOnchainAccount(getConnection(), TXN_OPTS),
            controller.getOnchainAccount(getConnection(), TXN_OPTS),
        ]);

        const estimatedProfitsValue = await depository.calculateProfitsValue(getConnection());

        const uiEstimatedProfitsValue = nativeToUi(estimatedProfitsValue.toNumber(), depository.collateralMint.decimals);

        // WHEN
        // Simulates user experience from the front end
        const txId = await collectProfitOfMercurialVaultDepository({
            authority,
            payer: payer ?? authority,
            controller,
            depository,
        });
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const [
            profitsRedeemAuthorityCollateralBalance_post,
            onChainDepository_post,
            onChainController_post,
        ] = await Promise.all([
            getBalance(profitsRedeemAuthorityCollateralATA),
            depository.getOnchainAccount(getConnection(), TXN_OPTS),
            controller.getOnchainAccount(getConnection(), TXN_OPTS),
        ]);

        // Use toFixed to avoid +0.010000000000000009 != than +0.01
        const collateralDelta = Number((profitsRedeemAuthorityCollateralBalance_post - profitsRedeemAuthorityCollateralBalance_pre).toFixed(depository.collateralMint.decimals));

        console.log(
            `🧾 Collected`, collateralDelta, depository.collateralMint.symbol,
        );

        // Check used collateral
        expect(collateralDelta).equal(uiEstimatedProfitsValue, "The amount of collected collateral should be close to the estimated amount");

        // Check depository accounting
        expect(nativeToUi(onChainDepository_post.profitsTotalCollected, depository.collateralMint.decimals))
            .equal(Number((nativeToUi(onChainDepository_pre.profitsTotalCollected, depository.collateralMint.decimals) + collateralDelta).toFixed(depository.collateralMint.decimals)));

        // Check controller accounting
        expect(nativeToUi(onChainController_post.profitsTotalCollected, depository.collateralMint.decimals))
            .equal(Number((nativeToUi(onChainController_pre.profitsTotalCollected, depository.collateralMint.decimals) + collateralDelta).toFixed(depository.collateralMint.decimals)));

        console.groupEnd();

        return collateralDelta;
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}