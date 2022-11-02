import { Signer } from "@solana/web3.js";
import { Controller, CredixLpDepository } from "@uxd-protocol/uxd-client";
import { registerCredixLpDepositoryTest } from "../cases/registerCredixLpDepositoryTest";

export const credixLpDepositorySetupSuite = function (
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: CredixLpDepository,
  mintingFeeInBps: number,
  redeemingFeeInBps: number,
  uiRedeemableAmountUnderManagementCap: number
) {
  it(`Initialize credixLpDepository`, async function () {
    await registerCredixLpDepositoryTest(
      authority,
      controller,
      depository,
      mintingFeeInBps,
      redeemingFeeInBps,
      uiRedeemableAmountUnderManagementCap,
      payer
    );
  });
};
