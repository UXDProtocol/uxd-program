import { PublicKey, Signer } from '@solana/web3.js';
import { Controller, MercurialVaultDepository } from '@uxd-protocol/uxd-client';
import { getConnection } from '../connection';
import { collectProfitsOfMercurialVaultDepositoryTest } from '../cases/collectProfitsOfMercurialVaultDepositoryTest';
import { transferLpTokenToDepositoryLpVault } from '../mercurial_vault_utils';
import { editMercurialVaultDepositoryTest } from '../cases/editMercurialVaultDepositoryTest';
import { expect } from 'chai';
import { getOrCreateAssociatedTokenAccount } from '@solana/spl-token';
import { createMercurialVaultDepositoryDevnet } from '../utils';

export const mercurialVaultDepositoryCollectProfitsSuite = async function ({
  authority,
  payer,
  profitsBeneficiary,
  controller,
}: {
  authority: Signer;
  payer: Signer;
  profitsBeneficiary: Signer;
  controller: Controller;
}) {
  const collateralSymbol = 'USDC';

  let depository: MercurialVaultDepository;

  before(
    'Setup: add LP token to mercurial vault depository LP token safe to simulate interests',
    async function () {
      depository = await createMercurialVaultDepositoryDevnet();

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

  describe('Collect profits of mercurial vault depository', () => {
    it(`Set profits beneficiary as empty Public key (All zeroes)`, async () =>
      editMercurialVaultDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          profitsBeneficiaryCollateral: PublicKey.default,
        },
      }));

    it(`Collect profits should fail before initializing profits beneficiary`, async function () {
      try {
        await collectProfitsOfMercurialVaultDepositoryTest({
          controller,
          depository,
          payer,
        });
      } catch {
        expect(true, 'Failing as planned');
      }

      expect(
        false,
        `Should have failed - Cannot collect profits before initializing valid beneficiary`
      );
    });

    it(`Set profits beneficiary before collecting profits`, async function () {
      const profitsBeneficiaryAccountInfo =
        await getOrCreateAssociatedTokenAccount(
          getConnection(),
          payer,
          depository.collateralMint.mint,
          profitsBeneficiary.publicKey
        );
      await editMercurialVaultDepositoryTest({
        authority,
        controller,
        depository,
        uiFields: {
          profitsBeneficiaryCollateral: profitsBeneficiaryAccountInfo.address,
        },
      });
    });

    it(`Collect some ${collateralSymbol} should work`, async () =>
      await collectProfitsOfMercurialVaultDepositoryTest({
        controller,
        depository,
        payer,
      }));
  });
};
