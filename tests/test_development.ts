import { Keypair, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, WSOL, USDC_DEVNET, BTC_DECIMALS, BTC_DEVNET, ETH_DECIMALS, ETH_DEVNET, BTC_DEVNET_ZO, USDC_DEVNET_ZO, ZoDepository, Zo, createAndInitializeZo } from "@uxdprotocol/uxd-client";
import { authority, bank, CLUSTER, slippageBase, uxdProgramId } from "./constants";
import { printDepositoryInfo, printUserInfo, transferAllSol, transferAllTokens, transferSol, transferTokens } from "./utils";
import { depositInsuranceMangoDepositoryTest } from "./cases/depositInsuranceMangoDepositoryTest";
import { registerMangoDepositoryTest } from "./cases/registerMangoDepositoryTest";
import { mango } from "./fixtures";
import { withdrawInsuranceMangoDepositoryTest } from "./cases/withdrawInsuranceMangoDepositoryTest";
import { mintWithMangoDepositoryTest } from "./cases/mintWithMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "./cases/redeemFromMangoDepositoryTest";
import { initializeControllerTest } from "./cases/initializeControllerTest";
import { MangoDepositoryRebalancingSuiteParameters, mangoDepositoryRebalancingSuite } from "./suite/mangoDepositoryRebalancingSuite";
import { registerZoDepositoryTest } from "./cases/registerZoDepositoryTest";
import { mintWithZoDepositoryTest } from "./cases/mintWithZoDepositoryTest";
import { getConnection, TXN_OPTS } from "./connection";
import { Wallet } from "@project-serum/anchor";
import { initializeZoDepositoryTest } from "./cases/initializeZoDepositoryTest";
import { depositInsuranceZoDepositoryTest } from "./cases/depositInsuranceZoDepositoryTest";
import { redeemFromZoDepositoryTest } from "./cases/redeemFromZoDepositoryTest";

console.log(uxdProgramId.toString());
const mangoDepositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const mangoDepositoryBTC = new MangoDepository(BTC_DEVNET, "BTC", BTC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const mangoDepositoryETH = new MangoDepository(ETH_DEVNET, "ETH", ETH_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const zoDepositorySOL = new ZoDepository(WSOL, "SOL", SOL_DECIMALS, USDC_DEVNET_ZO, "USDC", USDC_DECIMALS, uxdProgramId, CLUSTER);
const zoDepositoryBTC = new ZoDepository(BTC_DEVNET_ZO, "BTC", BTC_DECIMALS, USDC_DEVNET_ZO, "USDC", USDC_DECIMALS, uxdProgramId, CLUSTER);
const controller = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
const payer = bank;
const slippage = 50; // 5%

console.log(`SOL 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositorySOL.mangoAccountPda}'`);

beforeEach("\n", function () { console.log("=============================================\n\n") });

// Use SOL as it's the special case using more computing
describe("Integration tests SOL", function () {
    const user: Signer = new Keypair();
    let zo: Zo;

    this.beforeAll("Init and fund user (10 SOL and 10k usdc)", async function () {
        console.log("USER =>", user.publicKey.toString());
        await transferSol(10, bank, user.publicKey);
        await transferTokens(10000, USDC_DEVNET, USDC_DECIMALS, bank, user.publicKey);
        await transferTokens(10000, USDC_DEVNET_ZO, USDC_DECIMALS, bank, user.publicKey);
        await transferTokens(0.5, BTC_DEVNET_ZO, BTC_DECIMALS, bank, user.publicKey);
        // zo is user based cause they have everything tangled.
        zo = await createAndInitializeZo(getConnection(), new Wallet(user as Keypair), TXN_OPTS, CLUSTER);
    });

    describe("Init", async function () {
        it("Initialize Controller", async function () {
            await initializeControllerTest(authority, controller, payer);
        });

        it.skip(`Initialize ${mangoDepositorySOL.collateralMintSymbol} Depository`, async function () {
            await registerMangoDepositoryTest(authority, controller, mangoDepositorySOL, mango, payer);
        });
        it.skip(`Initialize ${mangoDepositoryBTC.collateralMintSymbol} Depository`, async function () {
            await registerMangoDepositoryTest(authority, controller, mangoDepositoryBTC, mango, payer);
        });
        it.skip(`Initialize ${mangoDepositoryETH.collateralMintSymbol} Depository`, async function () {
            await registerMangoDepositoryTest(authority, controller, mangoDepositoryETH, mango, payer);
        });

        it.skip(`Deposit 100 USDC of insurance`, async function () {
            await depositInsuranceMangoDepositoryTest(100, authority, controller, mangoDepositorySOL, mango);
        });

        it.skip(`Withdraw 1 USDC of insurance`, async function () {
            await withdrawInsuranceMangoDepositoryTest(1, authority, controller, mangoDepositorySOL, mango);
        });

        // it(`Mint 80 ${controller.redeemableMintSymbol} then redeem the outcome (${slippage / slippageBase * 100} % slippage)`, async function () {
        //     const mintedAmount = await mintWithMangoDepositoryTest(80, slippage, user, controller, depository, mango, payer);
        // });

        it(`Initialize ${zoDepositorySOL.collateralMintSymbol} ZoDepository and its OpenOrders account (2 IXs)`, async () => {
            await registerZoDepositoryTest(authority, controller, zoDepositorySOL, payer);
            await initializeZoDepositoryTest(authority, controller, zoDepositorySOL, zo, payer);
        });

        it(`ZO - Deposit 100 USDC of insurance`, async function () {
            await depositInsuranceZoDepositoryTest(100, authority, controller, zoDepositorySOL, zo);
        });
    });

    describe("Test minting/redeeming", async function () {

        it(`ZO Mint 300 UXD ${controller.redeemableMintSymbol} then redeem the outcome (${slippage / slippageBase * 100} % slippage)`, async function () {
            const perpPrice = await zoDepositorySOL.getPerpPriceUI(zo);
            const amount = 300 / perpPrice;
            console.log("[🧾 amount", amount, zoDepositorySOL.collateralMintSymbol, "]");
            const mintedAmount = await mintWithZoDepositoryTest(amount, slippage, user, controller, zoDepositorySOL, zo, payer);
            console.log("Minted", mintedAmount);
            await redeemFromZoDepositoryTest(mintedAmount, slippage, user, controller, zoDepositorySOL, zo, payer);
        });

        it.skip(`Mint 10 ${controller.redeemableMintSymbol} then redeem the outcome (${slippage / slippageBase * 100} % slippage)`, async function () {
            const perpPrice = await mangoDepositorySOL.getCollateralPerpPriceUI(mango);
            const amount = 10 / perpPrice;
            console.log("[🧾 amount", amount, mangoDepositorySOL.collateralMintSymbol, "]");
            const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, mangoDepositorySOL, mango, payer);
            await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, mangoDepositorySOL, mango, payer);
        });

        it.skip(`Mint twice min mint trading size, then redeem them (${slippage / slippageBase * 100}% slippage)`, async function () {
            const minRedeemAmount = await mangoDepositorySOL.getMinRedeemSizeQuoteUI(mango);
            const minTradingSize = await mangoDepositorySOL.getMinTradingSizeCollateralUI(mango);

            await mintWithMangoDepositoryTest(minTradingSize * 2, slippage, user, controller, mangoDepositorySOL, mango, payer);
            await redeemFromMangoDepositoryTest(minRedeemAmount, slippage, user, controller, mangoDepositorySOL, mango, payer);
        });
    });

    // Note - Keep a mint/redeem before rebalancing so that it creates the necessary accounts for computing
    describe.skip("mangoDepositoryRebalancingSuite SOL", function () {
        const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(slippage)
        mangoDepositoryRebalancingSuite(user, bank, controller, mangoDepositorySOL, paramsRebalancing);
    });

    describe.skip("info", async function () {
        it("info", async function () {
            await printUserInfo(user.publicKey, controller, mangoDepositorySOL);
            await printDepositoryInfo(controller, mangoDepositorySOL, mango);
        });
    });

    this.afterAll("Transfer funds back to bank", async function () {
        await transferAllSol(user, bank.publicKey);
        await transferAllTokens(USDC_DEVNET, USDC_DECIMALS, user, bank.publicKey);
        await transferAllTokens(USDC_DEVNET_ZO, USDC_DECIMALS, user, bank.publicKey);
        await transferAllTokens(BTC_DEVNET_ZO, BTC_DECIMALS, user, bank.publicKey);
    });
});