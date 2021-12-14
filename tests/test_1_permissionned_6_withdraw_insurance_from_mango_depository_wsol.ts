import { authority } from "./identities";
import { expect } from "chai";
import { getmangoDepositoryAccount, withdrawInsuranceFromMangoDepository } from "./test_0_uxd_api";
import { accountUpdateSleepingInterval, controllerUXD, depositoryWSOL, mango } from "./test_0_consts";
import { BN } from "@project-serum/anchor";
import { sleep } from "./integration_test_utils";

describe(" ======= [Suite 1-6 : Test withdrawing insurance from the mango WSOL depository (2 op)] ======= ", () => {
    beforeEach("\n", async () => { });
    afterEach("\n", async () => { });

    const insurance_amount = 25; // insurance token UI amount

    it(`1 - Withdraw ${insurance_amount} from the WSOL mango Depository`, async () => {
        // GIVEN
        const caller = authority;
        const controller = controllerUXD;
        const depository = depositoryWSOL;
        // const _preDepositoryMangoAccountInsuranceBalance = (await getMangoDepositoryCollateralBalance(depository, mango)) / 10 ** depository.insuranceMintdecimals;
        const _preInsuranceAmount = (await getmangoDepositoryAccount(depository)).insuranceAmountDeposited.div(new BN(10 ** depository.insuranceMintdecimals));

        // WHEN
        const txId = await withdrawInsuranceFromMangoDepository(caller, insurance_amount, controller, depository, mango);
        console.log(`txId : ${txId}`);

        // THEN
        // Sleep cause no crank on deposit..
        await sleep(accountUpdateSleepingInterval);
        const depositoryAccount = await getmangoDepositoryAccount(depository);
        // const _postDepositoryMangoAccountInsuranceBalance = (await getMangoDepositoryCollateralBalance(depository, mango)) / 10 ** depository.insuranceMintdecimals;
        const _postInsuranceAmount = depositoryAccount.insuranceAmountDeposited.div(new BN(10 ** depository.insuranceMintdecimals));
        // const expectedAmountBalance = _preDepositoryMangoAccountInsuranceBalance - insurance_amount;
        const expectedAmountAccounting = _preInsuranceAmount.toNumber() - insurance_amount;

        console.log(`    ==> Insurance Amount was ${_preInsuranceAmount.toString()}, now is ${_postInsuranceAmount.toString()} (withdrawn ${insurance_amount})`);

        // Check that the accounting match the actual balances
        // expect(_postInsuranceAmount).closeTo(_postDepositoryMangoAccountInsuranceBalance, Math.pow(10, -depository.insuranceMintdecimals), "Accounting and Balance mismatch");
        // Check actual balance
        // expect(_preDepositoryMangoAccountInsuranceBalance).closeTo(expectedAmountBalance, Math.pow(10, -depository.insuranceMintdecimals), "The mango depositories insurance BALANCE isn't correct.")
        // Check onchain accounting
        expect(_postInsuranceAmount.toNumber()).closeTo(expectedAmountAccounting, Math.pow(10, -depository.insuranceMintdecimals), "The mango depositories insurance ACCOUNTING isn't correct.");

        // controller.info();
        // console.log(controllerAccount);
    });
});