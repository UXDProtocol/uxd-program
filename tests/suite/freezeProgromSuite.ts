import { Signer } from "@solana/web3.js";
import { MercurialVaultDepository } from "@uxd-protocol/uxd-client";
import { Controller, Mango, MangoDepository } from "@uxd-protocol/uxd-client";
import { expect } from "chai";
import { editControllerTest } from "../cases/editControllerTest";
import { editMercurialVaultDepositoryTest } from "../cases/editMercurialVaultDepositoryTest";
import { freezeProgramTest } from "../cases/freezeProgramTest";
import { mintWithMercurialVaultDepositoryTest } from "../cases/mintWithMercurialVaultDepositoryTest";
import { redeemFromMercurialVaultDepositoryTest } from "../cases/redeemFromMercurialVaultDepositoryTest";
import { registerMercurialVaultDepositoryTest } from "../cases/registerMercurialVaultDepositoryTest";

export const freezeProgramSuite = async function (
  authority: Signer,
  user: Signer,
  payer: Signer,
  controller: Controller,
  mangoDepository: MangoDepository,
  mercurialVaultDepository: MercurialVaultDepository,
  mango: Mango
) {
  it(`Freeze program`, async function () {
    await freezeProgramTest(true, authority, controller);
  });

  it(`editControllerTest under frozen program`, async function () {
    try {
      await editControllerTest(authority, controller, {});
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - program is frozen");
  });

  it(`mintWithMercurialVaultDepositoryTest under frozen program`, async function () {
    try {
      await mintWithMercurialVaultDepositoryTest(1, user, controller, mercurialVaultDepository, payer);
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - program is frozen");
  });

  it(`redeemFromMercurialVaultDepositoryTest under frozen program`, async function () {
    try {
      await redeemFromMercurialVaultDepositoryTest(1, user, controller, mercurialVaultDepository, payer);
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - program is frozen");
  });

  it(`editMercurialVaultDepositoryTest under frozen program`, async function () {
    try {
      await editMercurialVaultDepositoryTest(authority, controller, mercurialVaultDepository, {});
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - program is frozen");
  });

  it(`registerMercurialVaultDepositoryTest under frozen program`, async function () {
    try {
      await registerMercurialVaultDepositoryTest(authority, controller, mercurialVaultDepository, 1, 1, 1, payer);
    } catch {
      expect(true, "Failing as planned");
    }
    expect(false, "Should have failed - program is frozen");
  });

  it(`Resume program`, async function () {
    await freezeProgramTest(false, authority, controller);
  });
};
