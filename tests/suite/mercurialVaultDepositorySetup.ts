import { Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository, UXD_DECIMALS } from "@uxd-protocol/uxd-client";
import { registerMercurialVaultDepositoryTest } from "../cases/registerMercurialVaultDepositoryTest";

export const mercurialVaultDepositorySetupSuite = function (authority: Signer, payer: Signer, controller: Controller, depository: MercurialVaultDepository, mintingFeeInBps: number, redeemingFeeInBps: number, redeemableDepositorySupplyCap: number) {
    it(`Initialize mercurialVaultDepository`, () => registerMercurialVaultDepositoryTest(authority, controller, depository, mintingFeeInBps, redeemingFeeInBps, redeemableDepositorySupplyCap, payer));
};