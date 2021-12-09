import { printUserBalances, printDepositoryInfo } from "./integration_test_utils";
import { depositoryWSOL, mango } from "./test_0_consts";

describe("==== print balances info ====", () => {
    it("", async () => {
        await printUserBalances();
        await printDepositoryInfo(depositoryWSOL, mango);
    });
});
