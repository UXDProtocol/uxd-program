import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, Mango, nativeToUi } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { withdrawInsuranceFromMangoDepository } from "../api";
import { CLUSTER, mangoCrankInterval } from "../constants";
import { getConnection, TXN_OPTS } from "../provider";
import { sleep } from "../utils";

export const withdrawInsuranceMangoDepositoryTest = async (amount: number, authority: Signer, controller: Controller, depository: MangoDepository, mango: Mango) => {
    const connection = getConnection();
    const options = TXN_OPTS;

    console.group("🧭 withdrawInsuranceMangoDepositoryTest");
    try {
        // GIVEN
        const depositoryOnchainAccount = await depository.getOnchainAccount(connection, options);
        const insuranceDepositedAmount = nativeToUi(depositoryOnchainAccount.insuranceAmountDeposited.toNumber(), depository.insuranceMintDecimals);

        // WHEN
        const txId = await withdrawInsuranceFromMangoDepository(authority, amount, controller, depository, mango);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN

        const depositoryOnchainAccount_post = await depository.getOnchainAccount(connection, options);
        const insuranceDepositedAmount_post = nativeToUi(depositoryOnchainAccount_post.insuranceAmountDeposited.toNumber(), depository.insuranceMintDecimals);
        const expectedAmount = insuranceDepositedAmount - amount;

        // Need the crank to run for update
        await sleep(mangoCrankInterval);

        // Check that the accounting match the actual balances - TODO
        // Check onchain accounting -- Only that for now cause need to refine how to fetch mango account data

        // expect(uxdHelpers.getMangoDepositoryInsuranceBalance.

        expect(insuranceDepositedAmount_post).closeTo(expectedAmount, Math.pow(10, -depository.insuranceMintDecimals), "The mango depositories insurance ACCOUNTING isn't correct.");

        console.log(`🧾 Insurance Amount deposited was`, insuranceDepositedAmount, "now is", insuranceDepositedAmount_post, "(withdrawn", amount, ")");
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}