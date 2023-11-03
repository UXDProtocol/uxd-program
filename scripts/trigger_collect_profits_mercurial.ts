import { web3 } from '@project-serum/anchor';
import { amountToUiAmount } from '@solana/spl-token';
import { Transaction } from '@solana/web3.js';
import { UXDClient } from '@uxd-protocol/uxd-client';
import {
  createController,
  createMercurialVaultDepository,
  getConnection,
  payer,
  TXN_OPTS,
  uxdProgramId,
} from './common';

async function main() {
  const controller = createController();
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
