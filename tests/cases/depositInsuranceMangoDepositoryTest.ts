
import { BN } from "@project-serum/anchor";
import { Signer } from "@solana/web3.js";
import { Controller, Mango, MangoDepository } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { depositInsuranceToMangoDepository, getMangoDepositoryAccount } from "../api";
import { CLUSTER, mangoCrankInterval } from "../constants";
import { sleep } from "../utils";

export const depositInsuranceMangoDepositoryTest = async (amount: number, authority: Signer, controller: Controller, depository: MangoDepository, mango: Mango) => {
    console.group("⏱ depositInsuranceMangoDepositoryTest");
    try {
        // GIVEN
        const insuranceDepositedAmount = (await getMangoDepositoryAccount(depository)).insuranceAmountDeposited.toNumber() / (10 ** depository.insuranceMintDecimals);

        // WHEN
        const txId = await depositInsuranceToMangoDepository(authority, amount, controller, depository, mango);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

        // THEN
        const insuranceDepositedAmount_post = (await getMangoDepositoryAccount(depository)).insuranceAmountDeposited.div(new BN(10 ** depository.insuranceMintDecimals));
        const expectedAmount = insuranceDepositedAmount + amount;

        // Need the crank to run for update
        await sleep(mangoCrankInterval);

        // Check that the accounting match the actual balances - TODO
        // Check onchain accounting -- Only that for now cause need to refine how to fetch mango account data
        expect(insuranceDepositedAmount_post.toNumber()).closeTo(expectedAmount, Math.pow(10, -depository.insuranceMintDecimals), "The mango depositories insurance ACCOUNTING isn't correct.");

        console.log(`🧾 Insurance Amount deposited was`, insuranceDepositedAmount, "now is", insuranceDepositedAmount_post.toString(), "(deposited", amount, ")");
        console.groupEnd();
    } catch (error) {
        console.groupEnd();
        throw error;
    }
}
