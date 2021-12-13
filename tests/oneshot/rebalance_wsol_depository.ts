// import { printUserBalances, printDepositoryInfo } from "../integration_test_utils";
// import { depositoryWSOL, mango } from "../test_0_consts";
// import { settleMangoDepositoryMangoAccountFees, settleMangoDepositoryMangoAccountPnl } from "../test_0_uxd_api";

// describe("==== Settle PNL then rebalance ====", () => {
//     beforeEach("\n", async () => { });
//     afterEach("", async () => {
//         await printUserBalances();
//         await printDepositoryInfo(depositoryWSOL, mango);
//     });

//     it(`0 - initial state`, async () => { /* no-op */ });


//     it(`1 - Settle WSOL depository PNL (mango client permissionless call)`, async () => {
//         await settleMangoDepositoryMangoAccountPnl(depositoryWSOL, mango);
//     });
//     it(`2 - Settle WSOL depository FEES (mango client permissionless call)`, async () => {
//         await settleMangoDepositoryMangoAccountFees(depositoryWSOL, mango);
//     });
// });
