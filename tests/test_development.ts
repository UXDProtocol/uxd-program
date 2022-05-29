import { Keypair, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, WSOL, USDC_DEVNET, BTC_DECIMALS, BTC_DEVNET, ETH_DECIMALS, ETH_DEVNET, UXD_DEVNET } from "@uxd-protocol/uxd-client";
import { authority, bank, slippageBase, uxdProgramId } from "./constants";
import { printDepositoryInfo, printUserInfo, transferAllSol, transferAllTokens, transferSol, transferTokens } from "./utils";
import { depositInsuranceMangoDepositoryTest } from "./cases/depositInsuranceMangoDepositoryTest";
import { registerMangoDepositoryTest } from "./cases/registerMangoDepositoryTest";
import { mango } from "./fixtures";
import { withdrawInsuranceMangoDepositoryTest } from "./cases/withdrawInsuranceMangoDepositoryTest";
import { mintWithMangoDepositoryTest } from "./cases/mintWithMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "./cases/redeemFromMangoDepositoryTest";
import { initializeControllerTest } from "./cases/initializeControllerTest";
import { MangoDepositoryRebalancingSuiteParameters, mangoDepositoryRebalancingSuite } from "./suite/mangoDepositoryRebalancingSuite";
import { quoteMintAndRedeemSuite } from "./suite/quoteMintAndRedeemSuite";
import { utils } from "@project-serum/anchor";
import { setMangoDepositoriesRedeemableSoftCap } from "./api";

console.log(uxdProgramId.toString());
const mangoDepositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const mangoDepositoryBTC = new MangoDepository(BTC_DEVNET, "BTC", BTC_DECIMALS, USDC_DEVNET, "USDC", UXD_DECIMALS, uxdProgramId);
const mangoDepositoryETH = new MangoDepository(ETH_DEVNET, "ETH", ETH_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const controller = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
const payer = bank;
const slippage = 500; // 5%

// console.log(`SOL 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositorySOL.mangoAccountPda}'`);

beforeEach("\n", function () { console.log("=============================================\n\n") });

// Use SOL as it's the special case using more computing
describe("Integration tests SOL", function () {
    const user: Signer = new Keypair();

    this.beforeAll("Init and fund user (1 SOL and 1k usdc)", async function () {
        console.log("USER =>", user.publicKey.toString());
        await transferSol(1, bank, user.publicKey);
        await transferTokens(1000, USDC_DEVNET, USDC_DECIMALS, bank, user.publicKey);
    });


    describe("Init", async function () {
        it("Initialize Controller", async function () {
            await initializeControllerTest(authority, controller, payer);
        });

        it(`Initialize ${mangoDepositorySOL.collateralMintSymbol} Depository`, async function () {
            await registerMangoDepositoryTest(authority, controller, mangoDepositorySOL, mango, payer);
        });
        // it.skip(`Initialize ${mangoDepositoryBTC.collateralMintSymbol} Depository`, async function () {
        //     await registerMangoDepositoryTest(authority, controller, mangoDepositoryBTC, mango, payer);
        // });
        // it(`Initialize ${mangoDepositoryETH.collateralMintSymbol} Depository`, async function () {
        //     await initializeMangoDepositoryTest(authority, controller, mangoDepositoryETH, mango, payer);
        // });

        it(`Deposit 100 USDC of insurance`, async function () {
            await depositInsuranceMangoDepositoryTest(100, authority, controller, mangoDepositorySOL, mango);
        });

        it("Increase soft cap", async function () {
            await setMangoDepositoriesRedeemableSoftCap(authority, controller, 10_000_000);
        });

        it("Mint 1 BTC", async function () {
            await mintWithMangoDepositoryTest(1, slippage, user, controller, mangoDepositorySOL, mango, payer);
        });
        // it(`Withdraw 10 USDC of insurance`, async function () {
        //     await withdrawInsuranceMangoDepositoryTest(10, authority, controller, mangoDepositorySOL, mango);
        // });

        // it(`Mint 80 ${controller.redeemableMintSymbol} then redeem the outcome (${slippage / slippageBase * 100} % slippage)`, async function () {
        //     const mintedAmount = await mintWithMangoDepositoryTest(80, slippage, user, controller, depository, mango, payer);
        // });

    });

    // describe.only("Quote Mint And Redeem Suite", async function () {
    //     quoteMintAndRedeemSuite(authority, user, payer, controller, mangoDepositoryBTC);
    // });

    // describe("Quote mint and redeem", async function () {
    //     it("Mint 10 BTC", async function() {
    //         await mintWithMangoDepositoryTest(10, slippage, user, controller, mangoDepositoryBTC, mango, payer);
    //     });
    // });

    // describe.skip("Test minting/redeeming SOL", async function () {
    //     it(`Mint 10 ${controller.redeemableMintSymbol} then redeem the outcome (${slippage / slippageBase * 100} % slippage)`, async function () {
    //         const perpPrice = await mangoDepositorySOL.getCollateralPerpPriceUI(mango);
    //         const amount = 10 / perpPrice;
    //         console.log("[🧾 amount", amount, mangoDepositorySOL.collateralMintSymbol, "]");
    //         const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, mangoDepositorySOL, mango, payer);
    //         await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, mangoDepositorySOL, mango, payer);
    //     });

    //     it(`Mint twice min mint trading size, then redeem them (${slippage / slippageBase * 100}% slippage)`, async function () {
    //         const minRedeemAmount = await mangoDepositorySOL.getMinRedeemSizeQuoteUI(mango);
    //         const minTradingSize = await mangoDepositorySOL.getMinTradingSizeCollateralUI(mango);

    //         await mintWithMangoDepositoryTest(minTradingSize * 2, slippage, user, controller, mangoDepositorySOL, mango, payer);
    //         await redeemFromMangoDepositoryTest(minRedeemAmount, slippage, user, controller, mangoDepositorySOL, mango, payer);
    //     });
    // });

    // Note - Keep a mint/redeem before rebalancing so that it creates the necessary accounts for computing
    // describe.skip("mangoDepositoryRebalancingSuite SOL", function () {
    //     const paramsRebalancing = new MangoDepositoryRebalancingSuiteParameters(slippage)
    //     mangoDepositoryRebalancingSuite(user, bank, controller, mangoDepositorySOL, paramsRebalancing);
    // });

    // describe.skip("info SOL", async function () {
    //     it("info", async function () {
    //         await printUserInfo(user.publicKey, controller, mangoDepositorySOL);
    //         await printDepositoryInfo(controller, mangoDepositorySOL, mango);
    //     });
    // });
    describe("info", async function () {
        it("info", async function () {
            await printUserInfo(user.publicKey, controller, mangoDepositorySOL);
            await printDepositoryInfo(controller, mangoDepositorySOL, mango);
        });
    });

    this.afterAll("Transfer funds back to bank", async function () {
        // await transferAllTokens(USDC_DEVNET, USDC_DECIMALS, user, bank.publicKey);
        // await transferAllTokens(BTC_DEVNET, BTC_DECIMALS, user, bank.publicKey);
        await transferAllSol(user, bank.publicKey);
        await transferAllSol(user, bank.publicKey);
    });
});