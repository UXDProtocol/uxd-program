import { Keypair, Signer } from "@solana/web3.js";
import { Controller, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, WSOL, USDC_DEVNET } from "@uxdprotocol/uxd-client";
import { authority, bank, slippageBase, uxdProgramId } from "./constants";
import { transferAllSol, transferSol } from "./utils";
import { controllerIntegrationSuite, controllerIntegrationSuiteParameters } from "./suite/controllerIntegrationSuite";
import { depositInsuranceMangoDepositoryTest } from "./cases/depositInsuranceMangoDepositoryTest";
import { initializeMangoDepositoryTest } from "./cases/initializeMangoDepositoryTest";
import { mango } from "./fixtures";
import { withdrawInsuranceMangoDepositoryTest } from "./cases/withdrawInsuranceMangoDepositoryTest";
import { mintWithMangoDepositoryTest } from "./cases/mintWithMangoDepositoryTest";
import { redeemFromMangoDepositoryTest } from "./cases/redeemFromMangoDepositoryTest";

// Should use the quote info from mango.quoteToken instead of guessing it, but it's not changing often... 
const depository = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const controller = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
const payer = bank;
const slippage = 20; // 2%

console.log(`SOL 🥭🔗 'https://devnet.mango.markets/account?pubkey=${depository.mangoAccountPda}'`);

beforeEach("\n", function () { console.log("=============================================\n\n") });

describe("UXD Controller Suite", function () {
    const params = new controllerIntegrationSuiteParameters(25_000_000, 500_000);
    controllerIntegrationSuite(authority, payer, controller, params);
});

// Use SOL as it's the special case using more computing
describe("Integration tests SOL", function () {
    const user: Signer = new Keypair();

    this.beforeAll("Init and fund user (5 SOL)", async function () {
        console.log("USER =>", user.publicKey.toString());
        await transferSol(5, bank, user.publicKey);
    });

    it(`Initialize ${depository.collateralMintSymbol} Depository`, async function () {
        await initializeMangoDepositoryTest(authority, controller, depository, mango, payer);
    });

    it(`Deposit 100 USDC of insurance`, async function () {
        await depositInsuranceMangoDepositoryTest(100, authority, controller, depository, mango);
    });

    it(`Withdraw 1 USDC of insurance`, async function () {
        await withdrawInsuranceMangoDepositoryTest(1, authority, controller, depository, mango);
    });

    it(`Mint 1 ${controller.redeemableMintSymbol} then redeem the outcome (${slippage / slippageBase * 100} % slippage)`, async function () {
        const perpPrice = await depository.getCollateralPerpPriceUI(mango);
        const amount = 1 / perpPrice;
        console.log("[🧾 amount", amount, depository.collateralMintSymbol, "]");
        const mintedAmount = await mintWithMangoDepositoryTest(amount, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(mintedAmount, slippage, user, controller, depository, mango, payer);
    });

    // Fees are taken from the input on the redeem (provide UXD amount, gets UXD amount minus fees converted back to collateral). 
    // So we need to factor in the fees
    it(`Mint twice min mint trading size, then redeem them (${slippage / slippageBase * 100}% slippage)`, async function () {
        const minRedeemAmount = await depository.getMinRedeemSizeQuoteUI(mango);
        const minTradingSize = await depository.getMinTradingSizeCollateralUI(mango);

        console.log("[🧾 $ value", minRedeemAmount, controller.redeemableMintSymbol, "]");
        await mintWithMangoDepositoryTest(minTradingSize * 2, slippage, user, controller, depository, mango, payer);
        await redeemFromMangoDepositoryTest(minRedeemAmount, slippage, user, controller, depository, mango, payer);
    });

    this.afterAll("Transfer funds back to bank", async function () {
        await transferAllSol(user, bank.publicKey);
    });
});