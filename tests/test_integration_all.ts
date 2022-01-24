import { web3 } from "@project-serum/anchor";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair } from "@solana/web3.js";
import { Controller, MangoDepository, SOL_DECIMALS, BTC_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, ETH_DECIMALS } from "@uxdprotocol/uxd-client";
import { authority, USDC, bank, WSOL, uxdProgramId, BTC, ETH } from "./constants";
import { getProvider } from "./provider";
import { mangoDepositoryIntegrationSuite } from "./suite/mangoDepositoryIntegrationSuite";
import { getBalance, getSolBalance } from "./utils";

const mangoDepositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);
const mangoDepositoryBTC = new MangoDepository(BTC, "BTC", BTC_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);
const mangoDepositoryETH = new MangoDepository(ETH, "ETH", ETH_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

const user = new Keypair();

console.log("USER =>", user.publicKey.toString());

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
        const btcToken = new Token(getProvider().connection, BTC, TOKEN_PROGRAM_ID, bank);
        const sender = await btcToken.getOrCreateAssociatedAccountInfo(bank.publicKey);
        const receiver = await btcToken.getOrCreateAssociatedAccountInfo(user.publicKey);
        const transferTokensIx = Token.createTransferInstruction(TOKEN_PROGRAM_ID, sender.address, receiver.address, bank.publicKey, [], 20 * 10 ** BTC_DECIMALS);
        const transaction = new web3.Transaction().add(transferTokensIx);
        await web3.sendAndConfirmTransaction(getProvider().connection, transaction, [
            bank,
        ]);
    });

    before("Transfer 20 ETH from bank to test user", async () => {
        const ethToken = new Token(getProvider().connection, ETH, TOKEN_PROGRAM_ID, bank);
        const sender = await ethToken.getOrCreateAssociatedAccountInfo(bank.publicKey);
        const receiver = await ethToken.getOrCreateAssociatedAccountInfo(user.publicKey);
        const transferTokensIx = Token.createTransferInstruction(TOKEN_PROGRAM_ID, sender.address, receiver.address, bank.publicKey, [], 20 * 10 ** ETH_DECIMALS);
        const transaction = new web3.Transaction().add(transferTokensIx);
        await web3.sendAndConfirmTransaction(getProvider().connection, transaction, [
            bank,
        ]);
    });

    describe("mangoDepositoryIntegrationSuite SOL", () => {
        mangoDepositoryIntegrationSuite(authority, user, controllerUXD, mangoDepositorySOL);
    });

    describe("mangoDepositoryIntegrationSuite BTC", () => {
        mangoDepositoryIntegrationSuite(authority, user, controllerUXD, mangoDepositoryBTC);
    });

    describe("mangoDepositoryIntegrationSuite ETH", () => {
        mangoDepositoryIntegrationSuite(authority, user, controllerUXD, mangoDepositoryETH);
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
        const btcToken = new Token(getProvider().connection, BTC, TOKEN_PROGRAM_ID, bank);
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
        const ethToken = new Token(getProvider().connection, ETH, TOKEN_PROGRAM_ID, bank);
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