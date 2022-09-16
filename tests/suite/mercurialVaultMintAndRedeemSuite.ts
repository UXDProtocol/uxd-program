import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, ControllerAccount, findATAAddrSync, MercurialVaultDepository, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { redeemFromMercurialVaultDepositoryTest } from "../cases/redeemFromMercurialVaultDepositoryTest";
import { mintWithMercurialVaultDepositoryTest } from "../cases/mintWithMercurialVaultDepositoryTest";
import { getBalance, transferAllTokens, transferTokens } from "../utils";
import { getConnection, TXN_OPTS } from "../connection";
import { setRedeemableGlobalSupplyCapTest } from "../cases/setRedeemableGlobalSupplyCapTest";
import { BN } from "@project-serum/anchor";
import { uxdProgramId } from "../constants";

export const mercurialVaultDepositoryMintRedeemSuite = async function (controllerAuthority: Signer, user: Signer, payer: Signer, controller: Controller, collateralMintInfo: {
    mint: PublicKey;
    name: string;
    symbol: string;
    decimals: number;
}) {
    const mercurialVaultDepository = await MercurialVaultDepository.initialize({
        connection: getConnection(),
        collateralMint: collateralMintInfo,
        uxdProgramId,
        cluster: 'devnet',
    });

    let initialRedeemableAccountBalance: number;
    let initialControllerGlobalRedeemableSupplyCap: BN;
    let userRedeemableATA: PublicKey;
    let onchainController: ControllerAccount;

    before(`Setup: Transfer 0.1 ${collateralMintInfo.symbol} from payer to user`, async function () {
        console.log('collateralMintInfo.mint', collateralMintInfo.mint.toBase58());
        console.log('collateralMintInfo.decimals', collateralMintInfo.decimals);
        console.log(' user.publicKey', user.publicKey.toBase58());

        await transferTokens(0.1, collateralMintInfo.mint, collateralMintInfo.decimals, payer, user.publicKey);

        userRedeemableATA = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];

        [
            initialRedeemableAccountBalance,
            onchainController,
        ] = await Promise.all([
            getBalance(userRedeemableATA),
            controller.getOnchainAccount(getConnection(), TXN_OPTS),
        ]);

        initialControllerGlobalRedeemableSupplyCap = onchainController.redeemableGlobalSupplyCap;
    });

    describe("Regular mint/redeem", () => {
        it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${collateralMintInfo.symbol}`, async function () {
            const collateralAmount = 0.001;

            console.log("[🧾 collateralAmount", collateralAmount, collateralMintInfo.symbol, "]");

            await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, mercurialVaultDepository, payer);
        });

        it(`Redeem all ${controller.redeemableMintSymbol} minted previously for ${collateralMintInfo.symbol}`, async function () {
            const redeemableAccountBalance = await getBalance(userRedeemableATA);

            const previouslyMintedRedeemableAmount = redeemableAccountBalance - initialRedeemableAccountBalance;

            console.log("[🧾 redeemableAmount", previouslyMintedRedeemableAmount, collateralMintInfo.symbol, "]");

            await redeemFromMercurialVaultDepositoryTest(previouslyMintedRedeemableAmount, user, controller, mercurialVaultDepository, payer);
        });
    });

    describe("Over limits", () => {
        it(`Mint for more ${collateralMintInfo.symbol} than possessed (should fail)`, async function () {
            const collateralAmount = 1_000_000;

            console.log("[🧾 collateralAmount", collateralAmount, collateralMintInfo.symbol, "]");

            try {
                await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, mercurialVaultDepository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - Do not own enough ${collateralMintInfo.symbol}`);
        });

        it(`Redeem for more ${controller.redeemableMintSymbol} than possessed (should fail)`, async function () {
            const redeemableAmount = initialRedeemableAccountBalance + 1;

            console.log("[🧾 redeemableAmount", redeemableAmount, controller.redeemableMintSymbol, "]");

            try {
                await redeemFromMercurialVaultDepositoryTest(redeemableAmount, user, controller, mercurialVaultDepository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - Only owned ${initialRedeemableAccountBalance} ${controller.redeemableMintSymbol}`);
        });

        it(`Mint for 0 ${collateralMintInfo.symbol} (should fail)`, async function () {
            const collateralAmount = 0;

            console.log("[🧾 collateralAmount", collateralAmount, collateralMintInfo.symbol, "]");

            try {
                await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, mercurialVaultDepository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - Cannot mint for 0 ${collateralMintInfo.symbol}`);
        });

        it(`Redeem for 0 ${controller.redeemableMintSymbol} (should fail)`, async function () {
            const redeemableAmount = 0;

            console.log("[🧾 redeemableAmount", redeemableAmount, controller.redeemableMintSymbol, "]");

            try {
                await redeemFromMercurialVaultDepositoryTest(redeemableAmount, user, controller, mercurialVaultDepository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - Cannot redeem for 0 ${controller.redeemableMintSymbol}`);
        });
    });

    describe("1 native unit mint/redeem", async () => {
        before(`Setup: Mint ${controller.redeemableMintSymbol} with 0.001 ${collateralMintInfo.symbol}`, async function () {
            const collateralAmount = 0.001;

            console.log("[🧾 collateralAmount", collateralAmount, collateralMintInfo.symbol, "]");

            await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, mercurialVaultDepository, payer);
        });

        it(`Mint for 1 native unit ${collateralMintInfo.symbol}`, async function () {
            const collateralAmount = Math.pow(10, -collateralMintInfo.decimals);

            console.log("[🧾 collateralAmount", collateralAmount, collateralMintInfo.symbol, "]");

            try {
                await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, mercurialVaultDepository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - User cannot mint for 0 ${controller.redeemableMintSymbol} (happens due to precision loss and fees)`);
        });

        it(`Redeem for 1 native unit ${controller.redeemableMintSymbol}`, async function () {
            const redeemableAmount = Math.pow(10, -controller.redeemableMintDecimals);

            console.log("[🧾 redeemableAmount", redeemableAmount, controller.redeemableMintSymbol, "]");

            try {
                await redeemFromMercurialVaultDepositoryTest(redeemableAmount, user, controller, mercurialVaultDepository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - User cannot get 0 ${controller.redeemableMintSymbol} from redeem (happens due to precision loss and fees)`);
        });

        after(`Cleanup: Redeem all ${controller.redeemableMintSymbol} minted previously for ${collateralMintInfo.symbol}`, async function () {
            const redeemableAccountBalance = await getBalance(userRedeemableATA);

            const previouslyMintedRedeemableAmount = redeemableAccountBalance - initialRedeemableAccountBalance;

            console.log("[🧾 redeemableAmount", previouslyMintedRedeemableAmount, collateralMintInfo.symbol, "]");

            await redeemFromMercurialVaultDepositoryTest(previouslyMintedRedeemableAmount, user, controller, mercurialVaultDepository, payer);
        });
    });

    describe("Global redeemable supply cap overflow", () => {
        it('Set global redeemable supply cap to 0', async function () {
            await setRedeemableGlobalSupplyCapTest(0, controllerAuthority, controller);
        });

        it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${collateralMintInfo.symbol} (should fail)`, async function () {
            const collateralAmount = 0.001;

            console.log("[🧾 collateralAmount", collateralAmount, collateralMintInfo.symbol, "]");

            try {
                await mintWithMercurialVaultDepositoryTest(collateralAmount, user, controller, mercurialVaultDepository, payer);
            } catch {
                expect(true, "Failing as planned");
            }

            expect(false, `Should have failed - amount of redeemable overflow the global redeemable supply cap`);
        });

        it(`Reset Global Redeemable supply cap back to its original value`, async function () {
            const globalRedeemableSupplyCap = nativeToUi(initialControllerGlobalRedeemableSupplyCap, controller.redeemableMintDecimals);

            await setRedeemableGlobalSupplyCapTest(globalRedeemableSupplyCap, controllerAuthority, controller);
        });
    });

    after(`Cleanup: Return remaining ${collateralMintInfo.symbol} user's balance to the payer`, () => transferAllTokens(collateralMintInfo.mint, collateralMintInfo.decimals, user, payer.publicKey));
};