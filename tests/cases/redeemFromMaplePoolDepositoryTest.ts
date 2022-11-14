import { PublicKey, Signer } from "@solana/web3.js";
import {
  Controller,
  MaplePoolDepository,
  findATAAddrSync,
  findMultipleATAAddSync,
  nativeToUi,
} from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { redeemFromMaplePoolDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";
import { getBalance, ceilAtDecimals } from "../utils";

export const redeemFromMaplePoolDepositoryTest = async function (
  redeemableAmount: number,
  user: Signer,
  controller: Controller,
  depository: MaplePoolDepository,
  payer?: Signer
): Promise<number> {
  console.group("🧭 redeemFromMaplePoolDepositoryTest");

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
    const txId = await redeemFromMaplePoolDepository(user, payer ?? user, controller, depository, redeemableAmount);
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
    const estimatedFeesPaid = ceilAtDecimals(
      redeemableAmount - ((10_000 - onChainDepository_pre.redeemingFeeInBps) * redeemableAmount) / 10_000,
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
      estimatedFeesPaid,
      "fees paid."
    );

    const estimatedCollateralAmount = Number(
      (redeemableAmount - estimatedFeesPaid).toFixed(depository.collateralDecimals)
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
          nativeToUi(onChainDepository_pre.collateralAmountDeposited, depository.collateralDecimals) - collateralDelta
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
          estimatedFeesPaid
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
