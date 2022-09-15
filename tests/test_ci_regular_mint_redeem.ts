import { Keypair, Signer } from "@solana/web3.js";
import {
  Controller,
  MangoDepository,
  SOL_DECIMALS,
  USDC_DECIMALS,
  UXD_DECIMALS,
  WSOL,
  USDC_DEVNET,
} from "@uxd-protocol/uxd-client";
import { authority, bank, uxdProgramId } from "./constants";
import { transferAllSol, transferSol } from "./utils";
import {
  MangoDepositoryAndControllerInteractionsSuiteParameters,
  mangoDepositoryAndControllerInteractionsSuite,
} from "./suite/mangoDepositoryAndControllerInteractionsSuite";
import { mangoDepositoryInsuranceSuite } from "./suite/depositoryInsuranceSuite";
import { mangoDepositoryMintRedeemSuite } from "./suite/mangoDepositoryMintRedeemSuite";

const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
const mangoDepositorySOL = new MangoDepository(
  WSOL,
  "SOL",
  SOL_DECIMALS,
  USDC_DEVNET,
  "USDC",
  USDC_DECIMALS,
  uxdProgramId
);

console.log(`SOL 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositorySOL.mangoAccountPda}'`);

beforeEach("\n", function () {
  console.log("=============================================\n\n");
});

// SOL
describe("Integration tests SOL", function () {
  const user: Signer = new Keypair();

  this.beforeAll("Init and fund user", async function () {
    console.log("USER =>", user.publicKey.toString());
    await transferSol(1, bank, user.publicKey);
  });

  describe("mangoDepositoryAndControllerInteractionsSuite", function () {
    const paramsSol = new MangoDepositoryAndControllerInteractionsSuiteParameters(10_000_000, 500, 50_000, 500, 20);
    mangoDepositoryAndControllerInteractionsSuite(authority, user, bank, controllerUXD, mangoDepositorySOL, paramsSol);
  });

  describe("mangoDepositoryInsuranceSuite SOL", function () {
    mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositorySOL);
  });

  describe("mangoDepositoryMintRedeemSuite SOL", function () {
    mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositorySOL, 20);
  });

  this.afterAll("Transfer funds back to bank", async function () {
    await transferAllSol(user, bank.publicKey);
  });
});
