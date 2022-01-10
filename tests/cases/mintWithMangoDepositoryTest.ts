import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { collateralUIPriceInMangoQuote, mintWithMangoDepository } from "../api";
import { CLUSTER, MANGO_QUOTE_DECIMALS, slippageBase, uxdHelpers } from "../constants";
import { getSolBalance, getBalance } from "../utils";

export const mintWithMangoDepositoryTest = async (collateralAmount: number, slippage: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango): Promise<number> => {
    console.group("🧭 mintWithMangoDepositoryTest");
    // GIVEN
    const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
    const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
    const userRedeemableBalance = await getBalance(userRedeemableATA);
    let userCollateralBalance: number = await getBalance(userCollateralATA);;
    if (NATIVE_MINT.equals(depository.collateralMint)) {
        // use SOL + WSOL balance
        userCollateralBalance += await getSolBalance(user.publicKey);
    }

    // WHEN
    const txId = await mintWithMangoDepository(user, slippage, collateralAmount, controller, depository, mango);
    // - Get the perp price at the same moment to have the less diff between exec and test price
    const mangoPerpPrice = await collateralUIPriceInMangoQuote(depository, mango);
    console.log("🪙  perp price is", Number(mangoPerpPrice.toFixed(MANGO_QUOTE_DECIMALS)));
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const userRedeemableBalance_post = await getBalance(userRedeemableATA);
    let userCollateralBalance_post = await getBalance(userCollateralATA);
    if (NATIVE_MINT.equals(depository.collateralMint)) {
        // use SOL + WSOL balance
        userCollateralBalance_post += await getSolBalance(user.publicKey);
    }
    const redeemableDelta = userRedeemableBalance_post - userRedeemableBalance;
    const collateralDelta = userCollateralBalance - userCollateralBalance_post;
    const collateralLeftOver = collateralAmount - collateralDelta;
    const maxRedeemableDelta = collateralDelta * mangoPerpPrice.toBig();
    // Will be a bit over
    const mangoTakerFee = await uxdHelpers.getMangoTakerFeeForPerp(depository, mango);
    const maxTakerFee = mangoTakerFee.toNumber() * maxRedeemableDelta;
    const collateralNativeUnitPrecision = Math.pow(10, -depository.collateralMintDecimals);

    console.log(
        `🧾 Minted`, Number(redeemableDelta.toFixed(controller.redeemableMintDecimals)), controller.redeemableMintSymbol,
        "for", Number(collateralDelta.toFixed(depository.collateralMintDecimals)), depository.collateralMintSymbol,
        "(perfect was", Number(maxRedeemableDelta.toFixed(controller.redeemableMintDecimals)),
        "|| returned unprocessed collateral due to odd lot", Number(collateralLeftOver.toFixed(depository.collateralMintDecimals)),
        "|| ~ max taker fees were", Number(maxTakerFee.toFixed(controller.redeemableMintDecimals)),
        "|| ~ loss in slippage", Number((maxRedeemableDelta - (redeemableDelta + maxTakerFee)).toFixed(controller.redeemableMintDecimals)),
        ")"
    );
    console.groupEnd();
    expect(collateralLeftOver + collateralDelta).closeTo(collateralAmount, collateralNativeUnitPrecision, "The amount of collateral used for redeem + carried over should be equal to the inputted amount.")
    expect(redeemableDelta).closeTo(maxRedeemableDelta, (maxRedeemableDelta * (slippage / slippageBase)) + maxTakerFee, "The amount minted is out of the slippage range");
    expect(collateralDelta).closeTo(collateralAmount - collateralLeftOver, collateralNativeUnitPrecision, "The collateral amount paid doesn't match the user wallet delta");

    return redeemableDelta;
}