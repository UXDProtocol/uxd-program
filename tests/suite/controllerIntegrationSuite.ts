import { Signer } from '@solana/web3.js';
import { Controller } from '@uxd-protocol/uxd-client';
import { initializeControllerTest } from '../cases/initializeControllerTest';
import { editControllerTest } from '../cases/editControllerTest';
import { createIdentityDepositoryDevnet } from '../utils';
import { createCredixLpDepositoryDevnetUSDC } from '../utils';
import { createMercurialVaultDepositoryDevnet } from '../utils';

export class controllerIntegrationSuiteParameters {
  public globalSupplyCap: number;

  public constructor(globalSupplyCap: number) {
    this.globalSupplyCap = globalSupplyCap;
  }
}

export const controllerIntegrationSuite = function ({
  authority,
  payer,
  controller,
}: {
  authority: Signer;
  payer: Signer;
  controller: Controller;
}) {
  it('Initialize Controller', () =>
    initializeControllerTest({
      authority,
      controller,
      payer,
    }));

  it('Initialize router depositories', async function () {
    const identityDepository = createIdentityDepositoryDevnet();
    const credixLpDepository = await createCredixLpDepositoryDevnetUSDC();
    const mercurialVaultDepository =
      await createMercurialVaultDepositoryDevnet();
    editControllerTest({
      authority,
      controller,
      uiFields: {
        routerDepositories: {
          identityDepository: identityDepository.pda,
          mercurialVaultDepository: mercurialVaultDepository.pda,
          credixLpDepository: credixLpDepository.pda,
        },
      },
    });
  });
};
