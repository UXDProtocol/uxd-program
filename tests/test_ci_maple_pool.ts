import { Signer, Keypair } from "@solana/web3.js";
import { Controller, UXD_DECIMALS } from "@uxd-protocol/uxd-client";
import { editControllerTest } from "./cases/editControllerTest";
import { authority, bank, uxdProgramId } from "./constants";
import { maplePoolDepositoryEditSuite } from "./suite/maplePoolDepositoryEditSuite";
import { maplePoolDepositoryMintSuite } from "./suite/maplePoolDepositoryMintSuite";
import {
  transferSol,
  transferAllSol,
  transferAllTokens,
  createMaplePoolDepositoryDevnetUSDC,
  transferTokens,
} from "./utils";

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

  const user: Signer = new Keypair();

  const maplePoolDepository = await createMaplePoolDepositoryDevnetUSDC();
  const collateralMint = maplePoolDepository.collateralMint;
  const collateralDecimals = maplePoolDepository.collateralDecimals;

  describe("Mercurial vault integration tests: USDC", async function () {
    this.beforeAll("Setup: fund user", async function () {
      await transferSol(1, bank, user.publicKey);
      await transferTokens(0.1, collateralMint, collateralDecimals, bank, user.publicKey);
    });

    describe("maplePoolDepositoryEditSuite", function () {
      maplePoolDepositoryEditSuite(authority, user, bank, controllerUXD, maplePoolDepository);
    });

    describe("maplePoolDepositoryMintSuite", function () {
      maplePoolDepositoryMintSuite(authority, user, bank, controllerUXD, maplePoolDepository);
    });

    this.afterAll("Transfer funds back to bank", async function () {
      await transferAllTokens(collateralMint, collateralDecimals, user, bank.publicKey);
      await transferAllSol(user, bank.publicKey);
    });
  });
})();
