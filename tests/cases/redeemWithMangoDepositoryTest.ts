import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { collateralUIPriceInMangoQuote, redeemFromMangoDepository } from "../api";
import { uxdHelpers } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const redeemWithMangoDepositoryTest = async (redeemableAmount: number, slippage: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango): Promise<number> => {
    console.groupCollapsed("🧭 redeemWithMangoDepositoryTest");
    // GIVEN
    const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
    const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
    const perpMarketTakerFee = uxdHelpers.getMangoTakerFeeForPerp(depository, mango);
    const userRedeemableBalance = await getBalance(userRedeemableATA);
    let userCollateralBalance: number = 0;

    userCollateralBalance = await getBalance(userCollateralATA);
    if (NATIVE_MINT.equals(depository.collateralMint)) {
        // use SOL + WSOL balance
        userCollateralBalance += await getSolBalance(user.publicKey);
    }

    // WHEN
    const txId = await redeemFromMangoDepository(user, slippage, redeemableAmount, controller, depository, mango);
    console.log(`🔗 'https://explorer.solana.com/address/${txId}?cluster=devnet'`);

    // THEN
    const maxCollateralDelta = redeemableAmount / (await collateralUIPriceInMangoQuote(depository, mango));
    const userRedeemableBalance_post = await getBalance(userRedeemableATA);
    let userCollateralBalance_post = await getBalance(userCollateralATA);
    if (NATIVE_MINT.equals(depository.collateralMint)) {
        // use SOL + WSOL balance
        userCollateralBalance_post += await getSolBalance(user.publicKey);
    }

    const redeemableDelta = userRedeemableBalance - userRedeemableBalance_post;
    const collateralDelta = userCollateralBalance_post - userCollateralBalance;
    // The amount of UXD that couldn't be redeemed due to odd lot size
    const unprocessedRedeemable = redeemableAmount - redeemableDelta;

    expect(redeemableDelta).closeTo(redeemableAmount - unprocessedRedeemable, redeemableAmount * perpMarketTakerFee, "The Redeemable delta is out of odd lot range");
    expect(collateralDelta).closeTo(maxCollateralDelta, maxCollateralDelta * (slippage), "The Collateral delta is out of the slippage range");
    expect(userRedeemableBalance_post).closeTo(userRedeemableBalance - redeemableAmount + unprocessedRedeemable, Math.pow(10, -controller.redeemableMintDecimals), "The amount of UnprocessedRedeemable carried over is wrong");

    console.log(`🧾 Redeemed`, collateralDelta.toFixed(depository.collateralMintDecimals), depository.collateralMintSymbol, "for", redeemableDelta.toFixed(controller.redeemableMintDecimals), controller.redeemableMintSymbol, "(perfect was", redeemableAmount.toFixed(controller.redeemableMintDecimals), "| unprocessed Redeemable due to odd lot", unprocessedRedeemable.toFixed(controller.redeemableMintDecimals), ")");
    console.groupEnd();
    return redeemableDelta;
}