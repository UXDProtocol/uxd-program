
import { Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository, nativeToUi } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { depositInsuranceToMangoDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection, TXN_OPTS } from "../connection";

export const depositInsuranceMangoDepositoryTest = async function (amount: number, authority: Signer, controller: Controller, depository: MangoDepository, mango: Mango) {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("⏱ depositInsuranceMangoDepositoryTest");
    try {
        // GIVEN
        const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);
        const insuranceDepositedAmount = nativeToUi(depositoryOnchainAccount.insuranceAmountDeposited.toNumber(), depository.insuranceMintDecimals);

        // WHEN
        const txId = await depositInsuranceToMangoDepository(authority, amount, controller, depository, mango);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
        const insuranceDepositedAmount_post = nativeToUi(depositoryOnchainAccount_post.insuranceAmountDeposited.toNumber(), depository.insuranceMintDecimals);
        const expectedAmount = insuranceDepositedAmount + amount;

        // Check that the accounting match the actual balances - TODO
        // Check onchain accounting -- Only that for now cause need to refine how to fetch mango account data
        expect(insuranceDepositedAmount_post).closeTo(expectedAmount, Math.pow(10, -depository.insuranceMintDecimals), "The mango depositories insurance ACCOUNTING isn't correct.");

        console.log(`🧾 Insurance Amount deposited was`, insuranceDepositedAmount, "now is", insuranceDepositedAmount_post, "(deposited", amount, ")");
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}
