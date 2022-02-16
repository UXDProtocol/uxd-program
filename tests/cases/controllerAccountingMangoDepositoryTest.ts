import { getProvider } from "@project-serum/anchor";
import { Controller, MangoDepository } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { getConnection, TXN_OPTS } from "../connection";

export const controllerAccountingMangoDepositoryTest = async(controller: Controller, depositories: [MangoDepository]) => {
    const connection = getConnection();
    const options = TXN_OPTS;
    
    console.group("⏱ controllerAccountingMangoDepositoryTest");
    try {
        const controllerAccount = await controller.getOnchainAccount(connection, options);
        let totalRedeemableAmountUnderManagement = 0;

        // CHECK redeemable_circulating_supply ACCOUNTING INFO
        const redeemableCirculatingSupply = controllerAccount.redeemableCirculatingSupply;
        const registeredMangoDepositores = controllerAccount.registeredMangoDepositories;
        for (var depository of depositories) {
            let hit = false;

            // Check to see if each depository is registered with Conteroller
            for (var mangoDepositoryPubkey of registeredMangoDepositores) {
                if (depository.pda.toString() == mangoDepositoryPubkey.toString()) {
                    hit = true;
                    totalRedeemableAmountUnderManagement += 
                        (await depository.getOnchainAccount(connection, options))
                        .redeemableAmountUnderManagement
                        .toNumber();
                    break;
                }
            }

            if (!hit) {
                throw "Depository not in registered depositories of Controller";
            }
        }
        // for (var mangoDepository of registeredMangoDepositores) {
        //     if (mangoDepository.toString() != "11111111111111111111111111111111") {
        //         let currentDepositoryRedeemable = (await controller.getMangoDepositoryAccountFromPubkey(
        //             connection, mangoDepository, options
        //         )).redeemableAmountUnderManagement;
        //         totalRedeemableAmountUnderManagement += currentDepositoryRedeemable.toNumber();
        //     }
        // }
        
        expect(redeemableCirculatingSupply.toNumber(), "RedeemableCirculatingSupply is not correct.")
            .equals(totalRedeemableAmountUnderManagement);

        console.log(`🧾 Controller Accounting info is correct`);
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}