import { web3 } from '@project-serum/anchor';
import { amountToUiAmount } from '@solana/spl-token';
import { Keypair, Transaction } from '@solana/web3.js';
import { Controller, UXDClient } from '@uxd-protocol/uxd-client';
import {
  createMercurialVaultDepository,
  getConnection,
  TXN_OPTS,
  uxdProgramId,
} from './common';

async function main() {
  // Dummy payer for mainnet tooling E7N44oZ3APNFjzv95xL6kSxSLgw3wVP3ixM7dgsMApzZ
  const payer = Keypair.fromSeed(
    Uint8Array.from([
      1, 56, 76, 89, 32, 55, 1, 128, 98, 23, 56, 22, 30, 12, 76, 23, 2, 9, 3, 5,
      1, 22, 120, 109, 0, 8, 5, 3, 2, 7, 6, 8,
    ])
  );
  console.log('payer', payer.publicKey.toBase58());

  const controller = new Controller('UXD', 6, uxdProgramId);
  const depository = await createMercurialVaultDepository();

  controller.info();
  depository.info();

  const profitsBeneficiaryCollateral = (
    await depository.getOnchainAccount(getConnection(), TXN_OPTS)
  ).profitsBeneficiaryCollateral;
  const profitsBeneficiaryCollateralAmountBefore = (
    await getConnection().getTokenAccountBalance(profitsBeneficiaryCollateral)
  ).value.uiAmount;
  console.log(
    'profitsBeneficiaryCollateral',
    profitsBeneficiaryCollateral.toBase58()
  );
  console.log(
    'profitsBeneficiaryCollateral amount before',
    profitsBeneficiaryCollateralAmountBefore
  );

  const estimatedProfitsCollectedAmount = await amountToUiAmount(
    getConnection(),
    payer,
    depository.collateralMint.mint,
    (
      await depository.calculateProfitsValue(getConnection(), TXN_OPTS)
    ).toNumber()
  );
  console.log(
    'estimatedProfitsCollectedAmount',
    estimatedProfitsCollectedAmount
  );

  const uxdClient = new UXDClient(uxdProgramId);

  const collectProfitsOfMercurialVaultDepositoryIx =
    uxdClient.createCollectProfitsOfMercurialVaultDepositoryInstruction(
      controller,
      depository,
      profitsBeneficiaryCollateral,
      TXN_OPTS,
      payer.publicKey
    );

  const tx = new Transaction();
  tx.add(collectProfitsOfMercurialVaultDepositoryIx);

  try {
    const txId = await web3.sendAndConfirmTransaction(
      getConnection(),
      tx,
      [payer],
      TXN_OPTS
    );
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}'`);

    const profitsBeneficiaryCollateralAmountAfter = (
      await getConnection().getTokenAccountBalance(profitsBeneficiaryCollateral)
    ).value.uiAmount;
    console.log(
      'profitsBeneficiaryCollateral amount after',
      profitsBeneficiaryCollateralAmountAfter
    );
    console.log(
      'actualProfitsCollectedAmount',
      profitsBeneficiaryCollateralAmountAfter! -
        profitsBeneficiaryCollateralAmountBefore!
    );
  } catch (error) {
    console.log('collectProfits', error);
  }
}

main();
