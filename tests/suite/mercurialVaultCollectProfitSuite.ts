import { PublicKey, Signer } from '@solana/web3.js';
import { Controller, MercurialVaultDepository } from '@uxd-protocol/uxd-client';
import { getConnection } from '../connection';
import { collectProfitOfMercurialVaultDepositoryTest } from '../cases/collectProfitOfMercurialVaultDepositoryTest';
import {
  MERCURIAL_USDC_DEVNET,
  MERCURIAL_USDC_DEVNET_DECIMALS,
  uxdProgramId,
} from '../constants';
import { transferLpTokenToDepositoryLpVault } from '../mercurial_vault_utils';
import { editMercurialVaultDepositoryTest } from '../cases/editMercurialVaultDepositoryTest';
import { expect } from 'chai';

export const mercurialVaultDepositoryCollectProfitSuite = async function ({
  authority,
  payer,
  profitsBeneficiaryKey,
  controller,
}: {
  authority: Signer;
  payer: Signer;
  profitsBeneficiaryKey: Signer;
  controller: Controller;
}) {
  const collateralSymbol = 'USDC';
  let depository: MercurialVaultDepository;

  before(
    'Setup: add LP token to mercurial vault depository LP token safe to simulate interests',
    async function () {
      depository = await MercurialVaultDepository.initialize({
        connection: getConnection(),
        collateralMint: {
          mint: MERCURIAL_USDC_DEVNET,
          name: 'USDC',
          symbol: collateralSymbol,
          decimals: MERCURIAL_USDC_DEVNET_DECIMALS,
        },
        uxdProgramId,
      });

      console.log(
        'depository.collateralMint.mint',
        depository.collateralMint.mint.toBase58()
      );
      console.log(
        'depository.collateralMint.decimals',
        depository.collateralMint.decimals
      );

      // Send LP token directly to depository LP token vault to simulate interest
      await transferLpTokenToDepositoryLpVault({
        amount: 0.001,
        depository,
        payer,
      });
    }
  );

  describe('Collect profit of mercurial vault depository', () => {
    it(`Set profit beneficiary as empty Public key (All zeroes)`, () =>
      editMercurialVaultDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          profitsBeneficiaryKey: PublicKey.default,
        },
      }));

    it(`Collect profits should fail before initializing profit beneficiary`, async function () {
      try {
        await collectProfitOfMercurialVaultDepositoryTest({
          controller,
          depository,
          payer,
        });
      } catch {
        expect(true, 'Failing as planned');
      }

      expect(
        false,
        `Should have failed - Cannot redeem for 0 ${controller.redeemableMintSymbol}`
      );
    });

    it(`Set profit beneficiary before collecting profits`, () =>
      editMercurialVaultDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          profitsBeneficiaryKey: profitsBeneficiaryKey.publicKey,
        },
      }));

    it(`Collect some ${collateralSymbol} should work`, () =>
      collectProfitOfMercurialVaultDepositoryTest({
        controller,
        depository,
        payer,
      }));
  });
};
