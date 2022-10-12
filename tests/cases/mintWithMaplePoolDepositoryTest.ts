import { BN } from "@project-serum/anchor";
import { Signer } from "@solana/web3.js";
import {
  Controller,
  MaplePoolDepository,
  findATAAddrSync,
  findMultipleATAAddSync,
  nativeToUi,
} from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { mintWithMaplePoolDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";
import { getBalance, ceilAtDecimals } from "../utils";

export const mintWithMaplePoolDepositoryTest = async function (
  uiAmmountCollateralDeposited: number,
  user: Signer,
  controller: Controller,
  depository: MaplePoolDepository,
  payer?: Signer
): Promise<number> {
  console.group("🧭 mintWithMaplePoolDepositoryTest");

  try {
    // GIVEN
    const [userCollateral] = findATAAddrSync(user.publicKey, depository.collateralMint);
    const [userRedeemable] = findATAAddrSync(user.publicKey, controller.redeemableMintPda);

    console.log("->>>>>>>>>>>>>>", depository.maplePoolLocker);

    const [userRedeemableBalance_pre, userCollateralBalance_pre, onchainController_pre, onchainDepository_pre] =
      await Promise.all([
        getBalance(userCollateral),
        getBalance(userRedeemable),
        controller.getOnchainAccount(getConnection(), TXN_OPTS),
        depository.getOnchainAccount(getConnection(), TXN_OPTS),
      ]);

    // WHEN
    // Simulates user experience from the front end
    const txId = await mintWithMaplePoolDepository(
      user,
      payer ?? user,
      controller,
      depository,
      uiAmmountCollateralDeposited
    );
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const [userRedeemableBalance_post, userCollateralBalance_post, onchainController_post, onchainDepository_post] =
      await Promise.all([
        getBalance(userCollateral),
        getBalance(userRedeemable),
        controller.getOnchainAccount(getConnection(), TXN_OPTS),
        depository.getOnchainAccount(getConnection(), TXN_OPTS),
      ]);

    // Use toFixed to avoid +0.010000000000000009 != than +0.01
    const collateralDelta = Number(
      (userCollateralBalance_pre - userCollateralBalance_post).toFixed(depository.collateralDecimals)
    );
    const redeemableDelta = Number(
      (userRedeemableBalance_post - userRedeemableBalance_pre).toFixed(controller.redeemableMintDecimals)
    );

    const collateralNativeUnitPrecision = Math.pow(10, -depository.collateralDecimals);

    const estimatedFeesPaid = ceilAtDecimals(
      uiAmmountCollateralDeposited -
        ((10_000 - onchainDepository_pre.mintingFeesBps) * uiAmmountCollateralDeposited) / 10_000,
      controller.redeemableMintDecimals
    );

    console.log(
      `🧾 Minted`,
      Number(redeemableDelta),
      controller.redeemableMintSymbol,
      "by locking",
      Number(collateralDelta.toFixed(depository.collateralDecimals)),
      "with",
      estimatedFeesPaid,
      "fees paid."
    );

    const estimatedRedeemableAmount = Number(
      (uiAmmountCollateralDeposited - estimatedFeesPaid).toFixed(controller.redeemableMintDecimals)
    );

    // Check used collateral
    expect(collateralDelta).equal(
      uiAmmountCollateralDeposited,
      "The amount of collateral used for the mint should be exactly the one specified by the user"
    );

    // Check minted redeemable amount
    // handle precision loss
    expect(redeemableDelta)
      .lte(estimatedRedeemableAmount)
      .gte(
        Number((estimatedRedeemableAmount - collateralNativeUnitPrecision).toFixed(controller.redeemableMintDecimals))
      );

    // Check depository accounting
    expect(
      nativeToUi(onchainDepository_post.depositedCollateralAmount, depository.collateralDecimals).toFixed(
        depository.collateralDecimals
      )
    ).equal(
      Number(
        (
          nativeToUi(onchainDepository_pre.depositedCollateralAmount, depository.collateralDecimals) +
          uiAmmountCollateralDeposited
        ).toFixed(depository.collateralDecimals)
      )
    );

    expect(
      nativeToUi(onchainDepository_post.mintedRedeemableAmount, controller.redeemableMintDecimals).toFixed(
        controller.redeemableMintDecimals
      )
    ).equal(
      Number(
        (
          nativeToUi(onchainDepository_pre.mintedRedeemableAmount, controller.redeemableMintDecimals) + redeemableDelta
        ).toFixed(controller.redeemableMintDecimals)
      )
    );

    expect(
      nativeToUi(onchainDepository_post.mintingFeesTotalPaid, depository.collateralDecimals).toFixed(
        controller.redeemableMintDecimals
      )
    ).equal(
      Number(
        (
          nativeToUi(onchainDepository_pre.mintingFeesTotalPaid, depository.collateralDecimals) + estimatedFeesPaid
        ).toFixed(controller.redeemableMintDecimals)
      )
    );

    // Check controller accounting
    expect(nativeToUi(onchainController_post.redeemableCirculatingSupply, depository.collateralDecimals)).equal(
      Number(
        (
          nativeToUi(onchainController_pre.redeemableCirculatingSupply, depository.collateralDecimals) + redeemableDelta
        ).toFixed(controller.redeemableMintDecimals)
      )
    );

    console.groupEnd();

    return redeemableDelta;
  } catch (error) {
    console.error("❌", error);
    console.groupEnd();
    throw error;
  }
};
