import { BTC_DECIMALS, Controller, MangoDepository, USDC_DECIMALS, UXD_DECIMALS } from "@uxdprotocol/uxd-client";
import { authority, USDC, user, uxdProgramId, BTC } from "./constants";
import { mangoDepositoryIntegrationSuite } from "./suite/mangoDepositoryIntegrationSuite";

const depositoryBTC = new MangoDepository(BTC, "BTC", BTC_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

describe("BTC integration tests", () => {
    mangoDepositoryIntegrationSuite(authority, user, controllerUXD, depositoryBTC);
});