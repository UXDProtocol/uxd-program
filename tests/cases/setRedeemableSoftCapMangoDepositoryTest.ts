import { BN } from "@project-serum/anchor";
import { Signer } from "@solana/web3.js";
import { Controller } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { getControllerAccount, setMangoDepositoriesRedeemableSoftCap } from "../api";

export const setRedeemableSoftCapMangoDepositoryTest = async (softCapAmount: number, authority: Signer, controller: Controller) => {
    console.groupCollapsed("🧭 setRedeemableSoftCapMangoDepositoryTest");
    // GIVEN
    const mangoDepositoryRedeemableSoftCap = (await getControllerAccount(controller)).mangoDepositoriesRedeemableSoftCap.div(new BN(10 ** controller.redeemableMintDecimals));

    // WHEN
    const txId = await setMangoDepositoriesRedeemableSoftCap(authority, controller, softCapAmount);
    console.log(`🔗 'https://explorer.solana.com/address/${txId}?cluster=devnet'`);

    // THEN
    const controllerAccount = await getControllerAccount(controller);
    const mangoDepositoryRedeemableSoftCap_post = controllerAccount.mangoDepositoriesRedeemableSoftCap.div(new BN(10 ** controller.redeemableMintDecimals));
    const redeemableCirculatingSupply = controllerAccount.redeemableCirculatingSupply.div(new BN(10 ** controller.redeemableMintDecimals));

    expect(mangoDepositoryRedeemableSoftCap_post.toNumber()).equals(softCapAmount, "The redeemable mango depository soft cap hasn't been updated.");
    console.log(`🧾 Previous mango depositories soft cap was`, mangoDepositoryRedeemableSoftCap.toString(), "now is", mangoDepositoryRedeemableSoftCap_post.toString(), "(circulating supply", redeemableCirculatingSupply.toString(), ")");
    controller.info();
    console.groupEnd();
}