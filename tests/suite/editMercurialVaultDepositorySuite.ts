import { Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository, MercurialVaultDepositoryAccount, nativeToUi } from "@uxd-protocol/uxd-client";
import { getConnection, TXN_OPTS } from "../connection";
import { editMercurialVaultDepositoryTest } from "../cases/editMercurialVaultDepositoryTest";

export const editMercurialVaultDepositorySuite = async function (controllerAuthority: Signer, user: Signer, payer: Signer, controller: Controller, depository: MercurialVaultDepository) {
    let beforeDepository: MercurialVaultDepositoryAccount;

    describe("Edit mint/redeem", () => {
        // Snapshot the initial depository values
        before(async () => {
            beforeDepository = await depository.getOnchainAccount(getConnection(), TXN_OPTS);
        });

        it(`Edit mintingFeeInBps alone should work`, async function () {
            const mintingFeeInBps = 50;

            console.log("[🧾 mintingFeeInBps", mintingFeeInBps, "]");

            await editMercurialVaultDepositoryTest(controllerAuthority, controller, depository, {
                mintingFeeInBps,
            });
        });

        it(`Edit redeemingFeeInBps alone should work`, async function () {
            const redeemingFeeInBps = 50;

            console.log("[🧾 redeemingFeeInBps", redeemingFeeInBps, "]");

            await editMercurialVaultDepositoryTest(controllerAuthority, controller, depository, {
                redeemingFeeInBps,
            });
        });

        it(`Edit redeemableDepositorySupplyCap alone should work`, async function () {
            const redeemableDepositorySupplyCap = 50;

            console.log("[🧾 redeemableDepositorySupplyCap", redeemableDepositorySupplyCap, "]");

            await editMercurialVaultDepositoryTest(controllerAuthority, controller, depository, {
                redeemableDepositorySupplyCap,
            });
        });

        it(`Edit mintingDisabled alone should work`, async function () {
            const mintingDisabled = true;

            console.log("[🧾 mintingDisabled", mintingDisabled, "]");

            await editMercurialVaultDepositoryTest(controllerAuthority, controller, depository, {
                mintingDisabled,
            });
        });

        // Restore initial depository values there
        it(`Edit mintingFeeInBps/redeemingFeeInBps/redeemableDepositorySupplyCap should work`, async function () {
            const {
                mintingFeeInBps,
                redeemingFeeInBps,
                redeemableDepositorySupplyCap,
                mintingDisabled,
            } = beforeDepository;

            const uiRedeemableDepositorySupplyCap = nativeToUi(redeemableDepositorySupplyCap, controller.redeemableMintDecimals);

            console.log("[🧾 mintingFeeInBps", mintingFeeInBps, "]");
            console.log("[🧾 redeemingFeeInBps", redeemingFeeInBps, "]");
            console.log("[🧾 redeemableDepositorySupplyCap", uiRedeemableDepositorySupplyCap, "]");
            console.log("[🧾 mintingDisabled", mintingDisabled, "]");

            await editMercurialVaultDepositoryTest(controllerAuthority, controller, depository, {
                mintingFeeInBps,
                redeemingFeeInBps,
                mintingDisabled,
                redeemableDepositorySupplyCap: uiRedeemableDepositorySupplyCap,
            });
        });
    });
};