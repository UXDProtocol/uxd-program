import { Keypair, PublicKey, Signer } from "@solana/web3.js";
import {
  Controller,
  UXD_DECIMALS,
  MercurialVaultDepository,
  IdentityDepository,
  USDC_DEVNET,
  USDC_DECIMALS,
  MangoDepository,
  SOL_DECIMALS,
  WSOL,
  nativeToUi,
} from "@uxd-protocol/uxd-client";
import { authority, bank, uxdProgramId } from "./constants";
import { transferAllSol, transferAllTokens, transferSol, transferTokens } from "./utils";
import { initializeControllerTest } from "./cases/initializeControllerTest";
import { getConnection, TXN_OPTS } from "./connection";
import { identityDepositorySetupSuite } from "./suite/identityDepositorySetup";
import { reinjectMangoToIdentityDepositoryTest } from "./cases/reinjectMangoToIdentityDepositoryTest";

console.log(uxdProgramId.toString());

const controller = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
const payer = bank;

const SOLEND_USDC_DEVNET = new PublicKey("zVzi5VAf4qMEwzv7NXECVx5v2pQ7xnqVVjCXZwS9XzA");
const SOLEND_USDC_DEVNET_DECIMALS = 6;

// Do not create the vault. We are building an object with utilities methods.
let mercurialVaultDepositoryUSDC: MercurialVaultDepository = null;
let identityDepository: IdentityDepository = null;
let mangoDepositorySOL: MangoDepository = null;

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

  describe("Initialize Identity depository", function () {
    identityDepository = new IdentityDepository(USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
    identityDepositorySetupSuite(authority, bank, controller, identityDepository);
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

  describe("Reinject mango depository managed redeemable amount to identity depository", async function () {
    mangoDepositorySOL = new MangoDepository(
      WSOL,
      "SOL",
      SOL_DECIMALS,
      USDC_DEVNET,
      "USDC",
      USDC_DECIMALS,
      uxdProgramId
    );

    it(`Reinject ${mangoDepositorySOL.collateralMintSymbol} mango depository`, async function () {
      // transfer enough identity collateral token to user for reinjection
      const mangoDepositoryOnChainAccount = await mangoDepositorySOL.getOnchainAccount(getConnection(), TXN_OPTS);
      const redeemableAmountUnderManagementUI = nativeToUi(
        mangoDepositoryOnChainAccount.redeemableAmountUnderManagement,
        controller.redeemableMintDecimals
      );

      await transferTokens(
        redeemableAmountUnderManagementUI,
        identityDepository.collateralMint,
        identityDepository.collateralMintDecimals,
        payer,
        user.publicKey
      );

      await reinjectMangoToIdentityDepositoryTest(user, bank, controller, identityDepository, mangoDepositorySOL);
    });
  });

  this.afterAll("Transfer funds back to bank", async function () {
    await transferAllTokens(USDC_DEVNET, USDC_DECIMALS, user, bank.publicKey);
    await transferAllSol(user, bank.publicKey);
  });
});
