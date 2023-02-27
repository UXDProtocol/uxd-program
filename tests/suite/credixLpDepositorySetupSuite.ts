import { Signer } from '@solana/web3.js';
import { Controller } from '@uxd-protocol/uxd-client';
import { editCredixLpDepositoryTest } from '../cases/editCredixLpDepositoryTest';
import { registerCredixLpDepositoryTest } from '../cases/registerCredixLpDepositoryTest';
import { createCredixLpDepositoryDevnetUSDC } from '../utils';

export const credixLpDepositorySetupSuite = async function ({
  authority,
  payer,
  controller,
  mintingFeeInBps,
  redeemingFeeInBps,
  uiRedeemableAmountUnderManagementCap,
}: {
  authority: Signer;
  payer: Signer;
  controller: Controller;
  mintingFeeInBps: number;
  redeemingFeeInBps: number;
  uiRedeemableAmountUnderManagementCap: number;
}) {
  let depository = await createCredixLpDepositoryDevnetUSDC();

  it(`Initialize credixLpDepository`, async function () {
    await registerCredixLpDepositoryTest({
      authority,
      controller,
      depository,
      accountingBpsStampFeeMint: mintingFeeInBps,
      accountingBpsStampFeeRedeem: redeemingFeeInBps,
      uiAccountingSupplyRedeemableSoftCap: uiRedeemableAmountUnderManagementCap,
      payer,
    });
    await editCredixLpDepositoryTest({
      authority,
      controller,
      depository,
      uiFields: {
        redeemableAmountUnderManagementCap: 25_000_000,
      },
    });
  });
};
