import { web3 } from "@project-serum/anchor";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair } from "@solana/web3.js";
import { Controller, MangoDepository, SOL_DECIMALS, BTC_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, ETH_DECIMALS, WSOL, USDC_DEVNET, BTC_DEVNET, ETH_DEVNET } from "@uxdprotocol/uxd-client";
import { authority, bank, uxdProgramId } from "./constants";
import { getProvider } from "./provider";
import { mangoDepositoryIntegrationSuite, MangoDepositoryTestSuiteParameters } from "./suite/mangoDepositoryIntegrationSuite";
import { migrationsSuite } from "./suite/migrationsSuite";
import { getBalance, getSolBalance } from "./utils";

// Should use the quote info from mango.quoteToken instead of guessing it, but it's not changing often... 
const mangoDepositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const mangoDepositoryBTC = new MangoDepository(BTC_DEVNET, "BTC", BTC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const mangoDepositoryETH = new MangoDepository(ETH_DEVNET, "ETH", ETH_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, USDC_DEVNET, "USDC", USDC_DECIMALS, uxdProgramId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

const user = new Keypair();

console.log("USER =>", user.publicKey.toString());

describe("Migrations", () => {

    beforeEach("\n", () => { console.log("=============================================\n\n") });
    
    migrationsSuite(authority, controllerUXD, mangoDepositorySOL);

    // Keep two unmigrated
    // migrationsSuite(authority, controllerUXD, mangoDepositoryBTC);

    // migrationsSuite(authority, controllerUXD, mangoDepositoryETH);

});

describe("Full Integration tests", () => {

    before("Transfer 20 SOL from bank to test user", async () => {
        const transaction = new web3.Transaction().add(
            web3.SystemProgram.transfer({
                fromPubkey: bank.publicKey,
                toPubkey: user.publicKey,
                lamports: web3.LAMPORTS_PER_SOL * 20
            }),
        );
        await web3.sendAndConfirmTransaction(getProvider().connection, transaction, [
            bank,
        ]);
    });

    before("Transfer 20 BTC from bank to test user", async () => {
        const btcToken = new Token(getProvider().connection, BTC_DEVNET, TOKEN_PROGRAM_ID, bank);
        const sender = await btcToken.getOrCreateAssociatedAccountInfo(bank.publicKey);
        const receiver = await btcToken.getOrCreateAssociatedAccountInfo(user.publicKey);
        const transferTokensIx = Token.createTransferInstruction(TOKEN_PROGRAM_ID, sender.address, receiver.address, bank.publicKey, [], 20 * 10 ** BTC_DECIMALS);
        const transaction = new web3.Transaction().add(transferTokensIx);
        await web3.sendAndConfirmTransaction(getProvider().connection, transaction, [
            bank,
        ]);
    });

    before("Transfer 20 ETH from bank to test user", async () => {
        const ethToken = new Token(getProvider().connection, ETH_DEVNET, TOKEN_PROGRAM_ID, bank);
        const sender = await ethToken.getOrCreateAssociatedAccountInfo(bank.publicKey);
        const receiver = await ethToken.getOrCreateAssociatedAccountInfo(user.publicKey);
        const transferTokensIx = Token.createTransferInstruction(TOKEN_PROGRAM_ID, sender.address, receiver.address, bank.publicKey, [], 20 * 10 ** ETH_DECIMALS);
        const transaction = new web3.Transaction().add(transferTokensIx);
        await web3.sendAndConfirmTransaction(getProvider().connection, transaction, [
            bank,
        ]);
    });

    describe("mangoDepositoryIntegrationSuite SOL", () => {
        const params = new MangoDepositoryTestSuiteParameters(3_000_000, 500, 50_000, 500, 20, 1_000);
        mangoDepositoryIntegrationSuite(authority, user, controllerUXD, mangoDepositorySOL, params);
    });

    describe("mangoDepositoryIntegrationSuite BTC", () => {
        // TODO: Make these dynamic regarding the price of the collateral
        const params = new MangoDepositoryTestSuiteParameters(3_000_000, 30_000, 1_000_000, 60_000, 20, 100_000);
        mangoDepositoryIntegrationSuite(authority, user, controllerUXD, mangoDepositoryBTC, params);
    });

    describe("mangoDepositoryIntegrationSuite ETH", () => {
        const params = new MangoDepositoryTestSuiteParameters(3_000_000, 8_000, 50_000, 5_000, 20, 6000);
        mangoDepositoryIntegrationSuite(authority, user, controllerUXD, mangoDepositoryETH, params);
    });

    after("Return remaining balance to the bank", async () => {
        const userBalance = await getSolBalance(user.publicKey);
        const transaction = new web3.Transaction().add(
            web3.SystemProgram.transfer({
                fromPubkey: user.publicKey,
                toPubkey: bank.publicKey,
                lamports: web3.LAMPORTS_PER_SOL * userBalance - 50000 // for fees
            }),
        );
        await web3.sendAndConfirmTransaction(getProvider().connection, transaction, [
            user,
        ]);
    });

    after("Return remaining BTC balance to the bank", async () => {
        const btcToken = new Token(getProvider().connection, BTC_DEVNET, TOKEN_PROGRAM_ID, bank);
        const sender = await btcToken.getOrCreateAssociatedAccountInfo(user.publicKey);
        const receiver = await btcToken.getOrCreateAssociatedAccountInfo(bank.publicKey);
        const amount = await getBalance(sender.address);
        const transferTokensIx = Token.createTransferInstruction(TOKEN_PROGRAM_ID, sender.address, receiver.address, user.publicKey, [], amount * 10 ** BTC_DECIMALS);
        const transaction = new web3.Transaction().add(transferTokensIx);
        await web3.sendAndConfirmTransaction(getProvider().connection, transaction, [
            user,
        ]);
    });

    after("Return remaining ETH balance to the bank", async () => {
        const ethToken = new Token(getProvider().connection, ETH_DEVNET, TOKEN_PROGRAM_ID, bank);
        const sender = await ethToken.getOrCreateAssociatedAccountInfo(user.publicKey);
        const receiver = await ethToken.getOrCreateAssociatedAccountInfo(bank.publicKey);
        const amount = await getBalance(sender.address);
        const transferTokensIx = Token.createTransferInstruction(TOKEN_PROGRAM_ID, sender.address, receiver.address, user.publicKey, [], amount * 10 ** ETH_DECIMALS);
        const transaction = new web3.Transaction().add(transferTokensIx);
        await web3.sendAndConfirmTransaction(getProvider().connection, transaction, [
            user,
        ]);
    });
});