import { Keypair, PublicKey, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, WSOL, USDC_DEVNET, BTC_DECIMALS, BTC_DEVNET, ETH_DECIMALS, ETH_DEVNET } from "@uxdprotocol/uxd-client";
import { bank, uxdProgramId } from "./constants";
import { transferAllSol, transferSol } from "./utils";
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


describe("Integration tests Rebalancing", function () {
    const user: Signer = new Keypair();

    this.beforeAll("Init and fund user", async function () {
        console.log("USER =>", user.publicKey.toString());
        await transferSol(1, bank, user.publicKey);
    });

    describe("mangoDepositoryRebalancingSuite SOL", function () {
        const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(20)
        mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositorySOL, paramsRebalancing);
    });

    describe("mangoDepositoryRebalancingSuite BTC", function () {
        const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(20)
        mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositoryBTC, paramsRebalancing);
    });

    describe("mangoDepositoryRebalancingSuite ETH", function () {
        const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(20)
        mangoDepositoryRebalancingSuite(user, bank, controllerUXD, mangoDepositoryETH, paramsRebalancing);
    });

    this.afterAll("Transfer funds back to bank", async function () {
        await transferAllSol(user, bank.publicKey);
    });
});
