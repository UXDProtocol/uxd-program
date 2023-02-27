import { Signer } from '@solana/web3.js';
import { Controller, CredixLpDepository } from '@uxd-protocol/uxd-client';
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
  let depository: CredixLpDepository;
  before(async () => {
    depository = await createCredixLpDepositoryDevnetUSDC();
  });

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
  });
};
