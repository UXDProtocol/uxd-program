import { Signer } from '@solana/web3.js';
import { Controller, IdentityDepository } from '@uxd-protocol/uxd-client';
import { initializeIdentityDepositoryTest } from '../cases/InitializeIdentityDepositoryTest';
import { createIdentityDepositoryDevnet } from '../utils';

export const identityDepositorySetupSuite = function ({
  authority,
  payer,
  controller,
}: {
  authority: Signer;
  payer: Signer;
  controller: Controller;
}) {
  let depository: IdentityDepository;

  before(async () => {
    depository = createIdentityDepositoryDevnet();
  });
  it('Initialize IdentityDepository', () =>
    initializeIdentityDepositoryTest({
      authority,
      controller,
      depository,
      payer,
    }));
};
