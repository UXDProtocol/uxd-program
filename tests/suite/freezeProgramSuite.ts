import { Signer } from '@solana/web3.js';
import {
  IdentityDepository,
  MercurialVaultDepository,
} from '@uxd-protocol/uxd-client';
import { Controller } from '@uxd-protocol/uxd-client';
import { expect } from 'chai';
import { editControllerTest } from '../cases/editControllerTest';
import { editIdentityDepositoryTest } from '../cases/editIdentityDepositoryTest';
import { editMercurialVaultDepositoryTest } from '../cases/editMercurialVaultDepositoryTest';
import { freezeProgramTest } from '../cases/freezeProgramTest';
import { initializeIdentityDepositoryTest } from '../cases/InitializeIdentityDepositoryTest';
import { mintWithIdentityDepositoryTest } from '../cases/mintWithIdentityDepositoryTest';
import { mintWithMercurialVaultDepositoryTest } from '../cases/mintWithMercurialVaultDepositoryTest';
import { redeemFromIdentityDepositoryTest } from '../cases/redeemFromIdentityDepositoryTest';
import { redeemFromMercurialVaultDepositoryTest } from '../cases/redeemFromMercurialVaultDepositoryTest';
import { registerMercurialVaultDepositoryTest } from '../cases/registerMercurialVaultDepositoryTest';

export const freezeProgramSuite = async function (
  authority: Signer,
  user: Signer,
  payer: Signer,
  controller: Controller,
  mercurialVaultDepository: MercurialVaultDepository,
  identityDepository: IdentityDepository
) {
  before(`Freeze program`, async function () {
    await freezeProgramTest(true, authority, controller);
  });

  it(`mintWithMercurialVaultDepositoryTest under frozen program`, async function () {
    try {
      await mintWithMercurialVaultDepositoryTest({
        collateralAmount: 1,
        user,
        controller,
        depository: mercurialVaultDepository,
        payer,
      });
    } catch {
      expect(true, 'Failing as planned');
    }
    expect(false, 'Should have failed - program is frozen');
  });

  it(`redeemFromMercurialVaultDepositoryTest under frozen program`, async function () {
    try {
      await redeemFromMercurialVaultDepositoryTest({
        redeemableAmount: 1,
        user,
        controller,
        depository: mercurialVaultDepository,
        payer,
      });
    } catch {
      expect(true, 'Failing as planned');
    }
    expect(false, 'Should have failed - program is frozen');
  });

  it(`editControllerTest under frozen program`, async function () {
    try {
      await editControllerTest({ authority, controller, uiFields: {} });
    } catch {
      expect(true, 'Failing as planned');
    }
    expect(false, 'Should have failed - program is frozen');
  });

  it(`editIdentityDepositoryTest under frozen program`, async function () {
    try {
      await editIdentityDepositoryTest({
        authority,
        controller,
        depository: identityDepository,
        uiFields: {},
      });
    } catch {
      expect(true, 'Failing as planned');
    }
    expect(false, 'Should have failed - program is frozen');
  });

  it(`editMercurialVaultDepositoryTest under frozen program`, async function () {
    try {
      await editMercurialVaultDepositoryTest({
        authority,
        controller,
        depository: mercurialVaultDepository,
        uiFields: {},
      });
    } catch {
      expect(true, 'Failing as planned');
    }
    expect(false, 'Should have failed - program is frozen');
  });

  it(`initializeIdentityDepositoryTest under frozen program`, async function () {
    try {
      await initializeIdentityDepositoryTest({
        authority,
        controller,
        depository: identityDepository,
        payer,
      });
    } catch {
      expect(true, 'Failing as planned');
    }
    expect(false, 'Should have failed - program is frozen');
  });

  it(`mintWithIdentityDepositoryTest under frozen program`, async function () {
    try {
      await mintWithIdentityDepositoryTest({
        collateralAmount: 1,
        user,
        controller,
        depository: identityDepository,
        payer,
      });
    } catch {
      expect(true, 'Failing as planned');
    }
    expect(false, 'Should have failed - program is frozen');
  });

  it(`redeemFromIdentityDepositoryTest under frozen program`, async function () {
    try {
      await redeemFromIdentityDepositoryTest({
        redeemableAmount: 1,
        user,
        controller,
        depository: identityDepository,
        payer,
      });
    } catch {
      expect(true, 'Failing as planned');
    }
    expect(false, 'Should have failed - program is frozen');
  });

  it(`registerMercurialVaultDepositoryTest under frozen program`, async function () {
    try {
      await registerMercurialVaultDepositoryTest({
        authority,
        controller,
        depository: mercurialVaultDepository,
        mintingFeeInBps: 1,
        redeemingFeeInBps: 1,
        redeemableAmountUnderManagementCap: 1,
        payer,
      });
    } catch {
      expect(true, 'Failing as planned');
    }
    expect(false, 'Should have failed - program is frozen');
  });

  after(`Resume program`, async function () {
    await freezeProgramTest(false, authority, controller);
  });
};
