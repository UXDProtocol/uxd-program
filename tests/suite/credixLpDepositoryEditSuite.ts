import { PublicKey, Signer } from "@solana/web3.js";
import { Controller, CredixLpDepository, nativeToUi } from "@uxd-protocol/uxd-client";
import { getConnection, TXN_OPTS } from "../connection";
import { editCredixLpDepositoryTest } from "../cases/editCredixLpDepositoryTest";

export const credixLpDepositoryEditSuite = async function (
  controllerAuthority: Signer,
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: CredixLpDepository
) {
  describe("Edit mint/redeem", async () => {
    let beforeDepository = await depository.getOnchainAccount(getConnection(), TXN_OPTS);

    it(`Edit mintingFeeInBps alone should work`, async function () {
      const mintingFeeInBps = 50;

      console.log("[🧾 mintingFeeInBps", mintingFeeInBps, "]");

      await editCredixLpDepositoryTest(controllerAuthority, controller, depository, {
        mintingFeeInBps,
      });
    });

    it(`Edit redeemingFeeInBps alone should work`, async function () {
      const redeemingFeeInBps = 50;

      console.log("[🧾 redeemingFeeInBps", redeemingFeeInBps, "]");

      await editCredixLpDepositoryTest(controllerAuthority, controller, depository, {
        redeemingFeeInBps,
      });
    });

    it(`Edit redeemableAmountUnderManagementCap alone should work`, async function () {
      const redeemableAmountUnderManagementCap = 50;

      console.log("[🧾 redeemableAmountUnderManagementCap", redeemableAmountUnderManagementCap, "]");

      await editCredixLpDepositoryTest(controllerAuthority, controller, depository, {
        redeemableAmountUnderManagementCap,
      });
    });

    it(`Edit mintingDisabled alone should work`, async function () {
      const mintingDisabled = true;

      console.log("[🧾 mintingDisabled", mintingDisabled, "]");

      await editCredixLpDepositoryTest(controllerAuthority, controller, depository, {
        mintingDisabled,
      });
    });

    it(`Edit profitTreasuryCollateral alone should work`, async function () {
      const profitTreasuryCollateral = new PublicKey("6U5vJ63PsiwevAVgzbrr5Lgd3jHEjZzdpEcMwULzoJVz");

      console.log("[🧾 profitTreasuryCollateral", profitTreasuryCollateral, "]");

      await editCredixLpDepositoryTest(controllerAuthority, controller, depository, {
        profitTreasuryCollateral,
      });
    });

    // Restore initial depository values there
    it(`Edit mintingFeeInBps/redeemingFeeInBps/redeemableAmountUnderManagementCap should work`, async function () {
      const {
        mintingFeeInBps,
        redeemingFeeInBps,
        redeemableAmountUnderManagementCap,
        mintingDisabled,
        profitTreasuryCollateral,
      } = beforeDepository;

      const uiRedeemableAmountUnderManagementCap = nativeToUi(
        redeemableAmountUnderManagementCap,
        controller.redeemableMintDecimals
      );

      console.log("[🧾 mintingFeeInBps", mintingFeeInBps, "]");
      console.log("[🧾 redeemingFeeInBps", redeemingFeeInBps, "]");
      console.log("[🧾 redeemableAmountUnderManagementCap", uiRedeemableAmountUnderManagementCap, "]");
      console.log("[🧾 mintingDisabled", mintingDisabled, "]");
      console.log("[🧾 profitTreasuryCollateral", profitTreasuryCollateral, "]");

      await editCredixLpDepositoryTest(controllerAuthority, controller, depository, {
        mintingFeeInBps,
        redeemingFeeInBps,
        redeemableAmountUnderManagementCap: uiRedeemableAmountUnderManagementCap,
        mintingDisabled,
        profitTreasuryCollateral,
      });
    });
  });
};