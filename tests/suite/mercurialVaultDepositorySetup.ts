import { PublicKey, Signer } from '@solana/web3.js';
import { Controller, MercurialVaultDepository } from '@uxd-protocol/uxd-client';
import { registerMercurialVaultDepositoryTest } from '../cases/registerMercurialVaultDepositoryTest';
import { getConnection } from '../connection';
import {
  MERCURIAL_USDC_DEVNET,
  MERCURIAL_USDC_DEVNET_DECIMALS,
  uxdProgramId,
} from '../constants';

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
    depository = await MercurialVaultDepository.initialize({
      connection: getConnection(),
      collateralMint: {
        mint: MERCURIAL_USDC_DEVNET,
        name: 'USDC',
        symbol: 'USDC',
        decimals: MERCURIAL_USDC_DEVNET_DECIMALS,
      },
      uxdProgramId,
    });
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
