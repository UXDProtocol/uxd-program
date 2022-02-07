import { web3 } from "@project-serum/anchor";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair } from "@solana/web3.js";
import { Controller, MangoDepository, USDC_DECIMALS, UXD_DECIMALS, ETH_DECIMALS } from "@uxdprotocol/uxd-client";
import { authority, USDC, bank, uxdProgramId, ETH } from "../constants";
import { getProvider } from "../provider";
import { mangoDepositoryInsuranceSuite } from "../suite/mangoDepositoryInsuranceSuite";
import { mangoDepositoryMintRedeemSuite } from "../suite/mangoDepositoryMintRedeemSuite";
import { mangoDepositorySetupSuite } from "../suite/mangoDepositorySetupSuite";
import { getBalance, getSolBalance } from "../utils";

describe("ETH Depositories tests", () => {
    const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
    const mangoDepositoryETH = new MangoDepository(ETH, "ETH", ETH_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);
    const user = new Keypair();

    console.log("parallel depositories ETH test USER =>", user.publicKey.toString());

    before("Transfer 1 SOL from bank to test user", async () => {
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

    mangoDepositorySetupSuite(authority, controllerUXD, mangoDepositoryETH, 8_000);
    mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositoryETH);
    mangoDepositoryMintRedeemSuite(user, controllerUXD, mangoDepositoryETH, 20);

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
                lamports: web3.LAMPORTS_PER_SOL * userBalance - 50000 // for fees
            }),
        );
        await web3.sendAndConfirmTransaction(getProvider().connection, transaction, [
            user,
        ]);
    });
});