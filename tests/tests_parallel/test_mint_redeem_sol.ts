import { web3 } from "@project-serum/anchor";
import { Keypair } from "@solana/web3.js";
import { Controller, createAndInitializeMango, Mango, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS } from "@uxdprotocol/uxd-client";
import { USDC, bank, WSOL, uxdProgramId, CLUSTER } from "../constants";
import { getProvider } from "../provider";
import { mangoDepositoryMintRedeemSuite } from "../suite/mangoDepositoryMintRedeemSuite";
import { getSolBalance } from "../utils";

describe("SOL Mint/Redeem tests", () => {
    const mangoDepositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);
    const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
    const user = new Keypair();

    console.log("Concurrent SOL mint/redeem USER =>", user.publicKey.toString());

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

    mangoDepositoryMintRedeemSuite(user, controllerUXD, mangoDepositorySOL, 20);

    after("Return remaining balance to the bank", async () => {
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