
import { Signer } from "@solana/web3.js";
import { Controller, Zo, ZoDepository, nativeToUi } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { depositInsuranceToZoDepository } from "../api";
import { CLUSTER } from "../constants";
import { getConnection, TXN_OPTS } from "../connection";

export const depositInsuranceZoDepositoryTest = async function (amount: number, authority: Signer, controller: Controller, depository: ZoDepository, zo: Zo) {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("⏱ depositInsuranceZoDepositoryTest");
    try {
        // GIVEN
        const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);
        const insuranceDepositedAmount = nativeToUi(depositoryOnchainAccount.insuranceAmountDeposited.toNumber(), depository.quoteMintDecimals);

        // WHEN
        const txId = await depositInsuranceToZoDepository(authority, amount, controller, depository, zo);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
        const insuranceDepositedAmount_post = nativeToUi(depositoryOnchainAccount_post.insuranceAmountDeposited.toNumber(), depository.quoteMintDecimals);
        const expectedAmount = insuranceDepositedAmount + amount;

        console.log(`🧾 Insurance Amount deposited was`, insuranceDepositedAmount, "now is", insuranceDepositedAmount_post, "(deposited", amount, ")");

        // Check that the accounting match the actual balances - TODO
        // Check onchain accounting -- Only that for now cause need to refine how to fetch zo account data
        expect(insuranceDepositedAmount_post).closeTo(expectedAmount, Math.pow(10, -depository.quoteMintDecimals), "The zo depositories insurance ACCOUNTING isn't correct.");

        console.groupEnd();
    } catch (error) {
        console.error("❌", error);
        console.groupEnd();
        throw error;
    }
}
