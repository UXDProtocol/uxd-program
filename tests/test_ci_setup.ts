import { IdentityDepository, MercurialVaultDepository, USDC_DECIMALS, USDC_DEVNET } from "@uxd-protocol/uxd-client";
import { Controller, UXD_DECIMALS } from "@uxd-protocol/uxd-client";
import { getConnection } from "./connection";
import { authority, bank, SOLEND_USDC_DEVNET, SOLEND_USDC_DEVNET_DECIMALS, uxdProgramId } from "./constants";
import { controllerIntegrationSuiteParameters, controllerIntegrationSuite } from "./suite/controllerIntegrationSuite";
import { identityDepositorySetupSuite } from "./suite/identityDepositorySetup";
import { mercurialVaultDepositorySetupSuite } from "./suite/mercurialVaultDepositorySetup";

(async () => {
  const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

  beforeEach("\n", function () {
    console.log("=============================================\n\n");
  });

  describe("controllerIntegrationSuite", function () {
    const params = new controllerIntegrationSuiteParameters(25_000_000);
    controllerIntegrationSuite(authority, bank, controllerUXD, params);
  });

  const mercurialVaultDepository = await MercurialVaultDepository.initialize({
    connection: getConnection(),
    collateralMint: {
      mint: SOLEND_USDC_DEVNET,
      name: "USDC",
      symbol: "USDC",
      decimals: SOLEND_USDC_DEVNET_DECIMALS,
    },
    uxdProgramId,
  });

  const mintingFeeInBps = 0;
  const redeemingFeeInBps = 5;
  const uiRedeemableDepositorySupplyCap = 1_000;

  describe("mercurialVaultDepositorySetupSuite", function () {
    mercurialVaultDepositorySetupSuite(
      authority,
      authority.publicKey,
      bank,
      controllerUXD,
      mercurialVaultDepository,
      mintingFeeInBps,
      redeemingFeeInBps,
      uiRedeemableDepositorySupplyCap
    );
  });

  const identityDepository = new IdentityDepository(USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);

  describe("identityDepositorySetupSuite", function () {
    identityDepositorySetupSuite(authority, bank, controllerUXD, identityDepository);
  });
})();
