import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, findATAAddrSync, MercurialVaultDepository, MercurialVaultDepositoryAccount } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { redeemFromMercurialVaultDepositoryTest } from "../cases/redeemFromMercurialVaultDepositoryTest";
import { mintWithMercurialVaultDepositoryTest } from "../cases/mintWithMercurialVaultDepositoryTest";
import { getBalance, transferAllTokens, transferTokens } from "../utils";
import { getConnection, TXN_OPTS } from "../connection";

let startingRedeemableAccountBalance = 0;
let userRedeemableATA: PublicKey;
let onchainDepository: MercurialVaultDepositoryAccount;

export const mercurialVaultDepositoryMintRedeemSuite = function (user: Signer, payer: Signer, controller: Controller, depository: MercurialVaultDepository) {
    before(`Setup: Transfer 1 ${depository.collateralMint.symbol} from payer to user`, async function () {
        await transferTokens(0.1, depository.collateralMint.mint, depository.collateralMint.decimals, payer, user.publicKey);

        userRedeemableATA = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];

        startingRedeemableAccountBalance = await getBalance(userRedeemableATA);

        onchainDepository = await depository.getOnchainAccount(getConnection(), TXN_OPTS)
    });

    describe("Regular mint/redeem", () => {
        it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralMint.symbol}`, async function () {
            const collateralAmount = 0.001;

            console.log("[🧾 collateralAmount", collateralAmount, depository.collateralMint.symbol, "]");

            await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, depository, payer);
        });

        it(`Redeem all ${controller.redeemableMintSymbol} minted previously for ${depository.collateralMint.symbol}`, async function () {
            const redeemableAccountBalance = await getBalance(userRedeemableATA);

            const previouslyMintedRedeemableAmount = redeemableAccountBalance - startingRedeemableAccountBalance;

            console.log("[🧾 redeemableAmount", previouslyMintedRedeemableAmount, depository.collateralMint.symbol, "]");

            await redeemFromMercurialVaultDepositoryTest(previouslyMintedRedeemableAmount, user, controller, depository, payer);
        });
    });

    describe("Over limits", () => {
        it(`Mint for more ${depository.collateralMint.symbol} than possessed (should fail)`, async function () {
            const collateralAmount = 1_000_000;

            console.log("[🧾 collateralAmount", collateralAmount, depository.collateralMint.symbol, "]");

            try {
                await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, depository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - Do not own enough ${depository.collateralMint.symbol}`);
        });

        it(`Redeem for more ${controller.redeemableMintSymbol} than possessed (should fail)`, async function () {
            const redeemableAmount = startingRedeemableAccountBalance + 1;

            console.log("[🧾 redeemableAmount", redeemableAmount, controller.redeemableMintSymbol, "]");

            try {
                await redeemFromMercurialVaultDepositoryTest(redeemableAmount, user, controller, depository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - Only owned ${startingRedeemableAccountBalance} ${controller.redeemableMintSymbol}`);
        });

        it(`Mint for 0 ${depository.collateralMint.symbol} (should fail)`, async function () {
            const collateralAmount = 0;

            console.log("[🧾 collateralAmount", collateralAmount, depository.collateralMint.symbol, "]");

            try {
                await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, depository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - Cannot mint for 0 ${depository.collateralMint.symbol}`);
        });

        it(`Redeem for 0 ${controller.redeemableMintSymbol} (should fail)`, async function () {
            const redeemableAmount = 0;

            console.log("[🧾 redeemableAmount", redeemableAmount, controller.redeemableMintSymbol, "]");

            try {
                await redeemFromMercurialVaultDepositoryTest(redeemableAmount, user, controller, depository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - Cannot redeem for 0 ${controller.redeemableMintSymbol}`);
        });
    });

    describe("1 native unit mint/redeem", async () => {
        before(`Setup: Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralMint.symbol}`, async function () {
            const collateralAmount = 0.001;

            console.log("[🧾 collateralAmount", collateralAmount, depository.collateralMint.symbol, "]");

            await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, depository, payer);
        });

        it(`Mint for 1 native unit ${depository.collateralMint.symbol}`, async function () {
            const collateralAmount = Math.pow(10, -depository.collateralMint.decimals);

            console.log("[🧾 collateralAmount", collateralAmount, depository.collateralMint.symbol, "]");

            try {
                await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, depository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - User cannot mint for 0 ${controller.redeemableMintSymbol} (happens due to precision loss and fees)`);
        });

        it(`Redeem for 1 native unit ${controller.redeemableMintSymbol}`, async function () {
            const redeemableAmount = Math.pow(10, -controller.redeemableMintDecimals);

            console.log("[🧾 redeemableAmount", redeemableAmount, controller.redeemableMintSymbol, "]");

            try {
                await redeemFromMercurialVaultDepositoryTest(redeemableAmount, user, controller, depository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - User cannot get 0 ${controller.redeemableMintSymbol} from redeem (happens due to precision loss and fees)`);
        });

        after(`Cleanup: Redeem all ${controller.redeemableMintSymbol} minted previously for ${depository.collateralMint.symbol}`, async function () {
            const redeemableAccountBalance = await getBalance(userRedeemableATA);

            const previouslyMintedRedeemableAmount = redeemableAccountBalance - startingRedeemableAccountBalance;

            console.log("[🧾 redeemableAmount", previouslyMintedRedeemableAmount, depository.collateralMint.symbol, "]");

            await redeemFromMercurialVaultDepositoryTest(previouslyMintedRedeemableAmount, user, controller, depository, payer);
        });
    });

    after(`Cleanup: Return remaining ${depository.collateralMint.symbol} user's balance to the payer`, () => transferAllTokens(depository.collateralMint.mint, depository.collateralMint.decimals, user, payer.publicKey));
};