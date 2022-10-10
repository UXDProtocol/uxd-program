import { Signer } from "@solana/web3.js";
import { Controller, ControllerAccount, MaplePoolDepository, nativeToUi } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { mintWithMaplePoolDepositoryTest } from "../cases/mintWithMaplePoolDepositoryTest";
import { transferTokens } from "../utils";
import { getConnection, TXN_OPTS } from "../connection";
import { setRedeemableGlobalSupplyCapTest } from "../cases/setRedeemableGlobalSupplyCapTest";
import { BN } from "@project-serum/anchor";

export const maplePoolDepositoryMintSuite = async function (
  controllerAuthority: Signer,
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: MaplePoolDepository
) {
  let initialControllerGlobalRedeemableSupplyCap: BN;
  let onchainController: ControllerAccount;

  before("Setup: fund user", async function () {
    console.log("depository.collateralMint", depository.collateralMint.toBase58());
    console.log("depository.collateralDecimals", depository.collateralDecimals);
    console.log("depository.collateralSymbol", depository.collateralSymbol);
    console.log("user.publicKey", user.publicKey.toBase58());

    await transferTokens(0.1, depository.collateralMint, depository.collateralDecimals, payer, user.publicKey);

    onchainController = await controller.getOnchainAccount(getConnection(), TXN_OPTS);

    initialControllerGlobalRedeemableSupplyCap = onchainController.redeemableGlobalSupplyCap;
  });

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

  describe("Global redeemable supply cap overflow", () => {
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
