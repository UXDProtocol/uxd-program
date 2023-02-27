import { Signer } from '@solana/web3.js';
import {
  CredixLpDepository,
  IdentityDepository,
  MercurialVaultDepository,
  USDC_DECIMALS,
  USDC_DEVNET,
} from '@uxd-protocol/uxd-client';
import { Controller } from '@uxd-protocol/uxd-client';
import { expect } from 'chai';
import { collectProfitOfCredixLpDepositoryTest } from '../cases/collectProfitsOfCredixLpDepositoryTest';
import { editControllerTest } from '../cases/editControllerTest';
import { editIdentityDepositoryTest } from '../cases/editIdentityDepositoryTest';
import { editMercurialVaultDepositoryTest } from '../cases/editMercurialVaultDepositoryTest';
import { freezeProgramTest } from '../cases/freezeProgramTest';
import { mintWithCredixLpDepositoryTest } from '../cases/mintWithCredixLpDepositoryTest';
import { mintWithIdentityDepositoryTest } from '../cases/mintWithIdentityDepositoryTest';
import { mintWithMercurialVaultDepositoryTest } from '../cases/mintWithMercurialVaultDepositoryTest';
import { redeemFromCredixLpDepositoryTest } from '../cases/redeemFromCredixLpDepositoryTest';
import { redeemFromIdentityDepositoryTest } from '../cases/redeemFromIdentityDepositoryTest';
import { redeemFromMercurialVaultDepositoryTest } from '../cases/redeemFromMercurialVaultDepositoryTest';
import { registerMercurialVaultDepositoryTest } from '../cases/registerMercurialVaultDepositoryTest';
import { uxdProgramId } from '../constants';
import {
  createCredixLpDepositoryDevnetUSDC,
  createMercurialVaultDepositoryDevnet,
} from '../utils';

export const freezeProgramSuite = async function ({
  authority,
  user,
  payer,
  controller,
}: {
  authority: Signer;
  user: Signer;
  payer: Signer;
  controller: Controller;
}) {
  let mercurialVaultDepository: MercurialVaultDepository;
  let credixLpDepository: CredixLpDepository;
  let identityDepository: IdentityDepository;

  before(async () => {
    mercurialVaultDepository = await createMercurialVaultDepositoryDevnet();
    credixLpDepository = await createCredixLpDepositoryDevnetUSDC();
    identityDepository = new IdentityDepository(
      USDC_DEVNET,
      'USDC',
      USDC_DECIMALS,
      uxdProgramId
    );
  });

  it(`Freeze program`, async function () {
    await freezeProgramTest({ freeze: true, authority, controller });
  });

  it(`mintWithMercurialVaultDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await mintWithMercurialVaultDepositoryTest({
        collateralAmount: 1,
        user,
        controller,
        depository: mercurialVaultDepository,
        payer,
      });
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`redeemFromMercurialVaultDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await redeemFromMercurialVaultDepositoryTest({
        redeemableAmount: 1,
        user,
        controller,
        depository: mercurialVaultDepository,
        payer,
      });
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`mintWithCredixLpDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await mintWithCredixLpDepositoryTest({
        uiAmountCollateralDeposited: 1,
        user,
        controller,
        depository: credixLpDepository,
        payer,
      });
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`redeemFromCredixLpDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await redeemFromCredixLpDepositoryTest({
        redeemableAmount: 1,
        user,
        controller,
        depository: credixLpDepository,
        payer,
      });
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`collectProfitsOfCredixLpDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await collectProfitOfCredixLpDepositoryTest({
        payer,
        profitsBeneficiaryCollateral: payer.publicKey,
        controller,
        depository: credixLpDepository,
      });
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`editControllerTest under frozen program`, async function () {
    let failure = false;
    try {
      await editControllerTest({ authority, controller, uiFields: {} });
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`editIdentityDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await editIdentityDepositoryTest({
        authority,
        controller,
        depository: identityDepository,
        uiFields: {},
      });
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`editMercurialVaultDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await editMercurialVaultDepositoryTest({
        authority,
        controller,
        depository: mercurialVaultDepository,
        uiFields: {},
      });
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`mintWithIdentityDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await mintWithIdentityDepositoryTest({
        collateralAmount: 1,
        user,
        controller,
        depository: identityDepository,
        payer,
      });
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`redeemFromIdentityDepositoryTest under frozen program`, async function () {
    let failure = false;
    try {
      await redeemFromIdentityDepositoryTest({
        redeemableAmount: 1,
        user,
        controller,
        depository: identityDepository,
        payer,
      });
    } catch {
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  it(`registerMercurialVaultDepositoryTest under frozen program`, async function () {
    let failure = false;
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
      failure = true;
    }
    expect(failure).eq(true, 'Should have failed - program is frozen');
  });

  after(`Resume program`, async function () {
    await freezeProgramTest({ freeze: false, authority, controller });
  });
};
