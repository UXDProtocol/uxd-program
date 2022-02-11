import { BN } from "@project-serum/anchor";
import { Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, ZoDepository } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { depositInsuranceToMangoDepository, getControllerAccount, getMangoDepositoryAccount } from "../api";
import { CLUSTER, mangoCrankInterval } from "../constants";
import { DepositoryAccountingInfo, sleep } from "../utils";

export const depositoryAccountingMangoDepositoryTest = async(depository: MangoDepository, depositoryAccounting: DepositoryAccountingInfo) => {
    console.group("⏱ depositoryAccountingMangoDepositoryTest");
    try {
        const depositoryAccount = await getMangoDepositoryAccount(depository);

        // CHECK insurance_amount_deposited ACCOUNTING INFO
        const insuranceAmountDeposited = depositoryAccount.insuranceAmountDeposited;
        expect(insuranceAmountDeposited.toNumber(), "InsuranceAmountDeposited is not correct.")
            .equals(
                (depositoryAccounting.insuranceDelta
                * (10 ** depositoryAccount.insuranceMintDecimals))
                + depositoryAccounting.insuranceInitial
                );

        // CHECK collateral_amount_deposited ACCOUNTING INFO
        const collateralAmountDeposited = depositoryAccount.collateralAmountDeposited;
        console.log(`chain: ${collateralAmountDeposited.toNumber()}`);
        console.log(`delta: ${depositoryAccounting.collateralDelta}`);
        console.log(`powr: ${10 ** depositoryAccount.collateralMintDecimals}`);
        console.log(`initial: ${depositoryAccounting.collateralInitial}`);
        expect(collateralAmountDeposited.toNumber(), "CollateralAmountDeposited is not correct.")
            .equals(
                (depositoryAccounting.collateralDelta
                * (10 ** depositoryAccount.collateralMintDecimals))
                + depositoryAccounting.collateralInitial
            );

        // CHECK redeemable_amount_under_management ACCOUNTING INFO
        const redeemableAmountUnderManagement = depositoryAccount.redeemableAmountUnderManagement;

        // CHECK total_amount_paid_taker_fee ACCOUNTING INFO
        const totalAmountPaidTakerFee = depositoryAccount.totalAmountPaidTakerFee;



        console.log(`🧾 Depository Accounting info is correct`);
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}