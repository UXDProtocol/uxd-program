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
import { quoteMintAndRedeemSuite } from "./suite/quoteMintAndRedeemSuite";

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

describe("Integration tests Quote Mint Redeem", function () {
  const user: Signer = new Keypair();

  this.beforeAll("Init and fund user", async function () {
    console.log("USER =>", user.publicKey.toString());
    await transferSol(1, bank, user.publicKey);
  });

  describe("mangoDepositoryQuoteMintRedeemSuite SOL", function () {
    quoteMintAndRedeemSuite(authority, user, bank, controllerUXD, mangoDepositorySOL);
  });

  this.afterAll("Transfer funds back to bank", async function () {
    await transferAllSol(user, bank.publicKey);
  });
});
