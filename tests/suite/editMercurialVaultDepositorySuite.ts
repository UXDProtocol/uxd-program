import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, ControllerAccount, findATAAddrSync, MercurialVaultDepository, MercurialVaultDepositoryAccount, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { redeemFromMercurialVaultDepositoryTest } from "../cases/redeemFromMercurialVaultDepositoryTest";
import { mintWithMercurialVaultDepositoryTest } from "../cases/mintWithMercurialVaultDepositoryTest";
import { getBalance, transferAllSol, transferAllTokens, transferSol, transferTokens } from "../utils";
import { getConnection, TXN_OPTS } from "../connection";
import { setRedeemableGlobalSupplyCapTest } from "../cases/setRedeemableGlobalSupplyCapTest";
import { BN } from "@project-serum/anchor";
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

        // Restore initial depository values there
        it(`Edit mintingFeeInBps/redeemingFeeInBps/redeemableDepositorySupplyCap should work`, async function () {
            const {
                mintingFeeInBps,
                redeemingFeeInBps,
                redeemableDepositorySupplyCap,
            } = beforeDepository;

            console.log("[🧾 mintingFeeInBps", mintingFeeInBps, "]");
            console.log("[🧾 redeemingFeeInBps", redeemingFeeInBps, "]");
            console.log("[🧾 redeemableDepositorySupplyCap", redeemableDepositorySupplyCap, "]");

            await editMercurialVaultDepositoryTest(controllerAuthority, controller, depository, {
                mintingFeeInBps,
                redeemingFeeInBps,
                redeemableDepositorySupplyCap,
            });
        });
    });
};