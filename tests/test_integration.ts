import { web3 } from "@project-serum/anchor";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, Signer } from "@solana/web3.js";
import { uiToNative, Controller, MangoDepository, SOL_DECIMALS, BTC_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, ETH_DECIMALS, WSOL, USDC_DEVNET, BTC_DEVNET, ETH_DEVNET } from "@uxdprotocol/uxd-client";
import { authority, bank, uxdProgramId } from "./constants";
import { getConnection } from "./connection";
import { mangoDepositoryMigrationsSuite } from "./suite/mangoDepositoryMigrationsSuite";
import { getBalance, getSolBalance, transferAllSol, transferAllTokens, transferSol, transferTokens } from "./utils";
import { controllerIntegrationSuite, controllerIntegrationSuiteParameters } from "./suite/controllerIntegrationSuite";
import { MangoDepositoryAndControllerInteractionsSuiteParameters, mangoDepositoryAndControllerInteractionsSuite } from "./suite/mangoDepositoryAndControllerInteractionsSuite";
import { mangoDepositoryInsuranceSuite } from "./suite/mangoDepositoryInsuranceSuite";
import { mangoDepositorySetupSuite } from "./suite/mangoDepositorySetupSuite";
import { mangoDepositoryMintRedeemSuite } from "./suite/mangoDepositoryMintRedeemSuite";
import { mangoDepositoryRebalancingSuite, MangoDepositoryRebalancingSuiteParameters } from "./suite/mangoDepositoryRebalancingSuite";

// Should use the quote info from mango.quoteToken instead of guessing it, but it's not changing often... 
const mangoDepositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const mangoDepositoryBTC = new MangoDepository(BTC_DEVNET, "BTC", BTC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const mangoDepositoryETH = new MangoDepository(ETH_DEVNET, "ETH", ETH_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

console.log(`SOL 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositorySOL.mangoAccountPda}'`);
console.log(`BTC 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositoryBTC.mangoAccountPda}'`);
console.log(`ETH 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositoryETH.mangoAccountPda}'`);

beforeEach("\n", function () { console.log("=============================================\n\n") });

describe("UXD Controller Suite", function () {
    const params = new controllerIntegrationSuiteParameters(10_000_000, 500_000);
    controllerIntegrationSuite(authority, bank, controllerUXD, params);
});

// SOL
describe.skip("Integration tests SOL", function () {
    const user: Signer = new Keypair();

    this.beforeAll("Init and fund user", async function () {
        console.log("USER =>", user.publicKey.toString());
        await transferSol(1, bank, user.publicKey);
    });

    mangoDepositorySetupSuite(authority, bank, controllerUXD, mangoDepositorySOL, 1_000);

    mangoDepositoryMigrationsSuite(authority, bank, controllerUXD, mangoDepositorySOL);

    const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(20)
    mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositorySOL, paramsRebalancing);

    mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositorySOL);

    mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositorySOL, 20);

    const paramsSol = new MangoDepositoryAndControllerInteractionsSuiteParameters(10_000_000, 500, 50_000, 500, 20);
    mangoDepositoryAndControllerInteractionsSuite(authority, user, bank, controllerUXD, mangoDepositorySOL, paramsSol);

});

// BTC
describe("Integration tests BTC", function () {
    const user: Signer = new Keypair();

    this.beforeAll("Init and fund user", async function () {
        console.log("USER =>", user.publicKey.toString());
        await transferSol(1, bank, user.publicKey);
    });

    mangoDepositorySetupSuite(authority, bank, controllerUXD, mangoDepositoryBTC, 100_000);

    describe.skip("mangoDepositoryMigrationsSuite BTC", function () {
        mangoDepositoryMigrationsSuite(authority, bank, controllerUXD, mangoDepositoryBTC);
    });

    const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(20)
    mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositoryBTC, paramsRebalancing);

    describe.skip("mangoDepositoryInsuranceSuite BTC", function () {
        mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositoryBTC);
    });

    mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositoryBTC, 20);

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
    const user: Signer = new Keypair();

    this.beforeAll("Init and fund user", async function () {
        console.log("USER =>", user.publicKey.toString());
        await transferSol(1, bank, user.publicKey);
    });

    mangoDepositorySetupSuite(authority, bank, controllerUXD, mangoDepositoryETH, 8_000);

    mangoDepositoryMigrationsSuite(authority, bank, controllerUXD, mangoDepositoryETH); // un-migrated yet (and this is skipped)

    const paramsETH = new MangoDepositoryRebalancingSuiteParameters(20)
    mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositoryETH, paramsETH);

    mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositoryETH);

    mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositoryETH, 20);

    const paramsEth = new MangoDepositoryAndControllerInteractionsSuiteParameters(10_000_000, 8_000, 50_000, 5_000, 20);
    mangoDepositoryAndControllerInteractionsSuite(authority, user, bank, controllerUXD, mangoDepositoryETH, paramsEth);

    this.afterAll("Transfer funds back to bank", async function () {
        await transferAllSol(user, bank.publicKey);
    });
});
