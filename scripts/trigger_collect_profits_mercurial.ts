import { web3 } from '@project-serum/anchor';
import { amountToUiAmount } from '@solana/spl-token';
import { Transaction } from '@solana/web3.js';
import { UXDClient } from '@uxd-protocol/uxd-client';
import {
  createControllerMainnet,
  createMercurialVaultDepositoryMainnet,
  getConnectionMainnet,
  payer,
  TXN_OPTS,
  uxdProgramIdMainnet,
} from './common';

async function main() {
  const controller = createControllerMainnet();
  const depository = await createMercurialVaultDepositoryMainnet();

  controller.info();
  depository.info();

  const profitsBeneficiaryCollateral = (
    await depository.getOnchainAccount(getConnectionMainnet(), TXN_OPTS)
  ).profitsBeneficiaryCollateral;
  const profitsBeneficiaryCollateralAmountBefore = (
    await getConnectionMainnet().getTokenAccountBalance(
      profitsBeneficiaryCollateral
    )
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
    getConnectionMainnet(),
    payer,
    depository.collateralMint.mint,
    (
      await depository.calculateProfitsValue(getConnectionMainnet(), TXN_OPTS)
    ).toNumber()
  );
  console.log(
    'estimatedProfitsCollectedAmount',
    estimatedProfitsCollectedAmount
  );

  const uxdClient = new UXDClient(uxdProgramIdMainnet);

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
      getConnectionMainnet(),
      tx,
      [payer],
      TXN_OPTS
    );
    console.log(`🔗 'https://explorer.solana.com/tx/${txId}'`);

    const profitsBeneficiaryCollateralAmountAfter = (
      await getConnectionMainnet().getTokenAccountBalance(
        profitsBeneficiaryCollateral
      )
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
