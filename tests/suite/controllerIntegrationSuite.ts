import { Signer } from '@solana/web3.js';
import { Controller } from '@uxd-protocol/uxd-client';
import { initializeControllerTest } from '../cases/initializeControllerTest';

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
};
