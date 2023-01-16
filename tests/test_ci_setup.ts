import {
  IdentityDepository,
  USDC_DECIMALS,
  USDC_DEVNET,
} from '@uxd-protocol/uxd-client';
import { Controller, UXD_DECIMALS } from '@uxd-protocol/uxd-client';
import { getConnection } from './connection';
import {
  authority,
  bank,
  SOLEND_USDC_DEVNET,
  SOLEND_USDC_DEVNET_DECIMALS,
  uxdProgramId,
} from './constants';
import {
  controllerIntegrationSuiteParameters,
  controllerIntegrationSuite,
} from './suite/controllerIntegrationSuite';
import { credixLpDepositorySetupSuite } from './suite/credixLpDepositorySetupSuite';
import { identityDepositorySetupSuite } from './suite/identityDepositorySetup';
import { mercurialVaultDepositorySetupSuite } from './suite/mercurialVaultDepositorySetup';
import { createCredixLpDepositoryDevnetUSDC } from './utils';

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

  const credixLpDepository = await createCredixLpDepositoryDevnetUSDC();

  const mintingFeeInBps = 0;
  const redeemingFeeInBps = 5;
  const uiRedeemableDepositorySupplyCap = 1_000;

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
    credixLpDepositorySetupSuite(
      authority,
      bank,
      controller,
      credixLpDepository,
      mintingFeeInBps,
      redeemingFeeInBps,
      uiRedeemableDepositorySupplyCap
    );
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
