import { Signer } from '@solana/web3.js';
import { Controller, CredixLpDepository } from '@uxd-protocol/uxd-client';
import { registerCredixLpDepository } from '../api';
import { CLUSTER } from '../constants';
import { getConnection } from '../connection';

export const registerCredixLpDepositoryTest = async function ({
  authority,
  controller,
  depository,
  accountingBpsStampFeeMint,
  accountingBpsStampFeeRedeem,
  uiAccountingSupplyRedeemableSoftCap,
  payer,
}: {
  authority: Signer;
  controller: Controller;
  depository: CredixLpDepository;
  accountingBpsStampFeeMint: number;
  accountingBpsStampFeeRedeem: number;
  uiAccountingSupplyRedeemableSoftCap: number;
  payer?: Signer;
}) {
  console.group('🧭 initializeCredixLpDepositoryTest');
  try {
    // WHEN
    if (await getConnection().getAccountInfo(depository.pda)) {
      console.log('🚧 Already registered.');
    } else {
      const txId = await registerCredixLpDepository(
        authority,
        payer ?? authority,
        controller,
        depository,
        accountingBpsStampFeeMint,
        accountingBpsStampFeeRedeem,
        uiAccountingSupplyRedeemableSoftCap
      );
      console.log(
        `🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`
      );
    }

    // THEN
    console.log(`🧾 Initialized`, 'Credix Lp Depository');
    depository.info();
    console.groupEnd();
  } catch (error) {
    console.error('❌', error);
    console.groupEnd();
    throw error;
  }
};
