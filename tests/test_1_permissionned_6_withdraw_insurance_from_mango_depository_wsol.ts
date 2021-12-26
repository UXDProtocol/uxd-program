import { authority } from "./identities";
import { expect } from "chai";
import { getMangoDepositoryAccount, withdrawInsuranceFromMangoDepository } from "./test_0_uxd_api";
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
        const _preInsuranceAmount = (await getMangoDepositoryAccount(depository)).insuranceAmountDeposited.div(new BN(10 ** depository.insuranceMintDecimals));

        // WHEN
        const txId = await withdrawInsuranceFromMangoDepository(caller, insurance_amount, controller, depository, mango);
        console.log(`txId : ${txId}`);

        // THEN
        // Sleep cause no crank on deposit..
        await sleep(accountUpdateSleepingInterval);
        const depositoryAccount = await getMangoDepositoryAccount(depository);
        const _postInsuranceAmount = depositoryAccount.insuranceAmountDeposited.div(new BN(10 ** depository.insuranceMintDecimals));
        const expectedAmountAccounting = _preInsuranceAmount.toNumber() - insurance_amount;

        console.log(`    ==> Insurance Amount was ${_preInsuranceAmount.toString()}, now is ${_postInsuranceAmount.toString()} (withdrawn ${insurance_amount})`);

        // Check that the accounting match the actual balances - TODO
        // Check onchain accounting
        expect(_postInsuranceAmount.toNumber()).closeTo(expectedAmountAccounting, Math.pow(10, -depository.insuranceMintDecimals), "The mango depositories insurance ACCOUNTING isn't correct.");

        // controller.info();
        // console.log(controllerAccount);
    });
});