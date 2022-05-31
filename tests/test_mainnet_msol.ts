import { Keypair, Signer } from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js";
import { Controller, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, WSOL } from "@uxd-protocol/uxd-client";
import { UXD_DECIMALS } from "@uxd-protocol/uxd-client";
import { initializeControllerTest } from "./cases/initializeControllerTest";
import { registerMangoDepositoryTest } from "./cases/registerMangoDepositoryTest";
import { mango } from "./fixtures";
import * as payerKeypair from "../../../../.config/solana/id.json";
import * as adminKeypair from "../../internal_mainnet_authority.json";
import { uxdProgramId } from "./constants";

const payer: Signer = Keypair.fromSecretKey(Uint8Array.from(payerKeypair.default));
console.log(`PAYER MAINNET => 🔗https://solscan.io/account/${payer.publicKey}`);

const authority: Signer = Keypair.fromSecretKey(Uint8Array.from(adminKeypair.default));
console.log(`CONTROLLER AUTHORITY MAINNET => 🔗 https://solscan.io/account/${authority.publicKey}`);

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

describe("Mainnet Integration tests SOL", function () {
  it("Initialize Controller (internal mainnet)", async function () {
    await initializeControllerTest(authority, controller, payer);
  });

  it(`Initialize ${mangoDepositorySOL.collateralMintSymbol} Depository`, async function () {
    await registerMangoDepositoryTest(authority, controller, mangoDepositorySOL, mango, payer);
  });
});
