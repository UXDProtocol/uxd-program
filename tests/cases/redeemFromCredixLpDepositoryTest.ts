import { Signer } from "@solana/web3.js";
import { Controller, CredixLpDepository, findMultipleATAAddSync, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { redeemFromCredixLpDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";
import { getBalance, ceilAtDecimals } from "../utils";

export const redeemFromCredixLpDepositoryTest = async function (
  redeemableAmount: number,
  user: Signer,
  controller: Controller,
  depository: CredixLpDepository,
  payer?: Signer
): Promise<number> {
  console.group("🧭 redeemFromCredixLpDepositoryTest");

  try {
    // GIVEN
    const [[userCollateralATA], [userRedeemableATA]] = findMultipleATAAddSync(user.publicKey, [
      depository.collateralMint,
      controller.redeemableMintPda,
    ]);

    const [userRedeemableBalance_pre, userCollateralBalance_pre, onchainController_pre, onChainDepository_pre] =
      await Promise.all([
        getBalance(userRedeemableATA),
        getBalance(userCollateralATA),
        controller.getOnchainAccount(getConnection(), TXN_OPTS),
        depository.getOnchainAccount(getConnection(), TXN_OPTS),
      ]);

    // WHEN
    // Simulates user experience from the front end
    const txId = await redeemFromCredixLpDepository(user, payer ?? user, controller, depository, redeemableAmount);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const [userRedeemableBalance_post, userCollateralBalance_post, onchainController_post, onChainDepository_post] =
      await Promise.all([
        getBalance(userRedeemableATA),
        getBalance(userCollateralATA),
        controller.getOnchainAccount(getConnection(), TXN_OPTS),
        depository.getOnchainAccount(getConnection(), TXN_OPTS),
      ]);

    const collateralDelta = Number(
      (userCollateralBalance_post - userCollateralBalance_pre).toFixed(depository.collateralDecimals)
    );
    const redeemableDelta = Number(
      (userRedeemableBalance_pre - userRedeemableBalance_post).toFixed(controller.redeemableMintDecimals)
    );

    const collateralNativeUnitPrecision = Math.pow(10, -depository.collateralDecimals);
    const estimatedRedeemingFeesPaid = ceilAtDecimals(
      redeemableAmount - ((10_000 - onChainDepository_pre.redeemingFeeInBps) * redeemableAmount) / 10_000,
      controller.redeemableMintDecimals
    );
    const estimatedWithdrawingFeesPaid = ceilAtDecimals(
      redeemableAmount - ((10_000 - 50) * redeemableAmount) / 10_000,
      controller.redeemableMintDecimals
    );

    console.log(
      `🧾 Redeemed`,
      Number(collateralDelta.toFixed(depository.collateralDecimals)),
      depository.collateralSymbol,
      "for",
      Number(redeemableDelta.toFixed(controller.redeemableMintDecimals)),
      controller.redeemableMintSymbol,
      "with",
      estimatedRedeemingFeesPaid,
      "redeeming fees paid",
      "with",
      estimatedWithdrawingFeesPaid,
      "withdrawing fees paid"
    );

    const estimatedCollateralAmount = Number(
      (redeemableAmount - estimatedRedeemingFeesPaid - estimatedWithdrawingFeesPaid).toFixed(
        depository.collateralDecimals
      )
    );

    // Check used redeemable
    expect(redeemableDelta).equal(
      redeemableAmount,
      "The amount of redeemable used for redeem should be exactly the one specified by the user"
    );

    // Check redeemed collateral amount
    // handle precision loss
    expect(collateralDelta)
      .lte(estimatedCollateralAmount)
      .gte(
        Number((estimatedCollateralAmount - collateralNativeUnitPrecision).toFixed(controller.redeemableMintDecimals))
      );

    // Check depository accounting
    expect(nativeToUi(onChainDepository_post.collateralAmountDeposited, depository.collateralDecimals)).equal(
      Number(
        (
          nativeToUi(onChainDepository_pre.collateralAmountDeposited, depository.collateralDecimals) -
          (collateralDelta + estimatedWithdrawingFeesPaid)
        ).toFixed(depository.collateralDecimals)
      )
    );

    expect(nativeToUi(onChainDepository_post.redeemableAmountUnderManagement, controller.redeemableMintDecimals)).equal(
      Number(
        (
          nativeToUi(onChainDepository_pre.redeemableAmountUnderManagement, controller.redeemableMintDecimals) -
          redeemableAmount
        ).toFixed(controller.redeemableMintDecimals)
      )
    );

    expect(nativeToUi(onChainDepository_post.redeemingFeeTotalAccrued, controller.redeemableMintDecimals)).equal(
      Number(
        (
          nativeToUi(onChainDepository_pre.redeemingFeeTotalAccrued, controller.redeemableMintDecimals) +
          estimatedRedeemingFeesPaid
        ).toFixed(controller.redeemableMintDecimals)
      )
    );

    // Check controller accounting
    expect(nativeToUi(onchainController_post.redeemableCirculatingSupply, controller.redeemableMintDecimals)).equal(
      Number(
        (
          nativeToUi(onchainController_pre.redeemableCirculatingSupply, controller.redeemableMintDecimals) -
          redeemableAmount
        ).toFixed(controller.redeemableMintDecimals)
      )
    );

    console.groupEnd();

    return collateralDelta;
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
