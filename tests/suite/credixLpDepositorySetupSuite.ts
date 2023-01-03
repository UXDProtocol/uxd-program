import { PublicKey, Signer } from "@solana/web3.js";
import { findATAAddrSync } from "@uxd-protocol/uxd-client";
import { Controller, CredixLpDepository } from "@uxd-protocol/uxd-client";
import { collectProfitOfCredixLpDepositoryTest } from "../cases/collectProfitOfCredixLpDepositoryTest";
import { editCredixLpDepositoryTest } from "../cases/editCredixLpDepositoryTest";
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
    await editCredixLpDepositoryTest(authority, controller, depository, {
      redeemableAmountUnderManagementCap: 25_000_000,
    });
  });

  it(`Collecting profit of credixLpDepository should work`, async function () {
    console.log("[🧾 collectProfit]");

    await collectProfitOfCredixLpDepositoryTest(authority, controller, depository, payer);
  });
};
