import { Controller, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, USDC_DEVNET, WSOL } from "@uxdprotocol/uxd-client";
import { authority, uxdProgramId } from "./constants";
import { setupControllerAndMangoDepositorySolSuite } from "./suite/setupControllerAndMangoDepositorySolSuite";

const depositoryWSOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

describe("Controller + Depository setup (SOL), deposit 100 USDC", () => {
    setupControllerAndMangoDepositorySolSuite(authority, controllerUXD, depositoryWSOL);
});