import { MangoDepository } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { getConnection, TXN_OPTS } from "../connection";
import { DepositoryAccountingInfo } from "../utils";

export const depositoryAccountingMangoDepositoryTest = async(depositories: [MangoDepository], depositoryAccounting: [DepositoryAccountingInfo]) => {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("⏱ depositoryAccountingMangoDepositoryTest");
    try {
        for (var depository of depositories) {
            let hit = false;
            let currentAccountingInfo: DepositoryAccountingInfo;
            for (var accountingInfo of depositoryAccounting) {
                if (depository.pda.toString() == accountingInfo.depository.pda.toString()){
                    hit = true;
                    currentAccountingInfo = accountingInfo;
                    break;
                }
            }
            if (!hit) {
                throw "No DepositoryAccountingInfo for this depository";
            }
            let depositoryAccount = await depository.getOnchainAccount(connection, options);
            
            // CHECK insurance_amount_deposited ACCOUNTING INFO
            const insuranceAmountDeposited = depositoryAccount.insuranceAmountDeposited;
            expect(insuranceAmountDeposited.toNumber(), "InsuranceAmountDeposited is not correct.")
                .equals(
                    (currentAccountingInfo.insuranceDelta
                    * (10 ** depository.insuranceMintDecimals))
                    + currentAccountingInfo.insuranceInitial
                );
            
            // CHECK collateral_amount_deposited ACCOUNTING INFO
            const collateralAmountDeposited = depositoryAccount.collateralAmountDeposited;
            expect(collateralAmountDeposited.toNumber(), "CollateralAmountDeposited is not correct.")
                .equals(
                    (currentAccountingInfo.collateralDelta
                    * (10 ** depository.collateralMintDecimals))
                    + currentAccountingInfo.collateralInitial
                );

            // // CHECK redeemable_amount_under_management ACCOUNTING INFO
            // const redeemableAmountUnderManagement = depositoryAccount.redeemableAmountUnderManagement;
            // expect(redeemableAmountUnderManagement.toNumber(), "RedeemableAmountUnderManagement is not correct.")
            //     .equals(
            //         (currentAccountingInfo.redeemableDelta)

            //     )
        }


        console.log(`🧾 Mango Depository Accounting info is correct`);
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}