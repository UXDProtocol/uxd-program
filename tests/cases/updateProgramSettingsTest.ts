import { BN } from "@project-serum/anchor";
import { Signer } from "@solana/web3.js";
import { Controller, ProgramSettings } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { getControllerAccount, setMangoDepositoriesRedeemableSoftCap, updateProgramSettings } from "../api";
import { CLUSTER } from "../constants";

export const updateProgramSettingsTest = async (programSettings: ProgramSettings, authority: Signer, controller: Controller) => {
    console.group("🧭 updateProgramSettingsTest");
    try {
        // GIVEN
        const redeemableGlobalSupplyCap = (await getControllerAccount(controller)).redeemableGlobalSupplyCap.div(new BN(10 ** controller.redeemableMintDecimals));
        const mangoDepositoryRedeemableSoftCap = (await getControllerAccount(controller)).mangoDepositoriesRedeemableSoftCap.div(new BN(10 ** controller.redeemableMintDecimals)); 
    
        // WHEN
        const txId = await updateProgramSettings(authority, controller, programSettings);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const controllerAccount = await getControllerAccount(controller);
        const redeemableGlobalSupplyCap_post = controllerAccount.redeemableGlobalSupplyCap.div(new BN(10 ** controller.redeemableMintDecimals));
        const mangoDepositoryRedeemableSoftCap_post = controllerAccount.mangoDepositoriesRedeemableSoftCap.div(new BN(10 ** controller.redeemableMintDecimals));
        const redeemableCirculatingSupply = controllerAccount.redeemableCirculatingSupply.div(new BN(10 ** controller.redeemableMintDecimals));

        expect(redeemableGlobalSupplyCap_post.toNumber()).equals(programSettings.redeemableGlobalSupplyCap, "The redeemable global supply cap hasn't been updated.");
        expect(mangoDepositoryRedeemableSoftCap_post.toNumber()).equals(programSettings.redeemableSoftCap, "The redeemable mango depository soft cap hasn't been updated.");
        console.log(`🧾 Previous global supply cap was`, redeemableGlobalSupplyCap.toString(), "now is", redeemableGlobalSupplyCap_post.toString(), "(circulating supply", redeemableCirculatingSupply.toString(), ")");
        console.log(`🧾 Previous mango depositories soft cap was`, mangoDepositoryRedeemableSoftCap.toString(), "now is", mangoDepositoryRedeemableSoftCap_post.toString(), "(circulating supply", redeemableCirculatingSupply.toString(), ")");
        controller.info();
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}
