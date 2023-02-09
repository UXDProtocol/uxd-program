import { Signer } from '@solana/web3.js';
import { Controller } from '@uxd-protocol/uxd-client';
import { expect } from 'chai';
import { freezeProgram } from '../api';
import { getConnection, TXN_OPTS } from '../connection';
import { CLUSTER } from '../constants';

export const freezeProgramTest = async function ({
  freeze,
  authority,
  controller,
}: {
  freeze: boolean;
  authority: Signer;
  controller: Controller;
}) {
  const connection = getConnection();
  const options = TXN_OPTS;

  console.group('🧭 freezeProgramTest');
  try {
    // GIVEN
    const controllerOnchainAccount = await controller.getOnchainAccount(
      connection,
      options
    );
    const isProgramFrozen = controllerOnchainAccount.isFrozen;

    // WHEN
    const txId = await freezeProgram(authority, controller, freeze);
    console.log(
      `🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`
    );

    // THEN
    const controllerOnchainAccount_post = await controller.getOnchainAccount(
      connection,
      options
    );
    const isProgramFrozen_post = controllerOnchainAccount_post.isFrozen;

    expect(isProgramFrozen_post).equals(
      freeze,
      'program freeze state is updated'
    );
    console.log(
      `🧾 Previous freeze state of program is`,
      isProgramFrozen,
      'now is',
      isProgramFrozen_post
    );
    console.groupEnd();
  } catch (error) {
    console.error('❌', error);
    console.groupEnd();
    throw error;
  }
};
