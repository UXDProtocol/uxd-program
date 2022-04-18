import { Signer } from "@solana/web3.js";
import { Controller, ZoDepository, Zo, nativeToUi } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { withdrawInsuranceFromZoDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection, TXN_OPTS } from "../connection";

export const withdrawInsuranceZoDepositoryTest = async function (amount: number, authority: Signer, controller: Controller, depository: ZoDepository, zo: Zo) {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("🧭 withdrawInsuranceZoDepositoryTest");
    try {
        // GIVEN
        const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);
        const insuranceDepositedAmount = nativeToUi(depositoryOnchainAccount.insuranceAmountDeposited.toNumber(), depository.quoteMintDecimals);

        // WHEN
        const txId = await withdrawInsuranceFromZoDepository(amount, authority, controller, depository, zo);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN

        const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
        const insuranceDepositedAmount_post = nativeToUi(depositoryOnchainAccount_post.insuranceAmountDeposited.toNumber(), depository.quoteMintDecimals);
        const expectedAmount = insuranceDepositedAmount - amount;

        console.log(`🧾 Insurance Amount deposited was`, insuranceDepositedAmount, "now is", insuranceDepositedAmount_post, "(withdrawn", amount, ")");

        expect(insuranceDepositedAmount_post).closeTo(expectedAmount, Math.pow(10, -depository.quoteMintDecimals), "The zo depositories insurance ACCOUNTING isn't correct.");

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}