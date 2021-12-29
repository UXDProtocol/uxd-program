import { Controller, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS } from "@uxdprotocol/uxd-client";
import { authority, USDC, WSOL, uxdProgram } from "./constants";
import { setupControllerAndMangoDepositorySolSuite } from "./suite/setupControllerAndMangoDepositorySolSuite";

const depositoryWSOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgram.programId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgram.programId);

describe("Controller + Depository setup (SOL), deposit 100 USDC", () => {
    setupControllerAndMangoDepositorySolSuite(authority, controllerUXD, depositoryWSOL);
});