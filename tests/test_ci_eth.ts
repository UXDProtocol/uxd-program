// Deprecated, eth is not supported as a collateral mint

import { Keypair, Signer } from "@solana/web3.js";
import {
  Controller,
  MangoDepository,
  USDC_DECIMALS,
  UXD_DECIMALS,
  ETH_DECIMALS,
  USDC_DEVNET,
  ETH_DEVNET,
} from "@uxd-protocol/uxd-client";
import { authority, bank, uxdProgramId } from "./constants";
import { transferAllSol, transferSol } from "./utils";
import { controllerIntegrationSuite, controllerIntegrationSuiteParameters } from "./suite/controllerIntegrationSuite";
import {
  MangoDepositoryAndControllerInteractionsSuiteParameters,
  mangoDepositoryAndControllerInteractionsSuite,
} from "./suite/mangoDepositoryAndControllerInteractionsSuite";
import { mangoDepositoryInsuranceSuite } from "./suite/depositoryInsuranceSuite";
import { mangoDepositorySetupSuite } from "./suite/mangoDepositorySetupSuite";
import { mangoDepositoryMintRedeemSuite } from "./suite/mangoDepositoryMintRedeemSuite";
import {
  mangoDepositoryRebalancingSuite,
  MangoDepositoryRebalancingSuiteParameters,
} from "./suite/mangoDepositoryRebalancingSuite";
import { quoteMintAndRedeemSuite } from "./suite/quoteMintAndRedeemSuite";

// Should use the quote info from mango.quoteToken instead of guessing it, but it's not changing often...
const mangoDepositoryETH = new MangoDepository(
  ETH_DEVNET,
  "ETH",
  ETH_DECIMALS,
  USDC_DEVNET,
  "USDC",
  USDC_DECIMALS,
  uxdProgramId
);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

console.log(`ETH 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositoryETH.mangoAccountPda}'`);

beforeEach("\n", function () {
  console.log("=============================================\n\n");
});

describe("UXD Controller Suite", function () {
  const params = new controllerIntegrationSuiteParameters(25_000_000, 500_000);
  controllerIntegrationSuite(authority, bank, controllerUXD, params);
});

// ETH
describe("Integration tests ETH", function () {
  const user: Signer = new Keypair();

  this.beforeAll("Init and fund user", async function () {
    console.log("USER =>", user.publicKey.toString());
    await transferSol(1, bank, user.publicKey);
  });

  describe("mangoDepositorySetupSuite ETH", function () {
    mangoDepositorySetupSuite(authority, bank, controllerUXD, mangoDepositoryETH, 8_000);
  });

  // Skipped as it's handle bu the test_ci_rebalancing.ts
  describe.skip("mangoDepositoryRebalancingSuite ETH", function () {
    const paramsETH = new MangoDepositoryRebalancingSuiteParameters(20);
    mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositoryETH, paramsETH);
  });

  // Skipped as it's handle bu the test_ci_quote_mint_redeem.ts
  describe.skip("mangoDepositoryQuoteMintRedeemSuite ETH", function () {
    quoteMintAndRedeemSuite(authority, user, bank, controllerUXD, mangoDepositoryETH);
  });

  describe("mangoDepositoryInsuranceSuite ETH", function () {
    mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositoryETH);
  });

  describe("mangoDepositoryMintRedeemSuite ETH", function () {
    mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositoryETH, 20);
  });

  // Mess with the redeemable caps and they are shared by these ci tests
  describe.skip("mangoDepositoryAndControllerInteractionsSuite ETH", function () {
    const paramsEth = new MangoDepositoryAndControllerInteractionsSuiteParameters(10_000_000, 8_000, 50_000, 5_000, 20);
    mangoDepositoryAndControllerInteractionsSuite(authority, user, bank, controllerUXD, mangoDepositoryETH, paramsEth);
  });

  this.afterAll("Transfer funds back to bank", async function () {
    await transferAllSol(user, bank.publicKey);
  });
});
