import { Signer } from '@solana/web3.js';
import {
  Controller,
  CredixLpDepository,
  findATAAddrSync,
  nativeToUi,
} from '@uxd-protocol/uxd-client';
import { expect } from 'chai';
import { collectProfitOfCredixLpDepository } from '../api';
import { getConnection, TXN_OPTS } from '../connection';
import { CLUSTER } from '../constants';
import { getBalance } from '../utils';

export const collectProfitOfCredixLpDepositoryTest = async function (
  authority: Signer,
  controller: Controller,
  depository: CredixLpDepository,
  payer?: Signer
): Promise<number> {
  console.group('🧭 collectProfitOfCredixLpDepositoryTest');

  const [authorityCollateralAta] = findATAAddrSync(
    authority.publicKey,
    depository.collateralMint
  );

  try {
    // GIVEN
    const [
      authorityCollateralBalance_pre,
      onchainController_pre,
      onChainDepository_pre,
    ] = await Promise.all([
      getBalance(authorityCollateralAta),
      controller.getOnchainAccount(getConnection(), TXN_OPTS),
      depository.getOnchainAccount(getConnection(), TXN_OPTS),
    ]);

    // WHEN
    // Simulates authority experience from the front end
    const txId = await collectProfitOfCredixLpDepository(
      authority,
      payer ?? authority,
      controller,
      depository
    );
    console.log(
      `🔗 'https://explorer.solana.com/tx/${txId}?cluster=${CLUSTER}'`
    );

    // THEN
    const [
      authorityCollateralBalance_post,
      onchainController_post,
      onChainDepository_post,
    ] = await Promise.all([
      getBalance(authorityCollateralAta),
      controller.getOnchainAccount(getConnection(), TXN_OPTS),
      depository.getOnchainAccount(getConnection(), TXN_OPTS),
    ]);

    const collateralDelta = Number(
      (
        authorityCollateralBalance_post - authorityCollateralBalance_pre
      ).toFixed(depository.collateralDecimals)
    );

    console.log(
      `🧾 Collected profit:`,
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
