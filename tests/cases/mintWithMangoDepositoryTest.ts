import { NATIVE_MINT } from "@solana/spl-token";
import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, findATAAddrSync } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { collateralUIPriceInMangoQuote, mintWithMangoDepository } from "../api";
import { getSolBalance, getBalance } from "../utils";

export const mintWithMangoDepositoryTest = async (collateralAmount: number, slippage: number, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango): Promise<number> => {
    console.groupCollapsed("🧭 mintWithMangoDepositoryTest");
    // GIVEN
    const userCollateralATA: PublicKey = findATAAddrSync(user.publicKey, depository.collateralMint)[0];
    const userRedeemableATA: PublicKey = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];
    const userRedeemableBalance = await getBalance(userRedeemableATA);
    let userCollateralBalance: number = 0;

    userCollateralBalance = await getBalance(userCollateralATA);
    if (NATIVE_MINT.equals(depository.collateralMint)) {
        // use SOL + WSOL balance
        userCollateralBalance += await getSolBalance(user.publicKey);
    }

    // WHEN
    const txId = await mintWithMangoDepository(user, slippage, collateralAmount, controller, depository, mango);
    console.log(`🔗 'https://explorer.solana.com/address/${txId}?cluster=devnet'`);

    // THEN
    // Could be wrong cause there is a diff between the oracle fetch price and the operation, but let's ignore that for now
    const maxRedeemableDelta = (await collateralUIPriceInMangoQuote(depository, mango)) * collateralAmount;
    const userRedeemableBalance_post = await getBalance(userRedeemableATA);
    let userCollateralBalance_post = await getBalance(userCollateralATA);
    if (NATIVE_MINT.equals(depository.collateralMint)) {
        // use SOL + WSOL balance
        userCollateralBalance_post += await getSolBalance(user.publicKey);
    }

    const redeemableDelta = userRedeemableBalance_post - userRedeemableBalance;
    const collateralDelta = userCollateralBalance - userCollateralBalance_post;

    expect(redeemableDelta).closeTo(maxRedeemableDelta, maxRedeemableDelta * (slippage), "The amount minted is out of the slippage range");
    expect(collateralDelta).closeTo(collateralAmount, Math.pow(10, -depository.collateralMintDecimals), "The collateral amount paid doesn't match the user wallet delta");

    console.log(`🧾 Minted`, redeemableDelta.toFixed(controller.redeemableMintDecimals), controller.redeemableMintSymbol, "for", collateralDelta.toFixed(depository.collateralMintDecimals), depository.collateralMintSymbol, "(perfect was", maxRedeemableDelta.toFixed(controller.redeemableMintDecimals), ")");
    console.groupEnd();
    return redeemableDelta;
}