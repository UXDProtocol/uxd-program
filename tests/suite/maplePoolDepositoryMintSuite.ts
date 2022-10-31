import { Signer } from "@solana/web3.js";
import { Controller, MaplePoolDepository, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { mintWithMaplePoolDepositoryTest } from "../cases/mintWithMaplePoolDepositoryTest";
import { transferTokens } from "../utils";
import { getConnection, TXN_OPTS } from "../connection";
import { setRedeemableGlobalSupplyCapTest } from "../cases/setRedeemableGlobalSupplyCapTest";
import { BN } from "@project-serum/anchor";
import { editMaplePoolDepositoryTest } from "../cases/editMaplePoolDepositoryTest";

export const maplePoolDepositoryMintSuite = async function (
  controllerAuthority: Signer,
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: MaplePoolDepository
) {
  describe("Regular mint", () => {
    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol}`, async function () {
      const uiAmmountCollateralDeposited = 0.001;

      console.log("[🧾 uiAmmountCollateralDeposited", uiAmmountCollateralDeposited, "]");

      await mintWithMaplePoolDepositoryTest(uiAmmountCollateralDeposited, user, controller, depository, payer);
    });
  });

  describe("Over limits", () => {
    it(`Mint for more ${depository.collateralSymbol} than possessed (should fail)`, async function () {
      const uiAmmountCollateralDeposited = 1_000_000;

      console.log("[🧾 uiAmmountCollateralDeposited", uiAmmountCollateralDeposited, depository.collateralSymbol, "]");

      try {
        await mintWithMaplePoolDepositoryTest(uiAmmountCollateralDeposited, user, controller, depository, payer);
      } catch {
        expect(true, "Failing as planned");
      }

      expect(false, `Should have failed - Do not own enough ${depository.collateralSymbol}`);
    });

    it(`Mint for 0 ${depository.collateralSymbol} (should fail)`, async function () {
      const uiAmmountCollateralDeposited = 0;

      console.log("[🧾 uiAmmountCollateralDeposited", uiAmmountCollateralDeposited, depository.collateralSymbol, "]");

      try {
        await mintWithMaplePoolDepositoryTest(uiAmmountCollateralDeposited, user, controller, depository, payer);
      } catch {
        expect(true, "Failing as planned");
      }

      expect(false, `Should have failed - Cannot mint for 0 ${depository.collateralSymbol}`);
    });
  });

  describe("1 native unit mint", async () => {
    before(
      `Setup: Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol}`,
      async function () {
        const uiAmmountCollateralDeposited = 0.001;

        console.log("[🧾 uiAmmountCollateralDeposited", uiAmmountCollateralDeposited, depository.collateralSymbol, "]");

        await mintWithMaplePoolDepositoryTest(uiAmmountCollateralDeposited, user, controller, depository, payer);
      }
    );

    it(`Mint for 1 native unit ${depository.collateralSymbol}`, async function () {
      const uiAmmountCollateralDeposited = Math.pow(10, -depository.collateralDecimals);

      console.log("[🧾 uiAmmountCollateralDeposited", uiAmmountCollateralDeposited, depository.collateralSymbol, "]");

      try {
        await mintWithMaplePoolDepositoryTest(uiAmmountCollateralDeposited, user, controller, depository, payer);
      } catch {
        expect(true, "Failing as planned");
      }

      expect(
        false,
        `Should have failed - User cannot mint for 0 ${controller.redeemableMintSymbol} (happens due to precision loss and fees)`
      );
    });
  });

  describe("Depository redeemable amount under management cap overflow", async () => {
    let initialDepository = await depository.getOnchainAccount(getConnection(), TXN_OPTS);
    let initialDepositoryAmountUnderManagementCap: BN = initialDepository.redeemableAmountUnderManagementCap;

    it("Set depository redeemable amount under management cap to 0", async function () {
      await editMaplePoolDepositoryTest(controllerAuthority, controller, depository, {
        redeemableAmountUnderManagementCap: 0,
      });
    });

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol} (should fail)`, async function () {
      const uiAmmountCollateralDeposited = 0.001;

      console.log("[🧾 uiAmmountCollateralDeposited", uiAmmountCollateralDeposited, depository.collateralSymbol, "]");

      try {
        await mintWithMaplePoolDepositoryTest(uiAmmountCollateralDeposited, user, controller, depository, payer);
      } catch {
        expect(true, "Failing as planned");
      }

      expect(false, `Should have failed - amount of redeemable overflow the despository redeemable supply cap`);
    });

    it(`Reset Global Redeemable supply cap back to its original value`, async function () {
      const depositoryAmountUnderManagementCap = nativeToUi(
        initialDepositoryAmountUnderManagementCap,
        controller.redeemableMintDecimals
      );
      await editMaplePoolDepositoryTest(controllerAuthority, controller, depository, {
        redeemableAmountUnderManagementCap: depositoryAmountUnderManagementCap,
      });
    });
  });

  describe("Global redeemable supply cap overflow", async () => {
    let initialController = await controller.getOnchainAccount(getConnection(), TXN_OPTS);
    let initialControllerGlobalRedeemableSupplyCap: BN = initialController.redeemableGlobalSupplyCap;

    it("Set global redeemable supply cap to 0", async function () {
      await setRedeemableGlobalSupplyCapTest(0, controllerAuthority, controller);
    });

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol} (should fail)`, async function () {
      const uiAmmountCollateralDeposited = 0.001;

      console.log("[🧾 uiAmmountCollateralDeposited", uiAmmountCollateralDeposited, depository.collateralSymbol, "]");

      try {
        await mintWithMaplePoolDepositoryTest(uiAmmountCollateralDeposited, user, controller, depository, payer);
      } catch {
        expect(true, "Failing as planned");
      }

      expect(false, `Should have failed - amount of redeemable overflow the global redeemable supply cap`);
    });

    it(`Reset Global Redeemable supply cap back to its original value`, async function () {
      const globalRedeemableSupplyCap = nativeToUi(
        initialControllerGlobalRedeemableSupplyCap,
        controller.redeemableMintDecimals
      );
      await setRedeemableGlobalSupplyCapTest(globalRedeemableSupplyCap, controllerAuthority, controller);
    });
  });
};
