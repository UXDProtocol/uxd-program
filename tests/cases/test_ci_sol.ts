import { Keypair, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, WSOL, USDC_DEVNET } from "@uxd-protocol/uxd-client";
import { authority, bank, uxdProgramId } from "../constants";
import { transferAllSol, transferSol } from "../utils";
import { controllerIntegrationSuite, controllerIntegrationSuiteParameters } from "../suite/controllerIntegrationSuite";
import { MangoDepositoryAndControllerInteractionsSuiteParameters, mangoDepositoryAndControllerInteractionsSuite } from "../suite/mangoDepositoryAndControllerInteractionsSuite";
import { mangoDepositoryInsuranceSuite } from "../suite/depositoryInsuranceSuite";
import { mangoDepositorySetupSuite } from "../suite/depositorySetupSuite";
import { mangoDepositoryMintRedeemSuite } from "../suite/mangoDepositoryMintRedeemSuite";
import { mangoDepositoryRebalancingSuite, MangoDepositoryRebalancingSuiteParameters } from "../suite/mangoDepositoryRebalancingSuite";
import { quoteMintAndRedeemSuite } from "../suite/quoteMintAndRedeemSuite";

// Should use the quote info from mango.quoteToken instead of guessing it, but it's not changing often... 
const mangoDepositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

console.log(`SOL 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositorySOL.mangoAccountPda}'`);

beforeEach("\n", function () { console.log("=============================================\n\n") });

describe("UXD Controller Suite", function () {
    const params = new controllerIntegrationSuiteParameters(25_000_000, 500_000);
    controllerIntegrationSuite(authority, bank, controllerUXD, params);
});

// SOL
describe("Integration tests SOL", function () {
    const user: Signer = new Keypair();

    this.beforeAll("Init and fund user", async function () {
        console.log("USER =>", user.publicKey.toString());
        await transferSol(1, bank, user.publicKey);
    });

    describe("mangoDepositorySetupSuite SOL", function () {
        mangoDepositorySetupSuite(authority, bank, controllerUXD, mangoDepositorySOL, 1_000);
    });

    // Skipped as it's handle bu the test_ci_rebalancing.ts
    describe.skip("mangoDepositoryRebalancingSuite SOL", function () {
        const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(20)
        mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositorySOL, paramsRebalancing);
    });

    // Skipped as it's handle bu the test_ci_quote_mint_redeem.ts
    describe.skip("mangoDepositoryQuoteMintRedeemSuite SOL", function () {
        quoteMintAndRedeemSuite(authority, user, bank, controllerUXD, mangoDepositorySOL);
    });

    describe("mangoDepositoryInsuranceSuite SOL", function () {
        mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositorySOL);
    });

    describe("mangoDepositoryMintRedeemSuite SOL", function () {
        mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositorySOL, 20);
    });

    // Mess with the redeemable caps and they are shared by these ci tests
    describe.skip("mangoDepositoryAndControllerInteractionsSuite SOL", function () {
        const paramsSol = new MangoDepositoryAndControllerInteractionsSuiteParameters(10_000_000, 500, 50_000, 500, 20);
        mangoDepositoryAndControllerInteractionsSuite(authority, user, bank, controllerUXD, mangoDepositorySOL, paramsSol);
    });

    this.afterAll("Transfer funds back to bank", async function () {
        await transferAllSol(user, bank.publicKey);
    });
});