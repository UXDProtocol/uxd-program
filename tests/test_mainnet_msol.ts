import { Keypair, Signer, Transaction } from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js";
import {
  Controller,
  createAssocTokenIx,
  findATAAddrSync,
  MangoDepository,
  MSOL,
  MsolConfig,
  SOL_DECIMALS,
  USDC_DECIMALS,
  WSOL,
} from "@uxd-protocol/uxd-client";
import { UXD_DECIMALS } from "@uxd-protocol/uxd-client";
import { initializeControllerTest } from "./cases/initializeControllerTest";
import { registerMangoDepositoryTest } from "./cases/registerMangoDepositoryTest";
import { mango } from "./fixtures";
import * as payerKeypair from "../../../../.config/solana/id.json";
import * as adminKeypair from "../../internal_mainnet_authority.json";
import * as userKeypair from "../../internal_mainnet_user_keypair.json";
import { uxdProgramId } from "./constants";
import { depositInsuranceMangoDepositoryTest } from "./cases/depositInsuranceMangoDepositoryTest";
import { transferAllSol, transferAllTokens, transferSol } from "./utils";
import { mintWithMangoDepositoryTest } from "./cases/mintWithMangoDepositoryTest";
import { web3 } from "@project-serum/anchor";
import { getConnection, TXN_OPTS } from "./connection";
import { createAta, createDepositoryMsolConfig, enableMsolSwap, swapDepositoryMsol } from "./api";
import { redeemFromMangoDepositoryTest } from "./cases/redeemFromMangoDepositoryTest";
import { setRedeemableSoftCapMangoDepositoryTest } from "./cases/setRedeemableSoftCapMangoDepositoryTest";

const payer: Signer = Keypair.fromSecretKey(Uint8Array.from(payerKeypair.default));
console.log(`PAYER MAINNET => 🔗https://solscan.io/account/${payer.publicKey}`);

// 8cJ5KH2ExX2rrY6DbzAqrBMDkQxYZfyedB1C4L4osc5N
const authority: Signer = Keypair.fromSecretKey(Uint8Array.from(adminKeypair.default));
console.log(`CONTROLLER AUTHORITY MAINNET => 🔗 https://solscan.io/account/${authority.publicKey}`);

// BjsGycpLGSFmUD2PbFBjrKahXjNnRxYBQMAEsBF3uJxb
const user: Signer = Keypair.fromSecretKey(Uint8Array.from(userKeypair.default));
console.log(`USER MAINNET => 🔗 https://solscan.io/account/${user.publicKey}`);

const USDC_MAINNET = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

const controller = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

const mangoDepositorySOL = new MangoDepository(
  WSOL,
  "SOL",
  SOL_DECIMALS,
  USDC_MAINNET,
  "USDC",
  USDC_DECIMALS,
  uxdProgramId
);

const msolConfig = new MsolConfig(mangoDepositorySOL.pda, uxdProgramId);
console.log(`msolConfigPda = ${msolConfig.pda.toString()}`);

describe.skip("Mainnet token transfer", function () {
  it.skip("Transfer all USDC to authority from payer", async function () {
    const txId = await transferAllTokens(USDC_MAINNET, USDC_DECIMALS, payer, authority.publicKey);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}'`);
  });

  it.skip("Transfer SOL to authority from payer", async function () {
    const txId = await transferSol(0.1, payer, authority.publicKey);
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}'`);
  });
});

describe("Mainnet Integration tests SOL", function () {
  this.beforeAll("Init and fund user", async function () {
    const uiAmount = 0.003;
    const txId = await transferSol(uiAmount, payer, user.publicKey);
    console.log("transfer", uiAmount, "SOL to", user.publicKey.toString());
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}'`);
  });

  // has initialized!
  it.skip("Initialize Controller (internal mainnet)", async function () {
    await initializeControllerTest(authority, controller, payer);
  });

  // has initialized!
  it.skip(`Initialize ${mangoDepositorySOL.collateralMintSymbol} Depository (internal mainnet)`, async function () {
    await registerMangoDepositoryTest(authority, controller, mangoDepositorySOL, mango, payer);
  });

  it.skip(`Deposit to insurance`, async function () {
    // just transfer all the usdc to depository mango account
    await depositInsuranceMangoDepositoryTest(23.300486, authority, controller, mangoDepositorySOL, mango);
  });

  it.skip(`Mint 0.01 ${controller.redeemableMintSymbol} for 2% slippage)`, async function () {
    await mintWithMangoDepositoryTest(0.01, 20, user, controller, mangoDepositorySOL, mango, payer);
  });

  it.skip(`Set Mango Depositories Redeemable soft cap to 1`, async function () {
    await setRedeemableSoftCapMangoDepositoryTest(100000000, authority, controller);
  });

  it(`Redeem 0.5 ${controller.redeemableMintSymbol} for 2% slippage)`, async function () {
    await redeemFromMangoDepositoryTest(0.5, 20, user, controller, mangoDepositorySOL, mango, payer);
  });

  describe("Test mSOL", async function () {
    it("register msol config", async function () {
      if (await getConnection().getAccountInfo(msolConfig.pda)) {
        console.log("🚧 Already initialized.");
      } else {
        const txId = await createDepositoryMsolConfig(
          authority,
          payer,
          controller,
          mangoDepositorySOL,
          msolConfig.pda,
          5000
        );
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}'`);
      }
    });

    it("create wsol ata", async function () {
      try {
        const txId = await createAta(user, payer, WSOL);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}'`);
      } catch (e) {
        console.error(e);
      }
    });

    it("create msol ata", async function () {
      try {
        const txId = await createAta(user, payer, MSOL);
        console.log(`🔗 'https://explorer.solana.com/tx/${txId}'`);
      } catch (e) {
        console.error(e);
      }
    });

    it.skip("enable msol config", async function () {
      const txId = await enableMsolSwap(authority, payer, controller, mangoDepositorySOL, msolConfig.pda, true);
      console.log(`🔗 'https://explorer.solana.com/tx/${txId}'`);
    });

    it("swap msol", async function () {
      const txId = await swapDepositoryMsol(user, payer, controller, mangoDepositorySOL, msolConfig.pda, mango);
      console.log(`🔗 'https://explorer.solana.com/tx/${txId}'`);
    });
  });

  this.afterAll("Transfer funds back to payer", async function () {
    const txId = await transferAllSol(user, payer.publicKey);
    console.log("transfer all SOL to", payer.publicKey.toString());
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}'`);
  });
});
