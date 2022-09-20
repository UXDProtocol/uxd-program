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
import { bank, uxdProgramId } from "./constants";
import { transferAllSol, transferSol } from "./utils";
import {
  mangoDepositoryRebalancingSuite,
  MangoDepositoryRebalancingSuiteParameters,
} from "./suite/mangoDepositoryRebalancingSuite";

// Should use the quote info from mango.quoteToken instead of guessing it, but it's not changing often...
const mangoDepositorySOL = new MangoDepository(
  WSOL,
  "SOL",
  SOL_DECIMALS,
  USDC_DEVNET,
  "USDC",
  USDC_DECIMALS,
  uxdProgramId
);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

console.log(`SOL 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositorySOL.mangoAccountPda}'`);

beforeEach("\n", function () {
  console.log("=============================================\n\n");
});

describe("Integration tests Rebalancing", function () {
  const user: Signer = new Keypair();

  this.beforeAll("Init and fund user", async function () {
    console.log("USER =>", user.publicKey.toString());
    await transferSol(1, bank, user.publicKey);
  });

  describe("mangoDepositoryRebalancingSuite SOL", function () {
    const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(20);
    mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositorySOL, paramsRebalancing);
  });

  this.afterAll("Transfer funds back to bank", async function () {
    await transferAllSol(user, bank.publicKey);
  });
});
