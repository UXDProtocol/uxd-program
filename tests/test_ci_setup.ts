import {
  IdentityDepository,
  USDC_DECIMALS,
  USDC_DEVNET,
} from '@uxd-protocol/uxd-client';
import { Controller, UXD_DECIMALS } from '@uxd-protocol/uxd-client';
import { authority, bank, uxdProgramId } from './constants';
import { controllerIntegrationSuite } from './suite/controllerIntegrationSuite';
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

  const identityDepository = new IdentityDepository(
    USDC_DEVNET,
    'USDC',
    USDC_DECIMALS,
    uxdProgramId
  );

  describe('identityDepositorySetupSuite', function () {
    identityDepositorySetupSuite({
      authority,
      controller,
      payer: bank,
      depository: identityDepository,
    });
  });
})();
