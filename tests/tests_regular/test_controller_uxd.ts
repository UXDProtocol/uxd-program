import { Controller, UXD_DECIMALS } from "@uxdprotocol/uxd-client";
import { authority, uxdProgramId } from "../constants";
import { controllerSuiteParameters, controllerIntegrationSuite } from "../suite/controllerIntegrationSuite";

describe("UXD Controller Tests", () => {
    const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
    const params = new controllerSuiteParameters(10_000_000, 500_000);

    controllerIntegrationSuite(authority, controllerUXD, params);
});