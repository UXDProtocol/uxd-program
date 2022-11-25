import { Keypair, Signer } from "@solana/web3.js";
import { Controller, UXD_DECIMALS, MercurialVaultDepository } from "@uxd-protocol/uxd-client";
import { authority, bank, SOLEND_USDC_DEVNET, SOLEND_USDC_DEVNET_DECIMALS, uxdProgramId } from "./constants";
import { transferAllSol, transferAllTokens, transferSol } from "./utils";
import { controllerIntegrationSuite, controllerIntegrationSuiteParameters } from "./suite/controllerIntegrationSuite";
import { mercurialVaultDepositorySetupSuite } from "./suite/mercurialVaultDepositorySetup";
import { mercurialVaultDepositoryMintRedeemSuite } from "./suite/mercurialVaultMintAndRedeemSuite";
import { getConnection } from "./connection";
import { editMercurialVaultDepositorySuite } from "./suite/editMercurialVaultDepositorySuite";
import { mercurialVaultDepositoryCollectProfitSuite } from "./suite/mercurialVaultCollectProfitSuite";

(async () => {
  const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

  beforeEach("\n", function () {
    console.log("=============================================\n\n");
  });

  describe("UXD Controller Suite", function () {
    const params = new controllerIntegrationSuiteParameters(25_000_000);
    controllerIntegrationSuite(authority, bank, controllerUXD, params);
  });

  let user: Signer = new Keypair();

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

  describe("Mercurial vault integration tests: USDC", async function () {
    this.beforeAll("Setup: fund user", async function () {
      console.log("USER =>", user.publicKey.toString());
      await transferSol(1, bank, user.publicKey);
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

    describe("mercurialVaultDepositoryMintRedeemSuite", function () {
      mercurialVaultDepositoryMintRedeemSuite(authority, user, bank, controllerUXD, mercurialVaultDepository);
    });

    describe("editMercurialVaultDepositorySuite", function () {
      editMercurialVaultDepositorySuite(authority, user, bank, controllerUXD, mercurialVaultDepository);
    });

    describe("mercurialVaultDepositoryCollectProfitSuite", function () {
      mercurialVaultDepositoryCollectProfitSuite(authority, authority, bank, controllerUXD, mercurialVaultDepository);
    });

    this.afterAll("Transfer funds back to bank", async function () {
      await transferAllTokens(SOLEND_USDC_DEVNET, SOLEND_USDC_DEVNET_DECIMALS, user, bank.publicKey);
      await transferAllSol(user, bank.publicKey);
    });
  });
})();
