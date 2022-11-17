import { Signer } from "@solana/web3.js";
import { IdentityDepository } from "@uxd-protocol/uxd-client";
import { Controller, MangoDepository, findATAAddrSync, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { reinjectMangoToIdentityDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";
import { getBalance } from "../utils";

export const reinjectMangoToIdentityDepositoryTest = async function (
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: IdentityDepository,
  mangoDepository: MangoDepository
): Promise<void> {
  console.group("🧭 reinjectMangoToIdentityDepository");

  try {
    // GIVEN
    const [userCollateralATA] = findATAAddrSync(user.publicKey, depository.collateralMint);

    const [userCollateralBalance_pre, collateralVaultBalance_pre, onChainDepository_pre, onChainMangoDepository] =
      await Promise.all([
        getBalance(userCollateralATA),
        getBalance(depository.collateralVaultPda),
        depository.getOnchainAccount(getConnection(), TXN_OPTS),
        mangoDepository.getOnchainAccount(getConnection(), TXN_OPTS),
      ]);

    // WHEN
    const txId = await reinjectMangoToIdentityDepository(user, payer, controller, depository, mangoDepository);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const [userCollateralBalance_post, collateralVaultBalance_post, onChainDepository_post] = await Promise.all([
      getBalance(userCollateralATA),
      getBalance(depository.collateralVaultPda),
      depository.getOnchainAccount(getConnection(), TXN_OPTS),
    ]);

    // Checked user collateral used
    const userCollateralDelta = Number(
      (userCollateralBalance_pre - userCollateralBalance_post).toFixed(depository.collateralMintDecimals)
    );

    // Checked identity depo's collateral vault increment
    const collateralVaultDelta = Number(
      (collateralVaultBalance_post - collateralVaultBalance_pre).toFixed(depository.collateralMintDecimals)
    );

    // Checked identity depo's accounting of collateral deposited
    const collateralAmountDepositedDelta = nativeToUi(
      Number(
        (onChainDepository_post.collateralAmountDeposited - onChainDepository_pre.collateralAmountDeposited).toFixed(
          depository.collateralMintDecimals
        )
      ),
      depository.collateralMintDecimals
    );

    // Checked identity depo's accounting of collateral deposited
    const redeemableAmountUnderManagementDelta = nativeToUi(
      Number(
        (
          onChainDepository_post.redeemableAmountUnderManagement - onChainDepository_pre.redeemableAmountUnderManagement
        ).toFixed(depository.collateralMintDecimals)
      ),
      depository.collateralMintDecimals
    );

    // Expected reinjected amount should equal redeemable under this mango depo
    const expectedReinjectedAmount = Number(
      nativeToUi(onChainMangoDepository.redeemableAmountUnderManagement, controller.redeemableMintDecimals)
    );

    console.log(
      `🧾 Reinjected`,
      userCollateralDelta.toLocaleString(),
      depository.collateralMintSymbol,
      "to identity depository"
    );

    expect(userCollateralDelta).equals(expectedReinjectedAmount);
    expect(collateralVaultDelta).equals(expectedReinjectedAmount);
    expect(collateralAmountDepositedDelta).equals(expectedReinjectedAmount);
    expect(redeemableAmountUnderManagementDelta).equals(expectedReinjectedAmount);

    console.groupEnd();
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
