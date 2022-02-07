import { web3 } from "@project-serum/anchor";
import { Keypair } from "@solana/web3.js";
import { Controller, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS } from "@uxdprotocol/uxd-client";
import { authority, USDC, bank, WSOL, uxdProgramId } from "../constants";
import { getProvider } from "../provider";
import { mangoDepositoryInsuranceSuite } from "../suite/mangoDepositoryInsuranceSuite";
import { mangoDepositoryMintRedeemSuite } from "../suite/mangoDepositoryMintRedeemSuite";
import { mangoDepositorySetupSuite } from "../suite/mangoDepositorySetupSuite";
import { getSolBalance } from "../utils";

describe("SOL Depositories tests", () => {
    const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
    const mangoDepositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);
    const user = new Keypair();

    console.log("parallel depositories SOL test USER =>", user.publicKey.toString());

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

    mangoDepositorySetupSuite(authority, controllerUXD, mangoDepositorySOL, 1_000);
    mangoDepositoryInsuranceSuite(authority, controllerUXD, mangoDepositorySOL);
    mangoDepositoryMintRedeemSuite(user, controllerUXD, mangoDepositorySOL, 20);

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