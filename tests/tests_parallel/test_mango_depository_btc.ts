import { web3 } from "@project-serum/anchor";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair } from "@solana/web3.js";
import { BTC_DECIMALS, Controller, MangoDepository, USDC_DECIMALS, UXD_DECIMALS } from "@uxdprotocol/uxd-client";
import { authority, USDC, bank, uxdProgramId, BTC } from "../constants";
import { getProvider } from "../provider";
import { mangoDepositoryInsuranceSuite } from "../suite/mangoDepositoryInsuranceSuite";
import { mangoDepositoryMintRedeemSuite } from "../suite/mangoDepositoryMintRedeemSuite";
import { mangoDepositorySetupSuite } from "../suite/mangoDepositorySetupSuite";
import { getBalance, getSolBalance } from "../utils";

describe("BTC Depositories tests", () => {
    const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
    const mangoDepositoryBTC = new MangoDepository(BTC, "BTC", BTC_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);
    const user = new Keypair();

    console.log("parallel depositories BTC test USER =>", user.publicKey.toString());

    before("Transfer 1 sol from bank to test user", async () => {
        const transaction = new web3.Transaction().add(
            web3.SystemProgram.transfer({
                fromPubkey: bank.publicKey,
                toPubkey: user.publicKey,
                lamports: web3.LAMPORTS_PER_SOL * 1
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

    mangoDepositorySetupSuite(authority, controllerUXD, mangoDepositoryBTC, 100_000);
    mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositoryBTC);
    mangoDepositoryMintRedeemSuite(user, controllerUXD, mangoDepositoryBTC, 20);

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