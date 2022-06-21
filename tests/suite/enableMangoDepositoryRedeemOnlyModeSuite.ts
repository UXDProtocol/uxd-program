import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { enableMangoDepositoryRedeemOnlyModeTest } from "../cases/disableDepositoryMintingTest";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { mango } from "../fixtures";

export const enableMangoDepositoryRedeemOnlyModeSuite = function (
  authority: Signer,
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: MangoDepository
) {
  it(`Disable ${depository.collateralMintSymbol} redeem only mode (should fail)`, async function () {
    try {
      await enableMangoDepositoryRedeemOnlyModeTest(false, authority, controller, depository);
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - Redeem only mode is disabled");
  });

  it(`Enable ${depository.collateralMintSymbol} redeem only mode`, async function () {
    await enableMangoDepositoryRedeemOnlyModeTest(true, authority, controller, depository);
  });

  it(`Enable ${depository.collateralMintSymbol} redeem only mode again (should fail)`, async function () {
    try {
      await enableMangoDepositoryRedeemOnlyModeTest(true, authority, controller, depository);
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - Redeem only mode is enabled");
  });

  it(`Mint when ${depository.collateralMintSymbol} redeem only mode is on (should fail)`, async function () {
    try {
      await mintWithMangoDepositoryTest(0.01, 20, user, controller, depository, mango, payer);
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, `Should have failed - Minting is not allowed on redeem only mode`);
  });

  it(`Disable ${depository.collateralMintSymbol} redeem only mode`, async function () {
    await enableMangoDepositoryRedeemOnlyModeTest(false, authority, controller, depository);
  });
};
