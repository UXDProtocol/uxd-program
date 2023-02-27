import { Signer } from '@solana/web3.js';
import { Controller, MercurialVaultDepository } from '@uxd-protocol/uxd-client';
import { registerMercurialVaultDepositoryTest } from '../cases/registerMercurialVaultDepositoryTest';
import { createMercurialVaultDepositoryDevnet } from '../utils';

export const mercurialVaultDepositorySetupSuite = function ({
  authority,
  payer,
  controller,
  mintingFeeInBps,
  redeemingFeeInBps,
  redeemableAmountUnderManagementCap,
}: {
  authority: Signer;
  payer: Signer;
  controller: Controller;
  mintingFeeInBps: number;
  redeemingFeeInBps: number;
  redeemableAmountUnderManagementCap: number;
}) {
  let depository: MercurialVaultDepository;

  before(async () => {
    depository = await createMercurialVaultDepositoryDevnet();
  });

  it('Registers mercurialVaultDepository', () =>
    registerMercurialVaultDepositoryTest({
      authority,
      controller,
      depository,
      mintingFeeInBps,
      redeemingFeeInBps,
      redeemableAmountUnderManagementCap,
      payer,
    }));
};
