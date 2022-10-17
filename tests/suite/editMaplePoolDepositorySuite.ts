import { Signer } from "@solana/web3.js";
import { Controller, MaplePoolDepository, nativeToUi } from "@uxd-protocol/uxd-client";
import { getConnection, TXN_OPTS } from "../connection";
import { editMaplePoolDepositoryTest } from "../cases/editMaplePoolDepositoryTest";

export const editMaplePoolDepositorySuite = async function (
  controllerAuthority: Signer,
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: MaplePoolDepository
) {
  describe("Edit mint/redeem", async () => {
    let beforeDepository = await depository.getOnchainAccount(getConnection(), TXN_OPTS);

    it(`Edit mintingFeeInBps alone should work`, async function () {
      const mintingFeeInBps = 50;

      console.log("[🧾 mintingFeeInBps", mintingFeeInBps, "]");

      await editMaplePoolDepositoryTest(controllerAuthority, controller, depository, {
        mintingFeeInBps,
      });
    });

    it(`Edit redeemingFeeInBps alone should work`, async function () {
      const redeemingFeeInBps = 50;

      console.log("[🧾 redeemingFeeInBps", redeemingFeeInBps, "]");

      await editMaplePoolDepositoryTest(controllerAuthority, controller, depository, {
        redeemingFeeInBps,
      });
    });

    it(`Edit redeemableAmountUnderManagementCap alone should work`, async function () {
      const redeemableAmountUnderManagementCap = 50;

      console.log("[🧾 redeemableAmountUnderManagementCap", redeemableAmountUnderManagementCap, "]");

      await editMaplePoolDepositoryTest(controllerAuthority, controller, depository, {
        redeemableAmountUnderManagementCap,
      });
    });

    // Restore initial depository values there
    it(`Edit mintingFeeInBps/redeemingFeeInBps/redeemableAmountUnderManagementCap should work`, async function () {
      const { mintingFeeInBps, redeemingFeeInBps, redeemableAmountUnderManagementCap } = beforeDepository;

      const uiRedeemableAmountUnderManagementCap = nativeToUi(
        redeemableAmountUnderManagementCap,
        controller.redeemableMintDecimals
      );

      console.log("[🧾 mintingFeeInBps", mintingFeeInBps, "]");
      console.log("[🧾 redeemingFeeInBps", redeemingFeeInBps, "]");
      console.log("[🧾 redeemableAmountUnderManagementCap", uiRedeemableAmountUnderManagementCap, "]");

      await editMaplePoolDepositoryTest(controllerAuthority, controller, depository, {
        mintingFeeInBps,
        redeemingFeeInBps,
        redeemableAmountUnderManagementCap: uiRedeemableAmountUnderManagementCap,
      });
    });
  });
};
