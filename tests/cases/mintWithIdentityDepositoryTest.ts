import { Signer } from "@solana/web3.js";
import { uiToNative } from "@uxd-protocol/uxd-client";
import { Controller, IdentityDepository, findMultipleATAAddSync, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { mintWithIdentityDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";
import { getBalance } from "../utils";

export const mintWithIdentityDepositoryTest = async function (
  collateralAmount: number,
  user: Signer,
  controller: Controller,
  depository: IdentityDepository,
  payer?: Signer
): Promise<number> {
  console.group("🧭 mintWithIdentityDepositoryTest");

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
    const txId = await mintWithIdentityDepository(user, payer ?? user, controller, depository, collateralAmount);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const [userRedeemableBalance_post, userCollateralBalance_post, onchainController_post, onChainDepository_post] =
      await Promise.all([
        getBalance(userRedeemableATA),
        getBalance(userCollateralATA),
        controller.getOnchainAccount(getConnection(), TXN_OPTS),
        depository.getOnchainAccount(getConnection(), TXN_OPTS),
      ]);

    // Use toFixed to avoid +0.010000000000000009 != than +0.01
    const collateralDelta = Number(
      (userCollateralBalance_pre - userCollateralBalance_post).toFixed(depository.collateralMintDecimals)
    );
    const redeemableDelta = Number(
      (userRedeemableBalance_post - userRedeemableBalance_pre).toFixed(controller.redeemableMintDecimals)
    );

    const collateralNativeUnitPrecision = Math.pow(10, -depository.collateralMintDecimals);
    const nativeCollateralAmount = uiToNative(collateralAmount, depository.collateralMintDecimals);

    console.log(
      `🧾 Minted`,
      Number(redeemableDelta.toFixed(controller.redeemableMintDecimals)),
      controller.redeemableMintSymbol,
      "by locking",
      Number(collateralDelta.toFixed(depository.collateralMintDecimals)),
      depository.collateralMintSymbol
    );

    // Check used collateral
    expect(collateralDelta).equal(
      collateralAmount,
      "The amount of collateral used for the mint should be exactly the one specified by the user"
    );

    // Check minted redeemable amount
    expect(redeemableDelta).gte(Number(collateralAmount.toFixed(depository.collateralMintDecimals)));

    // Check depository accounting
    expect(nativeToUi(onChainDepository_post.collateralAmountDeposited, depository.collateralMintDecimals)).equal(
      Number(
        (
          nativeToUi(onChainDepository_pre.collateralAmountDeposited, depository.collateralMintDecimals) +
          collateralAmount
        ).toFixed(depository.collateralMintDecimals)
      )
    );

    // expect(nativeToUi(onChainDepository_post.redeemableAmountUnderManagement, controller.redeemableMintDecimals))
    //     .equal(Number((nativeToUi(onChainDepository_pre.redeemableAmountUnderManagement, controller.redeemableMintDecimals) + redeemableDelta).toFixed(controller.redeemableMintDecimals)));

    // Check controller accounting
    expect(nativeToUi(onchainController_post.redeemableCirculatingSupply, controller.redeemableMintDecimals)).equal(
      Number(
        (
          nativeToUi(onchainController_pre.redeemableCirculatingSupply, controller.redeemableMintDecimals) +
          redeemableDelta
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
