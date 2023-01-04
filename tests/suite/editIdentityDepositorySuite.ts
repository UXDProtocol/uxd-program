import { Signer } from "@solana/web3.js";
import { Controller, IdentityDepository, IdentityDepositoryAccount, nativeToUi } from "@uxd-protocol/uxd-client";
import { getConnection, TXN_OPTS } from "../connection";
import { editIdentityDepositoryTest } from "../cases/editIdentityDepositoryTest";

export const editIdentityDepositorySuite = async function (
  controllerAuthority: Signer,
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: IdentityDepository
) {
  let beforeDepository: IdentityDepositoryAccount;

  describe("Edit mint/redeem", () => {
    // Snapshot the initial depository values
    before(async () => {
      beforeDepository = await depository.getOnchainAccount(getConnection(), TXN_OPTS);
    });

    it(`Edit redeemableAmountUnderManagementCap alone should work`, async function () {
      const redeemableAmountUnderManagementCap = 50;

      console.log("[🧾 redeemableAmountUnderManagementCap", redeemableAmountUnderManagementCap, "]");

      await editIdentityDepositoryTest(controllerAuthority, controller, depository, {
        redeemableAmountUnderManagementCap,
      });
    });

    it(`Edit mintingDisabled alone should work`, async function () {
      const mintingDisabled = true;

      console.log("[🧾 mintingDisabled", mintingDisabled, "]");

      await editIdentityDepositoryTest(controllerAuthority, controller, depository, {
        mintingDisabled,
      });
    });

    // Restore initial depository values there
    it(`Edit redeemableAmountUnderManagementCap should work`, async function () {
      const { redeemableAmountUnderManagementCap, mintingDisabled } = beforeDepository;

      const uiRedeemableAmountUnderManagementCap = nativeToUi(
        redeemableAmountUnderManagementCap,
        controller.redeemableMintDecimals
      );

      console.log("[🧾 redeemableAmountUnderManagementCap", uiRedeemableAmountUnderManagementCap, "]");
      console.log("[🧾 mintingDisabled", mintingDisabled, "]");

      await editIdentityDepositoryTest(controllerAuthority, controller, depository, {
        mintingDisabled,
        redeemableAmountUnderManagementCap: uiRedeemableAmountUnderManagementCap,
      });
    });
  });
};
