import { Controller } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { getControllerAccount, getMangoDepositoryAccountFromPubkey } from "../api";

export const controllerAccountingMangoDepositoryTest = async(controller: Controller) => {
    console.group("⏱ controllerAccountingMangoDepositoryTest");
    try {
        const controllerAccount = await getControllerAccount(controller);
        let totalRedeemableAmountUnderManagement = 0;

        // CHECK redeemable_circulating_supply ACCOUNTING INFO
        const redeemableCirculatingSupply = controllerAccount.redeemableCirculatingSupply;
        const registeredMangoDepositories = controllerAccount.registeredMangoDepositories;
        for (var mangoDepository of registeredMangoDepositories) {
            if (mangoDepository.toString() == "11111111111111111111111111111111"){
                continue;
            }
            let currentDepositoryRedeemable = (await getMangoDepositoryAccountFromPubkey(mangoDepository))
                .redeemableAmountUnderManagement;
            totalRedeemableAmountUnderManagement += currentDepositoryRedeemable.toNumber();
        }

        expect(redeemableCirculatingSupply.toNumber(), "RedeemableCirculatingSupply is not correct.")
            .equals(totalRedeemableAmountUnderManagement);

        console.log(`🧾 Controller Accounting info is correct`);
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}