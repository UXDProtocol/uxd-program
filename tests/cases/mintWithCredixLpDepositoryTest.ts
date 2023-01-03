import { Signer } from "@solana/web3.js";
import { Controller, CredixLpDepository, findATAAddrSync, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { mintWithCredixLpDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";
import { getBalance, ceilAtDecimals } from "../utils";

export const mintWithCredixLpDepositoryTest = async function (
  uiAmmountCollateralDeposited: number,
  user: Signer,
  controller: Controller,
  depository: CredixLpDepository,
  payer?: Signer
): Promise<number> {
  console.group("🧭 mintWithCredixLpDepositoryTest");

  try {
    // GIVEN
    const [userCollateral] = findATAAddrSync(user.publicKey, depository.collateralMint);
    const [userRedeemable] = findATAAddrSync(user.publicKey, controller.redeemableMintPda);

    const [userCollateralBalance_pre, userRedeemableBalance_pre, onchainController_pre, onchainDepository_pre] =
      await Promise.all([
        getBalance(userCollateral),
        getBalance(userRedeemable),
        controller.getOnchainAccount(getConnection(), TXN_OPTS),
        depository.getOnchainAccount(getConnection(), TXN_OPTS),
      ]);

    // WHEN
    // Simulates user experience from the front end
    const txId = await mintWithCredixLpDepository(
      user,
      payer ?? user,
      controller,
      depository,
      uiAmmountCollateralDeposited
    );
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const [userCollateralBalance_post, userRedeemableBalance_post, onchainController_post, onchainDepository_post] =
      await Promise.all([
        getBalance(userCollateral),
        getBalance(userRedeemable),
        controller.getOnchainAccount(getConnection(), TXN_OPTS),
        depository.getOnchainAccount(getConnection(), TXN_OPTS),
      ]);

    // Use toFixed to avoid +0.010000000000000009 != than +0.01
    const userCollateralDelta = Number(
      (userCollateralBalance_post - userCollateralBalance_pre).toFixed(depository.collateralDecimals)
    );
    const userRedeemableDelta = Number(
      (userRedeemableBalance_post - userRedeemableBalance_pre).toFixed(controller.redeemableMintDecimals)
    );

    const collateralNativeUnitPrecision = Math.pow(10, -depository.collateralDecimals);

    const estimatedFeesPaid = ceilAtDecimals(
      uiAmmountCollateralDeposited -
        ((10_000 - onchainDepository_pre.mintingFeeInBps) * uiAmmountCollateralDeposited) / 10_000,
      controller.redeemableMintDecimals
    );

    console.log(
      `🧾 Minted`,
      userRedeemableDelta,
      controller.redeemableMintSymbol,
      "by locking",
      -userCollateralDelta,
      depository.collateralSymbol,
      "with",
      estimatedFeesPaid,
      "fees paid."
    );

    // Check used collateral
    expect(-userCollateralDelta).equal(
      uiAmmountCollateralDeposited,
      "The amount of collateral used for the mint should be exactly the one specified by the user"
    );

    // Check minted redeemable amount
    // handle precision loss
    const estimatedRedeemableAmount = Number(
      (uiAmmountCollateralDeposited - estimatedFeesPaid).toFixed(controller.redeemableMintDecimals)
    );
    expect(userRedeemableDelta)
      .lte(estimatedRedeemableAmount)
      .gte(
        Number(
          (estimatedRedeemableAmount - collateralNativeUnitPrecision * 10).toFixed(controller.redeemableMintDecimals)
        )
      );

    // Check depository accounting
    expect(
      nativeToUi(onchainDepository_post.depositedCollateralAmount, depository.collateralDecimals).toFixed(
        depository.collateralDecimals
      )
    ).equal(
      (
        nativeToUi(onchainDepository_pre.depositedCollateralAmount, depository.collateralDecimals) +
        uiAmmountCollateralDeposited
      ).toFixed(depository.collateralDecimals)
    );

    expect(
      nativeToUi(onchainDepository_post.redeemableAmountUnderManagement, controller.redeemableMintDecimals).toFixed(
        controller.redeemableMintDecimals
      )
    ).equal(
      (
        nativeToUi(onchainDepository_pre.redeemableAmountUnderManagement, controller.redeemableMintDecimals) +
        userRedeemableDelta
      ).toFixed(controller.redeemableMintDecimals)
    );

    expect(
      nativeToUi(onchainDepository_post.mintingFeeTotalAccrued, depository.collateralDecimals).toFixed(
        controller.redeemableMintDecimals
      )
    ).equal(
      (
        nativeToUi(onchainDepository_pre.mintingFeeTotalAccrued, depository.collateralDecimals) + estimatedFeesPaid
      ).toFixed(controller.redeemableMintDecimals)
    );

    // Check controller accounting
    expect(
      nativeToUi(onchainController_post.redeemableCirculatingSupply, depository.collateralDecimals).toFixed(
        controller.redeemableMintDecimals
      )
    ).equal(
      (
        nativeToUi(onchainController_pre.redeemableCirculatingSupply, depository.collateralDecimals) +
        userRedeemableDelta
      ).toFixed(controller.redeemableMintDecimals)
    );

    console.groupEnd();

    return userRedeemableDelta;
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
