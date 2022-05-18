import { Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { quoteRedeemFromMangoDepository } from "../api";
import { getConnection, TXN_OPTS } from "../connection";
import { CLUSTER } from "../constants";

export const quoteRedeemFromMangoDepositoryAccountingTest = async function (redeemableAmount, user: Signer, controller: Controller, depository: MangoDepository, mango: Mango, payer?: Signer) {
    console.group("🧭 quoteRedeemFromMangoDepositoryAccountingTest");
    try {
        const connection = getConnection();
        const options = TXN_OPTS;

        // GIVEN
        const depositoryAccount = await depository.getOnchainAccount(connection, options);
        const controllerAccount = await controller.getOnchainAccount(connection, options);

        const depositoryNetQuoteMinted = nativeToUi(depositoryAccount.netQuoteMinted.toNumber(), depository.quoteMintDecimals);
        const depositoryRedeemableAmountUnderManagement = nativeToUi(depositoryAccount.redeemableAmountUnderManagement.toNumber(), controller.redeemableMintDecimals);
        const depositoryTotalQuoteMintAndRedeemFees = nativeToUi(depositoryAccount.totalQuoteMintAndRedeemFees.toNumber(), depository.quoteMintDecimals);
        const controllerRedeemableCirculatingSupply = nativeToUi(controllerAccount.redeemableCirculatingSupply.toNumber(), controller.redeemableMintDecimals);

        // WHEN
        const txId = await quoteRedeemFromMangoDepository(user, payer ?? user, redeemableAmount, controller, depository, mango);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const depositoryAccount_post = await depository.getOnchainAccount(connection, options);
        const controllerAccount_post = await controller.getOnchainAccount(connection, options);

        const bps_pow = Math.pow(10, 4);
        const feesAccruedMultiple = depositoryAccount.quoteMintAndRedeemFee / bps_pow;
        const lessFeesMultiple = 1 - feesAccruedMultiple;
        const quoteNativeUnitPrecision = Math.pow(10, -depository.quoteMintDecimals);
        const redeemableNativeUnitPrecision = Math.pow(10, -controller.redeemableMintDecimals);

        const depositoryNetQuoteMinted_post = nativeToUi(depositoryAccount_post.netQuoteMinted.toNumber(), depository.quoteMintDecimals);
        const depositoryRedeemableAmountUnderManagement_post = nativeToUi(depositoryAccount_post.redeemableAmountUnderManagement.toNumber(), controller.redeemableMintDecimals);
        const depositoryTotalQuoteMintAndRedeemFees_post = nativeToUi(depositoryAccount_post.totalQuoteMintAndRedeemFees.toNumber(), depository.quoteMintDecimals);
        const controllerRedeemableCirculatingSupply_post = nativeToUi(controllerAccount_post.redeemableCirculatingSupply.toNumber(), controller.redeemableMintDecimals);

        // Accounting tests
        expect(depositoryNetQuoteMinted_post).closeTo(depositoryNetQuoteMinted - (redeemableAmount * lessFeesMultiple), quoteNativeUnitPrecision);
        expect(depositoryRedeemableAmountUnderManagement_post).closeTo(depositoryRedeemableAmountUnderManagement - redeemableAmount, redeemableNativeUnitPrecision);
        expect(depositoryTotalQuoteMintAndRedeemFees_post).closeTo(depositoryTotalQuoteMintAndRedeemFees + (redeemableAmount * feesAccruedMultiple), redeemableNativeUnitPrecision);
        expect(controllerRedeemableCirculatingSupply_post).closeTo(controllerRedeemableCirculatingSupply - redeemableAmount, redeemableNativeUnitPrecision);

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}
