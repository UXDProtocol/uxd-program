import { web3, getProvider } from "@project-serum/anchor";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair } from "@solana/web3.js";
import { BTC_DECIMALS, Controller, createAndInitializeMango, ETH_DECIMALS, Mango, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS } from "@uxdprotocol/uxd-client";
import { authority, bank, BTC, CLUSTER, ETH, USDC, uxdProgramId, WSOL } from "../constants";
import { MangoDepositoriesAndControllerInteractionsSuiteParameters, mangoDepositoriesAndControllerInteractionsSuite } from "../suite/mangoDepositoriesAndControllerInteractionsSuite";
import { getBalance, getSolBalance } from "../utils";

describe("UXD Controller Depositories interactions Tests", () => {
    const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
    const mangoDepositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);
    const mangoDepositoryBTC = new MangoDepository(BTC, "BTC", BTC_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);
    const mangoDepositoryETH = new MangoDepository(ETH, "ETH", ETH_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);

    const user = new Keypair();

    console.log("MangoDepositories x Controller interactions tests USER =>", user.publicKey.toString());


    before("Transfer 20 sol from bank to test user", async () => {
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


    const paramsSol = new MangoDepositoriesAndControllerInteractionsSuiteParameters(10_000_000, 500, 50_000, 500, 20);
    mangoDepositoriesAndControllerInteractionsSuite(authority, user, controllerUXD, mangoDepositorySOL, paramsSol);

    const paramsBtc = new MangoDepositoriesAndControllerInteractionsSuiteParameters(10_000_000, 30_000, 1_000_000, 60_000, 20);
    mangoDepositoriesAndControllerInteractionsSuite(authority, user, controllerUXD, mangoDepositoryBTC, paramsBtc);

    const paramsEth = new MangoDepositoriesAndControllerInteractionsSuiteParameters(10_000_000, 8_000, 50_000, 5_000, 20);
    mangoDepositoriesAndControllerInteractionsSuite(authority, user, controllerUXD, mangoDepositoryETH, paramsEth);

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

    after("Return remaining SOL balance to the bank", async () => {
        const userBalance = await getSolBalance(user.publicKey);
        const transaction = new web3.Transaction().add(
            web3.SystemProgram.transfer({
                fromPubkey: user.publicKey,
                toPubkey: bank.publicKey,
                lamports: web3.LAMPORTS_PER_SOL * userBalance - 50000
            }),
        );
        await web3.sendAndConfirmTransaction(getProvider().connection, transaction, [
            user,
        ]);
    });
});
