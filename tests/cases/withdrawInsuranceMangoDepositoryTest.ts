import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, Mango, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { withdrawInsuranceFromMangoDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection, TXN_OPTS } from "../connection";

export const withdrawInsuranceMangoDepositoryTest = async function (amount: number, authority: Signer, controller: Controller, depository: MangoDepository, mango: Mango) {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("🧭 withdrawInsuranceMangoDepositoryTest");
    try {
        // GIVEN
        const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);
        const insuranceDepositedAmount = nativeToUi(depositoryOnchainAccount.insuranceAmountDeposited.toNumber(), depository.quoteMintDecimals);

        // WHEN
        const txId = await withdrawInsuranceFromMangoDepository(amount, authority, controller, depository, mango);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN

        const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
        const insuranceDepositedAmount_post = nativeToUi(depositoryOnchainAccount_post.insuranceAmountDeposited.toNumber(), depository.quoteMintDecimals);
        const expectedAmount = insuranceDepositedAmount - amount;

        console.log(`🧾 Insurance Amount deposited was`, insuranceDepositedAmount, "now is", insuranceDepositedAmount_post, "(withdrawn", amount, ")");

        expect(insuranceDepositedAmount_post).closeTo(expectedAmount, Math.pow(10, -depository.quoteMintDecimals), "The mango depositories insurance ACCOUNTING isn't correct.");

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}