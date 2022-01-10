import { BN } from "@project-serum/anchor";
import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { collateralUIPriceInMangoQuote, redeemFromMangoDepository } from "../api";
import { CLUSTER, MANGO_QUOTE_DECIMALS, uxdHelpers } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const redeemWithMangoDepositoryTest = async (redeemableAmount: number, slippage: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango): Promise<number> => {
    console.group("🧭 redeemWithMangoDepositoryTest");
    // GIVEN
    const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
    const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
    const userRedeemableBalance = await getBalance(userRedeemableATA);
    let userCollateralBalance: number = await getBalance(userCollateralATA);
    if (NATIVE_MINT.equals(depository.collateralMint)) {
        // use SOL + WSOL balance
        userCollateralBalance += await getSolBalance(user.publicKey);
    }

    // WHEN
    const txId = await redeemFromMangoDepository(user, slippage, redeemableAmount, controller, depository, mango);
    // - Get the perp price at the same moment to have the less diff between exec and test price
    const mangoPerpPrice = await collateralUIPriceInMangoQuote(depository, mango);
    console.log("🪙  perp price is", Number(mangoPerpPrice.toFixed(MANGO_QUOTE_DECIMALS)));
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const redeemableMintNativePrecision = Math.pow(10, -controller.redeemableMintDecimals);

    const maxCollateralDelta = redeemableAmount / mangoPerpPrice.toBig();

    const userRedeemableBalance_post = await getBalance(userRedeemableATA);
    let userCollateralBalance_post = await getBalance(userCollateralATA);
    if (NATIVE_MINT.equals(depository.collateralMint)) {
        // use SOL + WSOL balance
        userCollateralBalance_post += await getSolBalance(user.publicKey);
    }

    const redeemableDelta = userRedeemableBalance - userRedeemableBalance_post;
    const collateralDelta = userCollateralBalance_post - userCollateralBalance;
    const mangoTakerFee = await uxdHelpers.getMangoTakerFeeForPerp(depository, mango);
    const maxTakerFee = mangoTakerFee.toNumber() * redeemableAmount;
    // The amount of UXD that couldn't be redeemed due to odd lot size
    const unprocessedRedeemable = redeemableAmount - redeemableDelta;

    console.log(
        `🧾 Redeemed`, Number(redeemableDelta.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol,
        "for", Number(collateralDelta.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
        "(perfect was", Number(redeemableAmount.toFixed(controller.redeemableMintDecimals)),
        "|| ~ returned unprocessed Redeemable due to odd lot ", Number(unprocessedRedeemable.toFixed(controller.redeemableMintDecimals)),
        "|| ~ max taker fees were", Number(maxTakerFee.toFixed(controller.redeemableMintDecimals)),
        "|| ~ loss in slippage", Number((maxCollateralDelta - (collateralDelta)).toFixed(depository.collateralMintDecimals)),
        ")"
    );
    console.groupEnd();

    expect(redeemableDelta + unprocessedRedeemable).closeTo(redeemableAmount, redeemableMintNativePrecision, "Some Redeemable tokens are missing the count.");
    expect(redeemableDelta).closeTo(redeemableAmount - unprocessedRedeemable, maxTakerFee, "The Redeemable delta is out of odd lot range");
    expect(collateralDelta).closeTo(maxCollateralDelta, maxCollateralDelta * (slippage) + maxTakerFee, "The Collateral delta is out of the slippage range");
    expect(userRedeemableBalance_post).closeTo(userRedeemableBalance - redeemableAmount + unprocessedRedeemable, redeemableMintNativePrecision, "The amount of UnprocessedRedeemable carried over is wrong");
    return redeemableDelta;
}