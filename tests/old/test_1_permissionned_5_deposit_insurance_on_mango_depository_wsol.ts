import { authority } from "./identities";
import { expect } from "chai";
import { depositInsuranceToMangoDepository, getMangoDepositoryAccount } from "./test_0_uxd_api";
import { accountUpdateSleepingInterval, controllerUXD, depositoryWSOL, mango } from "./test_0_consts";
import { BN } from "@project-serum/anchor";
import { sleep } from "./integration_test_utils";

describe(" ======= [Suite 1-5 : Test depositing insurance on the mango WSOL depository (1 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("\n", async () => { });

    const insurance_amount = 100; // insurance token UI amount

    it(`1 - Deposit ${insurance_amount} to the WSOL mango Depository`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;
        const depository = depositoryWSOL;
        const _preMangoDepositoryAccount = await getMangoDepositoryAccount(depository);
        const _preInsuranceAmount = _preMangoDepositoryAccount.insuranceAmountDeposited.toNumber() / (10 ** depository.insuranceMintDecimals);

        // WHEN
        const txId = await depositInsuranceToMangoDepository(caller, insurance_amount, controller, depository, mango);
        console.log(`txId : ${txId}`);

        // THEN
        const depositoryAccount = await getMangoDepositoryAccount(depository);
        const _postInsuranceAmount = depositoryAccount.insuranceAmountDeposited.div(new BN(10 ** depository.insuranceMintDecimals));
        const expectedAmount = _preInsuranceAmount + insurance_amount;

        // No crank on the deposit.. so gotta wait
        await sleep(accountUpdateSleepingInterval);

        // Check that the accounting match the actual balances - TODO
        // Check onchain accounting -- Only that for now cause need to refine how to fetch mango account data
        expect(_postInsuranceAmount.toNumber()).closeTo(expectedAmount, Math.pow(10, -depository.insuranceMintDecimals), "The mango depositories insurance ACCOUNTING isn't correct.");

        console.log(`    ==> Insurance Amount was ${_preInsuranceAmount.toString()}, now is ${_postInsuranceAmount.toString()} (deposited ${insurance_amount})`);
        // controller.info();
        // console.log(controllerAccount);
    });
});