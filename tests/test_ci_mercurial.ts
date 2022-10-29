import { Signer, Keypair } from "@solana/web3.js";
import { findATAAddrSync } from "@uxd-protocol/uxd-client";
import { Controller, MercurialVaultDepository, UXD_DECIMALS } from "@uxd-protocol/uxd-client";
import { editControllerTest } from "./cases/editControllerTest";
import { getConnection } from "./connection";
import { authority, bank, SOLEND_USDC_DEVNET, SOLEND_USDC_DEVNET_DECIMALS, uxdProgramId } from "./constants";
import { controllerIntegrationSuiteParameters, controllerIntegrationSuite } from "./suite/controllerIntegrationSuite";
import { editMercurialVaultDepositorySuite } from "./suite/editMercurialVaultDepositorySuite";
import { mercurialVaultDepositoryMintRedeemSuite } from "./suite/mercurialVaultMintAndRedeemSuite";
import { transferSol, transferAllSol, transferAllTokens, getBalance } from "./utils";

(async () => {
  const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

  beforeEach("\n", function () {
    console.log("=============================================\n\n");
  });

  it("Set controller global supply cap to 25mm", async function () {
    await editControllerTest(authority, controllerUXD, {
      redeemableGlobalSupplyCap: 25_000_000,
    });
  });

  let user: Signer = new Keypair();

  let mercurialVaultDepository = await MercurialVaultDepository.initialize({
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

    describe("mercurialVaultDepositoryMintRedeemSuite", function () {
      mercurialVaultDepositoryMintRedeemSuite(authority, user, bank, controllerUXD, mercurialVaultDepository);
    });

    describe("editMercurialVaultDepositorySuite", function () {
      editMercurialVaultDepositorySuite(authority, user, bank, controllerUXD, mercurialVaultDepository);
    });

    this.afterAll("Transfer funds back to bank", async function () {
      await transferAllTokens(SOLEND_USDC_DEVNET, SOLEND_USDC_DEVNET_DECIMALS, user, bank.publicKey);
      await transferAllSol(user, bank.publicKey);
    });
  });
})();
