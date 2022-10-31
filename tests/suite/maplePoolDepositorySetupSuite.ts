import { Signer } from "@solana/web3.js";
import { Controller, MaplePoolDepository } from "@uxd-protocol/uxd-client";
import { registerMaplePoolDepositoryTest } from "../cases/registerMaplePoolDepositoryTest";

export const maplePoolDepositorySetupSuite = function (
  authority: Signer,
  payer: Signer,
  controller: Controller,
  depository: MaplePoolDepository,
  mintingFeeInBps: number,
  redeemingFeeInBps: number,
  uiRedeemableAmountUnderManagementCap: number
) {
  it(`Initialize maplePoolDepository`, async function () {
    await registerMaplePoolDepositoryTest(
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
