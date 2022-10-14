import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, ControllerAccount, findATAAddrSync, MercurialVaultDepository, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { redeemFromMercurialVaultDepositoryTest } from "../cases/redeemFromMercurialVaultDepositoryTest";
import { mintWithMercurialVaultDepositoryTest } from "../cases/mintWithMercurialVaultDepositoryTest";
import { getBalance, transferTokens } from "../utils";
import { getConnection, TXN_OPTS } from "../connection";
import { BN } from "@project-serum/anchor";
import { editMercurialVaultDepositoryTest } from "../cases/editMercurialVaultDepositoryTest";
import { MercurialVaultDepositoryAccount, uiToNative } from "@uxd-protocol/uxd-client";
import { editControllerTest } from "../cases/editControllerTest";

export const mercurialVaultDepositoryMintRedeemSuite = async function (controllerAuthority: Signer, user: Signer, payer: Signer, controller: Controller, depository: MercurialVaultDepository) {
    let initialRedeemableAccountBalance: number;
    let initialControllerGlobalRedeemableSupplyCap: BN;
    let initialRedeemableDepositorySupplyCap: BN;
    let userRedeemableATA: PublicKey;
    let onchainController: ControllerAccount;
    let onChainDepository: MercurialVaultDepositoryAccount;

    before('Setup: fund user', async function () {
        console.log('depository.collateralMint.mint', depository.collateralMint.mint.toBase58());
        console.log('depository.collateralMint.decimals', depository.collateralMint.decimals);
        console.log(' user.publicKey', user.publicKey.toBase58());

        await transferTokens(0.1, depository.collateralMint.mint, depository.collateralMint.decimals, payer, user.publicKey);

        userRedeemableATA = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];

        [
            initialRedeemableAccountBalance,
            onchainController,
            onChainDepository,
        ] = await Promise.all([
            getBalance(userRedeemableATA),
            controller.getOnchainAccount(getConnection(), TXN_OPTS),
            depository.getOnchainAccount(getConnection(), TXN_OPTS),
        ]);

        initialControllerGlobalRedeemableSupplyCap = onchainController.redeemableGlobalSupplyCap;
        initialRedeemableDepositorySupplyCap = onChainDepository.redeemableDepositorySupplyCap;
    });

    describe("Regular mint/redeem", () => {
        it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralMint.symbol}`, async function () {
            const collateralAmount = 0.001;

            console.log("[🧾 collateralAmount", collateralAmount, depository.collateralMint.symbol, "]");

            await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, depository, payer);
        });

        it(`Redeem all ${controller.redeemableMintSymbol} minted previously for ${depository.collateralMint.symbol}`, async function () {
            const redeemableAccountBalance = await getBalance(userRedeemableATA);

            const previouslyMintedRedeemableAmount = redeemableAccountBalance - initialRedeemableAccountBalance;

            console.log("[🧾 redeemableAmount", previouslyMintedRedeemableAmount, depository.collateralMint.symbol, "]");

            await redeemFromMercurialVaultDepositoryTest(previouslyMintedRedeemableAmount, user, controller, depository, payer);
        });
    });

    describe("Over limits", () => {
        it(`Mint for more ${depository.collateralMint.symbol} than owned (should fail)`, async function () {
            const collateralAmount = 1_000_000;

            console.log("[🧾 collateralAmount", collateralAmount, depository.collateralMint.symbol, "]");

            try {
                await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, depository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - Do not own enough ${depository.collateralMint.symbol}`);
        });

        it(`Redeem for more ${controller.redeemableMintSymbol} than owned (should fail)`, async function () {
            const redeemableAmount = initialRedeemableAccountBalance + 1;

            console.log("[🧾 redeemableAmount", redeemableAmount, controller.redeemableMintSymbol, "]");

            try {
                await redeemFromMercurialVaultDepositoryTest(redeemableAmount, user, controller, depository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - Only owned ${initialRedeemableAccountBalance} ${controller.redeemableMintSymbol}`);
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

            const previouslyMintedRedeemableAmount = redeemableAccountBalance - initialRedeemableAccountBalance;

            console.log("[🧾 redeemableAmount", previouslyMintedRedeemableAmount, depository.collateralMint.symbol, "]");

            await redeemFromMercurialVaultDepositoryTest(previouslyMintedRedeemableAmount, user, controller, depository, payer);
        });
    });

    describe("Global redeemable supply cap overflow", () => {
        it('Set global redeemable supply cap to 0', () => editControllerTest(controllerAuthority, controller, {
            redeemableGlobalSupplyCap: 0
        }));

        it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralMint.symbol} (should fail)`, async function () {
            const collateralAmount = 0.001;

            console.log("[🧾 collateralAmount", collateralAmount, depository.collateralMint.symbol, "]");

            try {
                await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, depository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - amount of redeemable overflow the global redeemable supply cap`);
        });

        it(`Reset Global Redeemable supply cap back to its original value`, async function () {
            const globalRedeemableSupplyCap = nativeToUi(initialControllerGlobalRedeemableSupplyCap, controller.redeemableMintDecimals);

            await editControllerTest(controllerAuthority, controller, {
                redeemableGlobalSupplyCap: globalRedeemableSupplyCap,
            });
        });
    });

    describe("Redeemable depository supply cap overflow", () => {
        it('Set redeemable depository supply cap to 0,0005 more than actual minted amount', async function () {
            const onChainDepository = await depository.getOnchainAccount(getConnection(), TXN_OPTS);

            await editMercurialVaultDepositoryTest(controllerAuthority, controller, depository, {
                redeemableDepositorySupplyCap: onChainDepository.mintedRedeemableAmount + uiToNative(0.0005, controller.redeemableMintDecimals),
            });
        });

        it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralMint.symbol} (should fail)`, async function () {
            const collateralAmount = 0.001;

            console.log("[🧾 collateralAmount", collateralAmount, depository.collateralMint.symbol, "]");

            try {
                await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, depository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - amount of redeemable overflow the redeemable depository supply cap`);
        });

        it(`Reset redeemable depository supply cap back to its original value`, async function () {
            const redeemableDepositorySupplyCap = nativeToUi(initialRedeemableDepositorySupplyCap, controller.redeemableMintDecimals);

            await editMercurialVaultDepositoryTest(controllerAuthority, controller, depository, {
                redeemableDepositorySupplyCap,
            });
        });
    });

    describe('Disabled minting', () => {
        it('Disable minting on mercurial vault depository', async function () {
            await editMercurialVaultDepositoryTest(controllerAuthority, controller, depository, {
                mintingDisabled: true,
            });
        });

        it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralMint.symbol} (should fail)`, async function () {
            const collateralAmount = 0.001;

            console.log("[🧾 collateralAmount", collateralAmount, depository.collateralMint.symbol, "]");

            try {
                await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, depository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - minting is disabled`);
        });

        it(`Re-enable minting for mercurial vault depository`, async function () {
            await editMercurialVaultDepositoryTest(controllerAuthority, controller, depository, {
                mintingDisabled: false,
            });
        });
    });
};