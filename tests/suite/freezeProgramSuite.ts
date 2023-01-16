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
    let failure = false;
    try {
      await mintWithMercurialVaultDepositoryTest(
        1,
        user,
        controller,
        mercurialVaultDepository,
        payer
      );
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`redeemFromMercurialVaultDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await redeemFromMercurialVaultDepositoryTest(
        1,
        user,
        controller,
        mercurialVaultDepository,
        payer
      );
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`editControllerTest under frozen program`, async function () {
    let failure = false;
    try {
      await editControllerTest(authority, controller, {});
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`editIdentityDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await editIdentityDepositoryTest(
        authority,
        controller,
        identityDepository,
        {}
      );
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`editMercurialVaultDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await editMercurialVaultDepositoryTest(
        authority,
        controller,
        mercurialVaultDepository,
        {}
      );
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`initializeIdentityDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await initializeIdentityDepositoryTest(
        authority,
        controller,
        identityDepository,
        payer
      );
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`mintWithIdentityDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await mintWithIdentityDepositoryTest(
        1,
        user,
        controller,
        identityDepository,
        payer
      );
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`redeemFromIdentityDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await redeemFromIdentityDepositoryTest(
        1,
        user,
        controller,
        identityDepository,
        payer
      );
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`registerMercurialVaultDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await registerMercurialVaultDepositoryTest(
        authority,
        controller,
        mercurialVaultDepository,
        1,
        1,
        1,
        payer
      );
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  after(`Resume program`, async function () {
    await freezeProgramTest(false, authority, controller);
  });
};
