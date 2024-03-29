import { Signer } from '@solana/web3.js';
import { Controller } from '@uxd-protocol/uxd-client';
import { initializeController } from '../api';
import { CLUSTER } from '../constants';
import { getConnection } from '../connection';

export const initializeControllerTest = async function ({
  authority,
  controller,
  payer,
}: {
  authority: Signer;
  controller: Controller;
  payer?: Signer;
}) {
  console.group('⏱ initializeControllerTest');
  try {
    // WHEN
    if (await getConnection().getAccountInfo(controller.pda)) {
      console.log('🚧 Already initialized.');
    } else {
      const txId = await initializeController({
        authority,
        payer: payer ?? authority,
        controller,
      });
      console.log(
        `🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`
      );
    }

    // THEN
    // const controllerAccount = await controller.getOnchainAccount(connection, options);
    console.log(
      `🧾 Initialized`,
      controller.redeemableMintSymbol,
      'Controller'
    );
    controller.info();
    console.groupEnd();
  } catch (error) {
    console.error('❌', error);
    console.groupEnd();
    throw error;
  }
};
