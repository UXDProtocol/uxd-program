import { BN } from "@project-serum/anchor";
import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository, Mango } from "@uxdprotocol/uxd-client";
import { expect } from "chai";
import { getMangoDepositoryAccount, withdrawInsuranceFromMangoDepository } from "../api";
import { CLUSTER, mangoCrankInterval } from "../constants";
import { sleep } from "../utils";

export const withdrawInsuranceMangoDepositoryTest = async (amount: number, authority: Signer, controller: Controller, depository: MangoDepository, mango: Mango) => {
    console.group("🧭 withdrawInsuranceMangoDepositoryTest");
    // GIVEN
    const insuranceDepositedAmount = (await getMangoDepositoryAccount(depository)).insuranceAmountDeposited.toNumber() / (10 ** depository.insuranceMintDecimals);

    // WHEN
    const txId = await withdrawInsuranceFromMangoDepository(authority, amount, controller, depository, mango);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`);

    // THEN
    const insuranceDepositedAmount_post = (await getMangoDepositoryAccount(depository)).insuranceAmountDeposited.div(new BN(10 ** depository.insuranceMintDecimals));
    const expectedAmount = insuranceDepositedAmount - amount;

    // Need the crank to run for update
    await sleep(mangoCrankInterval);

    // Check that the accounting match the actual balances - TODO
    // Check onchain accounting -- Only that for now cause need to refine how to fetch mango account data

    // expect(uxdHelpers.getMangoDepositoryInsuranceBalance.

    expect(insuranceDepositedAmount_post.toNumber()).closeTo(expectedAmount, Math.pow(10, -depository.insuranceMintDecimals), "The mango depositories insurance ACCOUNTING isn't correct.");

    console.log(`🧾 Insurance Amount deposited was`, insuranceDepositedAmount.toString(), "now is", insuranceDepositedAmount_post.toString(), "(withdrawn", amount, ")");
    console.groupEnd();
}