import { Keypair, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, SOL_DECIMALS, BTC_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, ETH_DECIMALS, WSOL, USDC_DEVNET, BTC_DEVNET, ETH_DEVNET } from "@uxdprotocol/uxd-client";
import { authority, bank, uxdProgramId } from "./constants";
import { transferAllSol, transferSol } from "./utils";
import { controllerIntegrationSuite, controllerIntegrationSuiteParameters } from "./suite/controllerIntegrationSuite";
import { MangoDepositoryAndControllerInteractionsSuiteParameters, mangoDepositoryAndControllerInteractionsSuite } from "./suite/mangoDepositoryAndControllerInteractionsSuite";
import { mangoDepositoryInsuranceSuite } from "./suite/depositoryInsuranceSuite";
import { mangoDepositorySetupSuite } from "./suite/depositorySetupSuite";
import { mangoDepositoryMintRedeemSuite } from "./suite/mangoDepositoryMintRedeemSuite";
import { mangoDepositoryRebalancingSuite, MangoDepositoryRebalancingSuiteParameters } from "./suite/mangoDepositoryRebalancingSuite";
import { mangoDepositoryAndControllerAccountingSuite } from "./suite/mangoDepositoryAndControllerAccountingSuite";

// Should use the quote info from mango.quoteToken instead of guessing it, but it's not changing often... 
const mangoDepositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const mangoDepositoryBTC = new MangoDepository(BTC_DEVNET, "BTC", BTC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const mangoDepositoryETH = new MangoDepository(ETH_DEVNET, "ETH", ETH_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

console.log(`SOL 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositorySOL.mangoAccountPda}'`);
console.log(`BTC 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositoryBTC.mangoAccountPda}'`);
console.log(`ETH 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositoryETH.mangoAccountPda}'`);

beforeEach("\n", function () { console.log("=============================================\n\n") });

describe("UXD Controller Suite", function () {
    const params = new controllerIntegrationSuiteParameters(25_000_000, 500_000);
    controllerIntegrationSuite(authority, bank, controllerUXD, params);
});

let user: Signer = new Keypair();

// SOL
describe("Integration tests SOL", function () {

    this.beforeAll("Init and fund user", async function () {
        console.log("USER =>", user.publicKey.toString());
        await transferSol(1, bank, user.publicKey);
    });

    describe("mangoDepositorySetupSuite SOL", function () {
        mangoDepositorySetupSuite(authority, bank, controllerUXD, mangoDepositorySOL, 1_000);
    });

    describe("mangoDepositoryRebalancingSuite SOL", function () {
        const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(20)
        mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositorySOL, paramsRebalancing);
    });

    describe("mangoDepositoryInsuranceSuite SOL", function () {
        mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositorySOL);
    });

    describe.only("mangoDepositoryMintRedeemSuite SOL", function () {
        mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositorySOL, 20);
    });

    describe("mangoDepositoryAndControllerInteractionsSuite SOL", function () {
        const paramsSol = new MangoDepositoryAndControllerInteractionsSuiteParameters(10_000_000, 500, 50_000, 500, 20);
        mangoDepositoryAndControllerInteractionsSuite(authority, user, bank, controllerUXD, mangoDepositorySOL, paramsSol);
    });

    describe("mangoDepositoryAndControllerAccountingSuite SOL", function () {
        const slippage = 20;
        const collateralUnitShift = SOL_DECIMALS - 2; // SOL units - target units
        mangoDepositoryAndControllerAccountingSuite(authority, user, bank, controllerUXD, mangoDepositorySOL, slippage, collateralUnitShift);
    })

    this.afterAll("Transfer funds back to bank", async function () {
        await transferAllSol(user, bank.publicKey);
    });
});

// BTC
describe.skip("Integration tests BTC", function () {

    this.beforeAll("Init and fund user", async function () {
        console.log("USER =>", user.publicKey.toString());
        await transferSol(1, bank, user.publicKey);
    });

    describe("mangoDepositorySetupSuite BTC", function () {
        mangoDepositorySetupSuite(authority, bank, controllerUXD, mangoDepositoryBTC, 100_000);
    });

    describe("mangoDepositoryRebalancingSuite BTC", function () {
        const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(20)
        mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositoryBTC, paramsRebalancing);
    });

    describe.skip("mangoDepositoryInsuranceSuite BTC", function () {
        mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositoryBTC);
    });

    describe("mangoDepositoryMintRedeemSuite BTC", function () {
        mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositoryBTC, 20);
    });

    describe.skip("mangoDepositoryAndControllerInteractionsSuite BTC", function () {
        const paramsBtc = new MangoDepositoryAndControllerInteractionsSuiteParameters(10_000_000, 30_000, 1_000_000, 60_000, 20);
        mangoDepositoryAndControllerInteractionsSuite(authority, user, bank, controllerUXD, mangoDepositoryBTC, paramsBtc);
    });

    this.afterAll("Transfer funds back to bank", async function () {
        await transferAllSol(user, bank.publicKey);
    });
});

// ETH
describe.skip("Integration tests ETH", function () {

    this.beforeAll("Init and fund user", async function () {
        console.log("USER =>", user.publicKey.toString());
        await transferSol(1, bank, user.publicKey);
    });

    describe("mangoDepositorySetupSuite ETH", function () {
        mangoDepositorySetupSuite(authority, bank, controllerUXD, mangoDepositoryETH, 8_000);
    });

    describe("mangoDepositoryRebalancingSuite ETH", function () {
        const paramsETH = new MangoDepositoryRebalancingSuiteParameters(20)
        mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositoryETH, paramsETH);
    });

    describe.skip("mangoDepositoryInsuranceSuite ETH", function () {
        mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositoryETH);
    });

    describe("mangoDepositoryMintRedeemSuite ETH", function () {
        mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositoryETH, 20);
    });

    describe.skip("mangoDepositoryAndControllerInteractionsSuite ETH", function () {
        const paramsEth = new MangoDepositoryAndControllerInteractionsSuiteParameters(10_000_000, 8_000, 50_000, 5_000, 20);
        mangoDepositoryAndControllerInteractionsSuite(authority, user, bank, controllerUXD, mangoDepositoryETH, paramsEth);
    });

    this.afterAll("Transfer funds back to bank", async function () {
        await transferAllSol(user, bank.publicKey);
    });
});
