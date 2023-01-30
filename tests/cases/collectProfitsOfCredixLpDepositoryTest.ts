import { PublicKey, Signer } from '@solana/web3.js';
import {
  Controller,
  CredixLpDepository,
  nativeToUi,
} from '@uxd-protocol/uxd-client';
import { expect } from 'chai';
import { collectProfitsOfCredixLpDepository } from '../api';
import { getConnection, TXN_OPTS } from '../connection';
import { CLUSTER } from '../constants';
import { getBalance } from '../utils';

export const collectProfitsOfCredixLpDepositoryTest = async function (
  payer: Signer,
  profitsBeneficiaryCollateral: PublicKey,
  controller: Controller,
  depository: CredixLpDepository
): Promise<number> {
  console.group('🧭 collectProfitsOfCredixLpDepositoryTest');

  try {
    // GIVEN
    const [
      profitsBeneficiaryCollateralBalance_pre,
      onchainController_pre,
      onChainDepository_pre,
    ] = await Promise.all([
      getBalance(profitsBeneficiaryCollateral),
      controller.getOnchainAccount(getConnection(), TXN_OPTS),
      depository.getOnchainAccount(getConnection(), TXN_OPTS),
    ]);

    // WHEN
    // Simulates user experience from the front end
    const txId = await collectProfitsOfCredixLpDepository(
      payer,
      profitsBeneficiaryCollateral,
      controller,
      depository
    );
    console.log(
      `🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`
    );

    // THEN
    const [
      profitsBeneficiaryCollateralBalance_post,
      onchainController_post,
      onChainDepository_post,
    ] = await Promise.all([
      getBalance(profitsBeneficiaryCollateral),
      controller.getOnchainAccount(getConnection(), TXN_OPTS),
      depository.getOnchainAccount(getConnection(), TXN_OPTS),
    ]);

    const collateralDelta = Number(
      (
        profitsBeneficiaryCollateralBalance_post -
        profitsBeneficiaryCollateralBalance_pre
      ).toFixed(depository.collateralDecimals)
    );

    console.log(
      `🧾 Collected profits:`,
      Number(collateralDelta.toFixed(depository.collateralDecimals)),
      depository.collateralSymbol
    );

    // Check redeemed collateral amount has not decreased lol
    expect(collateralDelta).gte(0);

    // Check depository accounting
    expect(
      nativeToUi(
        onChainDepository_post.profitsTotalCollected,
        depository.collateralDecimals
      )
    ).equal(
      Number(
        (
          nativeToUi(
            onChainDepository_pre.profitsTotalCollected,
            depository.collateralDecimals
          ) + collateralDelta
        ).toFixed(depository.collateralDecimals)
      )
    );

    console.groupEnd();

    return collateralDelta;
  } catch (error) {
    console.error('❌', error);
    console.groupEnd();
    throw error;
  }
};
