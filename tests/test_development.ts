import { Keypair, PublicKey, Signer } from "@solana/web3.js";
import {
  Controller,
  MangoDepository,
  SOL_DECIMALS,
  USDC_DECIMALS,
  UXD_DECIMALS,
  WSOL,
  USDC_DEVNET,
  BTC_DECIMALS,
  BTC_DEVNET,
  MercurialVaultDepository,
  MaplePoolDepository,
} from "@uxd-protocol/uxd-client";
import {
  authority,
  bank,
  MAPLE_USDC_DEVNET,
  MAPLE_USDC_DEVNET_DECIMALS,
  slippageBase,
  SOLEND_USDC_DEVNET,
  SOLEND_USDC_DEVNET_DECIMALS,
  uxdProgramId,
} from "./constants";
import {
  createMaplePoolDepositoryDevnetUSDC,
  printDepositoryInfo,
  printUserInfo,
  transferAllSol,
  transferAllTokens,
  transferSol,
  transferTokens,
} from "./utils";
import { depositInsuranceMangoDepositoryTest } from "./cases/depositInsuranceMangoDepositoryTest";
import { registerMangoDepositoryTest } from "./cases/registerMangoDepositoryTest";
import { mango } from "./fixtures";
import { withdrawInsuranceMangoDepositoryTest } from "./cases/withdrawInsuranceMangoDepositoryTest";
import { mintWithMangoDepositoryTest } from "./cases/mintWithMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "./cases/redeemFromMangoDepositoryTest";
import { initializeControllerTest } from "./cases/initializeControllerTest";
import {
  MangoDepositoryRebalancingSuiteParameters,
  mangoDepositoryRebalancingSuite,
} from "./suite/mangoDepositoryRebalancingSuite";
import { quoteMintAndRedeemSuite } from "./suite/quoteMintAndRedeemSuite";
import { editMaplePoolDepository, setMangoDepositoriesRedeemableSoftCap } from "./api";
import { getConnection } from "./connection";
import { registerMercurialVaultDepositoryTest } from "./cases/registerMercurialVaultDepositoryTest";
import { mintWithMercurialVaultDepositoryTest } from "./cases/mintWithMercurialVaultDepositoryTest";
import { redeemFromMercurialVaultDepositoryTest } from "./cases/redeemFromMercurialVaultDepositoryTest";
import { registerMaplePoolDepositoryTest } from "./cases/registerMaplePoolDepositoryTest";
import { uiToNative } from "@blockworks-foundation/mango-client";
import { mintWithMaplePoolDepositoryTest } from "./cases/mintWithMaplePoolDepositoryTest";
import { editMangoDepositoryTest } from "./cases/editMangoDepositoryTest";
import { editMaplePoolDepositoryTest } from "./cases/editMaplePoolDepositoryTest";

console.log(uxdProgramId.toString());
const mangoDepositorySOL = new MangoDepository(
  WSOL,
  "SOL",
  SOL_DECIMALS,
  USDC_DEVNET,
  "USDC",
  USDC_DECIMALS,
  uxdProgramId
);
// const mangoDepositoryBTC = new MangoDepository(BTC_DEVNET, "BTC", BTC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
// const mangoDepositoryETH = new MangoDepository(ETH_DEVNET, "ETH", ETH_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const controller = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
const payer = bank;
const slippage = 50; // 5%

// Do not create the vault. We are building an object with utilities methods.
let mercurialVaultDepositoryUSDC: MercurialVaultDepository = null;

// Do not create the maple pool depository. We are building an object with utilities methods.
let maplePoolDepository: MaplePoolDepository = null;

let mintedRedeemableAmountWithMercurialVaultDepository = 0;

// console.log(`SOL 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositorySOL.mangoAccountPda}'`);

beforeEach("\n", function () {
  console.log("=============================================\n\n");
});

describe("Integration tests", function () {
  const user: Signer = new Keypair();

  this.beforeAll("Init and fund user (10 SOL and 100 usdc)", async function () {
    console.log("USER =>", user.publicKey.toString());

    await transferSol(10, bank, user.publicKey);
    await transferTokens(200, USDC_DEVNET, USDC_DECIMALS, bank, user.publicKey);
    await transferTokens(0.001, SOLEND_USDC_DEVNET, SOLEND_USDC_DEVNET_DECIMALS, bank, user.publicKey);
    await transferTokens(0.001, MAPLE_USDC_DEVNET, MAPLE_USDC_DEVNET_DECIMALS, bank, user.publicKey);
  });

  describe("Init", async function () {
    it("Initialize Controller", async function () {
      await initializeControllerTest(authority, controller, payer);
    });

    /*
    it(`Initialize and register mercurial USDC vault depository`, async function () {
      mercurialVaultDepositoryUSDC = await MercurialVaultDepository.initialize({
        connection: getConnection(),
        collateralMint: {
          mint: SOLEND_USDC_DEVNET,
          decimals: SOLEND_USDC_DEVNET_DECIMALS,
          symbol: "USDC",
          name: "USDC",
        },
        uxdProgramId,
        cluster: "devnet",
      });

      const mintingFeeInBps = 2;
      const redeemingFeeInBps = 2;
      const redeemableDepositorySupplyCap = 1_000_000_000;

      await registerMercurialVaultDepositoryTest(
        authority,
        controller,
        mercurialVaultDepositoryUSDC,
        mintingFeeInBps,
        redeemingFeeInBps,
        redeemableDepositorySupplyCap,
        payer
      );
    });
    */

    it(`Initialize and register maple pool depository (credora?)`, async function () {
      maplePoolDepository = await createMaplePoolDepositoryDevnetUSDC();

      const uiRedeemableAmountUnderManagementCap = 1000;
      const mintingFeeInBps = 0;
      const redeemingFeeInBps = 0;

      await registerMaplePoolDepositoryTest(
        authority,
        controller,
        maplePoolDepository,
        uiRedeemableAmountUnderManagementCap,
        mintingFeeInBps,
        redeemingFeeInBps,
        payer
      );
    });

    it(`Reset the depository fee and cap in case it was already modified by another test`, async function () {
      await editMaplePoolDepositoryTest(authority, controller, maplePoolDepository, {
        redeemableAmountUnderManagementCap: 1_000,
        mintingFeeInBps: 0,
      });
    });

    it(`Mint for 0.0001 fake USDC with no fees`, async function () {
      await mintWithMaplePoolDepositoryTest(0.0001, user, controller, maplePoolDepository, payer);
    });

    it(`Set minting fee to 200 bps`, async function () {
      await editMaplePoolDepositoryTest(authority, controller, maplePoolDepository, {
        mintingFeeInBps: 200,
      });
    });

    it(`Mint for 0.0001 fake USDC with fees`, async function () {
      await mintWithMaplePoolDepositoryTest(0.0001, user, controller, maplePoolDepository, payer);
    });

    it(`Set cap to 0`, async function () {
      await editMaplePoolDepositoryTest(authority, controller, maplePoolDepository, {
        redeemableAmountUnderManagementCap: 0,
      });
    });

    it(`Mint for 0.0001 fake USDC`, async function () {
      await mintWithMaplePoolDepositoryTest(0.0001, user, controller, maplePoolDepository, payer);
    });

    /*
    it(`Initialize ${mangoDepositorySOL.collateralMintSymbol} Depository`, async function () {
      const redeemableDepositorySupplyCap = 1_000;

      await registerMangoDepositoryTest(
        authority,
        controller,
        mangoDepositorySOL,
        mango,
        redeemableDepositorySupplyCap,
        payer
      );
    });

    it(`Deposit 100 USDC of insurance`, async function () {
      await depositInsuranceMangoDepositoryTest(100, authority, controller, mangoDepositorySOL, mango);
    });

    it("Increase mango depositories redeemable soft cap to 10_000_000", async function () {
      await setMangoDepositoriesRedeemableSoftCap(authority, controller, 10_000_000);
    });

    it("Mint 1 SOL", async function () {
      await mintWithMangoDepositoryTest(1, slippage, user, controller, mangoDepositorySOL, mango, payer);
    });

    it(`Withdraw 10 USDC of insurance`, async function () {
      await withdrawInsuranceMangoDepositoryTest(10, authority, controller, mangoDepositorySOL, mango);
    });
    */
  });

  /*
  describe("Regular Mint/Redeem with Mercurial Vault USDC Depository", async function () {
    it(`Mint for 0.001 USDC`, async function () {
      mintedRedeemableAmountWithMercurialVaultDepository = await mintWithMercurialVaultDepositoryTest(
        0.001,
        user,
        controller,
        mercurialVaultDepositoryUSDC,
        payer
      );
    });

    it(`Redeem all previously minted redeemable`, async function () {
      console.log(`Redeem for ${mintedRedeemableAmountWithMercurialVaultDepository} UXD`);

      await redeemFromMercurialVaultDepositoryTest(
        mintedRedeemableAmountWithMercurialVaultDepository,
        user,
        controller,
        mercurialVaultDepositoryUSDC,
        payer
      );
    });
  });

  describe("Quote Mint And Redeem Suite", async function () {
    quoteMintAndRedeemSuite(authority, user, payer, controller, mangoDepositorySOL);
  });

  describe("Test minting/redeeming SOL", async function () {
    it(`Mint 10 ${controller.redeemableMintSymbol} then redeem the outcome (${
      (slippage / slippageBase) * 100
    } % slippage)`, async function () {
      const perpPrice = await mangoDepositorySOL.getCollateralPerpPriceUI(mango);
      const amount = 10 / perpPrice;
      console.log("[🧾 amount", amount, mangoDepositorySOL.collateralMintSymbol, "]");
      const mintedAmount = await mintWithMangoDepositoryTest(
        amount,
        slippage,
        user,
        controller,
        mangoDepositorySOL,
        mango,
        payer
      );
      await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, mangoDepositorySOL, mango, payer);
    });

    it(`Mint twice min mint trading size, then redeem them (${
      (slippage / slippageBase) * 100
    }% slippage)`, async function () {
      const minRedeemAmount = await mangoDepositorySOL.getMinRedeemSizeQuoteUI(mango);
      const minTradingSize = await mangoDepositorySOL.getMinTradingSizeCollateralUI(mango);

      await mintWithMangoDepositoryTest(
        minTradingSize * 2,
        slippage,
        user,
        controller,
        mangoDepositorySOL,
        mango,
        payer
      );
      await redeemFromMangoDepositoryTest(
        minRedeemAmount,
        slippage,
        user,
        controller,
        mangoDepositorySOL,
        mango,
        payer
      );
    });
  });

  // Note - Keep a mint/redeem before rebalancing so that it creates the necessary accounts for computing
  describe.skip("mangoDepositoryRebalancingSuite SOL", function () {
    const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(slippage);
    mangoDepositoryRebalancingSuite(user, bank, controller, mangoDepositorySOL, paramsRebalancing);
  });

  describe.skip("info SOL", async function () {
    it("info", async function () {
      await printUserInfo(user.publicKey, controller, mangoDepositorySOL);
      await printDepositoryInfo(controller, mangoDepositorySOL, mango);
    });
  });
  */

  this.afterAll("Transfer funds back to bank", async function () {
    await transferAllTokens(USDC_DEVNET, USDC_DECIMALS, user, bank.publicKey);
    await transferAllTokens(BTC_DEVNET, BTC_DECIMALS, user, bank.publicKey);
    await transferAllSol(user, bank.publicKey);
  });
});
