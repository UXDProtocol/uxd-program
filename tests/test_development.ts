import { Keypair, Signer } from "@solana/web3.js";
import {
  Controller,
  UXD_DECIMALS,
  MercurialVaultDepository,
  IdentityDepository,
  USDC_DEVNET,
  USDC_DECIMALS,
} from "@uxd-protocol/uxd-client";
import { authority, bank, SOLEND_USDC_DEVNET, SOLEND_USDC_DEVNET_DECIMALS, uxdProgramId } from "./constants";
import { transferAllSol, transferAllTokens, transferSol, transferTokens } from "./utils";
import { getConnection } from "./connection";
import { freezeProgramSuite } from "./suite/freezeProgramSuite";
import { editControllerTest } from "./cases/editControllerTest";

(async () => {
  const controller = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

  beforeEach("\n", function () {
    console.log("=============================================\n\n");
  });

  it("Set controller global supply cap to 25mm", async function () {
    await editControllerTest(authority, controller, {
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

  const identityDepository = new IdentityDepository(USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);

  describe("Integration tests", function () {
    this.beforeAll("Init and fund user (10 SOL and 100 usdc)", async function () {
      console.log("USER =>", user.publicKey.toString());

      await transferSol(1, bank, user.publicKey);
      await transferTokens(10, USDC_DEVNET, USDC_DECIMALS, bank, user.publicKey);
      await transferTokens(10, SOLEND_USDC_DEVNET, SOLEND_USDC_DEVNET_DECIMALS, bank, user.publicKey);
    });

    describe("freezeProgramSuite", async function () {
      await freezeProgramSuite(authority, user, bank, controller, mercurialVaultDepository, identityDepository);
    });

    this.afterAll("Transfer funds back to bank", async function () {
      await transferAllTokens(USDC_DEVNET, USDC_DECIMALS, authority, bank.publicKey);
      await transferAllTokens(SOLEND_USDC_DEVNET, SOLEND_USDC_DEVNET_DECIMALS, authority, bank.publicKey);
      await transferAllSol(user, bank.publicKey);
    });
  });
})();
