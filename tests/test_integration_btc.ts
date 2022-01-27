import { web3 } from "@project-serum/anchor";
import { Keypair } from "@solana/web3.js";
import { BTC_DECIMALS, Controller, MangoDepository, USDC_DECIMALS, UXD_DECIMALS } from "@uxdprotocol/uxd-client";
import { authority, USDC, bank, uxdProgramId, BTC } from "./constants";
import { getProvider } from "./provider";
import { mangoDepositoryIntegrationSuite, MangoDepositoryTestSuiteParameters } from "./suite/mangoDepositoryIntegrationSuite";
import { getSolBalance } from "./utils";

const depositoryBTC = new MangoDepository(BTC, "BTC", BTC_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);
const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);

const user = new Keypair();

console.log("USER =>", user.publicKey.toString());

describe("BTC integration tests", () => {
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

    const params = new MangoDepositoryTestSuiteParameters(3_000_000, 30_000, 1_000_000, 60_000, 20);
    mangoDepositoryIntegrationSuite(authority, user, controllerUXD, depositoryBTC, params);

    // TODO: Add program close

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