import { BTC_DECIMALS, Controller, MangoDepository, USDC_DECIMALS, UXD_DECIMALS } from "@uxdprotocol/uxd-client";
import { authority, USDC, user, uxdProgram, BTC } from "./constants";
import { mangoDepositoryIntegrationSuite } from "./suite/mangoDepositoryIIntegrationSuite";

const depositoryBTC = new MangoDepository(BTC, "BTC", BTC_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgram.programId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgram.programId);

describe("BTC integration tests", () => {
    mangoDepositoryIntegrationSuite(authority, user, controllerUXD, depositoryBTC);
});