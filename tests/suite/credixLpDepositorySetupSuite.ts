import { PublicKey, Signer } from '@solana/web3.js';
import { findATAAddrSync } from '@uxd-protocol/uxd-client';
import { Controller, CredixLpDepository } from '@uxd-protocol/uxd-client';
import { expect } from 'chai';
import { collectProfitOfCredixLpDepositoryTest } from '../cases/collectProfitOfCredixLpDepositoryTest';
import { editCredixLpDepositoryTest } from '../cases/editCredixLpDepositoryTest';
import { registerCredixLpDepositoryTest } from '../cases/registerCredixLpDepositoryTest';

export const credixLpDepositorySetupSuite = function (
  authority: Signer,
  payer: Signer,
  profitsBeneficiary: Signer,
  controller: Controller,
  depository: CredixLpDepository,
  mintingFeeInBps: number,
  redeemingFeeInBps: number,
  uiRedeemableAmountUnderManagementCap: number
) {
  it(`Initialize credixLpDepository`, async function () {
    await registerCredixLpDepositoryTest(
      authority,
      controller,
      depository,
      mintingFeeInBps,
      redeemingFeeInBps,
      uiRedeemableAmountUnderManagementCap,
      payer
    );
    await editCredixLpDepositoryTest(authority, controller, depository, {
      redeemableAmountUnderManagementCap: 25_000_000,
    });
  });

  it(`Collecting profits of credixLpDepository should work`, async function () {
    console.log('[🧾 collectProfit]');
    const profitsBeneficiaryCollateral = findATAAddrSync(
      profitsBeneficiary.publicKey,
      depository.collateralMint
    )[0];
    await editCredixLpDepositoryTest(authority, controller, depository, {
      profitsBeneficiaryCollateral: profitsBeneficiaryCollateral,
    });
    await collectProfitOfCredixLpDepositoryTest(
      payer,
      profitsBeneficiaryCollateral,
      controller,
      depository
    );
  });

  it(`Collecting profits of credixLpDepository should not work for invalid collateral address`, async function () {
    console.log('[🧾 collectProfit]');
    await editCredixLpDepositoryTest(authority, controller, depository, {
      profitsBeneficiaryCollateral: PublicKey.default,
    });
    let failure = false;
    try {
      await collectProfitOfCredixLpDepositoryTest(
        payer,
        PublicKey.default,
        controller,
        depository
      );
    } catch {
      failure = true;
    }
    expect(failure).eq(
      true,
      `Should have failed - Invalid profits beneficiary`
    );
  });
};
