import {
  Controller,
  MangoDepository,
  SOL_DECIMALS,
  USDC_DECIMALS,
  USDC_DEVNET,
  UXD_DECIMALS,
  WSOL,
} from "@uxd-protocol/uxd-client";
import { authority, bank, uxdProgramId } from "./constants";
import { controllerIntegrationSuiteParameters, controllerIntegrationSuite } from "./suite/controllerIntegrationSuite";
import { mangoDepositorySetupSuite } from "./suite/depositorySetupSuite";

const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
const mangoDepositorySOL = new MangoDepository(
  WSOL,
  "SOL",
  SOL_DECIMALS,
  USDC_DEVNET,
  "USDC",
  USDC_DECIMALS,
  uxdProgramId
);

beforeEach("\n", function () {
  console.log("=============================================\n\n");
});

describe("UXD Setup", function () {
  describe("controllerIntegrationSuite", function () {
    const params = new controllerIntegrationSuiteParameters(10_000_000, 50_000);
    controllerIntegrationSuite(authority, bank, controllerUXD, params);
  });

  describe("mangoDepositorySetupSuite", function () {
    mangoDepositorySetupSuite(authority, bank, controllerUXD, mangoDepositorySOL, 1_000);
  });
});
