import { Keypair, PublicKey, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, WSOL, USDC_DEVNET } from "@uxdprotocol/uxd-client";
import { authority, bank } from "./constants";
import { mangoDepositoryMigrationsSuite } from "./suite/mangoDepositoryMigrationsSuite";
import { transferAllSol, transferSol } from "./utils";
import { controllerIntegrationSuite, controllerIntegrationSuiteParameters } from "./suite/controllerIntegrationSuite";
import { MangoDepositoryAndControllerInteractionsSuiteParameters, mangoDepositoryAndControllerInteractionsSuite } from "./suite/mangoDepositoryAndControllerInteractionsSuite";
import { mangoDepositoryInsuranceSuite } from "./suite/mangoDepositoryInsuranceSuite";
import { mangoDepositorySetupSuite } from "./suite/mangoDepositorySetupSuite";
import { mangoDepositoryMintRedeemSuite } from "./suite/mangoDepositoryMintRedeemSuite";
import { mangoDepositoryRebalancingSuite, MangoDepositoryRebalancingSuiteParameters } from "./suite/mangoDepositoryRebalancingSuite";

// This test require a stack of UXD onchain account that has been live for a while, as such we reuse
const ancientProgramId = new PublicKey("3kjnUzQgP8AruD7UpngGw2buFvRZdxXocAbrtqpeDdsW");

// Should use the quote info from mango.quoteToken instead of guessing it, but it's not changing often... 
const mangoDepositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, ancientProgramId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, ancientProgramId);

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

    describe("mangoDepositoryMigrationsSuite SOL", function () {
        mangoDepositoryMigrationsSuite(authority, bank, controllerUXD, mangoDepositorySOL);
    });

    describe("mangoDepositoryRebalancingSuite SOL", function () {
        const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(20)
        mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositorySOL, paramsRebalancing);
    });

    describe("mangoDepositoryInsuranceSuite SOL", function () {
        mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositorySOL);
    });

    describe("mangoDepositoryMintRedeemSuite SOL", function () {
        mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositorySOL, 20);
    });

    describe("mangoDepositoryAndControllerInteractionsSuite SOL", function () {
        const paramsSol = new MangoDepositoryAndControllerInteractionsSuiteParameters(10_000_000, 500, 50_000, 500, 20);
        mangoDepositoryAndControllerInteractionsSuite(authority, user, bank, controllerUXD, mangoDepositorySOL, paramsSol);
    });

    this.afterAll("Transfer funds back to bank", async function () {
        await transferAllSol(user, bank.publicKey);
    });
});
