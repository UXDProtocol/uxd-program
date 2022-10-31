import { PublicKey, Signer } from "@solana/web3.js";
import {
  Controller,
  ControllerAccount,
  findATAAddrSync,
  MaplePoolDepository,
  nativeToUi,
} from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { mintWithMaplePoolDepositoryTest } from "../cases/mintWithMaplePoolDepositoryTest";
import { getBalance, transferTokens } from "../utils";
import { getConnection, TXN_OPTS } from "../connection";
import { BN } from "@project-serum/anchor";
import { editMaplePoolDepositoryTest } from "../cases/editMaplePoolDepositoryTest";
import { MaplePoolDepositoryAccount, uiToNative } from "@uxd-protocol/uxd-client";
import { editControllerTest } from "../cases/editControllerTest";

export const maplePoolDepositoryMintSuite = async function (
  controllerAuthority: Signer,
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: MaplePoolDepository
) {
  let initialRedeemableAccountBalance: number;
  let initialControllerGlobalRedeemableSupplyCap: BN;
  let initialRedeemableDepositorySupplyCap: BN;
  let userRedeemableATA: PublicKey;
  let onchainController: ControllerAccount;
  let onChainDepository: MaplePoolDepositoryAccount;

  before("Setup: fund user", async function () {
    console.log("depository.collateralMint", depository.collateralMint.toBase58());
    console.log("depository.collateralDecimals", depository.collateralDecimals);
    console.log("user.publicKey", user.publicKey.toBase58());

    await transferTokens(0.002, depository.collateralMint, depository.collateralDecimals, payer, user.publicKey);

    userRedeemableATA = findATAAddrSync(user.publicKey, controller.redeemableMintPda)[0];

    [initialRedeemableAccountBalance, onchainController, onChainDepository] = await Promise.all([
      getBalance(userRedeemableATA),
      controller.getOnchainAccount(getConnection(), TXN_OPTS),
      depository.getOnchainAccount(getConnection(), TXN_OPTS),
    ]);

    initialControllerGlobalRedeemableSupplyCap = onchainController.redeemableGlobalSupplyCap;
    initialRedeemableDepositorySupplyCap = onChainDepository.redeemableAmountUnderManagementCap;
  });

  describe("Regular mint", () => {
    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol}`, async function () {
      const collateralAmount = 0.001;

      console.log("[🧾 collateralAmount", collateralAmount, depository.collateralSymbol, "]");

      await mintWithMaplePoolDepositoryTest(collateralAmount, user, controller, depository, payer);
    });
  });

  describe("Over limits", () => {
    it(`Mint for more ${depository.collateralSymbol} than owned (should fail)`, async function () {
      const collateralAmount = 1_000_000;

      console.log("[🧾 collateralAmount", collateralAmount, depository.collateralSymbol, "]");

      try {
        await mintWithMaplePoolDepositoryTest(collateralAmount, user, controller, depository, payer);
      } catch {
        expect(true, "Failing as planned");
      }

      expect(false, `Should have failed - Do not own enough ${depository.collateralSymbol}`);
    });

    it(`Mint for 0 ${depository.collateralSymbol} (should fail)`, async function () {
      const collateralAmount = 0;

      console.log("[🧾 collateralAmount", collateralAmount, depository.collateralSymbol, "]");

      try {
        await mintWithMaplePoolDepositoryTest(collateralAmount, user, controller, depository, payer);
      } catch {
        expect(true, "Failing as planned");
      }

      expect(false, `Should have failed - Cannot mint for 0 ${depository.collateralSymbol}`);
    });
  });

  describe("1 native unit mint/redeem", async () => {
    before(
      `Setup: Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol}`,
      async function () {
        const collateralAmount = 0.001;

        console.log("[🧾 collateralAmount", collateralAmount, depository.collateralSymbol, "]");

        await mintWithMaplePoolDepositoryTest(collateralAmount, user, controller, depository, payer);
      }
    );

    it(`Mint for 1 native unit ${depository.collateralSymbol}`, async function () {
      const collateralAmount = Math.pow(10, -depository.collateralDecimals);

      console.log("[🧾 collateralAmount", collateralAmount, depository.collateralSymbol, "]");

      try {
        await mintWithMaplePoolDepositoryTest(collateralAmount, user, controller, depository, payer);
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
    it("Set global redeemable supply cap to 0", () =>
      editControllerTest(controllerAuthority, controller, {
        redeemableGlobalSupplyCap: 0,
      }));

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol} (should fail)`, async function () {
      const collateralAmount = 0.001;

      console.log("[🧾 collateralAmount", collateralAmount, depository.collateralSymbol, "]");

      try {
        await mintWithMaplePoolDepositoryTest(collateralAmount, user, controller, depository, payer);
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

      await editControllerTest(controllerAuthority, controller, {
        redeemableGlobalSupplyCap: globalRedeemableSupplyCap,
      });
    });
  });

  describe("Redeemable depository supply cap overflow", () => {
    it("Set redeemable depository supply cap to 0,0005 more than actual minted amount", async function () {
      const onChainDepository = await depository.getOnchainAccount(getConnection(), TXN_OPTS);

      await editMaplePoolDepositoryTest(controllerAuthority, controller, depository, {
        redeemableAmountUnderManagementCap:
          onChainDepository.redeemableAmountUnderManagement + uiToNative(0.0005, controller.redeemableMintDecimals),
      });
    });

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol} (should fail)`, async function () {
      const collateralAmount = 0.001;

      console.log("[🧾 collateralAmount", collateralAmount, depository.collateralSymbol, "]");

      try {
        await mintWithMaplePoolDepositoryTest(collateralAmount, user, controller, depository, payer);
      } catch {
        expect(true, "Failing as planned");
      }

      expect(false, `Should have failed - amount of redeemable overflow the redeemable depository supply cap`);
    });

    it(`Reset redeemable depository supply cap back to its original value`, async function () {
      const redeemableAmountUnderManagementCap = nativeToUi(
        initialRedeemableDepositorySupplyCap,
        controller.redeemableMintDecimals
      );

      await editMaplePoolDepositoryTest(controllerAuthority, controller, depository, {
        redeemableAmountUnderManagementCap,
      });
    });
  });

  describe("Disabled minting", () => {
    it("Disable minting on maple pool depository", async function () {
      await editMaplePoolDepositoryTest(controllerAuthority, controller, depository, {
        mintingDisabled: true,
      });
    });

    it(`Mint ${controller.redeemableMintSymbol} with 0.001 ${depository.collateralSymbol} (should fail)`, async function () {
      const collateralAmount = 0.001;

      console.log("[🧾 collateralAmount", collateralAmount, depository.collateralSymbol, "]");

      try {
        await mintWithMaplePoolDepositoryTest(collateralAmount, user, controller, depository, payer);
      } catch {
        expect(true, "Failing as planned");
      }

      expect(false, `Should have failed - minting is disabled`);
    });

    it(`Re-enable minting for maple pool depository`, async function () {
      await editMaplePoolDepositoryTest(controllerAuthority, controller, depository, {
        mintingDisabled: false,
      });
    });
  });
};
