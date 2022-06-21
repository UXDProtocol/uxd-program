import { Signer } from "@solana/web3.js";
import { Controller, MangoDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { disableDepositoryRegularMintingTest } from "../cases/disableDepositoryMintingTest";
import { mintWithMangoDepositoryTest } from "../cases/mintWithMangoDepositoryTest";
import { mango } from "../fixtures";

export const disableDepositoryMintingSuite = function (
  authority: Signer,
  user: Signer,
  payer: Signer,
  controller: Controller,
  depository: MangoDepository
) {
  it(`Enable ${depository.collateralMintSymbol} minting (should fail)`, async function () {
    try {
      await disableDepositoryRegularMintingTest(false, authority, controller, depository);
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - Minting is already enabled");
  });

  it(`Disable ${depository.collateralMintSymbol} minting`, async function () {
    await disableDepositoryRegularMintingTest(true, authority, controller, depository);
  });

  it(`Disable ${depository.collateralMintSymbol} minting again (should fail)`, async function () {
    try {
      await disableDepositoryRegularMintingTest(true, authority, controller, depository);
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - Minting is already disabled");
  });

  it(`Mint when ${depository.collateralMintSymbol} depository is disabled (should fail)`, async function () {
    try {
      await mintWithMangoDepositoryTest(0.01, 20, user, controller, depository, mango, payer);
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, `Should have failed - Minting is already disabled`);
  });

  it(`Enable ${depository.collateralMintSymbol} minting`, async function () {
    await disableDepositoryRegularMintingTest(false, authority, controller, depository);
  });
};
