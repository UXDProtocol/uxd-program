import { BN } from "@project-serum/anchor";
import { Signer } from "@solana/web3.js";
import { Controller } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { getControllerAccount, setRedeemableGlobalSupplyCap } from "../api";

export const setRedeemableGlobalSupplyCapTest = async (supplyCapAmount: number, authority: Signer, controller: Controller) => {
    console.groupCollapsed("🧭 setRedeemableGlobalSupplyCapTest");
    // GIVEN
    const redeemableGlobalSupplyCap = (await getControllerAccount(controller)).redeemableGlobalSupplyCap.div(new BN(10 ** controller.redeemableMintDecimals));

    // WHEN
    const txId = await setRedeemableGlobalSupplyCap(authority, controller, supplyCapAmount);
    console.log(`🔗 'https://explorer.solana.com/address/${txId}?cluster=devnet'`);

    // THEN
    const controllerAccount = await getControllerAccount(controller);
    const redeemableGlobalSupplyCap_post = controllerAccount.redeemableGlobalSupplyCap.div(new BN(10 ** controller.redeemableMintDecimals));
    const redeemableCirculatingSupply = controllerAccount.redeemableCirculatingSupply.div(new BN(10 ** controller.redeemableMintDecimals));

    expect(redeemableGlobalSupplyCap_post.toNumber()).equals(supplyCapAmount, "The redeemable global supply cap hasn't been updated.");
    console.log(`🧾 Previous global supply cap was`, redeemableGlobalSupplyCap.toString(), "now is", redeemableGlobalSupplyCap_post.toString(), "(circulating supply", redeemableCirculatingSupply.toString(), ")");
    controller.info();
    console.groupEnd();
}