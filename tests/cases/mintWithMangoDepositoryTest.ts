import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { collateralUIPriceInMangoQuote, mintWithMangoDepository } from "../api";
import { CLUSTER, MANGO_QUOTE_DECIMALS, slippageBase, uxdHelpers } from "../constants";
import { provider } from "../provider";
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
    // - Get the perp price at the same moment to have the less diff between exec and test price
    const mangoPerpPrice = await collateralUIPriceInMangoQuote(depository, mango);
    const txId = await mintWithMangoDepository(user, slippage, collateralAmount, controller, depository, mango);
    await provider.connection.confirmTransaction(txId, 'confirmed');

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
    // There will be issues due to the TX fee + account creation fee, in some case that will fail the slippage test
    // So for now, until we implement a separate payer/user for mint and redeem, don't use tiny amounts for test where the 0.00203928
    // could create a fail positive for wrong slippage
    const collateralDelta = userCollateralBalance - userCollateralBalance_post;
    const collateralLeftOver = collateralAmount - collateralDelta;
    const maxRedeemableDelta = collateralDelta * mangoPerpPrice.toBig();
    // Will be a bit over
    const mangoTakerFee = uxdHelpers.getMangoTakerFeeForPerp(depository, mango);
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