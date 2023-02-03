import { Controller, UXD_DECIMALS } from '@uxd-protocol/uxd-client';
import { authority, bank, uxdProgramId } from './constants';
import { controllerIntegrationSuite } from './suite/controllerIntegrationSuite';
import { credixLpDepositorySetupSuite } from './suite/credixLpDepositorySetupSuite';
import { identityDepositorySetupSuite } from './suite/identityDepositorySetup';
import { mercurialVaultDepositorySetupSuite } from './suite/mercurialVaultDepositorySetup';

(async () => {
  const controller = new Controller('UXD', UXD_DECIMALS, uxdProgramId);

  beforeEach('\n', function () {
    console.log('=============================================\n\n');
  });

  describe('controllerIntegrationSuite', function () {
    controllerIntegrationSuite({
      authority,
      controller,
      payer: bank,
    });
  });

  const mintingFeeInBps = 0;
  const redeemingFeeInBps = 5;
  const uiRedeemableAmountUnderManagementCap = 1_000;

  describe('mercurialVaultDepositorySetupSuite', function () {
    mercurialVaultDepositorySetupSuite({
      authority,
      controller,
      mintingFeeInBps: 0,
      redeemingFeeInBps: 5,
      redeemableAmountUnderManagementCap: 1_000,
      payer: bank,
    });
  });

  describe('credixLpDepositorySetupSuite', function () {
    credixLpDepositorySetupSuite({
      authority,
      payer: bank,
      controller,
      mintingFeeInBps,
      redeemingFeeInBps,
      uiRedeemableAmountUnderManagementCap,
    });
  });

  describe('identityDepositorySetupSuite', function () {
    identityDepositorySetupSuite({
      authority,
      controller,
      payer: bank,
    });
  });
})();
