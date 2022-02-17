import { Keypair, PublicKey, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, BTC_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, USDC_DEVNET, BTC_DEVNET } from "@uxdprotocol/uxd-client";
import { authority, bank, uxdProgramId } from "./constants";
import { mangoDepositoryMigrationsSuite } from "./suite/mangoDepositoryMigrationsSuite";
import { transferAllSol, transferSol } from "./utils";
import { controllerIntegrationSuite, controllerIntegrationSuiteParameters } from "./suite/controllerIntegrationSuite";
import { MangoDepositoryAndControllerInteractionsSuiteParameters, mangoDepositoryAndControllerInteractionsSuite } from "./suite/mangoDepositoryAndControllerInteractionsSuite";
import { mangoDepositoryInsuranceSuite } from "./suite/mangoDepositoryInsuranceSuite";
import { mangoDepositorySetupSuite } from "./suite/mangoDepositorySetupSuite";
import { mangoDepositoryMintRedeemSuite } from "./suite/mangoDepositoryMintRedeemSuite";
import { mangoDepositoryRebalancingSuite, MangoDepositoryRebalancingSuiteParameters } from "./suite/mangoDepositoryRebalancingSuite";

// CXzEE9YjFgw3Ggz2r1oLHqJTd4mpzFWRKm9fioTjpk45

// Should use the quote info from mango.quoteToken instead of guessing it, but it's not changing often... 
const mangoDepositoryBTC = new MangoDepository(BTC_DEVNET, "BTC", BTC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

console.log(`BTC 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositoryBTC.mangoAccountPda}'`);

beforeEach("\n", function () { console.log("=============================================\n\n") });

describe("UXD Controller Suite", function () {
    const params = new controllerIntegrationSuiteParameters(25_000_000, 500_000);
    controllerIntegrationSuite(authority, bank, controllerUXD, params);
});

// BTC
describe("Integration tests BTC", function () {
    const user: Signer = new Keypair();

    this.beforeAll("Init and fund user", async function () {
        console.log("USER =>", user.publicKey.toString());
        await transferSol(1, bank, user.publicKey);
    });

    describe("mangoDepositorySetupSuite BTC", function () {
        mangoDepositorySetupSuite(authority, bank, controllerUXD, mangoDepositoryBTC, 100_000);
    });

    describe("mangoDepositoryMigrationsSuite BTC", function () {
        mangoDepositoryMigrationsSuite(authority, bank, controllerUXD, mangoDepositoryBTC);
    });

    // Skipped as it's handle bu the test_ci_rebalancing.ts
    describe("mangoDepositoryRebalancingSuite BTC", function () {
        const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(20)
        mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositoryBTC, paramsRebalancing);
    });

    describe("mangoDepositoryInsuranceSuite BTC", function () {
        mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositoryBTC);
    });

    describe("mangoDepositoryMintRedeemSuite BTC", function () {
        mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositoryBTC, 20);
    });

    // Mess with the redeemable caps and they are shared by these ci tests
    describe.skip("mangoDepositoryAndControllerInteractionsSuite BTC", function () {
        const paramsBtc = new MangoDepositoryAndControllerInteractionsSuiteParameters(10_000_000, 30_000, 1_000_000, 60_000, 20);
        mangoDepositoryAndControllerInteractionsSuite(authority, user, bank, controllerUXD, mangoDepositoryBTC, paramsBtc);
    });

    this.afterAll("Transfer funds back to bank", async function () {
        await transferAllSol(user, bank.publicKey);
    });
});