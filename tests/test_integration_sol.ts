import { Controller, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS } from "@uxdprotocol/uxd-client";
import { authority, USDC, user, WSOL, uxdProgramId } from "./constants";
import { mangoDepositoryIntegrationSuite } from "./suite/mangoDepositoryIntegrationSuite";

const depositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

describe("SOL integration tests", () => {
    mangoDepositoryIntegrationSuite(authority, user, controllerUXD, depositorySOL);
});