import { web3 } from "@project-serum/anchor";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair } from "@solana/web3.js";
import { uiToNative, Controller, MangoDepository, SOL_DECIMALS, BTC_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, ETH_DECIMALS, WSOL, USDC_DEVNET, BTC_DEVNET, ETH_DEVNET } from "@uxdprotocol/uxd-client";
import { authority, bank, uxdProgramId } from "./constants";
import { getConnection } from "./provider";
import { mangoDepositoryIntegrationSuite, MangoDepositoryTestSuiteParameters } from "./suite/mangoDepositoryIntegrationSuite";
import { mangoDepositoriesMigrationsSuite } from "./suite/mangoDepositoriesMigrationsSuite";
import { getBalance, getSolBalance } from "./utils";
import { controllerIntegrationSuite, controllerSuiteParameters } from "./suite/controllerIntegrationSuite";
import { MangoDepositoriesAndControllerInteractionsSuiteParameters, mangoDepositoriesAndControllerInteractionsSuite } from "./suite/mangoDepositoriesAndControllerInteractionsSuite";
import { mangoDepositoryInsuranceSuite } from "./suite/mangoDepositoriesInsuranceSuite";
import { mangoDepositorySetupSuite } from "./suite/mangoDepositoriesSetupSuite";
import { mangoDepositoryMintRedeemSuite } from "./suite/mangoDepositoryMintRedeemSuite";

// Should use the quote info from mango.quoteToken instead of guessing it, but it's not changing often... 
const mangoDepositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const mangoDepositoryBTC = new MangoDepository(BTC_DEVNET, "BTC", BTC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const mangoDepositoryETH = new MangoDepository(ETH_DEVNET, "ETH", ETH_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

console.log(`SOL 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositorySOL.mangoAccountPda}'`);
console.log(`BTC 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositoryBTC.mangoAccountPda}'`);
console.log(`ETH 🥭🔗 'https://devnet.mango.markets/account?pubkey=${mangoDepositoryETH.mangoAccountPda}'`);

beforeEach("\n", () => { console.log("=============================================\n\n") });

describe("UXD Controller Suite", () => {
    const params = new controllerSuiteParameters(10_000_000, 500_000);
    controllerIntegrationSuite(authority, bank, controllerUXD, params);
});

describe("UXD MangoDepositories Migrations Suite", () => {
    mangoDepositoriesMigrationsSuite(authority, bank, controllerUXD, mangoDepositorySOL);

    // Keep two unmigrated for now
    // migrationsSuite(authority, controllerUXD, mangoDepositoryBTC);

    // migrationsSuite(authority, controllerUXD, mangoDepositoryETH);

});

// SOL
describe("Integration tests SOL", () => {
    const user = new Keypair();
    console.log("USER =>", user.publicKey.toString());

    before("Transfer 100 SOL from bank to test user", async () => {
        const transaction = new web3.Transaction().add(
            web3.SystemProgram.transfer({
                fromPubkey: bank.publicKey,
                toPubkey: user.publicKey,
                lamports: web3.LAMPORTS_PER_SOL * 20
            }),
        );
        await web3.sendAndConfirmTransaction(getConnection(), transaction, [
            bank,
        ]);
    });

    after("Return remaining SOL balance to the bank", async () => {
        const userBalance = await getSolBalance(user.publicKey);
        const transaction = new web3.Transaction().add(
            web3.SystemProgram.transfer({
                fromPubkey: user.publicKey,
                toPubkey: bank.publicKey,
                lamports: web3.LAMPORTS_PER_SOL * userBalance - 50000 // for fees
            }),
        );
        await web3.sendAndConfirmTransaction(getConnection(), transaction, [
            user,
        ]);
    });

    ///

    mangoDepositorySetupSuite(authority, bank, controllerUXD, mangoDepositorySOL, 1_000);

    mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositorySOL);

    mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositorySOL, 20);

    const paramsSol = new MangoDepositoriesAndControllerInteractionsSuiteParameters(10_000_000, 500, 50_000, 500, 20);
    mangoDepositoriesAndControllerInteractionsSuite(authority, user, bank, controllerUXD, mangoDepositorySOL, paramsSol);

});

// BTC
describe("Integration tests BTC", () => {
    const user = new Keypair();
    console.log("USER =>", user.publicKey.toString());

    before("Transfer 20 BTC from bank to test user", async () => {
        const btcToken = new Token(getConnection(), BTC_DEVNET, TOKEN_PROGRAM_ID, bank);
        const sender = await btcToken.getOrCreateAssociatedAccountInfo(bank.publicKey);
        const receiver = await btcToken.getOrCreateAssociatedAccountInfo(user.publicKey);
        const transferTokensIx = Token.createTransferInstruction(TOKEN_PROGRAM_ID, sender.address, receiver.address, bank.publicKey, [], uiToNative(20, BTC_DECIMALS).toNumber());
        const transaction = new web3.Transaction().add(transferTokensIx);
        await web3.sendAndConfirmTransaction(getConnection(), transaction, [
            bank,
        ]);
    });

    after("Return remaining BTC balance to the bank", async () => {
        const btcToken = new Token(getConnection(), BTC_DEVNET, TOKEN_PROGRAM_ID, bank);
        const sender = await btcToken.getOrCreateAssociatedAccountInfo(user.publicKey);
        const receiver = await btcToken.getOrCreateAssociatedAccountInfo(bank.publicKey);
        const amount = await getBalance(sender.address);
        const transferTokensIx = Token.createTransferInstruction(TOKEN_PROGRAM_ID, sender.address, receiver.address, user.publicKey, [], uiToNative(amount, BTC_DECIMALS).toNumber());
        const transaction = new web3.Transaction().add(transferTokensIx);
        await web3.sendAndConfirmTransaction(getConnection(), transaction, [
            user,
        ]);
    });

    ///

    mangoDepositorySetupSuite(authority, bank, controllerUXD, mangoDepositoryBTC, 100_000);

    mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositoryBTC);

    mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositoryBTC, 20);

    const paramsBtc = new MangoDepositoriesAndControllerInteractionsSuiteParameters(10_000_000, 30_000, 1_000_000, 60_000, 20);
    mangoDepositoriesAndControllerInteractionsSuite(authority, user, bank, controllerUXD, mangoDepositoryBTC, paramsBtc);
});

// ETH
describe("Integration tests ETH", () => {
    const user = new Keypair();
    console.log("USER =>", user.publicKey.toString());

    before("Transfer 20 ETH from bank to test user", async () => {
        const ethToken = new Token(getConnection(), ETH_DEVNET, TOKEN_PROGRAM_ID, bank);
        const sender = await ethToken.getOrCreateAssociatedAccountInfo(bank.publicKey);
        const receiver = await ethToken.getOrCreateAssociatedAccountInfo(user.publicKey);
        const transferTokensIx = Token.createTransferInstruction(TOKEN_PROGRAM_ID, sender.address, receiver.address, bank.publicKey, [], uiToNative(20, ETH_DECIMALS).toNumber());
        const transaction = new web3.Transaction().add(transferTokensIx);
        await web3.sendAndConfirmTransaction(getConnection(), transaction, [
            bank,
        ]);
    });

    after("Return remaining ETH balance to the bank", async () => {
        const ethToken = new Token(getConnection(), ETH_DEVNET, TOKEN_PROGRAM_ID, bank);
        const sender = await ethToken.getOrCreateAssociatedAccountInfo(user.publicKey);
        const receiver = await ethToken.getOrCreateAssociatedAccountInfo(bank.publicKey);
        const amount = await getBalance(sender.address);
        const transferTokensIx = Token.createTransferInstruction(TOKEN_PROGRAM_ID, sender.address, receiver.address, user.publicKey, [], uiToNative(amount, ETH_DECIMALS).toNumber());
        const transaction = new web3.Transaction().add(transferTokensIx);
        await web3.sendAndConfirmTransaction(getConnection(), transaction, [
            user,
        ]);
    });

    ///

    mangoDepositorySetupSuite(authority, bank, controllerUXD, mangoDepositoryETH, 8_000);

    mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositoryETH);

    mangoDepositoryMintRedeemSuite(user, bank, controllerUXD, mangoDepositoryETH, 20);

    const paramsEth = new MangoDepositoriesAndControllerInteractionsSuiteParameters(10_000_000, 8_000, 50_000, 5_000, 20);
    mangoDepositoriesAndControllerInteractionsSuite(authority, user, bank, controllerUXD, mangoDepositoryETH, paramsEth);
});
