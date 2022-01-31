import { web3 } from "@project-serum/anchor";
import { Keypair } from "@solana/web3.js";
import { Controller, MangoDepository, SOL_DECIMALS, USDC_DECIMALS, UXD_DECIMALS, ZoDepository } from "@uxdprotocol/uxd-client";
import { authority, USDC, bank, WSOL, uxdProgramId } from "./constants";
import { getProvider } from "./provider";

import { mangoDepositoryIntegrationSuite, MangoDepositoryTestSuiteParameters } from "./suite/mangoDepositoryIntegrationSuite";
import { zoDepositoryIntegrationSuite, ZoDepositoryTestSuiteParameters } from "./suite/zoDepositoryIntegrationSuite";
import { getSolBalance } from "./utils";

const controllerUXD = new Controller("UXD", UXD_DECIMALS, uxdProgramId);
const mangoDepositorySOL = new MangoDepository(WSOL, "SOL", SOL_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);
const zoDepositorySOL = new ZoDepository(WSOL, "SOL", SOL_DECIMALS, USDC, "USDC", USDC_DECIMALS, uxdProgramId);

const user = new Keypair();

console.log("USER =>", user.publicKey.toString());

describe("SOL integration tests", () => {
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

    // Skip for now cause dev on the zo one, uncomment later

    // const params = new MangoDepositoryTestSuiteParameters(3_000_000, 500, 50_000, 500, 20, 1_000);
    // mangoDepositoryIntegrationSuite(authority, user, controllerUXD, depositorySOL, params);

    const zoSolParams = new ZoDepositoryTestSuiteParameters(3_000_000, 500, 50_000, 500, 20, 1_000);
    zoDepositoryIntegrationSuite(authority, user, controllerUXD, zoDepositorySOL, zoSolParams);


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