import { Keypair, PublicKey, Signer } from "@solana/web3.js";
import {
  Controller,
  UXD_DECIMALS,
  MercurialVaultDepository,
  IdentityDepository,
  USDC_DEVNET,
  USDC_DECIMALS,
} from "@uxd-protocol/uxd-client";
import { authority, bank, uxdProgramId } from "./constants";
import { transferAllSol, transferAllTokens, transferSol } from "./utils";
import { initializeControllerTest } from "./cases/initializeControllerTest";
import { identityDepositorySetupSuite } from "./suite/identityDepositorySetup";
import { identityDepositoryMintRedeemSuite } from "./suite/identityDepositoryMintAndRedeemSuite";
import { editIdentityDepositoryTest } from "./cases/editIdentityDepositoryTest";
import { editIdentityDepositorySuite } from "./suite/editIdentityDepositorySuite";

console.log(uxdProgramId.toString());

const controller = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
const payer = bank;

const SOLEND_USDC_DEVNET = new PublicKey("6L9fgyYtbz34JvwvYyL6YzJDAywz9PKGttuZuWyuoqje");
const SOLEND_USDC_DEVNET_DECIMALS = 6;

// Do not create the vault. We are building an object with utilities methods.
let mercurialVaultDepositoryUSDC: MercurialVaultDepository = null;
let identityDepository: IdentityDepository = null;

let mintedRedeemableAmountWithMercurialVaultDepository = 0;

beforeEach("\n", function () {
  console.log("=============================================\n\n");
});

describe("Integration tests", function () {
  const user: Signer = new Keypair();

  this.beforeAll("Init and fund user (10 SOL and 100 usdc)", async function () {
    console.log("USER =>", user.publicKey.toString());

    await transferSol(1, bank, user.publicKey);
    // await transferTokens(0.001, SOLEND_USDC_DEVNET, SOLEND_USDC_DEVNET_DECIMALS, bank, user.publicKey);
  });

  describe("Init", async function () {
    it("Initialize Controller", async function () {
      await initializeControllerTest(authority, controller, payer);
    });

    //   it(`Initialize and register Mercurial USDC vault depository`, async function () {
    //     mercurialVaultDepositoryUSDC = await MercurialVaultDepository.initialize({
    //       connection: getConnection(),
    //       collateralMint: {
    //         mint: SOLEND_USDC_DEVNET,
    //         decimals: SOLEND_USDC_DEVNET_DECIMALS,
    //         symbol: "USDC",
    //         name: "USDC",
    //       },
    //       uxdProgramId,
    //     });

    //     const mintingFeeInBps = 2;
    //     const redeemingFeeInBps = 2;
    //     const redeemableAmountUnderManagementCap = 1_000;

    //     await registerMercurialVaultDepositoryTest(
    //       authority,
    //       controller,
    //       mercurialVaultDepositoryUSDC,
    //       mintingFeeInBps,
    //       redeemingFeeInBps,
    //       redeemableAmountUnderManagementCap,
    //       payer
    //     );
    //   });
    // });

    // describe("Regular Mint/Redeem with Mercurial Vault USDC Depository", async function () {
    //   it(`Mint for 0.001 USDC`, async function () {
    //     mintedRedeemableAmountWithMercurialVaultDepository = await mintWithMercurialVaultDepositoryTest(
    //       0.001,
    //       user,
    //       controller,
    //       mercurialVaultDepositoryUSDC,
    //       payer
    //     );
    //   });

    //   it(`Redeem all previously minted redeemable`, async function () {
    //     console.log(`Redeem for ${mintedRedeemableAmountWithMercurialVaultDepository} UXD`);

    //     await redeemFromMercurialVaultDepositoryTest(
    //       mintedRedeemableAmountWithMercurialVaultDepository,
    //       user,
    //       controller,
    //       mercurialVaultDepositoryUSDC,
    //       payer
    //     );
    //   });
    // });
  });

  describe("Initialize Identity depository", async function () {
    identityDepository = new IdentityDepository(USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
    await editIdentityDepositoryTest(authority, controller, identityDepository, {
      mintingDisabled: false,
    });
    identityDepositorySetupSuite(authority, bank, controller, identityDepository);
  });

  describe("Edit Identity depository test suite", function () {
    editIdentityDepositorySuite(authority, user, bank, controller, identityDepository);
  });

  describe("Mint/Redeem Identity depository test suite ", function () {
    identityDepositoryMintRedeemSuite(authority, user, bank, controller, identityDepository);
  });

  // describe("Regular Mint/Redeem with Mercurial Vault USDC Depository", async function () {
  //   it(`Mint for 0.001 USDC`, async function () {
  //     mintedRedeemableAmountWithMercurialVaultDepository = await mintWithMercurialVaultDepositoryTest(
  //       0.001,
  //       user,
  //       controller,
  //       mercurialVaultDepositoryUSDC,
  //       payer
  //     );
  //   });

  //   it(`Redeem all previously minted redeemable`, async function () {
  //     console.log(`Redeem for ${mintedRedeemableAmountWithMercurialVaultDepository} UXD`);

  //     await redeemFromMercurialVaultDepositoryTest(
  //       mintedRedeemableAmountWithMercurialVaultDepository,
  //       user,
  //       controller,
  //       mercurialVaultDepositoryUSDC,
  //       payer
  //     );
  //   });
  // });

  this.afterAll("Transfer funds back to bank", async function () {
    await transferAllTokens(USDC_DEVNET, USDC_DECIMALS, authority, bank.publicKey);
    await transferAllSol(user, bank.publicKey);
  });
});
