import { printUserBalances, printDepositoryInfo } from "../utils";
import { depositoryWSOL, mango } from "../constants";

describe("==== print balances info ====", () => {
    it("", async () => {
        await printUserBalances();
        await printDepositoryInfo(depositoryWSOL, mango);
    });
});
