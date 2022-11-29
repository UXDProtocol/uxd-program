import { Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository, MercurialVaultDepositoryAccount, nativeToUi } from "@uxd-protocol/uxd-client";
import { getConnection, TXN_OPTS } from "../connection";
import { editMercurialVaultDepositoryTest } from "../cases/editMercurialVaultDepositoryTest";
import { MERCURIAL_USDC_DEVNET, MERCURIAL_USDC_DEVNET_DECIMALS, uxdProgramId } from "../constants";

export const editMercurialVaultDepositorySuite = async function ({
    authority,
    controller,
}: {
    authority: Signer;
    controller: Controller;
}) {
    let depository: MercurialVaultDepository;
    let beforeDepository: MercurialVaultDepositoryAccount;

    describe("Edit mint/redeem", () => {
        before(async () => {
            depository = await MercurialVaultDepository.initialize({
                connection: getConnection(),
                collateralMint: {
                    mint: MERCURIAL_USDC_DEVNET,
                    name: "USDC",
                    symbol: "USDC",
                    decimals: MERCURIAL_USDC_DEVNET_DECIMALS,
                },
                uxdProgramId,
            });

            // Snapshot the initial depository values
            beforeDepository = await depository.getOnchainAccount(getConnection(), TXN_OPTS);
        });

        it("Edit mintingFeeInBps alone should work", async function () {
            const mintingFeeInBps = 50;

            console.log("[🧾 mintingFeeInBps", mintingFeeInBps, "]");

            await editMercurialVaultDepositoryTest({
                authority,
                controller,
                depository,
                uiFields: {
                    mintingFeeInBps,
                },
            });
        });

        it("Edit redeemingFeeInBps alone should work", async function () {
            const redeemingFeeInBps = 50;

            console.log("[🧾 redeemingFeeInBps", redeemingFeeInBps, "]");

            await editMercurialVaultDepositoryTest({
                authority,
                controller,
                depository,
                uiFields: {
                    redeemingFeeInBps,
                },
            });
        });

        it("Edit redeemableAmountUnderManagementCap alone should work", async function () {
            const redeemableAmountUnderManagementCap = 50;

            console.log("[🧾 redeemableAmountUnderManagementCap", redeemableAmountUnderManagementCap, "]");

            await editMercurialVaultDepositoryTest({
                authority,
                controller,
                depository,
                uiFields: {
                    redeemableAmountUnderManagementCap,
                },
            });
        });

        it("Edit mintingDisabled alone should work", async function () {
            const mintingDisabled = true;

            console.log("[🧾 mintingDisabled", mintingDisabled, "]");

            await editMercurialVaultDepositoryTest({
                authority,
                controller,
                depository,
                uiFields: {
                    mintingDisabled,
                },
            });
        });

        // Restore initial depository values there
        it("Edit mintingFeeInBps/redeemingFeeInBps/redeemableAmountUnderManagementCap should work", async function () {
            const {
                mintingFeeInBps,
                redeemingFeeInBps,
                redeemableAmountUnderManagementCap,
                mintingDisabled,
            } = beforeDepository;

            const uiRedeemableAmountUnderManagementCap = nativeToUi(redeemableAmountUnderManagementCap, controller.redeemableMintDecimals);

            console.log("[🧾 mintingFeeInBps", mintingFeeInBps, "]");
            console.log("[🧾 redeemingFeeInBps", redeemingFeeInBps, "]");
            console.log("[🧾 redeemableAmountUnderManagementCap", uiRedeemableAmountUnderManagementCap, "]");
            console.log("[🧾 mintingDisabled", mintingDisabled, "]");

            await editMercurialVaultDepositoryTest({
                authority,
                controller,
                depository,
                uiFields: {
                    mintingFeeInBps,
                    redeemingFeeInBps,
                    mintingDisabled,
                    redeemableAmountUnderManagementCap: uiRedeemableAmountUnderManagementCap,
                },
            });
        });
    });
};