import { Signer } from "@solana/web3.js";
import { Controller, MercurialVaultDepository } from "@uxd-protocol/uxd-client";
import { registerMercurialVaultDepositoryTest } from "../cases/registerMercurialVaultDepositoryTest";

export const mercurialVaultDepositorySetupSuite = function (
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: MercurialVaultDepository,
  mintingFeeInBps: number,
  redeemingFeeInBps: number,
  redeemableAmountUnderManagementCap: number
) {
  it(`Registers mercurialVaultDepository`, async function () {
    await registerMercurialVaultDepositoryTest(
      authority,
      controller,
      depository,
      mintingFeeInBps,
      redeemingFeeInBps,
      redeemableAmountUnderManagementCap,
      payer
    );
  });
};
