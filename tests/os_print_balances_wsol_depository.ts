import { printUserBalances, printDepositoryInfo } from "./integration_test_utils";
import { TXN_OPTS } from "./provider";
import { controllerUXD, depositoryWSOL, mango, uxdClient } from "./test_0_consts";

describe("==== print balances info ====", () => {
    it("", async () => {
        await printUserBalances();
        await printDepositoryInfo(depositoryWSOL, mango);
    });
});
